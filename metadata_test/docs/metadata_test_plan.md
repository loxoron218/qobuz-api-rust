# Qobuz API Rust - Metadata Testing and Comparison Plan

## Overview
This document outlines the plan to automate the download of 10 diverse tracks across different genres and eras, download them in both MP3 and FLAC formats, extract their metadata using `exiftool -G1`, and compare the metadata with the original C# implementation to identify areas for improvement in the Rust implementation.

## Goals
1. Automate the download of 10 tracks representing diverse genres and eras
2. Download each track in both MP3 and FLAC formats simultaneously
3. Extract comprehensive metadata using `exiftool -G1` for each format
4. Compare metadata between Rust and C# implementations
5. Document differences and provide recommendations for improving the Rust implementation
6. Ensure metadata consistency between the Rust implementation and the C# reference implementation

## Track Selection Strategy (Updated)
Select 10 tracks covering:
1. **Hip-Hop/Rap** - Modern (2017-2025)
2. **Rock** - Classic (1960-1980)
3. **Rock** - Modern (1990-2000)
4. **Classical** - Orchestral
5. **Classical** - Solo/Chamber
6. **Jazz** - Vintage (1940-1960)
7. **Jazz** - Modern (1970-1990)
8. **Electronic** - Techno/House
9. **Pop** - 1980s
10. **Pop** - 2010s

### Specific Tracks to Download and Compare with C# Implementation
The following specific tracks should be downloaded and compared with the existing C# implementation files in `@/qobuz-api-rust/metadata_test/C#-songs`:

```rust
// Define search queries for diverse genres and eras
let track_searches = vec![
    // Kendrick Lamar - BLOOD. (Hip-Hop/Rap, Modern)
    ("Kendrick Lamar BLOOD.", "Hip-Hop/Rap", "Modern (2017)"),
    // The Beatles - Hey Jude (Rock, Classic)
    ("The Beatles Hey Jude", "Rock", "Classic (1968)"),
    // Nirvana - Smells Like Teen Spirit (Rock, Modern)
    ("Nirvana Smells Like Teen Spirit", "Rock", "Modern (1991)"),
    // Mozart - Symphony No. 40 (Classical, Orchestral)
    ("Mozart Symphony No. 40", "Classical", "Orchestral"),
    // Bach - Goldberg Variations (Classical, Solo)
    ("Bach Goldberg Variations", "Classical", "Solo"),
    // Miles Davis - Kind of Blue (Jazz, Vintage)
    ("Miles Davis Kind of Blue", "Jazz", "Vintage (1959)"),
    // Herbie Hancock - Head Hunters (Jazz, Modern)
    ("Herbie Hancock Head Hunters", "Jazz", "Modern (1973)"),
    // Daft Punk - Around the World (Electronic)
    ("Daft Punk Around the World", "Electronic", "Modern"),
    // Madonna - Like a Virgin (Pop, 1980s)
    ("Madonna Like a Virgin", "Pop", "1980s"),
    // Adele - Hello (Pop, 2010s)
    ("Adele Hello", "Pop", "2010s"),
];
```

## Implementation Plan

### Phase 1: Automated Download System
Create a Rust application that:
- Uses the Qobuz API service to search for and select tracks matching our criteria
- Downloads each track in both MP3 and FLAC formats
- Organizes files with a consistent naming convention:
  - `{artist_name} - {track_title} - {format} - {track_id}.{ext}`
  - Example: `Kendrick Lamar - BLOOD. - FLAC - 40128300.flac`
- Ensures metadata embedding matches the C# implementation exactly

### Phase 2: Metadata Extraction System
Create a script that:
- Runs `exiftool -G1` on each downloaded file
- Saves output to `.txt` files with corresponding names:
  - `{artist_name} - {track_title} - {format} - {track_id}.txt`
  - Example: `Kendrick Lamar - BLOOD. - FLAC - 40128300.txt`
- Also extracts metadata from the C#-downloaded files for comparison

### Phase 3: Comparison Framework
Develop a comparison system that:
- Compares metadata between Rust-embedded and C#-embedded versions
- Generates detailed reports highlighting differences
- Identifies missing or incorrectly formatted metadata fields
- Specifically focuses on classical music metadata handling differences
- Compares performer/artist role mapping between implementations

### Phase 4: Improvement Implementation
Based on comparison results:
- Update the Rust metadata embedding logic to match C# implementation
- Ensure identical metadata embedding for all track types
- Add any missing metadata fields or improve existing ones
- Enhance classical music metadata handling to match C# behavior
- Improve performer/artist role mapping consistency

## Technical Implementation

### Directory Structure
```
metadata_test/
├── downloads/
│   ├── mp3/
│   └── flac/
├── metadata/
│   ├── mp3/
│   └── flac/
├── reports/
├── scripts/
│   ├── download_tracks.rs
│   ├── extract_metadata.sh
│   ├── compare_metadata.rs
│   └── generate_report.rs
├── C#-songs/  # Existing C# implementation files for comparison
└── config.json
```

### Download Script (Rust)
The download script will:
1. Initialize the Qobuz API service
2. Search for tracks matching our genre/era criteria
3. For each track, get file URLs for both MP3 and FLAC formats
4. Download both formats simultaneously using async operations
5. Embed metadata using the existing `embed_metadata_in_file` function
6. Log download status and any errors
7. Verify that embedded metadata matches C# implementation behavior

### Metadata Extraction Script (Shell/Rust)
The extraction script will:
1. Iterate through all downloaded files (both Rust and C# versions)
2. Run `exiftool -G1` on each file
3. Save output to corresponding text files
4. Verify successful extraction
5. Create a standardized format for comparison

### Comparison Script (Rust)
The comparison script will:
1. Parse metadata from text files
2. Compare field-by-field between formats and implementations
3. Generate difference reports
4. Highlight missing or differently formatted fields
5. Specifically focus on classical music metadata differences
6. Identify inconsistencies in performer/artist role mapping

## Rust Implementation Details

### Enhanced Metadata Embedding
The current Rust implementation already handles:
- Basic tags (title, album, artist, composer)
- Cover art embedding
- ISRC and copyright information
- Date fields (year, release date)
- Track and disc numbers
- Genre and label information

Potential improvements needed (based on C# comparison):
- More sophisticated composer handling for classical music
- Better performer/artist role mapping consistency with C# implementation
- Additional technical metadata fields to match C# output
- Consistency with C# implementation regarding classical music metadata
- Proper handling of multiple artists and composers in classical music
- Ensuring album artist field matches C# behavior
- Consistent date formatting and field usage

### Testing Configuration
Create a `config.json` file with:
- Qobuz API credentials
- Search parameters for each genre/era
- Output directory paths
- Track selection criteria
- Comparison settings for C# vs Rust metadata

## Expected Outcomes

### Short-term
1. 10 tracks downloaded in both MP3 and FLAC formats
2. Comprehensive metadata extracted for all tracks
3. Detailed comparison reports between formats and implementations
4. Identification of all metadata differences between Rust and C# implementations

### Long-term
1. Improved Rust metadata embedding to match C# implementation
2. Consistent metadata across all track types and formats
3. Enhanced classical music metadata handling
4. Better performer/artist role mapping
5. Verified compatibility with various music players and taggers
6. Complete metadata parity between Rust and C# implementations

## Success Metrics
- All 10 tracks successfully downloaded in both formats
- Metadata extraction successful for all files (Rust and C# versions)
- Identification of all metadata differences between implementations
- Implementation of improvements to achieve metadata parity
- Verification that embedded metadata matches C# implementation exactly
- Successful handling of classical music metadata with multiple artists/composers
- Consistent performer/artist role mapping across all genres

## Potential Challenges
1. Qobuz API rate limiting during testing
2. Availability of tracks in both MP3 and FLAC formats
3. Differences in API responses between implementations
4. Complex classical music metadata structures
5. Varying metadata formats across different track types
6. Ensuring exact metadata parity with C# implementation
7. Handling of special characters and encoding differences
8. Proper comparison of embedded cover art between implementations