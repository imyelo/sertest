use anyhow::{Context, Result};
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Serial port path
    #[arg(short, long, default_value = "/dev/ttyUSB0")]
    port: String,

    /// Baud rate
    #[arg(short, long, default_value_t = 9600)]
    baud: u32,

    /// Send interval (milliseconds)
    #[arg(short, long, default_value_t = 100)]
    interval: u64,

    /// Data to send
    #[arg(short, long, default_value = "PING")]
    data: String,

    /// Data encoding type (utf8 or hex)
    #[arg(short, long, default_value = "utf8")]
    encoding: String,

    /// Number of times to repeat sending (infinite loop if not specified)
    #[arg(short, long)]
    count: Option<u32>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.encoding != "utf8" && args.encoding != "hex" {
        anyhow::bail!("Encoding type must be 'utf8' or 'hex'");
    }

    let data = if args.encoding == "hex" {
        hex::decode(&args.data).context("Invalid hex data")?
    } else {
        args.data.into_bytes()
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\nGracefully stopping the script...");
        r.store(false, Ordering::SeqCst);
    })?;

    let mut port = serialport::new(&args.port, args.baud)
        .timeout(Duration::from_millis(1000))
        .open()
        .context("Unable to open serial port")?;

    println!(
        "Serial port opened {}, baud rate: {}",
        args.port, args.baud
    );
    println!(
        "Sending data every {}ms: \"{}\", ({} encoding)",
        args.interval,
        hex::encode(&data),
        args.encoding
    );

    if let Some(count) = args.count {
        println!("Will send {} times", count);
    } else {
        println!("Will send indefinitely");
    }

    let mut sent_count = 0;
    while running.load(Ordering::SeqCst) {
        port.write_all(&data)
            .context("Error writing to serial port")?;
        sent_count += 1;

        if let Some(count) = args.count {
            if sent_count >= count {
                println!("Completed sending {} times", sent_count);
                break;
            }
        }

        std::thread::sleep(Duration::from_millis(args.interval));
    }

    println!("Serial port closed.");
    Ok(())
} 