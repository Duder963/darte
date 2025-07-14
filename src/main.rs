use audiotags::{AudioTag, Tag};
use std::env::args;
use std::path::PathBuf;

struct MusicFile {
    path: PathBuf,
    tag: Box<dyn AudioTag + Send + Sync>,
}

fn main() {
    let rl = rustyline::DefaultEditor::new().expect("Rustyline exploded");
    let args: Vec<String> = args().skip(1).collect();

    if args.len() == 1 {
        let path = PathBuf::from(&args[0]);
        if path.is_dir() {
            let mut music_files = Vec::<MusicFile>::new();
            let dir_files = match path.read_dir() {
                Ok(d) => d,
                Err(_) => {
                    println!("Error: Failed to read directory");
                    return;
                }
            };
            for file in dir_files {
                let file_path = file.unwrap().path();
                let tag = Tag::new().read_from_path(&file_path).unwrap();
                let music_file = MusicFile {
                    path: file_path,
                    tag: tag,
                };
                music_files.push(music_file);
            }
            if music_files.len() > 0 {
                edit_multiple_files(music_files, rl)
            } else {
                println!("Error: No files in directory");
                return;
            }
        } else if path.is_file() {
            let music_file = MusicFile {
                path: path.to_path_buf(),
                tag: Tag::new().read_from_path(path).unwrap(),
            };
            edit_single_file(music_file, rl);
        }
    } else if args.len() > 1 {
        println!("TODO: Multiple files");
    } else {
        println!("Error: No arguments provided");
        return;
    }

    println!("Closing Program");
    return;
}

fn edit_single_file(music_file: MusicFile, mut rl: rustyline::DefaultEditor) {
    let mut modified = false;
    let file_name = music_file.path.file_name().unwrap().to_str().unwrap();
    let mut tag = music_file.tag;

    clearscreen::clear().expect("Failed to clear screen");

    loop {
        println!("File: {file_name}");
        println!("1) Title: {}", tag.title().unwrap_or("<none>"));
        println!("2) Artist: {}", tag.artist().unwrap_or("<none>"));
        println!(
            "3) Album Artist: {}",
            tag.album_artist().unwrap_or("<none>")
        );
        println!("4) Album: {}", tag.album_title().unwrap_or("<none>"));
        println!("5) Year: {}", tag.year().unwrap_or_default());
        println!(
            "6) Track #: {}/{}",
            tag.track_number().unwrap_or_default(),
            tag.total_tracks().unwrap_or_default()
        );
        println!("7) Genre: {}", tag.genre().unwrap_or("<none>"));
        println!("8) Comment: {}", tag.comment().unwrap_or("<none>"));
        println!();
        println!("9) Save");
        println!("0) Exit");
        println!();

        let selection: char = rl
            .readline("Selection? ")
            .unwrap_or_default()
            .parse()
            .unwrap_or('_');

        match selection {
            '1' => {
                let old_title = tag.title().unwrap_or("");
                let new_title = rl
                    .readline_with_initial("Title: ", (old_title, ""))
                    .expect("");
                if old_title == new_title {
                    continue;
                }
                modified = true;
                tag.set_title(&new_title);
            }

            '2' => {
                let old_artist = tag.artist().unwrap_or("");
                let new_artist = rl
                    .readline_with_initial("Artist: ", (old_artist, ""))
                    .expect("");
                if old_artist == new_artist {
                    continue;
                }
                tag.set_artist(&new_artist);
                modified = true
            }

            '3' => {
                let old_album_artist = tag.album_artist().unwrap_or("");
                let new_album_artist = rl
                    .readline_with_initial("Album Artist: ", (old_album_artist, ""))
                    .expect("");
                if old_album_artist == new_album_artist {
                    continue;
                }
                tag.set_album_artist(&new_album_artist);
                modified = true
            }

            '4' => {
                let old_album = tag.album_title().unwrap_or("");
                let new_album = rl
                    .readline_with_initial("Album: ", (old_album, ""))
                    .expect("");
                if old_album == new_album {
                    continue;
                }
                tag.set_album_title(&new_album);
                modified = true
            }

            '5' => {
                let old_year = tag.year().unwrap_or(0);
                let new_year = rl
                    .readline_with_initial("Year: ", (&old_year.to_string(), ""))
                    .expect("");
                let new_year: i32 = match new_year.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        clearscreen::clear().expect("Failed to clear screen");
                        println!("Error: Not a number\n");
                        continue;
                    }
                };
                if old_year == new_year {
                    continue;
                }
                tag.set_year(new_year);
                modified = true
            }

            '6' => {
                let old_number = tag.track_number().unwrap_or(0);
                let new_number = rl
                    .readline_with_initial("Track #: ", (&old_number.to_string(), ""))
                    .expect("");
                let new_number: u16 = match new_number.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        clearscreen::clear().expect("Failed to clear screen");
                        println!("Error: Not a number\n");
                        continue;
                    }
                };
                if old_number != new_number {
                    tag.set_track_number(new_number);
                    modified = true;
                }

                let old_total = tag.total_tracks().unwrap_or(0);
                let new_total = rl
                    .readline_with_initial("Total Tracks: ", (&old_total.to_string(), ""))
                    .expect("");
                let new_total: u16 = match new_total.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        clearscreen::clear().expect("Failed to clear screen");
                        println!("Error: Not a number\n");
                        continue;
                    }
                };
                if old_number != new_number {
                    continue;
                }
                tag.set_total_tracks(new_total);
                modified = true
            }

            '7' => {
                let old_genre = tag.genre().unwrap_or("");
                let new_genre = rl
                    .readline_with_initial("Genre: ", (old_genre, ""))
                    .expect("");
                tag.set_genre(&new_genre);
                modified = true
            }

            '8' => {
                let old_comment = tag.comment().unwrap_or("");
                let new_comment = rl
                    .readline_with_initial("Comment: ", (old_comment, ""))
                    .expect("");
                tag.set_comment(new_comment);
                modified = true
            }

            '9' => match tag.write_to_path(music_file.path.to_str().expect("")) {
                Ok(_) => {
                    modified = false;
                    clearscreen::clear().expect("Failed to clear screen");
                    println!("Save Successful!");
                    continue;
                }
                Err(_) => {
                    clearscreen::clear().expect("Failed to clear screen");
                    println!("WARNING!");
                    println!("Failed to write tag data. Returning to selection");
                    println!("Please verify the provided file still exists\n");
                    continue;
                }
            },

            '0' => {
                if !modified {
                    break;
                };

                println!("You have unsaved changes!");
                let selection: char = rl
                    .readline("Are you sure? y/N ")
                    .unwrap_or("n".to_string())
                    .parse()
                    .unwrap_or('n');
                match selection {
                    'y' | 'Y' => break,
                    _ => {}
                }
            }
            _ => println!("Error: Invalid option"),
        };
        clearscreen::clear().expect("Failed to clear screen");
    }
}

fn edit_multiple_files(mut music_files: Vec<MusicFile>, mut rl: rustyline::DefaultEditor) {
    let mut modified = false;

    let mut group_artist = music_files[0].tag.artist().unwrap_or("<none>").to_string();
    let mut group_album_artist = music_files[0]
        .tag
        .album_artist()
        .unwrap_or("<none>")
        .to_string();
    let mut group_album = music_files[0]
        .tag
        .album_title()
        .unwrap_or("<none>")
        .to_string();
    let mut group_year = music_files[0].tag.year().unwrap_or(0);
    let mut group_total_tracks = music_files[0].tag.total_tracks().unwrap_or(0);

    for file in music_files.iter().skip(1) {
        if group_artist != "<differs>" && file.tag.artist().unwrap_or("<none>") != group_artist {
            group_artist = "<differs>".to_string();
        }
        if group_album_artist != "<differs>"
            && file.tag.album_artist().unwrap_or("<none>") != group_album_artist
        {
            group_album_artist = "<differs>".to_string();
        }
        if group_album != "<differs>" && file.tag.album_title().unwrap_or("<none>") != group_album {
            group_album = "<differs>".to_string();
        }
        if group_year != 0 && file.tag.year().unwrap_or(0) != group_year {
            group_year = 0;
        }
        if group_total_tracks != 0 && file.tag.total_tracks().unwrap_or(0) != group_total_tracks {
            group_total_tracks = 0;
        }
    }
    loop {
        println!("1) Artist: {group_artist}");
        println!("2) Album Artist: {group_album_artist}");
        println!("3) Album: {group_album}");
        println!("4) Year: {group_year}");
        println!("5) Total Tracks: {group_total_tracks}");
        println!();
        println!("9) Save");
        println!("0) Exit");
        println!();
        println!("Batch Operations");
        println!("Edit All Sequentially");

        let selection: char = rl
            .readline("Selection? ")
            .unwrap_or_default()
            .parse()
            .unwrap_or('_');

        match selection {
            '1' => {
                let new_artist = rl
                    .readline("Artist: ")
                    .expect("");
                if group_artist == new_artist {
                    continue;
                }
                group_artist = new_artist.to_string();
                for file in music_files.iter_mut() {
                    file.tag.set_artist(&new_artist)
                }
                modified = true;
            }

            '2' => {
                let new_album_artist = rl
                    .readline("Album Artist: ")
                    .expect("");
                if group_album_artist == new_album_artist {
                    continue;
                }
                group_album_artist = new_album_artist.to_string();
                for file in music_files.iter_mut() {
                    file.tag.set_album_artist(&new_album_artist)
                }
                modified = true;
            }

            '3' => {
                let new_album = rl
                    .readline("Album: ")
                    .expect("");
                if group_album == new_album {
                    continue;
                }
                group_album = new_album.to_string();
                for file in music_files.iter_mut() {
                    file.tag.set_album_title(&new_album)
                }
                modified = true;
            }

            '4' => {
                let new_year = rl
                    .readline("Year: ")
                    .expect("");
                let new_year: i32 = match new_year.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        clearscreen::clear().expect("Failed to clear screen");
                        println!("Error: Not a number\n");
                        continue;
                    }
                };
                if group_year == new_year {
                    continue;
                }
                group_year = new_year;
                for file in music_files.iter_mut() {
                    file.tag.set_year(group_year)
                }
                modified = true;
            }

            '5' => {
                let new_total_tracks = rl
                    .readline("Total Tracks: ")
                    .expect("");
                let new_total_tracks: u16 = match new_total_tracks.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        clearscreen::clear().expect("Failed to clear screen");
                        println!("Error: Not a number\n");
                        continue;
                    }
                };
                if group_total_tracks == new_total_tracks {
                    continue;
                }
                group_total_tracks = new_total_tracks;
                for file in music_files.iter_mut() {
                    file.tag.set_total_tracks(group_total_tracks)
                }
                modified = true;
            }

            '9' => {
                for music_file in &mut music_files {
                    match music_file.tag.write_to_path(music_file.path.to_str().expect("")) {
                        Ok(_) => {
                            modified = false;
                            clearscreen::clear().expect("Failed to clear screen");
                            println!("Save Successful!");
                            continue;
                        }
                        Err(_) => {
                            clearscreen::clear().expect("Failed to clear screen");
                            println!("WARNING!");
                            println!("Failed to write tag data. Returning to selection");
                            println!("Please verify the provided file still exists\n");
                            continue;
                        }
                    }
                }
            },

            '0' => {
                if !modified {
                    break;
                };

                println!("You have unsaved changes!");
                let selection: char = rl
                    .readline("Are you sure? y/N ")
                    .unwrap_or("n".to_string())
                    .parse()
                    .unwrap_or('n');
                match selection {
                    'y' | 'Y' => break,
                    _ => {}
                }
            },
            _ => println!("Error: Invalid option"),
        };
    }
}
