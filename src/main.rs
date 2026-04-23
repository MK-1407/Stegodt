mod core;
mod utils;
use colored::Colorize;
use core::{chi_squared_analysis, odd_even_analysis, pair_analysis, rs_analysis};
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
        "rs" => {
            rs_analysis::analyze(&img);
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

    let heuristic = odd_even_analysis::analyze(&img);
    let pair = core::pair_analysis::analyze(&img);
    let chi = core::chi_squared_analysis::analyze(&img);
    let rs = core::rs_analysis::analyze(&img);

    // --- Log individual scores ---
    logger::info("Heuristic Score", format!("{:.2}", heuristic));
    logger::info("Pair Analysis Score", format!("{:.2}", pair));
    logger::info("Chi-Square Score", format!("{:.2}", chi));
    logger::info("RS Score", format!("{:.2}", rs));

    // --- Dynamic RS weighting ---
    let rs_weight: f64 = if rs > 0.6 {
        0.30  // strong RS signal → trust it more
    } else if rs > 0.3 {
        0.20
    } else {
        0.10  // weak RS → reduce influence
    };

    // --- Base weights ---
    let chi_w = 0.30;
    let heuristic_w = 0.20;
    let pair_w = 0.20;

    // Normalize remaining weight
    let remaining = 1.0 - (chi_w + heuristic_w + pair_w);
    let rs_w = rs_weight.min(remaining);

    let total_weight = chi_w + heuristic_w + pair_w + rs_w;

    // --- Final score ---
    let final_score =
        (chi * chi_w +
         heuristic * heuristic_w +
         pair * pair_w +
         rs * rs_w) / total_weight;

    // --- Agreement boost (very important) ---
    let agreement_count = [chi, heuristic, pair, rs]
        .iter()
        .filter(|&&v| v > 0.4)
        .count();

    let final_score = if agreement_count >= 3 {
        (final_score + 0.1).clamp(0.0, 1.0)
    } else {
        final_score
    };

    logger::heading("Final Analysis");

    match final_score {
        s if s > 0.80 => logger::dangerous("HIGH confidence steganography detected", ""),
        s if s > 0.60 => logger::dangerous("Strong suspicion", ""),
        s if s > 0.45 => logger::warn("Moderate suspicion", ""),
        s if s > 0.25 => logger::info("Weak signal", ""),
        _ => logger::success("Likely natural image", ""),
    }

    logger::success("Final Suspicion Score", format!("{:.2}", final_score));
}