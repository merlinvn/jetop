# Jetop 🚀

**A lightweight terminal-based system monitor for NVIDIA Jetson devices, written in Rust.**

Jetop provides real-time monitoring of CPU, GPU, RAM, SWAP, temperatures, and power consumption using NVIDIA's `tegrastats` utility.

Designed to be simple, fast, and resource-efficient, Jetop runs as a native executable without requiring Python or a virtual environment.

## Features

- **CPU Monitoring** — Per-core utilization and clock frequency
- **GPU Monitoring** — GPU utilization (GR3D)
- **Memory Monitoring** — RAM and SWAP usage with progress bars
- **EMC Monitoring** — External Memory Controller utilization
- **Temperature Monitoring** — SoC and GPU temperatures
- **Power Monitoring** — GPU/SoC and CPU/CV power consumption
- **Real-Time Updates** — Refreshes every second
- **Lightweight** — Native Rust binary with minimal dependencies
- **Terminal Friendly** — Works over SSH and local terminals
- **Clean Exit** — Handles Ctrl+C and terminates the monitoring process

## Supported Platforms

Jetop is designed for NVIDIA Jetson devices running Jetson Linux / JetPack with `tegrastats` available.

| Platform | Status |
|---|---|
| Jetson AGX Orin | Tested |
| Jetson Orin NX | Expected to work |
| Jetson Orin Nano | Expected to work |
| Jetson AGX Xavier | Expected to work |
| Jetson Xavier NX | Expected to work |
| Other Jetson devices | Not yet tested |

**Note:** Different Jetson models and JetPack versions may expose different telemetry fields. Some metrics may not be available on all devices.

## Requirements

- NVIDIA Jetson device
- Jetson Linux / JetPack
- `tegrastats`
- Rust toolchain (for building from source)

Check whether `tegrastats` is available:

```bash
command -v tegrastats
```

## Installation

### 1. Install Rust

The recommended method is using [mise](https://mise.jdx.dev/):

```bash
mise use -g rust@stable
```

Verify:

```bash
rustc --version
cargo --version
```

Alternatively, install Rust using the official rustup installer:

https://rustup.rs/

### 2. Install build dependencies

On Ubuntu / Jetson Linux:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config
```

### 3. Clone the repository

```bash
git clone https://github.com/merlinvn/jetop.git
cd jetop
```

### 4. Build

```bash
cargo build --release
```

The resulting executable will be located at:

```text
target/release/jetop
```

### 5. Install

```bash
mkdir -p ~/.local/bin
install -m 755 target/release/jetop ~/.local/bin/jetop
```

Make sure `~/.local/bin` is in your `PATH`.

### 6. Run

```bash
jetop
```

Press `Ctrl+C` to exit.

## Example Output

```text
╔══════════════════════════════════════════════════════╗
║             JETOP  •  NVIDIA JETSON                  ║
╚══════════════════════════════════════════════════════╝

RAM    ████░░░░░░░░░░░░░░░░░░░░░   4.9/29.9 GiB
GPU    ░░░░░░░░░░░░░░░░░░░░░░░░░   0%
EMC    ░░░░░░░░░░░░░░░░░░░░░░░░░   0%

Temp   SoC: 46.718°C    GPU: 41.625°C
Power  GPU+SoC: 4.81W    CPU+CV: 1.60W

CPU CORES
  CPU 00  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 01  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 02  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 03  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 04  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 05  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 06  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 07  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 08  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 09  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 10  ░░░░░░░░░░░░░░░░   0%  2201 MHz
  CPU 11  ░░░░░░░░░░░░░░░░   0%  2201 MHz

Ctrl+C to quit | Refresh: 1s
```

*Illustrative output. Actual readings depend on the device and workload.*

## How It Works

Jetop launches NVIDIA's `tegrastats` utility and reads its output at one-second intervals.

The Rust application parses the telemetry values and displays them in a terminal dashboard using Unicode progress bars and ANSI escape sequences.

```text
       NVIDIA Jetson
             |
             v
        tegrastats
             |
             v
      Rust telemetry parser
             |
             v
       Terminal dashboard
             |
             v
       Refresh every 1s
```

Jetop does not modify GPU clocks, power modes, fan settings, or system configuration.

## Dependencies

Jetop uses only two external Rust crates:

- `regex` — Parses telemetry data from `tegrastats`.
- `ctrlc` — Handles shutdown signals.

All other functionality is implemented using the Rust standard library.

## Limitations

- Requires NVIDIA `tegrastats`.
- Currently supports a fixed one-second refresh interval.
- GPU process-level memory utilization is not available.
- Telemetry parsing depends on the output format of `tegrastats`.
- Some temperature and power fields may differ between Jetson models.
- The current version uses ANSI terminal rendering rather than a full TUI framework.

## Roadmap

Potential improvements for future releases:

- [ ] Full terminal UI with Ratatui
- [ ] CPU and GPU utilization history graphs
- [ ] GPU and EMC clock monitoring
- [ ] Configurable refresh interval
- [ ] JetPack and CUDA version information
- [ ] Jetson power mode (`nvpmodel`) information
- [ ] Fan speed monitoring
- [ ] Additional power rail monitoring
- [ ] JSON and CSV telemetry export
- [ ] System process monitoring
- [ ] More robust parsing across Jetson platforms

## Contributing

Contributions, bug reports, and feature requests are welcome!

If you encounter an issue on a specific Jetson device, please include:

- Jetson model
- JetPack / Jetson Linux version
- A sample of your `tegrastats` output
- Description of the problem

Pull requests are welcome.

## License

**MIT License**

Copyright (c) 2026 Jetop Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## Acknowledgments

- NVIDIA for the Jetson platform and `tegrastats` utility
- The Rust community for its excellent tooling and ecosystem

---

**Jetop is an independent open-source project and is not affiliated with or endorsed by NVIDIA Corporation.**

NVIDIA and Jetson are trademarks of NVIDIA Corporation.
