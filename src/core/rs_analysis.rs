use crate::utils::logger;

const MASKS: [[u8; 4]; 4] = [
    [0, 1, 1, 0],
    [1, 0, 0, 1],
    [0, 1, 0, 1],
    [1, 0, 1, 0],
];

pub fn analyze(img: &image::DynamicImage) -> f64 {
    logger::heading("RS Multimask Analysis");

    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let channel = 2; // Blue

    let mut estimates = Vec::new();

    for mask in MASKS.iter() {
        let inv_mask = invert_mask(mask);

        let mut rm = 0.0;
        let mut sm = 0.0;
        let mut rm_neg = 0.0;
        let mut sm_neg = 0.0;

        // Horizontal scan
        for y in 0..height {
            for x in 0..(width - 3) {
                let group = [
                    img.get_pixel(x, y)[channel],
                    img.get_pixel(x + 1, y)[channel],
                    img.get_pixel(x + 2, y)[channel],
                    img.get_pixel(x + 3, y)[channel],
                ];

                classify_group(&group, mask, &inv_mask, &mut rm, &mut sm, &mut rm_neg, &mut sm_neg);
            }
        }

        // Vertical scan
        for x in 0..width {
            for y in 0..(height - 3) {
                let group = [
                    img.get_pixel(x, y)[channel],
                    img.get_pixel(x, y + 1)[channel],
                    img.get_pixel(x, y + 2)[channel],
                    img.get_pixel(x, y + 3)[channel],
                ];

                classify_group(&group, mask, &inv_mask, &mut rm, &mut sm, &mut rm_neg, &mut sm_neg);
            }
        }

        let d0 = rm - sm;
        let d1 = rm_neg - sm_neg;
        let p = estimate_p(rm, sm, rm_neg, sm_neg);

        logger::info(
            "Mask stats",
            format!("{:?} => p={:.6}, d0={:.2}, d1={:.2}", mask, p, d0, d1),
        );

        // Keep only valid masks
        if p.is_finite() && p > 0.0 && p < 0.5 && d0.abs() > 1e-6 {
            estimates.push(p);
        }
    }

    // --- Clean & validate ---
    estimates.retain(|v| v.is_finite());

    if estimates.len() < 2 {
        logger::warn("RS result unstable (insufficient valid masks)", "");
        return 0.0;
    }

    // --- Aggregation ---
    estimates.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = estimates[estimates.len() / 2];
    let mean = estimates.iter().sum::<f64>() / estimates.len() as f64;
    let deviation = (mean - median).abs();

    logger::info("RS Median (embedding rate)", format!("{:.4}%", median * 100.0));
    logger::info("RS Mean", format!("{:.4}%", mean * 100.0));
    logger::info("RS Deviation", format!("{:.6}", deviation));
    logger::info("Valid Masks", format!("{}", estimates.len()));

    // --- Confidence scoring ---
    let mut confidence: f64 = 0.0;

    // Signal strength
    if median > 0.05 {
        confidence += 0.5;
    } else if median > 0.02 {
        confidence += 0.3;
    } else if median > 0.01 {
        confidence += 0.1;
    }

    // Consistency
    if deviation < 0.01 {
        confidence += 0.4;
    } else if deviation < 0.02 {
        confidence += 0.2;
    }

    // Mask agreement
    if estimates.len() >= 3 {
        confidence += 0.2;
    }

    confidence = confidence.clamp(0.0, 1.0);

    // --- Decision logic ---
    if confidence > 0.75 {
        logger::dangerous("Strongly suspicious (consistent RS embedding signal)", "");
    } else if confidence > 0.4 {
        logger::warn("Suspicious Image (weak/moderate RS signal)", "");
    } else if median > 0.01 {
        logger::warn("Possible low-rate embedding (low confidence)", "");
    } else {
        logger::success("Likely natural image (no reliable RS signal)", "");
    }

    logger::info("RS Confidence Score", format!("{:.2}", confidence));

    confidence
}

// ================= Helpers =================

fn classify_group(
    group: &[u8; 4],
    mask: &[u8; 4],
    inv_mask: &[u8; 4],
    rm: &mut f64,
    sm: &mut f64,
    rm_neg: &mut f64,
    sm_neg: &mut f64,
) {
    let f_orig = discrimination(group);

    let g_pos = apply_mask(group, mask);
    let f_pos = discrimination(&g_pos);

    if f_pos > f_orig {
        *rm += 1.0;
    } else if f_pos < f_orig {
        *sm += 1.0;
    }

    let g_neg = apply_mask(group, inv_mask);
    let f_neg = discrimination(&g_neg);

    if f_neg > f_orig {
        *rm_neg += 1.0;
    } else if f_neg < f_orig {
        *sm_neg += 1.0;
    }
}

fn discrimination(p: &[u8; 4]) -> i32 {
    (p[1] as i32 - p[0] as i32).abs()
        + (p[2] as i32 - p[1] as i32).abs()
        + (p[3] as i32 - p[2] as i32).abs()
}

fn apply_mask(pixels: &[u8; 4], mask: &[u8; 4]) -> [u8; 4] {
    let mut out = *pixels;
    for i in 0..4 {
        if mask[i] == 1 {
            out[i] ^= 1; // LSB flip
        }
    }
    out
}

fn invert_mask(mask: &[u8; 4]) -> [u8; 4] {
    let mut inv = [0; 4];
    for i in 0..4 {
        inv[i] = if mask[i] == 1 { 0 } else { 1 };
    }
    inv
}

fn estimate_p(rm: f64, sm: f64, rm_neg: f64, sm_neg: f64) -> f64 {
    let d0 = rm - sm;
    let d1 = rm_neg - sm_neg;

    if d0.abs() < 1e-10 {
        return f64::NAN;
    }

    let ratio = d1 / d0;

    if ratio <= 0.0 {
        return f64::NAN;
    }

    let p = 0.5 * (1.0 - ratio.sqrt());
    p.clamp(0.0, 0.5)
}