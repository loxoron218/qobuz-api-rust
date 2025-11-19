# Metadata Comparison Report

## Summary
- Total fields with differences: 7
- Total FLAC metadata differences found: 70
- Ignored differences (always different): 120

## Note on Ignored Fields

The following fields were intentionally ignored in the comparison as they are always different and irrelevant to the actual metadata quality:
- Media / Mediatype: 10
- Directory and File Name Tags:
  -> Directory: 10
  -> File Name: 10
- Duration and Total Samples Tags:
  -> Duration: 10
  -> Total Samples: 10
- File Date/Time Tags:
  -> File Access Date/Time: 10
  -> File Inode Change Date/Time: 10
  -> File Modification Date/Time: 10
- ID3 and File Size Tags:
  -> File Size: 10
- Picture Tags:
  -> Picture Bits Per Pixel: 10
  -> Picture Height: 10
  -> Picture Width: 10

These differences occur due to file processing, storage locations, and encoding differences but do not reflect actual metadata discrepancies.

#==================================================
# 1. Field: Frame Size Max: 10 Cases
#==================================================

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Adele - Hello.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Adele_Hello_FLAC_28014963.txt
Field 'Frame Size Max' differs:
  Rust: 12043
  C#:   14335

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Daft Punk - Around the World.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Daft Punk_Around the World_FLAC_1065482.txt
Field 'Frame Size Max' differs:
  Rust: 15389
  C#:   15532

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Glenn Gould - Aria _ Sarabande (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Glenn Gould_Aria _ Sarabande (Remastered)_FLAC_26890072.txt
Field 'Frame Size Max' differs:
  Rust: 16704
  C#:   17126

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Herbie Hancock - Chameleon (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Herbie Hancock_Chameleon_FLAC_37905.txt
Field 'Frame Size Max' differs:
  Rust: 19307
  C#:   19749

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Kendrick Lamar - BLOOD.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Kendrick Lamar_BLOOD._FLAC_40128300.txt
Field 'Frame Size Max' differs:
  Rust: 10972
  C#:   14011

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Madonna - Like a Virgin.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Madonna_Like a Virgin_FLAC_6004880.txt
Field 'Frame Size Max' differs:
  Rust: 16781
  C#:   16994

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Miles Davis - So What.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Miles Davis_So What_FLAC_13176083.txt
Field 'Frame Size Max' differs:
  Rust: 16454
  C#:   16950

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Nirvana - Smells Like Teen Spirit (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Nirvana_Smells Like Teen Spirit_FLAC_14158543.txt
Field 'Frame Size Max' differs:
  Rust: 21171
  C#:   21379

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Staatskapelle Dresden - Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Staatskapelle Dresden_Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro_FLAC_162673929.txt
Field 'Frame Size Max' differs:
  Rust: 18356
  C#:   18483

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/The Beatles - Hey Jude (Remastered 2015).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/The Beatles_Hey Jude_FLAC_235803319.txt
Field 'Frame Size Max' differs:
  Rust: 20352
  C#:   21233

#==================================================
# 2. Field: Frame Size Min: 10 Cases
#==================================================

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Adele - Hello.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Adele_Hello_FLAC_28014963.txt
Field 'Frame Size Min' differs:
  Rust: 6731
  C#:   14

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Daft Punk - Around the World.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Daft Punk_Around the World_FLAC_1065482.txt
Field 'Frame Size Min' differs:
  Rust: 5559
  C#:   14

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Glenn Gould - Aria _ Sarabande (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Glenn Gould_Aria _ Sarabande (Remastered)_FLAC_26890072.txt
Field 'Frame Size Min' differs:
  Rust: 14231
  C#:   302

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Herbie Hancock - Chameleon (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Herbie Hancock_Chameleon_FLAC_37905.txt
Field 'Frame Size Min' differs:
  Rust: 14377
  C#:   1567

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Kendrick Lamar - BLOOD.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Kendrick Lamar_BLOOD._FLAC_40128300.txt
Field 'Frame Size Min' differs:
  Rust: 7486
  C#:   2738

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Madonna - Like a Virgin.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Madonna_Like a Virgin_FLAC_6004880.txt
Field 'Frame Size Min' differs:
  Rust: 13257
  C#:   8854

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Miles Davis - So What.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Miles Davis_So What_FLAC_13176083.txt
Field 'Frame Size Min' differs:
  Rust: 13357
  C#:   1305

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Nirvana - Smells Like Teen Spirit (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Nirvana_Smells Like Teen Spirit_FLAC_14158543.txt
Field 'Frame Size Min' differs:
  Rust: 18307
  C#:   6278

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Staatskapelle Dresden - Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Staatskapelle Dresden_Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro_FLAC_162673929.txt
Field 'Frame Size Min' differs:
  Rust: 13938
  C#:   1550

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/The Beatles - Hey Jude (Remastered 2015).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/The Beatles_Hey Jude_FLAC_235803319.txt
Field 'Frame Size Min' differs:
  Rust: 16219
  C#:   4898

#==================================================
# 3. Field: Involvedpeople: 10 Cases
#==================================================

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Adele - Hello.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Adele_Hello_FLAC_28014963.txt
Field 'Involvedpeople' exists in C# but not in Rust: Adele

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Daft Punk - Around the World.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Daft Punk_Around the World_FLAC_1065482.txt
Field 'Involvedpeople' exists in C# but not in Rust: Daft Punk, MainArtist - Thomas Bangalter, Composer, Producer, Writer - Guy-Manuel de Homem-Christo, Composer, Writer - Guy-Manuel de Homem Christo, Producer

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Glenn Gould - Aria _ Sarabande (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Glenn Gould_Aria _ Sarabande (Remastered)_FLAC_26890072.txt
Field 'Involvedpeople' exists in C# but not in Rust: Glenn Gould, Piano, Producer - Johann Sebastian Bach, Composer - (Samuel H. Carter, Producer - Stan Tonkel, Recording Engineer - Martin Greenblatt, Recording Engineer - Ray Moore, Recording Engineer - John Johnson, Recording Engineer)

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Herbie Hancock - Chameleon (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Herbie Hancock_Chameleon_FLAC_37905.txt
Field 'Involvedpeople' exists in C# but not in Rust: H. Mason, Composer. - H. Mason, Lyricist. - Bennie Maupin, Performer. - Material, Producer. - Paul Jackson, Acoustic Bass. - Herbie Hancock, Composer. - Herbie Hancock, Lyricist. - Herbie Hancock, Producer. - Herbie Hancock, Keyboards. - Herbie Hancock, Performer. - B. Maupin, Composer. - B. Maupin, Lyricist. - David Rubinson, Producer. - Bill Summers, Percussion. - P. Jackson, Composer. - P. Jackson, Lyricist. - Harvey Mason, Drums.

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Kendrick Lamar - BLOOD.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Kendrick Lamar_BLOOD._FLAC_40128300.txt
Field 'Involvedpeople' exists in C# but not in Rust: Kendrick Lamar, MainArtist, ComposerLyricist - D. Tanenbaum, ComposerLyricist - Bekon, Producer - Anthony 'Top Dawg' Tiffith, Producer

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Madonna - Like a Virgin.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Madonna_Like a Virgin_FLAC_6004880.txt
Field 'Involvedpeople' exists in C# but not in Rust: Bob Ludwig, Masterer - Billy Steinberg, Writer - TOM KELLY, Writer - NILE RODGERS, Producer, Engineer - BERNARD EDWARDS, Bass - Tony Thompson, Drums - Rob Sabino, Synthesizer - Madonna, Vocals, MainArtist - Jason Corsaro, Mixer - Rob Eaton, Engineer

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Miles Davis - So What.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Miles Davis_So What_FLAC_13176083.txt
Field 'Involvedpeople' exists in C# but not in Rust: Miles Davis, Associated Performer, Main Artist, Trumpet, Associated Performer, Trumpet. - Irving Townsend, Producer. - Jimmy Cobb, Drums. - M. Davis, Composer, Lyricist. - Bill Evans, Piano. - Paul Chambers, Bass. - Julian "Cannonball" Adderley, Alto Saxophone. - John Coltrane, Tenor Saxophone

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Nirvana - Smells Like Teen Spirit (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Nirvana_Smells Like Teen Spirit_FLAC_14158543.txt
Field 'Involvedpeople' exists in C# but not in Rust: Butch Vig, Producer, RecordingEngineer - Kurt Cobain, Guitar, Vocalist, ComposerLyricist - Krist Novoselic, ComposerLyricist, BassGuitar - ANDY WALLACE, MixingEngineer - Craig Doubet, MixingSecondEngineer - Dave Grohl, ComposerLyricist, DrumKit - Nirvana, MainArtist

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Staatskapelle Dresden - Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Staatskapelle Dresden_Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro_FLAC_162673929.txt
Field 'Involvedpeople' exists in C# but not in Rust: Wolfgang Amadeus Mozart, Composer - Staatskapelle Dresden, Orchestra, MainArtist - Copyright Control, MusicPublisher - Otmar Suitner, Conductor, MainArtist

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/The Beatles - Hey Jude (Remastered 2015).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/The Beatles_Hey Jude_FLAC_235803319.txt
Field 'Involvedpeople' exists in C# but not in Rust: Giles Martin, Producer, MixingEngineer - John Lennon, ComposerLyricist - Paul Mccartney, ComposerLyricist - Andrew Walter, RecordingEngineer - Simon Gibson, RecordingEngineer - George Martin, Producer - Miles Showell, MasteringEngineer - James Clarke, RecordingEngineer - Sam Okell, MixingEngineer - The Beatles, MainArtist - Matt Mysko, MixingSecondEngineer - Al Sirkett, SoundDesigner

#==================================================
# 4. Field: Itunesadvisory: 10 Cases
#==================================================

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Adele - Hello.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Adele_Hello_FLAC_28014963.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Daft Punk - Around the World.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Daft Punk_Around the World_FLAC_1065482.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Glenn Gould - Aria _ Sarabande (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Glenn Gould_Aria _ Sarabande (Remastered)_FLAC_26890072.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Herbie Hancock - Chameleon (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Herbie Hancock_Chameleon_FLAC_37905.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Kendrick Lamar - BLOOD.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Kendrick Lamar_BLOOD._FLAC_40128300.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 1

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Madonna - Like a Virgin.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Madonna_Like a Virgin_FLAC_6004880.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Miles Davis - So What.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Miles Davis_So What_FLAC_13176083.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Nirvana - Smells Like Teen Spirit (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Nirvana_Smells Like Teen Spirit_FLAC_14158543.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Staatskapelle Dresden - Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Staatskapelle Dresden_Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro_FLAC_162673929.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/The Beatles - Hey Jude (Remastered 2015).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/The Beatles_Hey Jude_FLAC_235803319.txt
Field 'Itunesadvisory' exists in C# but not in Rust: 0

#==================================================
# 5. Field: Organization: 10 Cases
#==================================================

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Adele - Hello.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Adele_Hello_FLAC_28014963.txt
Field 'Organization' exists in C# but not in Rust: XL Recordings

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Daft Punk - Around the World.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Daft Punk_Around the World_FLAC_1065482.txt
Field 'Organization' exists in C# but not in Rust: Parlophone France

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Glenn Gould - Aria _ Sarabande (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Glenn Gould_Aria _ Sarabande (Remastered)_FLAC_26890072.txt
Field 'Organization' exists in C# but not in Rust: Sony Classical

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Herbie Hancock - Chameleon (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Herbie Hancock_Chameleon_FLAC_37905.txt
Field 'Organization' exists in C# but not in Rust: Columbia - Legacy

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Kendrick Lamar - BLOOD.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Kendrick Lamar_BLOOD._FLAC_40128300.txt
Field 'Organization' exists in C# but not in Rust: Aftermath

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Madonna - Like a Virgin.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Madonna_Like a Virgin_FLAC_6004880.txt
Field 'Organization' exists in C# but not in Rust: Sire - Warner Records

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Miles Davis - So What.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Miles Davis_So What_FLAC_13176083.txt
Field 'Organization' exists in C# but not in Rust: Columbia

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Nirvana - Smells Like Teen Spirit (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Nirvana_Smells Like Teen Spirit_FLAC_14158543.txt
Field 'Organization' exists in C# but not in Rust: Geffen

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Staatskapelle Dresden - Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Staatskapelle Dresden_Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro_FLAC_162673929.txt
Field 'Organization' exists in C# but not in Rust: Eterna

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/The Beatles - Hey Jude (Remastered 2015).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/The Beatles_Hey Jude_FLAC_235803319.txt
Field 'Organization' exists in C# but not in Rust: UMC (Universal Music Catalogue)

#==================================================
# 6. Field: Upc: 10 Cases
#==================================================

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Adele - Hello.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Adele_Hello_FLAC_28014963.txt
Field 'Upc' exists in C# but not in Rust: 0634904074067

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Daft Punk - Around the World.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Daft Punk_Around the World_FLAC_1065482.txt
Field 'Upc' exists in C# but not in Rust: 0724384260958

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Glenn Gould - Aria _ Sarabande (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Glenn Gould_Aria _ Sarabande (Remastered)_FLAC_26890072.txt
Field 'Upc' exists in C# but not in Rust: 0886445085471

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Herbie Hancock - Chameleon (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Herbie Hancock_Chameleon_FLAC_37905.txt
Field 'Upc' exists in C# but not in Rust: 0074646512326

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Kendrick Lamar - BLOOD.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Kendrick Lamar_BLOOD._FLAC_40128300.txt
Field 'Upc' exists in C# but not in Rust: 0060255760871

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Madonna - Like a Virgin.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Madonna_Like a Virgin_FLAC_6004880.txt
Field 'Upc' exists in C# but not in Rust: 0093624951117

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Miles Davis - So What.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Miles Davis_So What_FLAC_13176083.txt
Field 'Upc' exists in C# but not in Rust: 5099749522428

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Nirvana - Smells Like Teen Spirit (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Nirvana_Smells Like Teen Spirit_FLAC_14158543.txt
Field 'Upc' exists in C# but not in Rust: 0060253749865

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Staatskapelle Dresden - Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Staatskapelle Dresden_Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro_FLAC_162673929.txt
Field 'Upc' exists in C# but not in Rust: 0885470024561

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/The Beatles - Hey Jude (Remastered 2015).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/The Beatles_Hey Jude_FLAC_235803319.txt
Field 'Upc' exists in C# but not in Rust: 0602458682199

#==================================================
# 7. Field: Url: 10 Cases
#==================================================

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Adele - Hello.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Adele_Hello_FLAC_28014963.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/25-adele/0634904074067

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Daft Punk - Around the World.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Daft Punk_Around the World_FLAC_1065482.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/homework-daft-punk/0724384260958

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Glenn Gould - Aria _ Sarabande (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Glenn Gould_Aria _ Sarabande (Remastered)_FLAC_26890072.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/bach-the-goldberg-variations-1981-gould-remastered-glenn-gould/0886445085471

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Herbie Hancock - Chameleon (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Herbie Hancock_Chameleon_FLAC_37905.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/head-hunters-herbie-hancock/0074646512326

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Kendrick Lamar - BLOOD.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Kendrick Lamar_BLOOD._FLAC_40128300.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/damn-kendrick-lamar/0060255760871

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Madonna - Like a Virgin.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Madonna_Like a Virgin_FLAC_6004880.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/like-a-virgin-hi-res-version-madonna/0093624951117

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Miles Davis - So What.txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Miles Davis_So What_FLAC_13176083.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/kind-of-blue-miles-davis/5099749522428

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Nirvana - Smells Like Teen Spirit (Album Version).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Nirvana_Smells Like Teen Spirit_FLAC_14158543.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/nevermind-nirvana/0060253749865

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/Staatskapelle Dresden - Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro (Remastered).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/Staatskapelle Dresden_Symphony No. 40 in G Minor, K. 550_ I. Molto Allegro_FLAC_162673929.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/mozart-symphony-no-39-40-staatskapelle-dresden-otmar-suitner/h00nx1b5bbu1a

## @/qobuz-api-rust/metadata_test/metadata/C#-songs/flac/The Beatles - Hey Jude (Remastered 2015).txt vs @/qobuz-api-rust/metadata_test/metadata/flac/The Beatles_Hey Jude_FLAC_235803319.txt
Field 'Url' exists in C# but not in Rust: https://www.qobuz.com/fr-fr/album/the-beatles-1967-1970-the-beatles/rlrv742tr79jb


