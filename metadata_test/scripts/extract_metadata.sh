#!/bin/bash

# Extract metadata from audio files using exiftool -G1
# This script processes both Rust-downloaded and C#-downloaded files

# Check if exiftool is installed
if ! command -v exiftool &> /dev/null; then
    echo "Error: exiftool is not installed. Please install it first."
    exit 1
fi

# Create directories for metadata output
mkdir -p metadata_test/metadata/mp3
mkdir -p metadata_test/metadata/flac
mkdir -p metadata_test/metadata/C#-songs/mp3
mkdir -p metadata_test/metadata/C#-songs/flac

echo "Extracting metadata from Rust-downloaded MP3 files..."
for file in metadata_test/downloads/mp3/*.mp3; do
    if [ -f "$file" ]; then
        filename=$(basename "$file" .mp3)
        output_file="metadata_test/metadata/mp3/${filename}.txt"
        echo "Processing: $file"
        exiftool -G1 "$file" > "$output_file"
    fi
done

echo "Extracting metadata from Rust-downloaded FLAC files..."
for file in metadata_test/downloads/flac/*.flac; do
    if [ -f "$file" ]; then
        filename=$(basename "$file" .flac)
        output_file="metadata_test/metadata/flac/${filename}.txt"
        echo "Processing: $file"
        exiftool -G1 "$file" > "$output_file"
    fi
done

echo "Extracting metadata from C#-downloaded MP3 files..."
for file in metadata_test/downloads/C#-songs/mp3/*; do
    if [ -f "$file" ] && [[ "$file" == *.mp3 ]]; then
        filename=$(basename "$file")
        output_file="metadata_test/metadata/C#-songs/mp3/${filename%.*}.txt"
        echo "Processing C# MP3: $file"
        exiftool -G1 "$file" > "$output_file"
    fi
done

echo "Extracting metadata from C#-downloaded FLAC files..."
for file in metadata_test/downloads/C#-songs/flac/*.flac; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        output_file="metadata_test/metadata/C#-songs/flac/${filename%.*}.txt"
        echo "Processing C# FLAC: $file"
        exiftool -G1 "$file" > "$output_file"
    fi
done

echo "Metadata extraction completed."