use libc::c_int;
use std::env;
use jsonrpc_core::IoHandler as JsonRpcCoreIoHandler;
use jsonrpc_core::Params as JsonRpcCoreParams;
use jsonrpc_http_server::ServerBuilder as ServerBuilderHttp;
use serde_json::json;
use signal_hook::consts::signal::*;

use std::thread;
use std::io::Error;
use serde_json;


#[cfg(not(feature = "extended-siginfo"))]
use signal_hook::iterator::Signals;

fn main() -> Result<(), Error>  {

    const SIGNALS: &[c_int] = &[SIGTERM];
    let mut sigs = Signals::new(SIGNALS)?;

    thread::spawn(move || {
        for signal in &mut sigs {
            eprintln!("Received signal {:?}", signal);
            std::process::exit(0);
        }
    });

    let addr = env::var("USERCODE_PROXY_ADDR").expect("$USERCODE_PROXY_ADDR is not set");

    let mut io = JsonRpcCoreIoHandler::default();
    io.add_method("Usercode.Run", |_params: JsonRpcCoreParams| {
        let contacts = json!({
            "rust": "Hello, world!"
        });
        Ok(contacts)
    });
    let server = ServerBuilderHttp::new(io)
        .start_http(&addr.parse().unwrap())
        .expect("Server must start with no issues.");
    server.wait();

    Ok(())
}
