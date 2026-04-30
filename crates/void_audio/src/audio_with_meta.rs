/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

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
    /// sets audio file (as bytes)
    pub fn file(&mut self, audio: Vec<u8>) {
        info!("get binary audio file");
        self.file = Some(audio)
    }

    /// sets audio meta picture
    pub fn picture(&mut self, picture_data: Vec<u8>) {
        info!("get audio file picture");
        self.picture = Some(picture_data);
    }

    /// sets audio meta artist
    pub fn artist(&mut self, artist: String) {
        info!("get audio file artist");
        self.artist = Some(artist);
    }

    /// sets audio meta duration
    pub fn duration(&mut self, audio_duration: Duration) {
        info!("get audio file duration");
        self.duration = Some(audio_duration);
    }

    /// sets audio meta title
    pub fn title(&mut self, audio_title: String) {
        info!("get audio file title");
        self.title = Some(audio_title);
    }
}
