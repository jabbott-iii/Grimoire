//! The stdio transport: newline-delimited JSON-RPC on stdin and stdout.
//!
//! Nothing but MCP messages is ever written to stdout (`MCP-SPEC`); this crate
//! logs to stderr only. The server stops when the client closes stdin, or on
//! `SIGINT`/`SIGTERM`, which is how Claude Code stops a stdio server.

use rmcp::ServerHandler;
use rmcp::ServiceExt as _;
use rmcp::service::ServerInitializeError;

use crate::{ServeError, shutdown_signal};

/// Serves `server` on stdin and stdout until the client disconnects or the
/// process is asked to stop.
///
/// A client that closes stdin before sending anything is a clean exit.
///
/// # Errors
///
/// Fails if the protocol exchange fails.
pub async fn serve_stdio<S: ServerHandler>(server: S) -> Result<(), ServeError> {
    let starting = server.serve(rmcp::transport::stdio());
    tokio::select! {
        started = starting => match started {
            Ok(running) => tokio::select! {
                quit = running.waiting() => quit.map(|_| ()).map_err(|e| ServeError(e.to_string())),
                // Dropping the running service cancels it.
                () = shutdown_signal() => Ok(()),
            },
            Err(ServerInitializeError::ConnectionClosed(_)) => Ok(()),
            Err(e) => Err(ServeError(e.to_string())),
        },
        () = shutdown_signal() => Ok(()),
    }
}
