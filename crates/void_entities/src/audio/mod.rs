use std::{default::Default, time::Duration};

#[derive(Default)]
pub struct AudioMetadata {
    file: Option<Vec<u8>>,
    picture: Option<Vec<u8>>,
    artist: Option<String>,
    duration: Option<Duration>,
    title: Option<String>,
}

impl AudioMetadata {
    pub fn file(&mut self, audio: Vec<u8>) {
        self.file = Some(audio)
    }

    /// sets audio meta picture
    pub fn picture(&mut self, picture_data: Vec<u8>) {
        self.picture = Some(picture_data);
    }

    /// sets audio meta artist
    pub fn artist(&mut self, artist: String) {
        self.artist = Some(artist);
    }

    /// sets audio meta duration
    pub fn duration(&mut self, audio_duration: Duration) {
        self.duration = Some(audio_duration);
    }

    /// sets audio meta title
    pub fn title(&mut self, audio_title: String) {
        self.title = Some(audio_title);
    }
}
