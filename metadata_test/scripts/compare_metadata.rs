use std::{
    collections::HashMap,
    error::Error,
    fs::{create_dir_all, read_dir, read_to_string, write},
    path::Path,
};

#[derive(Debug)]
struct Metadata {
    fields: HashMap<String, String>,
}

impl Metadata {
    fn new() -> Self {
        Metadata {
            fields: HashMap::new(),
        }
    }

    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = read_to_string(path)?;
        let mut metadata = Metadata::new();

        for line in content.lines() {
            // exiftool -G1 format: [Group Name] Field Name : Value
            if line.trim().is_empty() {
                continue; // Skip empty lines
            }

            // Find the colon that separates field name from value
            if let Some(colon_pos) = line.find(':') {
                let before_colon = &line[..colon_pos];
                let value = line[colon_pos + 1..].trim();

                let field_name = if let Some(last_bracket_pos) = before_colon.rfind(']') {
                    // If there's a ']' character, take the part after it.
                    // This handles `[Group Name] Field Name`
                    before_colon[last_bracket_pos + 1..].trim()
                } else {
                    // If no ']' character, the whole part before the colon is the field name.
                    // This handles `Field Name`
                    before_colon.trim()
                };

                if !field_name.is_empty() && !value.is_empty() {
                    metadata
                        .fields
                        .insert(field_name.to_string(), value.to_string());
                }
            }
        }

        Ok(metadata)
    }

    fn compare_with(&self, other: &Metadata) -> Vec<String> {
        let mut differences = Vec::new();

        // Check fields that exist in self but not in other
        for (key, value) in &self.fields {
            if let Some(other_value) = other.fields.get(key) {
                if value != other_value {
                    differences.push(format!(
                        "Field '{}' differs:\n  Rust: {}\n  C#:   {}",
                        key, value, other_value
                    ));
                }
            } else {
                differences.push(format!(
                    "Field '{}' exists in Rust but not in C#: {}",
                    key, value
                ));
            }
        }

        // Check fields that exist in other but not in self
        for (key, value) in &other.fields {
            if !self.fields.contains_key(key) {
                differences.push(format!(
                    "Field '{}' exists in C# but not in Rust: {}",
                    key, value
                ));
            }
        }

        differences
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting metadata comparison process...");

    // Create reports directory
    create_dir_all("metadata_test/reports")?;

    // Get all Rust-generated metadata files
    let rust_mp3_files = get_metadata_files("metadata_test/metadata/mp3")?;
    let rust_flac_files = get_metadata_files("metadata_test/metadata/flac")?;

    // Get all C#-generated metadata files from both MP3 and FLAC directories
    let csharp_mp3_files = get_metadata_files("metadata_test/metadata/C#-songs/mp3")?;
    let csharp_flac_files = get_metadata_files("metadata_test/metadata/C#-songs/flac")?;

    // Combine both lists
    let mut csharp_files = csharp_mp3_files;
    csharp_files.extend(csharp_flac_files);

    let mut total_differences = 0;

    // Compare MP3 files
    for rust_file in &rust_mp3_files {
        if let Some(csharp_file) = find_matching_csharp_file(rust_file, &csharp_files) {
            println!("Comparing {} with {}", rust_file, csharp_file);

            let rust_metadata =
                Metadata::from_file(&format!("metadata_test/metadata/mp3/{}", rust_file))?;
            let csharp_metadata = Metadata::from_file(&format!(
                "metadata_test/metadata/C#-songs/mp3/{}",
                csharp_file
            ))?;

            let differences = rust_metadata.compare_with(&csharp_metadata);
            if !differences.is_empty() {
                // Create a separate report file for this comparison
                let comparison_name = rust_file.trim_end_matches(".txt").replace(" - ", "_");
                let report_filename = format!(
                    "metadata_test/reports/{}_vs_{}.txt",
                    comparison_name,
                    csharp_file.trim_end_matches(".txt")
                );

                let report_content = format!(
                    "Differences between {} and {}:\n{}\n",
                    rust_file,
                    csharp_file,
                    differences.join("\n")
                );

                write(&report_filename, &report_content)?;
                total_differences += differences.len();

                println!("  Differences saved to: {}", report_filename);
            } else {
                println!(
                    "  No differences found between {} and {}",
                    rust_file, csharp_file
                );
            }
        } else {
            println!("  No matching C# file found for: {}", rust_file);
        }
    }

    // Compare FLAC files
    for rust_file in &rust_flac_files {
        if let Some(csharp_file) = find_matching_csharp_file(rust_file, &csharp_files) {
            println!("Comparing {} with {}", rust_file, csharp_file);

            let rust_metadata =
                Metadata::from_file(&format!("metadata_test/metadata/flac/{}", rust_file))?;
            let csharp_metadata = Metadata::from_file(&format!(
                "metadata_test/metadata/C#-songs/flac/{}",
                csharp_file
            ))?;

            let differences = rust_metadata.compare_with(&csharp_metadata);
            if !differences.is_empty() {
                // Create a separate report file for this comparison
                let comparison_name = rust_file.trim_end_matches(".txt").replace(" - ", "_");
                let report_filename = format!(
                    "metadata_test/reports/{}_vs_{}.txt",
                    comparison_name,
                    csharp_file.trim_end_matches(".txt")
                );

                let report_content = format!(
                    "Differences between {} and {}:\n{}\n",
                    rust_file,
                    csharp_file,
                    differences.join("\n")
                );

                write(&report_filename, &report_content)?;
                total_differences += differences.len();

                println!("  Differences saved to: {}", report_filename);
            } else {
                println!(
                    "  No differences found between {} and {}",
                    rust_file, csharp_file
                );
            }
        } else {
            println!("  No matching C# file found for: {}", rust_file);
        }
    }

    println!("\nComparison completed. Individual reports saved to: metadata_test/reports/");
    println!(
        "Total differences found across all comparisons: {}",
        total_differences
    );

    Ok(())
}

fn get_metadata_files(dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();

    if Path::new(dir).exists() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && path.extension().is_some_and(|ext| ext == "txt")
                && let Some(filename) = path.file_name().and_then(|name| name.to_str())
            {
                files.push(filename.to_string());
            }
        }
    }

    Ok(files)
}

fn find_matching_csharp_file(rust_file: &str, csharp_files: &[String]) -> Option<String> {
    // Extract the artist and track name from the Rust filename (format: "Artist - Track - Format - ID.txt")
    let rust_parts: Vec<&str> = rust_file.trim_end_matches(".txt").split(" - ").collect();
    if rust_parts.len() >= 2 {
        let rust_artist = rust_parts[0];
        let rust_track = rust_parts[1];

        // Normalize the Rust artist and track names to handle special characters
        let normalized_rust_artist = normalize_filename(rust_artist);
        let normalized_rust_track = normalize_filename(rust_track);

        // Look for a C# file that matches the artist and track name
        for csharp_file in csharp_files {
            let csharp_name = csharp_file.trim_end_matches(".txt");

            // Split C# filename to extract artist and track
            // Handle both formats: "Artist - Track.txt" and "Artist - Track (Version).txt"
            let csharp_parts: Vec<&str> = csharp_name.splitn(2, " - ").collect();
            if !csharp_parts.is_empty() {
                let csharp_artist = csharp_parts[0];
                let csharp_track = if csharp_parts.len() > 1 {
                    csharp_parts[1]
                } else {
                    "" // If no separator found, treat entire name as artist
                };

                // Normalize C# names
                let normalized_csharp_artist = normalize_filename(csharp_artist);
                let normalized_csharp_track = normalize_filename(csharp_track);

                // Check for matches with normalized names
                if normalized_csharp_artist.contains(&normalized_rust_artist)
                    || normalized_rust_artist.contains(&normalized_csharp_artist)
                {
                    // If artist matches, check if track also matches
                    if normalized_csharp_track.contains(&normalized_rust_track)
                        || normalized_rust_track.contains(&normalized_csharp_track)
                    {
                        return Some(csharp_file.clone());
                    }
                }
            }
        }
    }

    None
}

// Helper function to normalize filenames for comparison
fn normalize_filename(name: &str) -> String {
    // Replace common special characters that might differ between filenames
    // Convert to lowercase and replace common separators with spaces

    name.to_lowercase()
        .replace(":", "_") // Replace colons with underscores
        .replace(" : ", "_") // Replace colon with space around it
        .replace(" _ ", "_") // Normalize underscore spacing
        .replace("  ", " ") // Normalize multiple spaces
        .replace("(", "")
        .replace(")", "")
        .replace(",", "")
        .replace(".", "")
        .replace("'", "")
        .replace("!", "")
        .replace("?", "")
        .replace("-", " ")
        .replace("  ", " ") // Normalize multiple spaces again after replacements
        .trim()
        .to_string()
}
