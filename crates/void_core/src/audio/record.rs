/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::audio::recorder_errors::RecorderError;
use crate::filesystem::secure_fs::check_scope;
use std::path::PathBuf;
use void_audio::recorder::Recorder;
use void_config::GlobalConfig;

/// # small function that initialize recorder instance for later usage.
/// It requires PathBuf that points to new file in which recording will be saved.
/// I think that it's strateforward that I try to separate core VOID logic outside of core entities.
/// Maybe I need a trait for this so new contributors could create their own realization for maybe
/// new output file formats instead of simple wav implementation.
/// ```ignore
/// let mut recorder = init_recorder(&PathBuf::from("/home/transhumanist/test_audio.wav"));
/// start_record(&mut recorder).unwrap();
/// stop_record(&mut recorder).unwrap();
/// ```
pub fn init_recorder(
    config: &mut GlobalConfig,
    destination: &PathBuf,
) -> Result<Recorder, RecorderError> {
    let mut scope_check_path = destination.clone();
    scope_check_path.pop();
    check_scope(config, &scope_check_path).map_err(|s| RecorderError::InitError(s.to_string()))?;
    Ok(Recorder::new(destination.into()))
}

/// # starts recording
pub fn start_record(recorder: &mut Recorder) -> Result<(), RecorderError> {
    recorder
        .start_recording()
        .map_err(|e| RecorderError::StartError(e.to_string()))?;
    Ok(())
}

/// # stops recording
pub fn stop_record(recorder: &mut Recorder) -> Result<(), RecorderError> {
    recorder
        .stop_recording()
        .map_err(|e| RecorderError::StopError(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};
    use void_config::GlobalConfig;

    use crate::filesystem::files::delete_file;

    use super::*;

    #[test]
    fn perform_record() {
        let mut config = GlobalConfig::default();
        config.change_scope(PathBuf::from("/home/transhumanist/"));
        let mut recorder = init_recorder(
            &mut config,
            &PathBuf::from("/home/transhumanist/test_audio.wav"),
        )
        .unwrap();
        start_record(&mut recorder).unwrap();
        sleep(Duration::from_secs(2));
        stop_record(&mut recorder).unwrap();
        delete_file(
            &mut config,
            &PathBuf::from("/home/transhumanist/test_audio.wav"),
        )
        .unwrap();
    }
}
