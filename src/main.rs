// src/main.rs

mod config;

use config::{Config, load_config};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::Session;
use tokio::task;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let config: Config = load_config();
    let m = MultiProgress::new();
    let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));
    let config: Config = load_config();
    let m = MultiProgress::new();
    let progress_bars: Vec<ProgressBar> = config.servers.iter().map(|_| {
        let pb = m.add(ProgressBar::new(3));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {msg}")
                .progress_chars("#>-"),
        );
        pb
    }).collect();

    let tasks: Vec<_> = config.servers.iter().zip(progress_bars.iter()).map(|(server, pb)| {
      let user = config.user.clone();
      let reference_server = config.reference_server.clone();
      let server = server.clone();
      let pb = pb.clone();
      let permit = Arc::clone(&semaphore).acquire_owned().await;
      task::spawn(async move {
          pb.set_message(format!("Connecting to: {}", server));
          pb.inc(1);
          reconcile_packages(&user, &server, &reference_server, &pb).await;
          pb.finish_with_message(format!("Done: {}", server));
          drop(permit);
      })
  }).collect();

    m.join().unwrap();

    for task in tasks {
        task.await.unwrap();
    }
}

async fn reconcile_packages(user: &str, server: &str, reference_server: &str, pb: &ProgressBar) {
  let reference_packages = get_installed_packages(user, reference_server);
  let server_packages = get_installed_packages(user, server);

  pb.set_message(format!("Comparing packages for: {}", server));
  pb.inc(1);
  let missing_packages: Vec<String> = reference_packages
      .iter()
      .filter(|pkg| !server_packages.contains(pkg))
      .cloned()
      .collect();

  if missing_packages.is_empty() {
      println!("No packages to install on server: {}", server);
  } else {
      pb.set_message(format!("Installing packages on: {}", server));
      pb.inc(1);
      install_packages(user, server, &missing_packages);
  }
}

fn get_installed_packages(user: &str, server: &str) -> Vec<String> {
    let cmd = "dpkg-query -f='${binary:Package}\n' -W";
    let output = execute_remote_command(user, server, cmd);
    output.lines().map(|line| line.to_string()).collect()
}

fn install_packages(user: &str, server: &str, packages: &[String]) {
    let packages_str = packages.join(" ");
    let cmd = format!("sudo apt-get install -y {}", packages_str);
    execute_remote_command(user, server, &cmd);
}

fn execute_remote_command(user: &str, server: &str, cmd: &str) -> String {
  let tcp = TcpStream::connect(format!("{}:22", server)).unwrap();
  let mut session = Session::new().unwrap();
  session.set_tcp_stream(tcp);
  session.handshake().unwrap();
  session.userauth_agent(user).unwrap();

  let mut channel = session.shell().unwrap();
  channel.write_all(cmd.as_bytes()).unwrap();
  channel.send_eof().unwrap();
  channel.wait_close().unwrap();

  let mut output = String::new();
  channel.read_to_string(&mut output).unwrap();
  output
}
