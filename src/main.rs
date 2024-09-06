use clap::{Parser, Subcommand};
use std::io;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::thread::sleep;
use std::time::{Duration, Instant};

const SERVER_DEFAULT: &str = "1.1.1.1:443";


#[derive(Parser)]
#[command(name = "netmon")]
#[command(about = "A simple network stability monitoring tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Measure Round-Trip Time (RTT) to a server
    Rtt {
        /// Server to ping (default: cloudflare dns)
        #[arg(default_value = SERVER_DEFAULT)]
        server: String,

        /// Interval in milliseconds between pings
        #[arg(short, long, default_value_t = 1000)]
        interval: u64,

        /// Interval in milliseconds between pings
        #[arg(short, long, default_value_t = 500)]
        timeout: u64,
    },
    Version,
}


// This will set VERSION based on the build mode
#[cfg(debug_assertions)]
pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "--DEBUG");

#[cfg(not(debug_assertions))]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Strips the protocol (http/https) and ensures a port is added if missing
fn format_server_address(server: &str) -> String {
    let mut server = server.replace("https://", "").replace("http://", "");
    if !server.contains(':') {
        // Default to HTTPS port 443 if no port is provided
        server.push_str(":443");
    }
    server
}

fn measure_rtt_w_timeout(server: &str, timeout: u64) -> Result<Duration, io::Error> {
    let formatted_server = format_server_address(server);
    let addr = formatted_server
        .to_socket_addrs()?
        .next()
        .unwrap_or_else(|| {
            println!("Invalid server address: {}\nUsing default address {}", &formatted_server, &SERVER_DEFAULT);
            SocketAddr::V4("1.1.1.1:443".parse().unwrap())
        });
    let start = Instant::now();
    let _ = TcpStream::connect_timeout(&addr, Duration::from_millis(timeout))?;
    let duration = start.elapsed();
    Ok(duration)
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Version => {
            println!("{} version: {}", env!("CARGO_PKG_NAME"), VERSION);
        }
        Commands::Rtt { server, interval, timeout } => {
            let ping_interval = Duration::from_millis(*interval);

            loop {
                match measure_rtt_w_timeout(server, timeout.clone()) {
                    Ok(duration) => println!("RTT to {}: {:?}", server, duration),
                    Err(e) => eprintln!("Failed to ping {}: {}", server, e),
                }
                sleep(ping_interval);
            }
        }
    }
}
