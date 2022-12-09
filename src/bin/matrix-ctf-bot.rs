use clap::Parser;
use matrix_ctf_bot::{matrix, Config};
use miette::{miette, IntoDiagnostic, WrapErr};
use tracing::{debug, info, trace};

use std::env;

mod cli {
    use std::path::PathBuf;

    use clap::Parser;

    #[derive(Parser, Debug)]
    pub struct Args {
        /// Path to the configuration file to load.
        #[arg(short, long)]
        pub config: Option<PathBuf>,
    }
}
// use matrix_ctf_bot::matrix

fn read_env_var(key: &str) -> miette::Result<String> {
    env::var(key).map_err(|_| miette::miette!("Missing expected environment variable `{}'", key))
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let args = cli::Args::parse();

    let homeserver = read_env_var("MATRIX_HOMESERVER")?;

    // Use the specified configuration file or attempt to open one from the xdg config directory.
    let config_path = match args.config {
        Some(path) => path,
        None => {
            let config_dir = dirs::config_dir()
                .ok_or_else(|| miette!("no config dir"))?
                .join("matrix-ctf-bot");

            config_dir.join("config.toml")
        }
    };

    trace!(?config_path, "Loading configuration file");
    let config = matrix_ctf_bot::config::load(config_path).into_diagnostic()?;
    trace!(?config, "Configuration loaded");

    let username = read_env_var("MATRIX_USERNAME")?;
    let password = read_env_var("MATRIX_PASSWORD")?;

    info!("Initializing state");
    let state = matrix_ctf_bot::State::with_config(homeserver, username, password, config);

    let _ = matrix_ctf_bot::matrix::login_and_sync(state.clone()).await;

    Ok(())
}
