use std::{error::Error, fs::create_dir_all, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting metadata extraction process...");

    // Create directories for metadata output
    create_dir_all("metadata_test/metadata/mp3")?;
    create_dir_all("metadata_test/metadata/flac")?;
    create_dir_all("metadata_test/metadata/C#-songs/mp3")?;
    create_dir_all("metadata_test/metadata/C#-songs/flac")?;

    // Run the shell script to extract metadata from the project root directory
    let output = Command::new("sh")
        .arg("./metadata_test/scripts/extract_metadata.sh")
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Error running extraction script: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err("Extraction script failed".into());
    }

    println!("Metadata extraction completed successfully.");
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
