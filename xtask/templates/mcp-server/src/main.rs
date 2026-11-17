//! `@@BINARY@@`: one line saying what this MCP server does.
//!
//! With no command it speaks MCP over stdio, which is how clients start it.
//! `serve-http --no-auth` serves Streamable HTTP on a loopback address, and
//! the flag is required so that serving without authorization is a choice;
//! see `catalog/mcp-servers/project-docs` for the OAuth options.

use std::borrow::Cow;
use std::net::SocketAddr;
use std::process::ExitCode;

use agent_grimoire_mcp::limits::{LimitConfig, Limits};
use agent_grimoire_mcp::{
    HttpLimits, HttpOptions, HttpServer, PROTOCOL_VERSIONS, log, serve_stdio,
};
use clap::{Parser, Subcommand};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::{Json, Parameters};
use rmcp::model::{Implementation, ProtocolVersion, ServerCapabilities, ServerConfig};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(
    name = "@@BINARY@@",
    version,
    about = "One line saying what this MCP server does (stdio by default)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Serve Streamable HTTP on a loopback address, without authorization.
    ServeHttp {
        /// Serve without authorization (required; loopback addresses only).
        #[arg(long, required = true)]
        no_auth: bool,
        /// The address to listen on.
        #[arg(long, default_value = "127.0.0.1:8932")]
        listen: SocketAddr,
    },
}

#[derive(Clone)]
struct Server {
    limits: Limits,
    tool_router: ToolRouter<Server>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EchoArgs {
    /// The text to return.
    text: String,
}

#[derive(Debug, Serialize, JsonSchema)]
struct Echoed {
    /// The text that was sent.
    text: String,
}

#[tool_router]
impl Server {
    fn new() -> Self {
        Self {
            limits: Limits::new(LimitConfig::default()),
            tool_router: Self::tool_router(),
        }
    }

    /// Return the text unchanged.
    #[tool(annotations(
        title = "Echo",
        read_only_hint = true,
        destructive_hint = false,
        idempotent_hint = true,
        open_world_hint = false
    ))]
    async fn echo(&self, Parameters(args): Parameters<EchoArgs>) -> Result<Json<Echoed>, String> {
        // Blocking work goes through the limits: concurrency, rate and time.
        self.limits
            .run(move || Echoed { text: args.text })
            .await
            .map(Json)
            .map_err(|refused| refused.to_string())
    }
}

#[tool_handler(router = self.tool_router)]
#[expect(
    clippy::unused_async_trait_impl,
    reason = "rmcp's generated handler methods are async"
)]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(
            Implementation::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        )
    }

    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(&PROTOCOL_VERSIONS)
    }
}

async fn run(cli: Cli) -> Result<(), String> {
    let server = Server::new();
    match cli.command {
        None => serve_stdio(server).await.map_err(|e| e.to_string()),
        Some(Command::ServeHttp { listen, .. }) => {
            let options = HttpOptions {
                listen,
                path: "/mcp".to_owned(),
                allowed_origins: Vec::new(),
                allowed_hosts: Vec::new(),
                auth: None,
                limits: HttpLimits::default(),
            };
            let http = HttpServer::bind(move || server.clone(), options)
                .await
                .map_err(|e| e.to_string())?;
            http.run().await.map_err(|e| e.to_string())
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let Ok(runtime) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    else {
        log("@@BINARY@@: cannot start the async runtime");
        return ExitCode::FAILURE;
    };
    let code = match runtime.block_on(run(cli)) {
        Ok(()) => 0,
        Err(e) => {
            log(&format!("@@BINARY@@: {e}"));
            1
        }
    };
    // A read of stdin that cannot be cancelled would otherwise hold the runtime open.
    runtime.shutdown_background();
    std::process::exit(code);
}
