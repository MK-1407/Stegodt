// src/utils/logger.rs
use colored::Colorize;
pub fn heading(msg: &str) {
    println!("{}", format!("=== {} ===", msg).blue().bold().underline());
}
pub fn info(label: &str, value: impl std::fmt::Display) {
    if value.to_string().is_empty() {
        println!(
            "{} {}",
            "[INFO]".blue().bold(),
            format!("{}", label).white().bold()
        );
    } else {
        println!(
            "{} {}",
            "[INFO]".blue().bold(),
            format!("{}: {}", label, value).white().bold()
        );
    }
}
pub fn warn(msg: &str, value: impl std::fmt::Display) {
    if value.to_string().is_empty() {
        println!("{}", format!("[WARN] {}", msg).yellow().bold());
    } else {
        println!(
            "{} {}",
            "[WARN]".yellow().bold(),
            format!("{}: {}", msg, value).yellow().bold()
        );
    }
}

pub fn error(msg: &str, value: impl std::fmt::Display) {
    if value.to_string().is_empty() {
        println!("{}", format!("[ERROR] {}", msg).red().bold());
    } else {
        println!(
            "{} {}",
            "[ERROR]".red().bold(),
            format!("{}: {}", msg, value).red().bold()
        );
    }
}

pub fn success(label: &str, value: impl std::fmt::Display) {
    if value.to_string().is_empty() {
        println!(
            "{} {}",
            "[SUCCESS]".green().bold(),
            format!("{}", label).white().bold()
        );
    } else {
        println!(
            "{} {}",
            "[SUCCESS]".green().bold(),
            format!("{}: {}", label, value).white().bold()
        );
    }
}

pub fn dangerous(msg: &str, value: impl std::fmt::Display) {
    if value.to_string().is_empty() {
        println!(
            "{}",
            format!("[DANGEROUS] {}", msg).red().bold().underline()
        );
    } else {
        println!(
            "{} {}",
            "[DANGEROUS]".red().bold().underline(),
            format!("{}: {}", msg, value).red().bold()
        );
    }
}
