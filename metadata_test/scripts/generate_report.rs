use std::{
    collections::{HashMap, HashSet},
    fs::{read_dir, read_to_string, write},
    path::Path,
};

use regex::Regex;

// Function to read all comparison report files and generate a final markdown report
// Grouped by format and field, showing all cases for each category
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating comprehensive metadata comparison report...");

    let reports_dir = Path::new("metadata_test/reports");
    let mp3_output_file = "metadata_test/mp3_metadata_report.md";
    let flac_output_file = "metadata_test/flac_metadata_report.md";

    // Define fields to ignore (always different and irrelevant)
    let file_date_time_ignored_fields = [
        "File Modification Date/Time",
        "File Access Date/Time",
        "File Inode Change Date/Time",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>();

    let id3_file_size_ignored_fields = ["File Size", "ID3 Size"]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();

    let directory_file_name_ignored_fields = ["File Name", "Directory"]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();

    let picture_ignored_fields = ["Picture Width", "Picture Height", "Picture Bits Per Pixel"]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();

    let duration_total_samples_ignored_fields = ["Duration", "Total Samples"]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();

    let ignored_fields: HashSet<&'static str> = [].iter().cloned().collect::<HashSet<_>>();

    let lame_ignored_fields = [
        "Encoder",
        "Lame Bitrate",
        "Lame Low Pass Filter",
        "Lame Method",
        "Lame Quality",
        "Lame Stereo Mode",
        "Lame VBR Quality",
        "MS Stereo",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>();

    // Read all comparison report files
    let mut mp3_differences = HashMap::new();
    let mut flac_differences = HashMap::new();
    let mut mp3_ignored_fields_counts = HashMap::new();
    let mut flac_ignored_fields_counts = HashMap::new();
    let mut mp3_lame_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut mp3_file_date_time_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut flac_file_date_time_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut mp3_id3_file_size_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut flac_id3_file_size_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut mp3_directory_file_name_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut flac_directory_file_name_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut mp3_picture_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut flac_picture_ignored_fields_counts: HashMap<String, usize> = HashMap::new();
    let mut mp3_duration_total_samples_ignored_fields_counts: HashMap<String, usize> =
        HashMap::new();
    let mut flac_duration_total_samples_ignored_fields_counts: HashMap<String, usize> =
        HashMap::new();
    let flac_media_mediatype_ignored_fields_counts: HashMap<String, usize> = HashMap::new();

    // Pre-compile the regex outside the loop for better performance
    let re = Regex::new(r"^(.+)_(FLAC|MP3)_(\d+)$").unwrap();

    for entry in read_dir(reports_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "txt")
            && path.file_name().unwrap().to_str().unwrap().contains("_vs_")
        {
            let content = read_to_string(&path)?;

            let file_name = path.file_name().unwrap().to_str().unwrap();
            let parts: Vec<&str> = file_name.split("_vs_").collect();
            let rust_part = parts[0];
            let csharp_part = parts[1].replace(".txt", "");

            // Parse the Rust filename using regex to handle complex titles
            // Pattern: ..._{format}_{id} where format is FLAC/MP3 and id is numeric

            let file_header = if let Some(caps) = re.captures(rust_part) {
                let format = &caps[2];
                let format_lower = format.to_lowercase();

                // Construct C# path
                let csharp_path = format!(
                    "@/qobuz-api-rust/metadata_test/metadata/C#-songs/{}/{}.txt",
                    format_lower, csharp_part
                );

                // Construct Rust path - use the actual directory structure
                let rust_path = format!(
                    "@/qobuz-api-rust/metadata_test/metadata/{}/{}.txt",
                    format_lower, rust_part
                );

                format!("## {} vs {}\n", csharp_path, rust_path)
            } else {
                // Fallback for files that don't match the expected pattern
                // Try to determine format from filename
                let format_lower = if file_name.contains("_FLAC_") {
                    "flac".to_string()
                } else if file_name.contains("_MP3_") {
                    "mp3".to_string()
                } else {
                    "unknown".to_string()
                };

                let csharp_path = format!(
                    "@/qobuz-api-rust/metadata_test/metadata/C#-songs/{}/{}.txt",
                    format_lower, csharp_part
                );

                let rust_path = format!(
                    "@/qobuz-api-rust/metadata_test/metadata/{}/{}.txt",
                    format_lower, rust_part
                );

                format!("## {} vs {}\n", csharp_path, rust_path)
            };

            // Determine if this is MP3 or FLAC comparison based on filename
            let is_mp3 = file_name.contains("_MP3_");

            // Parse the content to extract only the actual field differences
            let lines: Vec<&str> = content.lines().collect();
            let mut i = 0;
            let mut file_diffs = HashMap::new();

            while i < lines.len() {
                let line = lines[i];
                let mut field_name_opt = None;
                let mut is_diff = false;

                if line.trim().starts_with("Field '") {
                    if line.contains(" differs:") {
                        field_name_opt = line
                            .split("Field '")
                            .nth(1)
                            .and_then(|s| s.split("' differs:").next());
                        is_diff = true;
                    } else if line.contains("exists in Rust but not in C#")
                        || line.contains("exists in C# but not in Rust")
                    {
                        field_name_opt = line
                            .split("Field '")
                            .nth(1)
                            .and_then(|s| s.split("' ").next());
                    }
                }

                if let Some(field_name) = field_name_opt {
                    // Trim whitespace from field_name before checking against ignored_fields
                    let trimmed_field_name = field_name.trim();
                    if is_mp3 && lame_ignored_fields.contains(trimmed_field_name) {
                        *mp3_lame_ignored_fields_counts
                            .entry(trimmed_field_name.to_string())
                            .or_insert(0) += 1;
                        i += 1;
                        while i < lines.len() && !lines[i].trim().starts_with("Field '") {
                            i += 1;
                        }
                        continue;
                    } else if file_date_time_ignored_fields.contains(trimmed_field_name) {
                        if is_mp3 {
                            *mp3_file_date_time_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        } else {
                            *flac_file_date_time_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        }
                        i += 1;
                        while i < lines.len() && !lines[i].trim().starts_with("Field '") {
                            i += 1;
                        }
                        continue;
                    } else if id3_file_size_ignored_fields.contains(trimmed_field_name) {
                        if is_mp3 {
                            *mp3_id3_file_size_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        } else {
                            *flac_id3_file_size_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        }
                        i += 1;
                        while i < lines.len() && !lines[i].trim().starts_with("Field '") {
                            i += 1;
                        }
                        continue;
                    } else if directory_file_name_ignored_fields.contains(trimmed_field_name) {
                        if is_mp3 {
                            *mp3_directory_file_name_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        } else {
                            *flac_directory_file_name_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        }
                        i += 1;
                        while i < lines.len() && !lines[i].trim().starts_with("Field '") {
                            i += 1;
                        }
                        continue;
                    } else if picture_ignored_fields.contains(trimmed_field_name) {
                        if is_mp3 {
                            *mp3_picture_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        } else {
                            *flac_picture_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        }
                        i += 1;
                        while i < lines.len() && !lines[i].trim().starts_with("Field '") {
                            i += 1;
                        }
                        continue;
                    } else if duration_total_samples_ignored_fields.contains(trimmed_field_name) {
                        if is_mp3 {
                            *mp3_duration_total_samples_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        } else {
                            *flac_duration_total_samples_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        }
                        i += 1;
                        while i < lines.len() && !lines[i].trim().starts_with("Field '") {
                            i += 1;
                        }
                        continue;
                    } else if ignored_fields.contains(trimmed_field_name) {
                        if is_mp3 {
                            *mp3_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        } else {
                            *flac_ignored_fields_counts
                                .entry(trimmed_field_name.to_string())
                                .or_insert(0) += 1;
                        }
                        i += 1;
                        while i < lines.len() && !lines[i].trim().starts_with("Field '") {
                            i += 1;
                        }
                        continue;
                    }

                    let mut diff_content = String::new();
                    diff_content.push_str(line);
                    diff_content.push('\n');
                    i += 1;

                    let mut value_content = String::new();
                    if is_diff {
                        // For "differs" cases, we capture the values for comparison
                        let mut temp_i = i;
                        while temp_i < lines.len() && !lines[temp_i].trim().starts_with("Field '") {
                            let current_line = lines[temp_i];
                            diff_content.push_str(current_line);
                            diff_content.push('\n');
                            if current_line.trim().starts_with("Rust:")
                                || current_line.trim().starts_with("C#:")
                            {
                                value_content.push_str(
                                    current_line
                                        .split_at(current_line.find(':').unwrap() + 1)
                                        .1
                                        .trim(),
                                );
                            }
                            temp_i += 1;
                        }
                    } else {
                        // For "exists in" cases, capture the value after the colon
                        let mut temp_i = i;
                        if let Some(colon_pos) = line.find(':') {
                            value_content.push_str(line.split_at(colon_pos + 1).1.trim());
                        }

                        while temp_i < lines.len() && !lines[temp_i].trim().starts_with("Field '") {
                            let current_line = lines[temp_i];
                            diff_content.push_str(current_line);
                            diff_content.push('\n');
                            temp_i += 1;
                        }
                    }
                    while i < lines.len() && !lines[i].trim().starts_with("Field '") {
                        i += 1;
                    }

                    file_diffs.insert(field_name.to_string(), (diff_content, value_content));
                } else {
                    i += 1;
                }
            }

            // Special handling for Musician Credits and Involved People
            let mc_key = "Musician Credits".to_string();
            let ip_key = "Involved People".to_string();
            let grouped_key = "Musician Credits / Involved People".to_string();

            let mc_diff = file_diffs.remove(&mc_key);
            let ip_diff = file_diffs.remove(&ip_key);

            match (mc_diff, ip_diff) {
                (Some((_mc_content, mc_val)), Some((_ip_content, ip_val))) => {
                    if mc_val == ip_val {
                        if is_mp3 {
                            *mp3_ignored_fields_counts
                                .entry(grouped_key.clone())
                                .or_insert(0) += 1;
                        } else {
                            *flac_ignored_fields_counts
                                .entry(grouped_key.clone())
                                .or_insert(0) += 1;
                        }
                    } else {
                        let diff_content = format!(
                            "Field '{}' differs:\n  Rust: {}\n  C#:   {}\n",
                            grouped_key, mc_val, ip_val
                        );
                        if is_mp3 {
                            mp3_differences
                                .entry(grouped_key.clone())
                                .or_insert_with(Vec::new)
                                .push((file_header.clone(), diff_content));
                        } else {
                            flac_differences
                                .entry(grouped_key.clone())
                                .or_insert_with(Vec::new)
                                .push((file_header.clone(), diff_content));
                        }
                    }
                }
                (Some((mc_content, _)), None) => {
                    if is_mp3 {
                        mp3_differences
                            .entry(grouped_key.clone())
                            .or_insert_with(Vec::new)
                            .push((file_header.clone(), mc_content));
                    } else {
                        flac_differences
                            .entry(grouped_key.clone())
                            .or_insert_with(Vec::new)
                            .push((file_header.clone(), mc_content));
                    }
                }
                (None, Some((ip_content, _))) => {
                    if is_mp3 {
                        mp3_differences
                            .entry(grouped_key.clone())
                            .or_insert_with(Vec::new)
                            .push((file_header.clone(), ip_content));
                    } else {
                        flac_differences
                            .entry(grouped_key.clone())
                            .or_insert_with(Vec::new)
                            .push((file_header.clone(), ip_content));
                    }
                }
                (None, None) => {}
            }

            // Special handling for Media and Mediatype in FLAC files
            let media_key = "Media".to_string();
            let mediatype_key = "Mediatype".to_string();
            let grouped_media_mediatype_key = "Media / Mediatype".to_string();

            let media_diff = file_diffs.remove(&media_key);
            let mediatype_diff = file_diffs.remove(&mediatype_key);

            match (media_diff, mediatype_diff) {
                (Some((_media_content, media_val)), Some((_mediatype_content, mediatype_val))) => {
                    if media_val == mediatype_val {
                        if !is_mp3 {
                            *flac_ignored_fields_counts
                                .entry(grouped_media_mediatype_key.clone())
                                .or_insert(0) += 1;
                        }
                    } else {
                        let diff_content = format!(
                            "Field '{}' differs:\n  Rust: {}\n  C#:   {}\n",
                            grouped_media_mediatype_key, media_val, mediatype_val
                        );
                        if !is_mp3 {
                            flac_differences
                                .entry(grouped_media_mediatype_key.clone())
                                .or_insert_with(Vec::new)
                                .push((file_header.clone(), diff_content));
                        }
                    }
                }
                (Some((media_content, _)), None) => {
                    if !is_mp3 {
                        flac_differences
                            .entry(grouped_media_mediatype_key.clone())
                            .or_insert_with(Vec::new)
                            .push((file_header.clone(), media_content));
                    }
                }
                (None, Some((mediatype_content, _))) => {
                    if !is_mp3 {
                        flac_differences
                            .entry(grouped_media_mediatype_key.clone())
                            .or_insert_with(Vec::new)
                            .push((file_header.clone(), mediatype_content));
                    }
                }
                (None, None) => {}
            }

            for (field_name, (diff_content, _)) in file_diffs {
                if is_mp3 {
                    mp3_differences
                        .entry(field_name)
                        .or_insert_with(Vec::new)
                        .push((file_header.clone(), diff_content));
                } else {
                    flac_differences
                        .entry(field_name)
                        .or_insert_with(Vec::new)
                        .push((file_header.clone(), diff_content));
                }
            }
        }
    }

    // Generate summary statistics
    let mp3_summary = format!(
        "# Metadata Comparison Report\n\n## Summary\n- Total fields with differences: {}\n- Total MP3 metadata differences found: {}\n- Ignored differences (always different): {}\n\n",
        mp3_differences.len(),
        mp3_differences.values().map(|v| v.len()).sum::<usize>(),
        mp3_ignored_fields_counts.values().sum::<usize>()
            + mp3_lame_ignored_fields_counts.values().sum::<usize>()
            + mp3_file_date_time_ignored_fields_counts
                .values()
                .sum::<usize>()
            + mp3_id3_file_size_ignored_fields_counts
                .values()
                .sum::<usize>()
            + mp3_directory_file_name_ignored_fields_counts
                .values()
                .sum::<usize>()
            + mp3_picture_ignored_fields_counts.values().sum::<usize>()
            + mp3_duration_total_samples_ignored_fields_counts
                .values()
                .sum::<usize>()
    );
    let flac_summary = format!(
        "# Metadata Comparison Report\n\n## Summary\n- Total fields with differences: {}\n- Total FLAC metadata differences found: {}\n- Ignored differences (always different): {}\n\n",
        flac_differences.len(),
        flac_differences.values().map(|v| v.len()).sum::<usize>(),
        flac_ignored_fields_counts.values().sum::<usize>()
            + flac_file_date_time_ignored_fields_counts
                .values()
                .sum::<usize>()
            + flac_id3_file_size_ignored_fields_counts
                .values()
                .sum::<usize>()
            + flac_directory_file_name_ignored_fields_counts
                .values()
                .sum::<usize>()
            + flac_picture_ignored_fields_counts.values().sum::<usize>()
            + flac_duration_total_samples_ignored_fields_counts
                .values()
                .sum::<usize>()
            + flac_media_mediatype_ignored_fields_counts
                .values()
                .sum::<usize>()
            + flac_media_mediatype_ignored_fields_counts
                .values()
                .sum::<usize>()
    );

    // Add note about ignored fields
    let mut mp3_ignored_note = String::from(
        "## Note on Ignored Fields\n\nThe following fields were intentionally ignored in the comparison as they are always different and irrelevant to the actual metadata quality:\n",
    );
    let mut sorted_mp3_ignored_fields: Vec<_> = mp3_ignored_fields_counts.iter().collect();
    sorted_mp3_ignored_fields.sort_by(|a, b| a.0.cmp(b.0));
    for (field, count) in sorted_mp3_ignored_fields {
        mp3_ignored_note.push_str(&format!("- {}: {}\n", field, count));
    }

    if !mp3_directory_file_name_ignored_fields_counts.is_empty() {
        mp3_ignored_note.push_str("- Directory and File Name Tags:\n");
        let mut sorted_directory_file_name: Vec<_> = mp3_directory_file_name_ignored_fields_counts
            .iter()
            .collect();
        sorted_directory_file_name.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_directory_file_name {
            mp3_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !mp3_duration_total_samples_ignored_fields_counts.is_empty() {
        mp3_ignored_note.push_str("- Duration and Total Samples Tags:\n");
        let mut sorted_duration_total_samples: Vec<_> =
            mp3_duration_total_samples_ignored_fields_counts
                .iter()
                .collect();
        sorted_duration_total_samples.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_duration_total_samples {
            mp3_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !mp3_file_date_time_ignored_fields_counts.is_empty() {
        mp3_ignored_note.push_str("- File Date/Time Tags:\n");
        let mut sorted_file_date_time: Vec<_> =
            mp3_file_date_time_ignored_fields_counts.iter().collect();
        sorted_file_date_time.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_file_date_time {
            mp3_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !mp3_id3_file_size_ignored_fields_counts.is_empty() {
        mp3_ignored_note.push_str("- ID3 and File Size Tags:\n");
        let mut sorted_id3_file_size: Vec<_> =
            mp3_id3_file_size_ignored_fields_counts.iter().collect();
        sorted_id3_file_size.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_id3_file_size {
            mp3_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !mp3_lame_ignored_fields_counts.is_empty() {
        mp3_ignored_note.push_str("- LAME Tags:\n");
        let mut sorted_lame: Vec<_> = mp3_lame_ignored_fields_counts.iter().collect();
        sorted_lame.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_lame {
            mp3_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !mp3_picture_ignored_fields_counts.is_empty() {
        mp3_ignored_note.push_str("- Picture Tags:\n");
        let mut sorted_picture_fields: Vec<_> = mp3_picture_ignored_fields_counts.iter().collect();
        sorted_picture_fields.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_picture_fields {
            mp3_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    mp3_ignored_note.push_str("\nThese differences occur due to file processing, storage locations, and encoding differences but do not reflect actual metadata discrepancies.\n\n");

    let mut flac_ignored_note = String::from(
        "## Note on Ignored Fields\n\nThe following fields were intentionally ignored in the comparison as they are always different and irrelevant to the actual metadata quality:\n",
    );
    let mut sorted_flac_ignored_fields: Vec<_> = flac_ignored_fields_counts.iter().collect();
    sorted_flac_ignored_fields.sort_by(|a, b| a.0.cmp(b.0));
    for (field, count) in sorted_flac_ignored_fields {
        flac_ignored_note.push_str(&format!("- {}: {}\n", field, count));
    }
    if !flac_directory_file_name_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- Directory and File Name Tags:\n");
        let mut sorted_directory_file_name: Vec<_> = flac_directory_file_name_ignored_fields_counts
            .iter()
            .collect();
        sorted_directory_file_name.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_directory_file_name {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !flac_duration_total_samples_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- Duration and Total Samples Tags:\n");
        let mut sorted_duration_total_samples: Vec<_> =
            flac_duration_total_samples_ignored_fields_counts
                .iter()
                .collect();
        sorted_duration_total_samples.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_duration_total_samples {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !flac_file_date_time_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- File Date/Time Tags:\n");
        let mut sorted_file_date_time: Vec<_> =
            flac_file_date_time_ignored_fields_counts.iter().collect();
        sorted_file_date_time.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_file_date_time {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !flac_id3_file_size_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- ID3 and File Size Tags:\n");
        let mut sorted_id3_file_size: Vec<_> =
            flac_id3_file_size_ignored_fields_counts.iter().collect();
        sorted_id3_file_size.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_id3_file_size {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !flac_picture_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- Picture Tags:\n");
        let mut sorted_picture_fields: Vec<_> = flac_picture_ignored_fields_counts.iter().collect();
        sorted_picture_fields.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_picture_fields {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !flac_media_mediatype_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- Media and Mediatype Tags:\n");
        let mut sorted_media_mediatype: Vec<_> =
            flac_media_mediatype_ignored_fields_counts.iter().collect();
        sorted_media_mediatype.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_media_mediatype {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !flac_media_mediatype_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- Media and Mediatype Tags:\n");
        let mut sorted_media_mediatype: Vec<_> =
            flac_media_mediatype_ignored_fields_counts.iter().collect();
        sorted_media_mediatype.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_media_mediatype {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !flac_media_mediatype_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- Media and Mediatype Tags:\n");
        let mut sorted_media_mediatype: Vec<_> =
            flac_media_mediatype_ignored_fields_counts.iter().collect();
        sorted_media_mediatype.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_media_mediatype {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    if !flac_media_mediatype_ignored_fields_counts.is_empty() {
        flac_ignored_note.push_str("- Media and Mediatype Tags:\n");
        let mut sorted_media_mediatype: Vec<_> =
            flac_media_mediatype_ignored_fields_counts.iter().collect();
        sorted_media_mediatype.sort_by(|a, b| a.0.cmp(b.0));
        for (field, count) in sorted_media_mediatype {
            flac_ignored_note.push_str(&format!("  -> {}: {}\n", field, count));
        }
    }
    flac_ignored_note.push_str("\nThese differences occur due to file processing, storage locations, and encoding differences but do not reflect actual metadata discrepancies.\n\n");

    // Generate MP3 differences section
    let mut mp3_section = String::new();
    if !mp3_differences.is_empty() {
        let mut sorted_mp3: Vec<_> = mp3_differences.into_iter().collect();
        sorted_mp3.sort_by(|a, b| a.0.cmp(&b.0)); // Sort alphabetically by field name

        for (i, (field, diffs)) in sorted_mp3.iter().enumerate() {
            mp3_section.push_str("#==================================================\n");
            mp3_section.push_str(&format!(
                "# {}. Field: {}: {} Cases\n",
                i + 1,
                field,
                diffs.len()
            ));
            mp3_section.push_str("#==================================================\n\n");
            for (file_header, diff) in diffs {
                mp3_section.push_str(file_header);
                mp3_section.push_str(diff);
                mp3_section.push('\n');
            }
        }
    } else {
        mp3_section.push_str("No differences found for MP3 format.\n\n");
    }

    // Generate FLAC differences section
    let mut flac_section = String::new();
    if !flac_differences.is_empty() {
        let mut sorted_flac: Vec<_> = flac_differences.into_iter().collect();
        sorted_flac.sort_by(|a, b| a.0.cmp(&b.0)); // Sort alphabetically by field name

        for (i, (field, diffs)) in sorted_flac.iter().enumerate() {
            flac_section.push_str("#==================================================\n");
            flac_section.push_str(&format!(
                "# {}. Field: {}: {} Cases\n",
                i + 1,
                field,
                diffs.len()
            ));
            flac_section.push_str("#==================================================\n\n");
            for (file_header, diff) in diffs {
                flac_section.push_str(file_header);
                flac_section.push_str(diff);
                flac_section.push('\n');
            }
        }
    } else {
        flac_section.push_str("No differences found for FLAC format.\n\n");
    }

    // Write MP3 report
    let mp3_content = format!("{}{}{}\n", mp3_summary, mp3_ignored_note, mp3_section);
    write(mp3_output_file, mp3_content)?;
    println!("MP3 report generated at {}", mp3_output_file);

    // Write FLAC report
    let flac_content = format!("{}{}{}\n", flac_summary, flac_ignored_note, flac_section);
    write(flac_output_file, flac_content)?;
    println!("FLAC report generated at {}", flac_output_file);
    Ok(())
}
