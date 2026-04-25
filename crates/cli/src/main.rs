use clap::Parser;
use std::fs;
use std::path::Path;
use anyhow::{Result, Context, anyhow};

/// A simple program to convert Pixaki files to Aseprite files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the .pixaki directory
    pixaki_path: std::path::PathBuf,

    /// The path to the output .aseprite file
    aseprite_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let doc = if cli.pixaki_path.join("document.json").exists() {
        handle_modern_format(&cli.pixaki_path)?
    } else if cli.pixaki_path.join("DocumentInfo.plist").exists() {
        handle_legacy_format(&cli.pixaki_path)?
    } else {
        return Err(anyhow!("No document.json or DocumentInfo.plist found in the pixaki directory"));
    };

    let aseprite_file = aseprite_converter::convert(doc)?;
    
    let mut buffer = Vec::new();
    aseprite_file.write_to(&mut buffer)
        .map_err(|e| anyhow!("Failed to write Aseprite file: {}", e))?;
    fs::write(&cli.aseprite_path, buffer)?;

    println!(
        "Successfully wrote Aseprite file to {:?}",
        cli.aseprite_path
    );

    Ok(())
}

fn handle_modern_format(pixaki_path: &Path) -> Result<pixel_art::Document> {
    let document_path = pixaki_path.join("document.json");
    let json_str = fs::read_to_string(document_path)?;
    let doc_v3: pixaki_v3::Document = serde_json::from_str(&json_str)
        .context("Unable to parse document.json")?;
    
    pixaki_v3_converter::convert(doc_v3, pixaki_path)
}

fn handle_legacy_format(pixaki_path: &Path) -> Result<pixel_art::Document> {
    let plist_path = pixaki_path.join("DocumentInfo.plist");
    let doc_v2: pixaki_v2::Document = plist::from_file(plist_path)
        .context("Failed to parse DocumentInfo.plist")?;
    
    pixaki_v2_converter::convert(doc_v2, pixaki_path)
}
