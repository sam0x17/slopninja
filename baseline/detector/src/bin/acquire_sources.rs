use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Acquire bounded, licensed historical human-proxy source records")]
struct Args {
    #[arg(long)]
    output: PathBuf,
    #[arg(long, default_value_t = 80)]
    plos: usize,
    #[arg(long, default_value_t = 80)]
    wikinews: usize,
    /// Export public attribution and content hashes, without corpus text.
    #[arg(long)]
    manifest: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let report = slop_ninja_detector::acquire::acquire(&args.output, args.plos, args.wikinews)?;
    if let Some(path) = args.manifest {
        slop_ninja_detector::acquire::export_source_manifest(
            &args.output.join("human-records.jsonl"),
            &path,
        )?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
