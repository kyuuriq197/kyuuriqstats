use colored::*;
use std::env;
use std::path::Path;
use std::process::Command;
use sysinfo::{Disks, System};

fn get_desktop_environment() -> String {
    let vars = [
        "XDG_CURRENT_DESKTOP",
        "XDG_SESSION_DESKTOP",
        "DESKTOP_SESSION",
    ];

    for var in vars {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() {
                return value;
            }
        }
    }

    "Unknown".to_string()
}

fn get_display() -> String {
    let output = Command::new("niri")
        .args(["msg", "outputs"])
        .output();

    match output {
        Ok(output) => {
            let text = String::from_utf8_lossy(&output.stdout);

            let mut monitor = "Unknown".to_string();
            let mut resolution = "Unknown".to_string();
            let mut refresh_rate = "Unknown".to_string();

            for line in text.lines() {
                let line = line.trim();

                // Monitor name
                if line.starts_with("Output \"") {
                    if let Some(name) = line.strip_prefix("Output \"") {
                        if let Some(name) = name.split('"').next() {
                            monitor = name
                                .replace("Microstep ", "")
                                .split(" CA8A402501940")
                                .next()
                                .unwrap_or(name)
                                .to_string();
                        }
                    }
                }

                // Current resolution and refresh rate
                if line.starts_with("Current mode:") {
                    if let Some(value) = line.strip_prefix("Current mode:") {
                        let value = value.trim();

                        if let Some((res, hz)) = value.split_once(" @ ") {
                            resolution = res.to_string();

                            if let Some(hz) = hz.strip_suffix(" Hz") {
                                if let Ok(hz) = hz.parse::<f64>() {
                                    refresh_rate = format!("{:.0}Hz", hz);
                                }
                            }
                        }
                    }
                }
            }

            format!(
                "{} — {} @ {}",
                monitor, resolution, refresh_rate
            )
        }

        Err(_) => "Unknown".to_string(),
    }
}

fn get_gpu() -> String {
    // NVIDIA
    if let Ok(output) = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name",
            "--format=csv,noheader",
        ])
        .output()
    {
        if output.status.success() {
            let gpu = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();

            if !gpu.is_empty() {
                return gpu;
            }
        }
    }

    // AMD / Intel / other GPUs
    if let Ok(output) = Command::new("lspci").output() {
        let text = String::from_utf8_lossy(&output.stdout);

        for line in text.lines() {
            let lower = line.to_lowercase();

            if lower.contains("vga compatible controller")
                || lower.contains("3d controller")
                || lower.contains("display controller")
            {
                if let Some((_, gpu)) = line.split_once(": ") {
                    return gpu.to_string();
                }
            }
        }
    }

    "Unknown".to_string()
}

fn get_terminal() -> String {
    if let Ok(term) = env::var("TERM_PROGRAM") {
        if !term.is_empty() {
            return term;
        }
    }

    if let Ok(term) = env::var("TERM") {
        if !term.is_empty() {
            return term;
        }
    }

    "Unknown".to_string()
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!(
        "{}",
        r#"
██╗  ██╗██╗   ██╗██╗   ██╗██╗   ██╗██████╗ ██╗ ██████╗ ███████╗████████╗ █████╗ ████████╗███████╗
██║ ██╔╝╚██╗ ██╔╝██║   ██║██║   ██║██╔══██╗██║██╔═══██╗██╔════╝╚══██╔══╝██╔══██╗╚══██╔══╝██╔════╝
█████╔╝  ╚████╔╝ ██║   ██║██║   ██║██████╔╝██║██║   ██║███████╗   ██║   ███████║   ██║   ███████╗
██╔═██╗   ╚██╔╝  ██║   ██║██║   ██║██╔══██╗██║██║   ██║╚════██║   ██║   ██╔══██║   ██║   ╚════██║
██║  ██╗   ██║   ╚██████╔╝╚██████╔╝██║  ██║██║╚██████╔╝███████║   ██║   ██║  ██║   ██║   ███████║
╚═╝  ╚═╝   ╚═╝    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝   ╚══════╝
"#
        .blue()
    );

    println!("{}", "kyuuriqstars".blue().bold());

    // OS
    println!(
        "{} {}",
        "OS:".white().bold(),
        System::name()
            .unwrap_or("Unknown".into())
            .blue()
    );

    // Kernel
    println!(
        "{} {}",
        "Kernel:".white().bold(),
        System::kernel_version()
            .unwrap_or("Unknown".into())
            .blue()
    );

    // CPU
    if let Some(cpu) = sys.cpus().first() {
        println!(
            "{} {}",
            "CPU:".white().bold(),
            cpu.brand().blue()
        );
    }

    // RAM
    let ram_used = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let ram_total = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    println!(
        "{} {} / {}",
        "RAM:".white().bold(),
        format!("{:.2} GB", ram_used).blue(),
        format!("{:.2} GB", ram_total).blue()
    );

    // WM
    let wm = get_desktop_environment();

    println!(
        "{} {}",
        "WM:".white().bold(),
        wm.blue()
    );

    // Shell
    let shell = env::var("SHELL").unwrap_or_default();

    let shell_name = Path::new(&shell)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    println!(
        "{} {}",
        "Shell:".white().bold(),
        shell_name.blue()
    );

    // Terminal
    let terminal = get_terminal();

    println!(
        "{} {}",
        "Terminal:".white().bold(),
        terminal.blue()
    );

    // Host
    let hostname = System::host_name()
        .unwrap_or_else(|| "Unknown".to_string());

    println!(
        "{} {}",
        "Host:".white().bold(),
        hostname.blue()
    );

    // Display
    let display = get_display();

    println!(
        "{} {}",
        "Display:".white().bold(),
        display.blue()
    );

    // Packages
    let packages = Command::new("pacman")
        .arg("-Q")
        .output();

    match packages {
        Ok(output) => {
            let count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .count();

            println!(
                "{} {}",
                "Packages:".white().bold(),
                count.to_string().blue()
            );
        }

        Err(_) => {
            println!(
                "{} {}",
                "Packages:".white().bold(),
                "Unknown".red()
            );
        }
    }
    
    // GPU
    let gpu = get_gpu();

    println!(
        "{} {}",
        "GPU:".white().bold(),
        gpu.blue()
    );

    // CPU Usage
    let cpu_usage = sys.global_cpu_info().cpu_usage();

    println!(
        "{} {}",
        "CPU Usage:".white().bold(),
        format!("{:.1}%", cpu_usage).blue()
);

    println!(
        "{} {}",
        "CPU Usage:".white().bold(),
        format!("{:.1}%", cpu_usage).blue()
    );

    // Uptime
    let uptime = System::uptime();

    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;

    println!(
        "{} {}h {}min",
        "Uptime:".white().bold(),
        hours.to_string().blue(),
        minutes.to_string().blue()
    );

    // Disks
    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        let mount = disk.mount_point().display();
        let total = disk.total_space() / 1024 / 1024 / 1024;
        let available = disk.available_space() / 1024 / 1024 / 1024;
        let used = total.saturating_sub(available);

        println!(
            "{} {}: {} GB / {} GB",
            "Disk:".white().bold(),
            mount,
            used.to_string().blue(),
            total.to_string().blue()
        );
    }
}
    

