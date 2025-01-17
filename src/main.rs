mod reg;

use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(package) = args.get(1) else {
        let cmd = args.get(0).map_or("chun", |s| s.as_str());
        println!("Usage: {} <package>", cmd);
        return ExitCode::SUCCESS;
    };

    println!(
        "{}",
        style(format!("🔍 Searching for package: {}", package)).bold()
    );

    let multi_progress = MultiProgress::new();
    let progress_style = ProgressStyle::with_template("{spinner:.blue} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let progress_bars: Vec<_> = reg::registries()
        .iter()
        .map(|registry| {
            let pb = multi_progress.add(ProgressBar::new_spinner());
            pb.set_style(progress_style.clone());
            pb.set_message(format!("Checking {}...", registry.name));
            (registry.name, pb)
        })
        .collect();

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        for registry in reg::registries() {
            match reg::check_package(&registry, package).await {
                Ok(found) => {
                    if let Some((_, pb)) = progress_bars
                        .iter()
                        .find(|(name, _)| *name == registry.name)
                    {
                        pb.finish_with_message(format!(
                            "{} {} {}",
                            if found {
                                style("✓").green()
                            } else {
                                style("✗").red()
                            },
                            registry.name,
                            if found {
                                style("(found)").green()
                            } else {
                                style("(not found)").red()
                            }
                        ));
                    }
                }
                Err(e) => {
                    println!(
                        "{} {} {}",
                        style("!").yellow(),
                        registry.name,
                        style(e.to_string()).yellow()
                    );
                }
            }
        }
    });

    ExitCode::SUCCESS
}
