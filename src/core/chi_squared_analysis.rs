use crate::utils::logger;

pub fn analyze(img: &image::DynamicImage) -> f64 {
    logger::heading("Chi-Squared Analysis");
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

    // Function to compute chi-square
    let compute_chi = |freq: &[u32; 256]| -> f64 {
        let mut chi = 0.0;
        let mut valid_pairs = 0.0;

        for k in (0..256).step_by(2) {
            let o0 = freq[k] as f64;
            let o1 = freq[k + 1] as f64;

            let sum = o0 + o1;
            if sum == 0.0 {
                continue;
            }

            let expected = sum / 2.0;

            chi += ((o0 - expected).powi(2) / expected) + ((o1 - expected).powi(2) / expected);

            valid_pairs += 1.0;
        }

        // normalize
        chi / valid_pairs
    };

    let chi_r = compute_chi(&freq_r);
    let chi_g = compute_chi(&freq_g);
    let chi_b = compute_chi(&freq_b);

    let overall = (chi_r + chi_g + chi_b) / 3.0;

    // Output
    logger::info("Chi-Square (R)", format!("{:.4}", chi_r));
    logger::info("Chi-Square (G)", format!("{:.4}", chi_g));
    logger::info("Chi-Square (B)", format!("{:.4}", chi_b));
    logger::info("Overall Chi-Square", format!("{:.4}", overall));

    // Interpretation
    match overall {
        v if v > 20.0 => logger::success("Clearly natural", ""),
        v if v > 10.0 => logger::success("Normal variation", ""),
        v if v > 5.0 => logger::warn("Weak suspicion", ""),
        v if v > 2.0 => logger::warn("Suspicious Image", ""),
        _ => logger::dangerous("Strongly suspicious (pair equalization detected)", ""),
    }

    let chi_score = if overall < 2.0 {
        1.0
    } else if overall < 5.0 {
        0.7
    } else if overall < 10.0 {
        0.3
    } else {
        0.0
    };

    return chi_score;
}
