# Qobuz API Rust - Metadata Testing and Comparison Framework

## Overview

This framework provides comprehensive testing and comparison of metadata embedding between the Rust and C# implementations of the Qobuz API library. It automates the download of diverse tracks across different genres and eras, extracts metadata using `exiftool`, and compares the results to ensure consistency between implementations.

## Directory Structure

```
metadata_test/
├── docs/
│   ├── analysis.md                 # Implementation analysis
│   ├── README.md                   # This file
│   ├── metadata_test_plan.md       # Initial plan to create this test
├── downloads/            # Downloaded audio files
│   ├── mp3/              # MP3 format files
│   └── flac/             # FLAC format files
|   └── C#-songs/         # Existing C# implementation files for comparison
├── metadata/             # Extracted metadata text files
│   ├── mp3/              # MP3 metadata extractions
│   ├── flac/             # FLAC metadata extractions
│   ├── json/             # Raw JSON metadata extractions
│   └── C#-songs/         # C#-downloaded metadata extractions
├── reports/              # Comparison reports
├── scripts/              # Automation scripts
│   ├── download_tracks.rs          # Download diverse tracks
│   ├── extract_metadata.sh         # Extract metadata with exiftool
│   ├── run_extraction.rs           # Run extraction script from Rust
│   ├── compare_metadata.rs         # Compare metadata implementations
│   └── generate_report.rs          # Generate final reports
├── flac_metadata_report.md         # Generated metadata report for FLAC files
├── mp3_metadata_report.md          # Generated metadata report for MP3 files

```

## Features

1. **Automated Track Download**: Downloads 10 diverse tracks across different genres and eras
2. **Dual Format Support**: Downloads each track in both MP3 and FLAC formats
3. **Metadata Embedding**: Embeds comprehensive metadata using the Rust implementation
4. **Metadata Extraction**: Extracts metadata using `exiftool -G1` for both Rust and C# files
5. **Comparison Framework**: Compares metadata field-by-field between implementations
6. **Reporting**: Generates detailed reports highlighting differences and similarities

## Track Selection

The framework downloads tracks covering these categories:

1. **Hip-Hop/Rap** - Modern (2017-2025)
2. **Rock** - Classic (1960-1980)
3. **Rock** - Modern (190-2000)
4. **Classical** - Orchestral
5. **Classical** - Solo/Chamber
6. **Jazz** - Vintage (1940-1960)
7. **Jazz** - Modern (1970-1990)
8. **Electronic** - Techno/House
9. **Pop** - 1980s
10. **Pop** - 2010s

## Usage

### Prerequisites

- Rust toolchain installed
- `exiftool` installed on your system
- Qobuz API credentials (app ID and secret)

### Running the Full Test Suite

1. **Download Tracks**:
   ```bash
   cd qobuz-api-rust
   cargo run --bin download-tracks
   ```

2. **Extract Metadata**:
   ```bash
   cargo run --bin run-extraction
   ```

3. **Compare Metadata**:
   ```bash
   cargo run --bin compare-metadata
   ```

4. **Generate Reports**:
   ```bash
   cargo run --bin generate-report
   ```

### Using Qobuz Credentials

If you have Qobuz API credentials, you can set them as environment variables to avoid fetching from the web player:

```bash
export QOBUZ_APP_ID="your_app_id"
export QOBUZ_APP_SECRET="your_app_secret"
cargo run --bin download-tracks
```

Without valid credentials, the download step will fail when trying to fetch app ID and secret from the web player.

### Individual Scripts

Each step can be run individually:

- Download diverse tracks: `cargo run --bin download-tracks`
- Extract metadata: `cargo run --bin run-extraction`
- Compare implementations: `cargo run --bin compare-metadata`
- Generate reports: `cargo run --bin generate-report`

## Metadata Fields Handled

The Rust implementation embeds the following metadata fields:

- **Basic**: Title, Album, Artist, Album Artist, Composer
- **Track Info**: Track Number, Total Tracks, Disc Number, Total Discs
- **Dates**: Year, Recording Date, Release Date
- **Identifiers**: ISRC, Copyright, Label
- **Technical**: Sampling Rate, Bit Depth, Duration
- **Classical**: Enhanced composer and performer role handling
- **Cover Art**: Embedded as front cover image
- **Qobuz-specific**: Track ID, Album ID, Commercial URL

## Classical Music Handling

The implementation includes special handling for classical music:

- Advanced performer/artist role mapping
- Proper composer extraction from performer strings
- Album artist extraction for multi-performer recordings
- Consistent handling of composer vs. performer roles

## Comparison Methodology

The comparison framework:

1. Extracts metadata from both Rust and C# implementations using `exiftool -G1`
2. Parses the extracted metadata into structured format
3. Compares field-by-field between implementations
4. Identifies missing, extra, or differently valued fields
5. Generates comprehensive reports

## Expected Outcomes

After running the full test suite:

1. 10 tracks downloaded in both MP3 and FLAC formats
2. Comprehensive metadata extracted for all tracks
3. Detailed comparison reports between formats and implementations
4. Identification of all metadata differences between Rust and C# implementations
5. Improved Rust metadata embedding to match C# implementation
6. Consistent metadata across all track types and formats
7. Enhanced classical music metadata handling
8. Verified compatibility with various music players and taggers

## Configuration

The framework can be configured via the `config.json` file (to be created) with:

- Qobuz API credentials
- Search parameters for each genre/era
- Output directory paths
- Track selection criteria
- Comparison settings for C# vs Rust metadata

## Success Metrics

- All 10 tracks successfully downloaded in both formats
- Metadata extraction successful for all files (Rust and C# versions)
- Identification of all metadata differences between implementations
- Implementation of improvements to achieve metadata parity
- Verification that embedded metadata matches C# implementation exactly
- Successful handling of classical music metadata with multiple artists/composers
- Consistent performer/artist role mapping across all genres

## Potential Challenges Addressed

1. Qobuz API rate limiting during testing
2. Availability of tracks in both MP3 and FLAC formats
3. Differences in API responses between implementations
4. Complex classical music metadata structures
5. Varying metadata formats across different track types
6. Ensuring exact metadata parity with C# implementation
7. Handling of special characters and encoding differences
8. Proper comparison of embedded cover art between implementations