use crate::utils::logger;

pub fn analyze(img: &image::DynamicImage) -> f64 {
    logger::heading("Pair Analysis");

    let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = img.to_rgb8();

    let mut freq_r = [0u32; 256];
    let mut freq_g = [0u32; 256];
    let mut freq_b = [0u32; 256];

    // Build histograms
    for pixel in img.pixels() {
        let r = pixel[0] as usize;
        let g = pixel[1] as usize;
        let b = pixel[2] as usize;

        freq_r[r] += 1;
        freq_g[g] += 1;
        freq_b[b] += 1;
    }

    let mut total_diff_r = 0.0;
    let mut total_diff_g = 0.0;
    let mut total_diff_b = 0.0;

    let mut near_equal_r = 0.0;
    let mut near_equal_g = 0.0;
    let mut near_equal_b = 0.0;

    let mut total_pairs = 0.0;

    for k in (0..256).step_by(2) {
        let r0 = freq_r[k] as f64;
        let r1 = freq_r[k + 1] as f64;

        let g0 = freq_g[k] as f64;
        let g1 = freq_g[k + 1] as f64;

        let b0 = freq_b[k] as f64;
        let b1 = freq_b[k + 1] as f64;

        if r0 + r1 > 0.0 {
            let diff = (r0 - r1).abs() / (r0 + r1);
            total_diff_r += diff;
            if diff < 0.1 {
                near_equal_r += 1.0;
            }
        }

        if g0 + g1 > 0.0 {
            let diff = (g0 - g1).abs() / (g0 + g1);
            total_diff_g += diff;
            if diff < 0.1 {
                near_equal_g += 1.0;
            }
        }

        if b0 + b1 > 0.0 {
            let diff = (b0 - b1).abs() / (b0 + b1);
            total_diff_b += diff;
            if diff < 0.1 {
                near_equal_b += 1.0;
            }
        }

        total_pairs += 1.0;
    }

    // Average diff
    let avg_r = (total_diff_r / total_pairs) * 100.0;
    let avg_g = (total_diff_g / total_pairs) * 100.0;
    let avg_b = (total_diff_b / total_pairs) * 100.0;

    let overall_avg = (avg_r + avg_g + avg_b) / 3.0;

    // Equalization ratio
    let eq_r = (near_equal_r / total_pairs) * 100.0;
    let eq_g = (near_equal_g / total_pairs) * 100.0;
    let eq_b = (near_equal_b / total_pairs) * 100.0;

    let overall_eq = (eq_r + eq_g + eq_b) / 3.0;

    // Output
    logger::info("Average Pair Diff (R)", format!("{:.4}%", avg_r));
    logger::info("Average Pair Diff (G)", format!("{:.4}%", avg_g));
    logger::info("Average Pair Diff (B)", format!("{:.4}%", avg_b));
    logger::info("Overall Pair Difference", format!("{:.4}%", overall_avg));

    logger::info("Equalized Pairs (R)", format!("{:.2}%", eq_r));
    logger::info("Equalized Pairs (G)", format!("{:.2}%", eq_g));
    logger::info("Equalized Pairs (B)", format!("{:.2}%", eq_b));
    logger::info("Overall Equalization", format!("{:.2}%", overall_eq));

    // Decision logic (combined)
    if overall_eq > 35.0 {
        logger::dangerous("Strongly suspicious (high pair equalization)", "");
    } else if overall_eq > 20.0 {
        logger::warn("Suspicious Image (moderate equalization)", "");
    } else if overall_avg < 8.0 {
        logger::warn("Suspicious Image (low structural difference)", "");
    } else {
        logger::success("Likely natural image", "");
    }
    let eq_score = if overall_eq > 60.0 {
        1.0
    } else if overall_eq > 40.0 {
        0.7
    } else if overall_eq > 25.0 {
        0.4
    } else {
        0.0
    };

    return eq_score;
}
