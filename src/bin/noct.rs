use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use home::home_dir;
use nocturne_tools::{
    backend::{configure_monitors, Compositor},
    monitors::Profile,
};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Wayland compositor to use
    #[arg(long)]
    compositor: Option<Compositor>,
    /// Command to use
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Select monitor profiles
    Monitors(MonCmd),
}

#[derive(Args)]
struct MonCmd {
    /// Name of the monitor profile to use
    name: String,
    /// Path to monitor profile configuration file
    #[arg(short = 'c', long, value_parser = parse_path)]
    config: Option<PathBuf>,
}

fn parse_path(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.as_path().exists() {
        Ok(path)
    } else {
        Err(format!("Path supplied does not exist"))
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Monitors(cmd) => set_monitor_profile(
            cmd.name.clone(),
            cmd,
            &cli.compositor.unwrap_or(Compositor::Hyprland),
        ),
    }
}

fn set_monitor_profile(target_profile: String, cmd: &MonCmd, compositor: &Compositor) {
    let config_file_path = match &cmd.config {
        Some(p) => p.clone(),
        None => match env::var("XDG_CONFIG_HOME") {
            Ok(path) => {
                let mut path = PathBuf::from(path);
                path.push("nocturne");
                path.push("monitors.json");
                path
            }
            Err(_) => {
                let mut path = home_dir().expect("could not find home directory");
                path.push(".config");
                path.push("nocturne");
                path.push("monitors.json");
                path
            }
        },
    };

    let mut file_data: Vec<u8> = vec![];
    File::open(config_file_path.as_path())
        .expect(
            format!(
                "unable to find or access config file at {}",
                config_file_path.display()
            )
            .as_str(),
        )
        .read_to_end(&mut file_data)
        .expect(
            format!(
                "unable to read config file at {}",
                config_file_path.display()
            )
            .as_str(),
        );

    let profiles: Vec<Profile> =
        serde_json::from_slice(&file_data.as_slice()).expect("failed to parse json");

    for profile in profiles {
        if profile.name == target_profile {
            println!(
                "Configuring Monitors According to Profile {}.",
                profile.name.bold().green()
            );
            for mon in &profile.monitors {
                println!("{}", mon.to_string());
            }
            configure_monitors(compositor, &profile);
            return;
        } else {
            continue;
        }
    }

    eprintln!("no profile named {} in confiuration", target_profile);
}
