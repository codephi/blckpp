use clap::Parser;
use libc;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path;

#[derive(Debug)]
struct Process {
    pub pid: i32,
    pub comm: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Settings {
    sleep: u64,
    blocked: Vec<String>,
    active: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    sleep: Option<u64>,

    #[clap(short, long)]
    blocked: Option<String>,

    #[clap(short, long)]
    active: Option<bool>,

    #[clap(long)]
    settings: bool,
}

fn list_processes() -> Vec<Process> {
    let mut processes = Vec::new();

    if let Ok(dir) = std::fs::read_dir("/proc") {
        for entry in dir.filter_map(Result::ok) {
            if let Ok(pid) = entry.file_name().to_string_lossy().parse::<i32>() {
                if let Ok(comm) = File::open(format!("/proc/{}/comm", pid))
                    .map(BufReader::new)
                    .and_then(|mut r| {
                        let mut s = String::new();
                        r.read_line(&mut s).map(|_| s)
                    })
                {
                    processes.push(Process {
                        pid,
                        comm: comm.trim().to_string(),
                    });
                }
            }
        }
    }

    processes
}

fn is_blocked(process: &Process, blocked: &[String]) -> bool {
    blocked.iter().any(|b| process.comm.contains(b))
}

fn kill_process(process: &Process) {
    println!("Killing process {} ({})", process.pid, process.comm);
    unsafe {
        libc::kill(process.pid, libc::SIGKILL);
    }
}

fn sleep(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

fn import_yaml_config(path: &path::Path) -> Result<Settings, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let settings: Settings = serde_yaml::from_reader(file)?;
    Ok(settings)
}

fn save_yaml_config(
    path: &path::Path,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    serde_yaml::to_writer(file, settings)?;
    Ok(())
}

fn merge_config(settings: &mut Settings, new_settings: &Args) {
    let mut update = false;

    if new_settings.sleep.is_some() {
        update = true;
        settings.sleep = new_settings.sleep.unwrap();
    }

    if new_settings.blocked.is_some() {
        update = true;
        settings.blocked = new_settings
            .blocked
            .clone()
            .unwrap()
            .split(",")
            .map(|a| a.to_string())
            .collect::<Vec<_>>();
    }

    if new_settings.active.is_some() {
        update = true;
        settings.active = new_settings.active.clone().unwrap();
    }

    if update {
        println!("Settings updated");
    }
}

fn listen_process(default_settings: Settings) {
    let mut default_active: bool = default_settings.active;

    println!("Listening for processes...");

    loop {
        let settings = match load_file_config() {
            Ok(settings) => settings,
            Err(_) => {
                println!("Failed to load config, using default");
                default_settings.clone()
            }
        };

        if settings.active != default_active {
            default_active = settings.active;

            if settings.active {
                println!("Process is now active");
            } else {
                println!("Process is now inactive");
            }
        }

        if settings.active {
            let processes = list_processes();

            for process in processes
                .iter()
                .filter(|p| is_blocked(p, &settings.blocked))
            {
                kill_process(process);
            }
        }

        sleep(settings.sleep);
    }
}

fn load_file_config() -> Result<Settings, Box<dyn std::error::Error>> {
    match import_yaml_config(path::Path::new("/etc/blckpp/config")) {
        Ok(settings) => Ok(settings),
        Err(err) => Err(err),
    }
}

fn load_and_update_config(args: &Args) -> Settings {
    let mut settings: Settings = match load_file_config() {
        Ok(settings) => settings,
        Err(err) => panic!("Failed to load config: {}", err),
    };

    merge_config(&mut settings, &args);

    save_yaml_config(path::Path::new("settings.yaml"), &settings)
        .unwrap_or_else(|e| panic!("Failed to save config: {}", e));

    settings
}

fn i_selt_in_exeution() -> bool {
    let processes = list_processes();

    if processes
        .iter()
        .filter(|p| p.comm.contains("blckpp"))
        .count()
        > 1
    {
        return true;
    }

    false
}

fn main() {
    let args = Args::parse();

    let default_settings = load_and_update_config(&args);

    if args.settings {
        return;
    } else if i_selt_in_exeution() {
        println!("blckpp is already running");
        return;
    }

    listen_process(default_settings);
}
