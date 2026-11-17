//! `grimoire-mcp-project-docs`: a read-only MCP server for a project's
//! documentation and architecture decision records.
//!
//! - With no command it speaks MCP over stdio, which is how clients start it.
//! - `serve-http` serves Streamable HTTP. It needs either OAuth settings
//!   (`--auth-*`) or an explicit `--no-auth`, which is allowed only on a
//!   loopback address.
//!
//! The project root is `--root`, or else `CLAUDE_PROJECT_DIR`, which Claude Code
//! sets for the servers it starts. The server never guesses a root from its
//! working directory.

mod docs;
mod server;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::ExitCode;

use agent_grimoire_mcp::{AuthConfig, HttpLimits, HttpOptions, HttpServer, log, serve_stdio};
use clap::{Args, Parser, Subcommand};

/// The variable Claude Code sets to the project root for stdio servers (`CC-P3`).
const PROJECT_DIR_VAR: &str = "CLAUDE_PROJECT_DIR";

const EXIT_STATUS: &str = "\
Exit status: 0 stopped cleanly, 1 error, 2 usage error";

#[derive(Debug, Parser)]
#[command(
    name = "grimoire-mcp-project-docs",
    version,
    about = "A read-only MCP server for a project's documentation and ADRs (stdio by default)",
    after_help = EXIT_STATUS
)]
struct Cli {
    #[arg(
        long,
        global = true,
        value_name = "DIR",
        help = "The project root. Defaults to $CLAUDE_PROJECT_DIR"
    )]
    root: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Serve Streamable HTTP instead of stdio.
    ServeHttp(HttpArgs),
}

#[derive(Debug, Args)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent command-line flags"
)]
struct HttpArgs {
    /// The address to listen on.
    #[arg(long, default_value = "127.0.0.1:8931")]
    listen: SocketAddr,
    /// The MCP endpoint's path (with --auth-resource, its path is used).
    #[arg(long, default_value = "/mcp")]
    path: String,
    #[arg(
        long,
        value_name = "ORIGIN",
        help = "A browser origin allowed to call the server, e.g. https://app.example:8443. Repeatable"
    )]
    allowed_origin: Vec<String>,
    /// Another Host header value to answer to, e.g. a reverse proxy's name. Repeatable.
    #[arg(long, value_name = "HOST")]
    allowed_host: Vec<String>,
    /// Serve without authorization. Allowed only on a loopback address.
    #[arg(long, conflicts_with_all = ["auth_resource", "auth_issuer", "auth_jwks"])]
    no_auth: bool,
    #[arg(long, value_name = "URL", requires_all = ["auth_issuer", "auth_jwks"], help = "The canonical URI of this endpoint, e.g. https://docs.example.com/mcp")]
    auth_resource: Option<String>,
    /// The authorization server's issuer; tokens' `iss` must equal it.
    #[arg(long, value_name = "URL", requires = "auth_resource")]
    auth_issuer: Option<String>,
    /// A JWKS file with the issuer's signing keys (RS256 or ES256), reloaded when it changes.
    #[arg(long, value_name = "FILE", requires = "auth_resource")]
    auth_jwks: Option<PathBuf>,
    /// Another accepted token audience, besides --auth-resource. Repeatable.
    #[arg(long, value_name = "AUD", requires = "auth_resource")]
    auth_audience: Vec<String>,
    /// A scope every request needs. Repeatable.
    #[arg(long, value_name = "SCOPE", requires = "auth_resource")]
    auth_scope: Vec<String>,
    /// Also accept access tokens typed `JWT` instead of `at+jwt`.
    #[arg(long, requires = "auth_resource")]
    auth_allow_jwt_typ: bool,
    /// Clock-skew tolerance in seconds (at most 60).
    #[arg(
        long,
        value_name = "SECS",
        default_value_t = 30,
        requires = "auth_resource"
    )]
    auth_leeway: u64,
}

/// The project root: `--root`, else `$CLAUDE_PROJECT_DIR`.
fn root(flag: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(root) = flag {
        return Ok(root);
    }
    match std::env::var_os(PROJECT_DIR_VAR) {
        Some(dir) if !dir.is_empty() => {
            let dir = PathBuf::from(dir);
            if dir.is_absolute() {
                Ok(dir)
            } else {
                Err(format!("${PROJECT_DIR_VAR} must be an absolute path"))
            }
        }
        _ => Err(format!(
            "no project root: pass --root DIR, or start the server from a client that sets ${PROJECT_DIR_VAR}"
        )),
    }
}

fn http_options(args: HttpArgs) -> Result<HttpOptions, String> {
    let auth = match (
        args.no_auth,
        args.auth_resource,
        args.auth_issuer,
        args.auth_jwks,
    ) {
        (true, _, _, _) => None,
        (false, Some(resource), Some(issuer), Some(jwks)) => Some(AuthConfig {
            resource,
            issuer,
            jwks,
            audiences: args.auth_audience,
            required_scopes: args.auth_scope,
            resource_name: Some("Project documentation".to_owned()),
            allow_jwt_typ: args.auth_allow_jwt_typ,
            leeway_secs: args.auth_leeway,
        }),
        _ => {
            return Err(
                "serve-http needs authorization (--auth-resource, --auth-issuer, --auth-jwks) or an explicit --no-auth on a loopback address"
                    .to_owned(),
            );
        }
    };
    Ok(HttpOptions {
        listen: args.listen,
        path: args.path,
        allowed_origins: args.allowed_origin,
        allowed_hosts: args.allowed_host,
        auth,
        limits: HttpLimits::default(),
    })
}

async fn run(cli: Cli) -> Result<(), String> {
    let root = root(cli.root)?;
    let docs = docs::Docs::open(&root)
        .map_err(|e| format!("cannot open the project root {}: {e}", root.display()))?;
    let server = server::DocsServer::new(docs);
    match cli.command {
        None => serve_stdio(server).await.map_err(|e| e.to_string()),
        Some(Command::ServeHttp(args)) => {
            let options = http_options(args)?;
            let http = HttpServer::bind(move || server.clone(), options)
                .await
                .map_err(|e| e.to_string())?;
            if let Ok(addr) = http.local_addr() {
                log(&format!(
                    "grimoire-mcp-project-docs: serving {} on http://{addr}",
                    root.display()
                ));
            }
            http.run().await.map_err(|e| e.to_string())
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            log(&format!("grimoire-mcp-project-docs: {e}"));
            return ExitCode::FAILURE;
        }
    };
    let code = match runtime.block_on(run(cli)) {
        Ok(()) => 0,
        Err(e) => {
            log(&format!("grimoire-mcp-project-docs: {e}"));
            1
        }
    };
    // A read of stdin that cannot be cancelled would otherwise hold the runtime open.
    runtime.shutdown_background();
    std::process::exit(code);
}
