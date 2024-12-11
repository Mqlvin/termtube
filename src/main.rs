use crate::playlist::Playlist;
use crate::youtube_api::{
    get_playlist_id, get_video_id, make_song_object, make_song_objects, YouTubeSong,
};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use url::Url;

mod playlist;
mod youtube_api;

#[derive(Parser)]
#[command(version, about = "\n\n\t** A simple command-line application to play headless YouTube audio **", long_about = None)]
struct CliArgs {
    /// Override the default yt-dlp binary path
    #[arg(long = "yt-dlp", default_value = "yt-dlp")]
    ytdlp_path: String,

    /// Override the default ffplay binary path
    #[arg(long = "ffplay", default_value = "ffplay")]
    ffplay_path: String,

    /// Provide a YouTube video URL, or a file containing newline
    /// delimited video URLs. You can use this flag multiple times.
    #[arg(short = 's', long = "source", required = true, default_missing_value = None)]
    sources: Vec<String>,

    /// Loop the playlist once all songs are finished.
    #[arg(long = "loop", action)]
    should_loop: bool,

    /// Randomise the playlist song order.
    #[arg(long = "shuffle", action)]
    should_shuffle: bool,
}

fn main() {
    let mut playlist_obj = Playlist {
        songs: Vec::new(),
        active_idx: 0,
    };

    let args = CliArgs::parse();

    // transform args
    for source in args.sources.iter() {
        let url_obj = Url::parse(source);
        match url_obj {
            Ok(val) => {
                // The str is a URL
                handle_youtube_url(&mut playlist_obj, &val, &args.ytdlp_path);
            }

            Err(_) => {
                // Error when parsing str as URL, so probably a file path

                let file = File::open(source);
                if file.is_err() {
                    println!(" ERR: Couldn't load source file '{}', skipping...", source);
                    continue;
                }

                let f_reader = BufReader::new(file.unwrap());
                for line in f_reader.lines() {
                    if let Ok(val) = line {
                        // This set of code is identical to the URL parsing above
                        let uncommented_value = val.split("#").next().unwrap_or(&val);

                        let url_obj = Url::parse(&uncommented_value);

                        if let Err(_) = url_obj {
                            println!(
                                " ERR: Error parsing the URL '{}' in the file '{}'",
                                uncommented_value, source
                            );
                            continue;
                        }

                        handle_youtube_url(&mut playlist_obj, &url_obj.unwrap(), &args.ytdlp_path);
                        continue;
                    }
                    println!(" ERR: Error reading file lines...");
                }
            }
        } // close match
    }

    println!("\n");
    println!(" INFO: Loaded {} song objects...", playlist_obj.songs.len());

    loop {
        if !playlist_obj.play_next_song(
            &args.ffplay_path,
            &args.ytdlp_path,
            args.should_loop,
            args.should_shuffle,
        ) {
            break;
        }
    }
}

fn handle_youtube_url(playlist: &mut Playlist, url: &Url, ytdlp_path: &str) -> () {
    if let Some(playlist_id) = get_playlist_id(url) {
        // returns None if not a playlist URL
        if let Some(generated_objects) = make_song_objects(playlist_id.as_str(), ytdlp_path) {
            for object in generated_objects {
                playlist.add_youtube_song(object);
            }
        } else {
            // Error message provided by `get_playlist_id` above
            // println!("Invalid playlist ID `{}` (playlist doesn't exist or is private)", playlist_id);
        }

        // it doesn't return if needs to be added to playlist
        return;
    }

    if let Some(video_id) = get_video_id(url) {
        // returns None if not a video URL
        if let Some(generated_object) = make_song_object(video_id.as_str()) {
            playlist.add_youtube_song(generated_object);
        } else {
            // Error message provided by `get_video_id` above
            // println!("Invalid video ID `{}` (video doesn't exist or is private)", video_id);
        }
    }
}
