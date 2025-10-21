use crate::filesystem::fs_errors::FsError;
use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    tag::Accessor,
};
use void_entities::audio::AudioMetadata;

pub fn get_metadata(file: TaggedFile) -> Result<AudioMetadata, FsError> {
    let tags = file.primary_tag();
    let mut meta = AudioMetadata::default();
    meta.duration(file.properties().duration());
    match tags {
        Some(tags) => {
            if let Some(p) = tags.pictures().first() {
                meta.picture(p.data().to_vec());
            };
            if let Some(t) = tags.title() {
                meta.title(t.to_string());
            };
            if let Some(a) = tags.artist() {
                meta.artist(a.to_string());
            }
            Ok(meta)
        }
        None => Ok(meta),
    }
}
