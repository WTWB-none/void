use super::metadata::get_metadata;
use crate::filesystem::{fs_errors::FsError, secure_fs::check_scope};
use lofty::probe::Probe;
use std::{fs, path::PathBuf};
use void_entities::{audio::AudioMetadata, config::GlobalConfig};

/// # simple safe function that reads audio file
/// requires mutable reference to GlobalConfig and PathBuf reference that points to audio file
/// returnes lofty TaggedFile for later manipulations or FsError::GetError(_s) where _s is exact
/// error that internal process returned
/// ```
/// use void_entities::config::GlobalConfig;
/// use void_core::audio::get_audio;
/// use std::path::PathBuf;
///
/// let mut config = GlobalConfig::default();
/// config.change_scope(PathBuf::from("/home/transhumanist/Downloads"));
/// let vec = get_audio(&mut config, &PathBuf::from("/home/transhumanist/Downloads/01.DearDiary.flac")).unwrap();
/// ```
pub fn get_audio(
    config: &mut GlobalConfig,
    audio_path: &PathBuf,
) -> Result<AudioMetadata, FsError> {
    check_scope(config, audio_path)?;
    let _file_to_metadata = Probe::open(audio_path)
        .map_err(|e| FsError::GetError(e.to_string()))?
        .guess_file_type()
        .map_err(|e| FsError::GetError(e.to_string()))?
        .read()
        .map_err(|e| FsError::GetError(e.to_string()))?;
    let mut meta = get_metadata(_file_to_metadata).map_err(|e| FsError::GetError(e.to_string()))?;
    meta.file(
        fs::read(audio_path)
            .map_err(|e| FsError::GetError(e.to_string()))?
            .to_vec(),
    );

    Ok(meta)
}
