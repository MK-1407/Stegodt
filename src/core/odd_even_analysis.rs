use crate::utils::logger;
pub fn analyze(img: &image::DynamicImage) -> f64 {
    logger::heading("Odd Even Analysis");
    // Basic LSB detection (Odd Even Analysis)
    let img = img.to_rgb8();
    let odd_count = img.pixels().filter(|p| p[0] % 2 == 1).count();
    let even_count = img.pixels().filter(|p| p[0] % 2 == 0).count();

    // calulating the percentage difference between odd and even counts
    let total_pixels = odd_count + even_count;
    let odd_percentage = (odd_count as f64 / total_pixels as f64) * 100.0;
    let even_percentage = (even_count as f64 / total_pixels as f64) * 100.0;
    let percentage_diff = (odd_percentage - even_percentage).abs();
    logger::info("Total pixels", total_pixels);
    logger::info("Odd LSB count", odd_count);
    logger::info("Even LSB count", even_count);
    logger::info("Odd LSB percentage", format!("{:.10}%", odd_percentage));
    logger::info("Even LSB percentage", format!("{:.10}%", even_percentage));
    logger::info("Percentage difference", format!("{:.10}%", percentage_diff));
    match percentage_diff {
        diff if diff > 2.0 => logger::success("Clearly natural", ""),
        diff if diff > 1.0 => logger::success("Normal variation", ""),
        diff if diff > 0.5 => logger::success("Slightly balanced (weak signal)", ""),
        diff if diff > 0.2 => logger::warn("Suspicious Image (potentially balanced)", ""),
        _ => logger::dangerous("Strongly suspicious (near-perfect balance)", ""),
    }
    let score = if percentage_diff < 0.2 {
        1.0
    } else if percentage_diff < 0.5 {
        0.7
    } else if percentage_diff < 1.0 {
        0.4
    } else {
        0.0
    };
    return score;
}
