use indicatif::{ProgressBar, ProgressStyle};

/// Create progress bar
pub fn create_progress(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);

    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)"
    )
    .unwrap()
    .progress_chars("##-");

    pb.set_style(style);

    pb
}