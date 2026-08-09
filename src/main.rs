mod weather;

use colored::*;
use std::env;
use std::path::Path;
use sysinfo::System;

fn get_desktop_environment() -> String {
    let vars = [
        "XDG_CURRENT_DESKTOP",
        "XDG_SESSION_DESKTOP",
        "DESKTOP_SESSION",
    ];

    for var in vars {
        if let Ok(value) = env::var(var)
            && !value.is_empty()
        {
            return value;
        }
    }

    "Unknown".to_string()
}

fn get_shell() -> String {
    let shell = env::var("SHELL").unwrap_or_default();

    Path::new(&shell)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

fn print_stats() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "     ╭──────╮".blue());

    println!(
        "{}{}{}{}",
        "     │ ".blue(),
        "KQ".white().bold(),
        "SS".blue().bold(),
        " │".blue()
    );

    println!("{}", "     ╰──────╯".blue());

    println!("{}", "╭────────────────╮".blue());

    println!(
        "{} {}",
        "│ OS     ".white().bold(),
        format!(
            "{:<5}  │",
            System::name()
                .unwrap_or("Unknown".into())
                .replace("Linux", "")
        )
        .blue()
        .bold()
    );

    println!(
        "{} {}",
        "│ Shell  ".white().bold(),
        format!("{:<5}  │", get_shell()).blue().bold()
    );

    println!(
        "{} {}",
        "│ WM     ".white().bold(),
        format!("{:<5}  │", get_desktop_environment())
            .blue()
            .bold()
    );

    let uptime = System::uptime();

    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;

    println!(
        "{} {}",
        "│ Uptime ".white().bold(),
        format!("{}h {}m │", hours, minutes).blue().bold()
    );

    println!("{}", "╰────────────────╯".blue());
}

fn print_usage() {
    println!();
    println!("{}", "Usage:".white().bold());
    println!("  kyuuriqstats");
    println!("  kyuuriqstats --stats");
    println!("  kyuuriqstats --weather");
    println!("  kyuuriqstats --weather <city>");
    println!("  kyuuriqstats --set-location <city>");
    println!("  kyuuriqstats --all");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut show_stats = true;
    let mut show_weather = true;
    let mut weather_city: Option<String> = None;

    if args.len() > 1 {
        match args[1].as_str() {
            "--stats" => {
                show_stats = true;
                show_weather = false;
            }

            "--weather" => {
                show_stats = false;
                show_weather = true;

                if args.len() > 2 {
                    weather_city = Some(args[2].clone());
                }
            }

            "--set-location" => {
                if args.len() < 3 {
                    eprintln!("{}", "Error: city is required.".red().bold());
                    println!();
                    println!("Example:");
                    println!("  kyuuriqstats --set-location Люберцы");
                    return;
                }

                let city = args[2..].join(" ");

                match weather::set_location(&city) {
                    Ok(()) => {
                        println!(
                            "{} {}",
                            "Location saved:".green().bold(),
                            city.white().bold()
                        );
                    }

                    Err(error) => {
                        eprintln!(
                            "{} {}",
                            "Location error:".red().bold(),
                            error
                        );
                    }
                }

                return;
            }

            "--all" => {
                show_stats = true;
                show_weather = true;
            }

            "--help" | "-h" => {
                print_usage();
                return;
            }

            _ => {
                println!("{}", "Unknown argument.".red().bold());
                print_usage();
                return;
            }
        }
    }

    if show_stats {
        print_stats();
    }

    if show_stats && show_weather {
        println!();
    }

    if show_weather {
        match weather_city {
            Some(city) => weather::print_weather(&city),
            None => weather::print_weather_auto(),
        }
    }
}