extern crate zinit;

use anyhow::Result;
use clap::{Parser, Subcommand};
use git_version::git_version;

use tokio_stream::StreamExt;
use zinit::app;

const GIT_VERSION: &str = git_version!(args = ["--tags", "--always", "--dirty=-modified"]);

#[derive(Parser, Debug)]
#[command(name = "zinit", author = "ThreeFold Tech, https://github.com/threefoldtech", version = GIT_VERSION, about = "A runit replacement")]
struct Cli {
    #[arg(
        short = 's',
        long = "socket",
        default_value = "/tmp/zinit.sock",
        value_name = "SOCKET",
        help = "path to unix socket"
    )]
    socket: String,

    #[arg(short = 'd', long = "debug", help = "run in debug mode")]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init {
        #[arg(
            short = 'c',
            long = "config",
            value_name = "DIR",
            help = "service configurations directory"
        )]
        config: Option<String>,
        #[arg(
            short = 'b',
            long = "buffer",
            value_name = "BUFFER",
            default_value_t = 2000,
            help = "buffer size (in lines) to keep services logs"
        )]
        buffer: usize,
        #[arg(long = "container", help = "run in container mode, shutdown on signal")]
        container: bool,
    },
    List,
    Shutdown,
    Reboot,
    Status {
        service: String,
    },
    Stop {
        service: String,
    },
    Start {
        service: String,
    },
    Forget {
        service: String,
    },
    Monitor {
        service: String,
    },
    Log {
        #[arg(
            short = 's',
            long = "snapshot",
            help = "if set log prints current buffer without following"
        )]
        snapshot: bool,
        filter: Option<String>,
    },
    Kill {
        service: String,
        #[arg(default_value = "SIGTERM")]
        signal: String,
    },
    Restart {
        service: String,
    },
    Stats {
        service: String,
    },
    Proxy {
        #[arg(
            short = 'a',
            long = "address",
            default_value = "127.0.0.1:8080",
            value_name = "ADDRESS"
        )]
        address: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let socket = cli.socket.as_str();
    let debug = cli.debug;

    let config_path = match &cli.command {
        Some(Commands::Init {
            config: Some(cfg), ..
        }) => cfg.clone(),
        _ => {
            #[cfg(target_os = "macos")]
            {
                let home_dir = dirs::home_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
                home_dir
                    .join("hero")
                    .join("cfg")
                    .join("zinit")
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid path for config directory"))?
                    .to_string()
            }
            #[cfg(not(target_os = "macos"))]
            {
                "/tmp/zinit/".to_string()
            }
        }
    };

    let result = match &cli.command {
        Some(Commands::Init {
            buffer, container, ..
        }) => {
            let _server = app::init(*buffer, &config_path, socket, *container, debug).await?;
            tokio::signal::ctrl_c().await?;
            Ok(())
        }
        Some(Commands::List) | None => app::list(socket).await,
        Some(Commands::Shutdown) => app::shutdown(socket).await,
        Some(Commands::Reboot) => app::reboot(socket).await,
        Some(Commands::Status { service }) => app::status(socket, service.clone()).await,
        Some(Commands::Stop { service }) => app::stop(socket, service.clone()).await,
        Some(Commands::Start { service }) => app::start(socket, service.clone()).await,
        Some(Commands::Forget { service }) => app::forget(socket, service.clone()).await,
        Some(Commands::Monitor { service }) => app::monitor(socket, service.clone()).await,
        Some(Commands::Kill { service, signal }) => {
            app::kill(socket, service.clone(), signal.clone()).await
        }
        Some(Commands::Log { snapshot, filter }) => {
            let mut stream = app::logs(socket, filter.clone(), !*snapshot).await?;

            loop {
                tokio::select! {
                    item = stream.next() => {
                        match item {
                            Some(log_entry) => {
                                println!("{log_entry}");
                            },
                            None => break
                        }
                    }
                    _ = tokio::signal::ctrl_c() => {
                        break
                    }
                }
            }

            Ok(())
        }
        Some(Commands::Restart { service }) => app::restart(socket, service.clone()).await,
        Some(Commands::Stats { service }) => app::stats(socket, service.clone()).await,
        Some(Commands::Proxy { address }) => app::proxy(socket, address.clone()).await,
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("{e:#}");
            std::process::exit(1);
        }
    }
}
