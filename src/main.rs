
use regex::Regex;
use std::{
    io::{self, BufRead, BufReader, Write},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

fn field(pattern: &str, line: &str) -> Option<String> {
    Regex::new(pattern)
        .ok()?
        .captures(line)?
        .get(1)
        .map(|m| m.as_str().to_string())
}

fn number(pattern: &str, line: &str) -> u64 {
    field(pattern, line)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

fn bar(value: f64, width: usize) -> String {
    let pct = value.clamp(0.0, 100.0);
    let filled = (pct * width as f64 / 100.0).round() as usize;

    format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(width - filled)
    )
}

fn memory(label: &str, pattern: &str, line: &str) {
    let re = Regex::new(pattern).unwrap();

    if let Some(c) = re.captures(line) {
        let used: f64 = c[1].parse().unwrap_or(0.0);
        let total: f64 = c[2].parse().unwrap_or(0.0);

        let pct = if total > 0.0 {
            used / total * 100.0
        } else {
            0.0
        };

        println!(
            "{label:<6} {} {:5.1}/{:.1} GiB",
            bar(pct, 25),
            used / 1024.0,
            total / 1024.0
        );
    }
}

fn render(line: &str) {
    let gpu = number(r"GR3D_FREQ (\d+)%", line);
    let emc = number(r"EMC_FREQ (\d+)%", line);

    let temp = field(r"tj@([\d.]+)C", line)
        .unwrap_or_else(|| "--".into());

    let gpu_temp = field(r"gpu@([\d.]+)C", line)
        .unwrap_or_else(|| "--".into());

    let gpu_power = field(r"VDD_GPU_SOC (\d+)mW", line)
        .and_then(|s| s.parse::<f64>().ok());

    let cpu_power = field(r"VDD_CPU_CV (\d+)mW", line)
        .and_then(|s| s.parse::<f64>().ok());

    // Move cursor home, rather than clearing entire screen
    print!("\x1b[H");

    println!("╔══════════════════════════════════════════════════════╗");
    println!("║             JETOP  •  NVIDIA JETSON                  ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    memory("RAM", r"RAM (\d+)/(\d+)MB", line);
    memory("SWAP", r"SWAP (\d+)/(\d+)MB", line);

    println!("GPU    {} {:3}%", bar(gpu as f64, 25), gpu);
    println!("EMC    {} {:3}%", bar(emc as f64, 25), emc);
    println!();

    println!("Temp   SoC: {temp}°C    GPU: {gpu_temp}°C");

    match (gpu_power, cpu_power) {
        (Some(g), Some(c)) => {
            println!(
                "Power  GPU+SoC: {:.2}W    CPU+CV: {:.2}W",
                g / 1000.0,
                c / 1000.0
            );
        }
        _ => println!("Power  --"),
    }

    println!();

    if let Some(cpu) = field(r"CPU \[([^\]]+)\]", line) {
        println!("CPU CORES");

        for (i, core) in cpu.split(',').enumerate() {
            let core = core.trim();

            if core == "off" {
                println!("  CPU {i:02}  OFF");
                continue;
            }

            let re = Regex::new(r"(\d+)%@(\d+)").unwrap();

            if let Some(c) = re.captures(core) {
                let pct: u64 = c[1].parse().unwrap_or(0);
                let freq = &c[2];

                println!(
                    "  CPU {i:02}  {} {:3}%  {} MHz",
                    bar(pct as f64, 16),
                    pct,
                    freq
                );
            }
        }
    }

    println!();
    println!("Ctrl+C to quit | Refresh: 1s");

    // Clear any remaining lines from previous frame
    print!("\x1b[J");

    let _ = io::stdout().flush();
}

fn main() -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let flag = Arc::clone(&running);

    ctrlc::set_handler(move || {
        flag.store(false, Ordering::SeqCst);
    })
    .expect("Failed to install Ctrl+C handler");

    let mut child = Command::new("tegrastats")
        .args(["--interval", "1000"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    // Enter alternate screen and hide cursor
    print!("\x1b[?1049h\x1b[?25l");
    io::stdout().flush()?;

    let result = (|| -> io::Result<()> {
        for line in reader.lines() {
            if !running.load(Ordering::SeqCst) {
                break;
            }

            let line = line?;

            if line.contains("RAM ") {
                render(&line);
            }
        }
        Ok(())
    })();

    // Always clean up the child and terminal
    let _ = child.kill();
    let _ = child.wait();

    print!("\x1b[?25h\x1b[?1049l");
    io::stdout().flush()?;

    result
}

