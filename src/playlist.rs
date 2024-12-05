use crate::YouTubeSong;
use rand::Rng;
use std::process::Command;
use std::process::Stdio;

pub struct Playlist {
    pub songs: Vec<YouTubeSong>,
    pub active_idx: usize,
}

impl Playlist {
    pub fn add_youtube_song(&mut self, obj: YouTubeSong) {
        self.songs.push(obj);
    }

    // get next song, right now linear, returns false is loop should exit
    pub fn play_next_song(
        &mut self,
        ffplay_path: &String,
        ytdlp_path: &String,
        is_looped: bool,
        is_shuffled: bool,
    ) -> bool {
        if self.songs.is_empty() {
            println!(" ERR: No songs are in the playlist... exiting");
            return false;
        }

        if is_shuffled {
            // do this here so first song doesnt play first if shuffled
            self.active_idx = rand::thread_rng().gen();
            println!("{}", self.active_idx);
        }
        self.active_idx %= self.songs.len();

        let m3u8_url = self
            .songs
            .get_mut(self.active_idx)
            .unwrap()
            .get_m3u8(ytdlp_path);

        if let None = m3u8_url {
            // error message already given... just remove the song

            self.songs.remove(self.active_idx);
            println!(" INFO: Song removed from in-memory playlist");

            return true;
        }

        println!(
            " INFO: Playing next song: {}",
            self.songs
                .get_mut(self.active_idx)
                .unwrap()
                .title
                .as_mut()
                .unwrap()
                .to_string()
        );

        let _ffplay_process = Command::new(ffplay_path)
            .arg("-nodisp")
            .arg("-autoexit")
            .arg("-vn")
            .arg(m3u8_url.unwrap()) // we know its safe from earlier
            .stdout(Stdio::null())
            .output()
            .expect("Failed to run the binary");

        self.active_idx += 1; // increment it for next song

        if self.active_idx == self.songs.len() {
            // if this is the last song played

            if !is_looped {
                return false;
            }
        }
        true
    }
}
