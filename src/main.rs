use audiotags::{self, Tag};
use std::env::{self};
use std::io;
use std::path::{self, Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("Error: No arguments provided");
        return;
    };

    let path = Path::new(&args[1]);
    if !path.is_file() {
        println!("Error: Not a file");
        return;
    };

    let file_name = path.file_name().unwrap().to_str().unwrap();

    let mut tag = match Tag::new().read_from_path(path) {
        Ok(t) => t,
        Err(_) => {
            println!("Error: Invalid file type");
            return;
        }
    };

    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut modified = false;
    clearscreen::clear().expect("Failed to clear screen");

    //Main loop
    loop {
        println!("{}",file_name);
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
        println!("9) Exit Without Saving");
        println!("0) Save and Exit");
        println!();

        let selection: char = rl
            .readline("Selection? ")
            .unwrap_or("_".to_string())
            .parse()
            .unwrap_or('_');

        match selection {
            '1' => {
                let old_title = tag.title().unwrap_or("");
                let new_title = rl
                    .readline_with_initial("Title: ", (old_title, ""))
                    .expect("");
                if old_title == new_title {continue;}
                modified = true;
                tag.set_title(&new_title);
            }
            '2' => {
                let old_artist = tag.artist().unwrap_or("");
                let new_artist = rl
                    .readline_with_initial("Artist: ", (old_artist, ""))
                    .expect("");
                if old_artist == new_artist {continue;}
                tag.set_artist(&new_artist);
                modified = true
            }
            '3' => {
                let old_album_artist = tag.album_artist().unwrap_or("");
                let new_album_artist = rl
                    .readline_with_initial("Album Artist: ", (old_album_artist, ""))
                    .expect("");
                if old_album_artist == new_album_artist {continue;}
                tag.set_album_artist(&new_album_artist);
                modified = true
            }
            '4' => {
                let old_album = tag.album_title().unwrap_or("");
                let new_album = rl
                    .readline_with_initial("Album: ", (old_album, ""))
                    .expect("");
                if old_album == new_album {continue;}
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
                if old_year == new_year {continue;}
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
                if old_number != new_number {continue;}
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
            '9' => {
                if !modified {
                    break;
                };

                println!("Are you sure? y/N");
                let mut selection = String::new();
                match io::stdin().read_line(&mut selection) {
                    Ok(_) => {}
                    Err(_) => {
                        continue;
                    }
                }
                let selection: char = match selection.trim().parse() {
                    Ok(sel) => sel,
                    Err(_) => {
                        clearscreen::clear().expect("Failed to clear screen");
                        println!("Error: Invalid input\n");
                        continue;
                    }
                };

                match selection {
                    'y' => break,
                    _ => {}
                }
            }
            '0' => match tag.write_to_path(path.to_str().expect("")) {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    clearscreen::clear().expect("Failed to clear screen");
                    println!("WARNING!");
                    println!("Failed to write tag data. Returning to selection");
                    println!("Please verify the provided file still exists\n");
                    continue;
                }
            },
            _ => println!("Error: Invalid option"),
        };
        clearscreen::clear().expect("Failed to clear screen");
    }
    println!("Closing Program");
    return;
}
