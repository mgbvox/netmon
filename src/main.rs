use clap::{Parser, Subcommand};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io;

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

fn measure_rtt(server: &str) -> io::Result<Duration> {
    let formatted_server = format_server_address(server);
    let start = Instant::now();

    // Try to establish a TCP connection
    match TcpStream::connect(&formatted_server) {
        Ok(_) => {
            let duration = start.elapsed();
            Ok(duration)
        }
        Err(e) => Err(e),
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Version => {
            println!("{} version: {}", env!("CARGO_PKG_NAME"), VERSION);
        }
        Commands::Rtt { server, interval } => {
            let ping_interval = Duration::from_millis(*interval);

            loop {
                match measure_rtt(server) {
                    Ok(duration) => println!("RTT to {}: {:?}", server, duration),
                    Err(e) => eprintln!("Failed to ping {}: {}", server, e),
                }
                sleep(ping_interval);
            }
        }
    }
}

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
        /// Server to ping (default: google.com:443)
        #[arg(default_value = "google.com:443")]
        server: String,

        /// Interval in milliseconds between pings
        #[arg(short, long, default_value_t = 1000)]
        interval: u64,
    },
    Version,
}
