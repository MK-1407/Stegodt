mod core;
mod utils;
use colored::Colorize;
use core::{chi_squared_analysis, odd_even_analysis, pair_analysis};
use figlet_rs::FIGlet;
use std::env;
use utils::logger;

fn main() {
    let standard_font = FIGlet::standard().unwrap();
    let figure = standard_font.convert("STEGODT").unwrap();
    println!("{}", figure.to_string().cyan().bold());
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_help();
        return;
    }

    let command = &args[1];
    let image_path = &args[2];

    let img = match image::open(image_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Failed to load image: {}", e);
            logger::error("Failed to load image", e);
            return;
        }
    };

    match command.as_str() {
        "odd" => {
            odd_even_analysis::analyze(&img);
        }
        "pair" => {
            pair_analysis::analyze(&img);
        }
        "chi" => {
            chi_squared_analysis::analyze(&img);
        }
        "complete" => {
            run_complete(&img);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("Usage:");
    println!("  stegodt <command> <image_path>\n");

    println!("Commands:");
    println!("  odd        → Odd-even analysis");
    println!("  pair       → Pair analysis");
    println!("  chi        → Chi-square analysis");
    println!("  complete   → Run all analyses");
}

fn run_complete(img: &image::DynamicImage) {
    logger::heading("Running Complete Analysis");
    let heuristic_analysis_result = odd_even_analysis::analyze(&img);
    let pair_analysis_result = core::pair_analysis::analyze(&img);
    let chi_squared_analysis_result = core::chi_squared_analysis::analyze(&img);

    let final_score = chi_squared_analysis_result * 0.30
        + heuristic_analysis_result * 0.20
        + pair_analysis_result * 0.15;

    logger::heading("Final Analysis");
    match final_score {
        s if s > 0.75 => logger::dangerous("HIGH confidence steganography detected", ""),
        s if s > 0.5 => logger::warn("Moderate suspicion", ""),
        s if s > 0.3 => logger::info("Weak signal", ""),
        _ => logger::success("Likely natural image", ""),
    }

    logger::success("Final Suspicion Score", format!("{:.2}", final_score));
}
