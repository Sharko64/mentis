#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;
use std::process::ExitCode;

use hello_trunk::generate_greeting;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

fn main() -> ExitCode {
    init_tracing();

    let name = env::args().nth(1);

    match generate_greeting(name.as_deref()) {
        Ok(message) => {
            info!(message = %message);
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!(error = %e);
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
