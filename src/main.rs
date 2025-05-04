// Be a perfectionist, no code is good enough!
#![deny(
    clippy::all,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
// Unwraps are a bad practice and do not provide useful error messages/handling.
#![warn(clippy::unwrap_used)]
// This lint happens regardless and is out of our control.
#![allow(clippy::multiple_crate_versions)]

use std::process::{Command, Stdio};

use clap::Parser;
use futures::StreamExt;
use tokio::runtime::Builder;
use tracing::info;

#[derive(serde::Deserialize)]
struct Repo {
    name: String,
    clone_url: String,
}

// CLI struct for command line arguments with clap
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The number of threads (Defaults to the number of cores).
    #[arg(short, long, default_value_t = num_cpus::get())]
    threads: usize,

    /// Whether the target is an organization or not
    #[arg(short, long)]
    org: bool,

    /// Whether to pass `--recursive` to git
    #[arg(short, long)]
    recursive: bool,

    /// Optional output directory
    #[arg(short, long)]
    clone_output: Option<String>,

    /// The target user/organization
    target: String,
}

/// CLI tool to download all repositories from a user or organization on Codeberg asynchronously using multiple threads.
fn main() {
    const THREAD_STACK_SIZE: usize = 3 * 1024 * 1024;

    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    info!(
        "Downloading all repositories for {} with {} threads",
        cli.target, cli.threads
    );

    // If the output directory is provided, create it and change to it
    if let Some(output) = &cli.clone_output {
        std::fs::create_dir_all(output)
            .unwrap_or_else(|_| panic!("Failed to create output directory: \"{output}\""));
        std::env::set_current_dir(output)
            .unwrap_or_else(|_| panic!("Failed to change directory to: \"{output}\""));
    }

    // Create a multi-threaded runtime with the specified number of threads
    let thread_name = "git-clone-worker";
    Builder::new_multi_thread()
        .worker_threads(cli.threads)
        .thread_name("git-clone-worker")
        .thread_stack_size(THREAD_STACK_SIZE)
        .enable_all()
        .build()
        .unwrap_or_else(|_| panic!("Failed to create runtime \"{thread_name}\" with {0} threads and {THREAD_STACK_SIZE} stack size", cli.threads))
        .block_on(async {
            download_repos(cli.target, cli.org, cli.threads, cli.recursive).await;
        });
}

async fn download_repos(target: String, is_org: bool, threads: usize, recursive: bool) {
    let repo_list = get_repos(target, is_org).await;

    // Create a tokio stream of tasks
    let tasks = futures::stream::iter(repo_list.into_iter().map(|repo| {
        let clone_url = repo.clone_url;
        let name = repo.name;
        tokio::spawn(async move {
            let mut cmd = Command::new("git");
            cmd.arg("clone")
                .arg(clone_url)
                .arg(&name)
                .stdout(Stdio::null())
                .stderr(Stdio::null()); // Redirect stderr to null as well
            if recursive {
                cmd.arg("--recursive");
            }
            let status = cmd
                .status()
                .unwrap_or_else(|_| panic!("Failed to execute command: {cmd:?}"));
            if !status.success() {
                eprintln!("Failed to clone {name}");
            }
        })
    }));

    // Use the runtime to execute the tasks concurrently
    let results: Vec<_> = tasks.buffer_unordered(threads).collect().await;
    // Count successful and failed clones
    let (success_count, failure_count) = results.iter().fold((0, 0), |(s, f), result| {
        if result.is_ok() {
            (s + 1, f)
        } else {
            (s, f + 1)
        }
    });
    info!("Cloned {} repositories successfully", success_count);
    if failure_count > 0 {
        info!("Failed to clone {} repositories", failure_count);
    }
}

async fn get_repos(target: String, is_org: bool) -> Vec<Repo> {
    let url = if is_org {
        format!("https://codeberg.org/api/v1/orgs/{target}/repos")
    } else {
        format!("https://codeberg.org/api/v1/users/{target}/repos")
    };

    let client = reqwest::Client::builder()
        .user_agent("GitHub-Repo-Downloader")
        .build()
        .expect("Failed to build HTTP client");

    let response: Vec<Repo> = client
        .get(&url)
        .header("Accept", "application/json")
        .header("User-Agent", "Coderberg-Repo-Hoarder")
        .send()
        .await
        .expect("Failed to send request")
        .json()
        .await
        .expect("Failed to parse JSON response");

    response
}
