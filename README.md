# Package Reconciler 📦🔄

Package Reconciler is a Rust utility that helps you synchronize installed packages on a fleet of Debian 11 machines. It compares the installed packages of each target server with a reference server and installs any missing packages.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [License](#license)

## Features

- 🌐 Concurrent execution on multiple servers
- 🔧 Easy configuration using a TOML file
- 📊 Real-time progress bars for server tasks
- 🚀 High performance with asynchronous processing

## Requirements

- Rust 1.54.0 or later
- Cargo package manager
- SSH access with pre-installed keys on target servers

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/package_reconciler.git
   ```

2. Navigate to the project directory:

   ```bash
   cd package_reconciler
   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

## Configuration

1. Open the `src/config.toml` file in your favorite text editor.

2. Configure the `user`, `reference_server`, and `max_concurrent_tasks` variables:

   ```toml
   user = "shaun"
   reference_server = "reference.example.com"
   max_concurrent_tasks = 5
   ```

3. Add your target servers to the `servers` list:

   ```toml
   servers = [
     "server1.example.com",
     "server2.example.com",
     # ...
   ]
   ```

## Usage

1. Run the Package Reconciler:

   ```bash
   cargo run
   ```

2. Monitor the progress bars for each server as the utility compares and installs packages.

3. Once all tasks are complete, review the output to ensure that all packages have been successfully installed.

## License

This project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).