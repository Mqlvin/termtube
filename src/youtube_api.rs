use serde::{Deserialize, Deserializer};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

const ID_LENGTH: usize = 11; // Length of YouTube video ID
const M3U8_CACHE_SECS: u64 = 5 * 60 * 60; // Cache each URL for 5 hours (6 is max)

#[derive(Deserialize, Debug)]
pub struct YouTubeSong {
    pub title: Option<String>,
    pub id: String,
    pub uploader_id: Option<String>,
    #[serde(deserialize_with = "deserialize_uploaded_at")]
    pub uploaded_at: Option<u32>,
    #[serde(deserialize_with = "deserialize_duration")]
    pub duration: Option<u16>,
    #[serde(deserialize_with = "deserialize_is_live")]
    pub is_live: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub m3u8_url: Option<String>,
    #[serde(skip_serializing)]
    pub m3u8_url_age: Option<u64>,
}

impl YouTubeSong {
    pub fn get_m3u8(&mut self, ytdlp_path: &String) -> Option<String> {
        if self.m3u8_url.is_some() {
            // url cached
            if self.m3u8_url_age.is_some()
                && self.m3u8_url_age.unwrap() + M3U8_CACHE_SECS > get_unix_secs()
            {
                return self.m3u8_url.clone();
            } // if URL cached but no time (somehow) OR if URL cache expired, gen URL
        } // if no URL cached, gen URL

        let ytdlp_output = Command::new(ytdlp_path)
            .arg(&self.id)
            .arg("-f")
            .arg("233")
            .arg("--print")
            .arg("{\x01title\x01:\x01%(title)s\x01, \x01id\x01:\x01%(id)s\x01, \x01uploader_id\x01:\x01%(uploader)s\x01, \x01uploaded_at\x01:\x01%(timestamp)s\x01, \x01duration\x01:\x01%(duration)s\x01, \x01is_live\x01:\x01%(is_live)s\x01, \x01m3u8_url\x01:\x01%(url)s\x01}")
            .output();

        let new_object: Option<YouTubeSong>;

        match ytdlp_output {
            Ok(output_object) => {
                if output_object.status.code().unwrap() != 0 {
                    println!(" ERR: Song lookup error (video `{}` may not exist or is private) (ytdlp:{})", self.id, output_object.status.code().unwrap());
                    return None;
                }

                let stdout_vec = String::from_utf8(output_object.stdout);
                if let Ok(term_response) = stdout_vec {
                    if let Ok(new_object_) = serde_json::from_str(
                        term_response
                            .replace("\"", "'")
                            .replace("\x01", "\"")
                            .as_str(),
                    ) {
                        new_object = new_object_;
                    } else {
                        println!(" ERR: Song lookup error: invalid ytdlp response (not json)");
                        return None;
                    }
                } else {
                    println!(" ERR: Song lookup error: error parsing stdout into string");
                    return None;
                }
            }
            Err(err) => {
                println!(
                    " ERR: Song lookup error (video `{}` may not exist or is private) (err:{})",
                    self.id, err
                );
                return None;
            }
        }

        if let Some(obj) = new_object {
            self.title = obj.title;
            self.uploader_id = obj.uploader_id;
            self.uploaded_at = obj.uploaded_at;
            self.duration = obj.duration;
            self.is_live = obj.is_live;

            self.m3u8_url = obj.m3u8_url.clone();
            self.m3u8_url_age = Some(get_unix_secs());
            return Some(obj.m3u8_url.unwrap());
        }

        None
    }
}

fn deserialize_uploaded_at<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    match buf.trim().parse::<u32>() {
        Ok(num) => Ok(Some(num)),
        Err(_) => Ok(Some(0)),
    }
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    match buf.parse::<u16>() {
        Ok(num) => Ok(Some(num)),
        Err(_) => Ok(None),
    }
}

fn deserialize_is_live<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    match buf.to_lowercase().as_str() {
        "true" => Ok(Some(true)),
        "false" => Ok(Some(false)),
        _ => Ok(None),
    }
}

pub fn make_song_object(id_val: &str) -> Option<YouTubeSong> {
    Some(YouTubeSong {
        duration: None,
        id: id_val.to_string(),
        is_live: None,
        title: None,
        m3u8_url: None,
        m3u8_url_age: None,
        uploaded_at: None,
        uploader_id: None,
    })
}

// have to request all the songs from youtube and make a list of empty song objects to return
pub fn make_song_objects(id: &str, ytdlp_path: &str) -> Option<Vec<YouTubeSong>> {
    let ytdlp_output = Command::new(ytdlp_path)
        .arg(id)
        .arg("--flat-playlist")
        .arg("--print")
        .arg(r#"%(id)s"#)
        .output();

    match ytdlp_output {
        Ok(val) => {
            if val.status.code().unwrap() != 0 {
                println!(" ERR: Playlist lookup error (playlist `{}` may not exist or is private) (ytdlp:{})", id, val.status.code().unwrap());
                return None;
            }

            let stdout = String::from_utf8(val.stdout);
            if let Ok(info) = stdout {
                // if stdout valid string
                let mut objects = Vec::new();
                for line in info.split('\n') {
                    objects.push(YouTubeSong {
                        duration: None,
                        id: line.to_string(),
                        is_live: None,
                        title: None,
                        m3u8_url: None,
                        m3u8_url_age: None,
                        uploaded_at: None,
                        uploader_id: None,
                    });
                }

                Some(objects)
            } else {
                println!(" ERR: Playlist lookup error: error parsing stdout into string");
                None
            }
        }
        Err(err) => {
            println!(
                " ERR: Playlist lookup error (playlist `{}` may not exist or is private) (err:{})",
                id, err
            );
            None
        }
    }
}

pub fn get_video_id(url: &Url) -> Option<String> {
    if url.path().starts_with("/watch") {
        // ?v=video_id
        if let Some(name) = url.query_pairs().find(|(key, _)| key == "v") {
            if name.1.len() != ID_LENGTH {
                println!(" ERR: YouTube URL {} is malformed", url);
                return None;
            }
            Some((name.1).to_string())
        } else {
            println!(
                " ERR: Malformed URL error (no `v` parameter in url path `{}`)",
                url.path()
            );
            None
        }
    } else {
        // youtu.be/<video_id>
        if url.path().len() - 1 != ID_LENGTH {
            println!(" ERR: YouTube URL {} is malformed", url);
        }
        Some(url.path()[1..ID_LENGTH + 1].to_string())
    }
}

pub fn get_playlist_id(url: &Url) -> Option<String> {
    if let Some(list_id) = url.query_pairs().find(|(key, _)| key == "list") {
        return Some(list_id.1.to_string());
    }

    None
}

fn get_unix_secs() -> u64 {
    return match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_secs(),
        Err(_) => 0,
    };
}
