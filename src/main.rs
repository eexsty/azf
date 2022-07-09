use std::fmt::Write;

use colored::Colorize;
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    let mut system = System::new();
    system.refresh_all();

    let uptime = {
        let seconds = system.uptime();
        let s = seconds % 60;
        let m = (seconds / 60) % 60;
        let h = (seconds / 60) / 60;
        let mut buf = String::new();
        if h > 0 {
            write!(buf, "{}h ", h).unwrap();
        }
        if m > 0 {
            write!(buf, "{}m ", m).unwrap();
        }
        write!(buf, "{}s", s).unwrap();
        buf
    };
    let mem = format!(
        "{}MiB / {}MiB",
        system.used_memory() / 1024,
        system.total_memory() / 1024
    );
    let cpu = format!("{}%", system.global_cpu_info().cpu_usage().round());
    let os = system
        .name()
        .map(|os| {
            format!(
                "{} {}",
                os.cyan(),
                match std::env::consts::OS {
                    "linux" => "Linux",
                    "macos" => "macOS",
                    "windows" => "Windows",
                    "android" => "Android",
                    "ios" => "iOS",
                    "freebsd" => "FreeBSD",
                    "dragonfly" => "DragonFlyBSD",
                    "openbsd" => "OpenBSD",
                    "netbsd" => "NetBSD",
                    "solaris" => "Solaris",
                    os => os,
                }
                .cyan()
            )
        })
        .unwrap_or_else(|| "Unknown OS".to_string());
    let shell = std::env::var("SHELL")
        .ok()
        .map(|sh| {
            let sh = sh.split('/').last().unwrap();
            if let Ok(terminal) = std::env::var("TERM") {
                let terminal = if terminal.as_str() != "xterm" {
                    terminal
                        .split("xterm-")
                        .last()
                        .unwrap()
                        .split("-256color")
                        .next()
                        .unwrap()
                } else {
                    &terminal
                };
                format!("{sh} on {terminal}")
            } else {
                sh.to_string()
            }
        })
        .unwrap_or_else(|| "sh".to_string());
    let wm = std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .map(|wm| {
            if wm.starts_with('/') {
                wm.split('/').last().unwrap().to_string()
            } else {
                wm
            }
        })
        .unwrap_or_else(|_| "unknown".to_string());
    let language = sys_locale::get_locale()
        .map(|lang| lang.replacen('_', "-", 1))
        .unwrap_or_else(|| "en-US".to_string());

    println!("                           {}", os.bold());
    println!("   |\\---/|                 {} {shell}", " Shell:".blue());
    println!("   | ,_, |                 {} {uptime}", " Uptime:".blue());
    println!("    \\_`_/-..----.          {} {cpu}", " CPU:".blue());
    println!(" ___/ `   ' ,\"\"+ \\         {} {mem}", " Memory:".blue());
    println!("(__...'   __\\    |`.___.'; {} {wm}", "缾WM/DE:".blue());
    println!(
        "  (_,...'(_,.`__)/'.....+  {} {language}",
        " Language:".blue()
    );
    println!(
        "                           {} {} {} {} {}",
        "󰮯".yellow(),
        "󰊠".red(),
        "󰊠".blue(),
        "󰊠".green(),
        "󰊠 ".magenta()
    );
}
