use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    tag::Accessor,
};
use void_entities::audio::AudioMetadata;

/// # simple function that extracts metadata from audio file
/// this function can only be visible inside audio crate. Basically if you need some of file metadata
/// you could simply use get_audio() function which returns `AudioMetadata` structure from which you could
/// get title, duration, picture and artist meta.
/// if meta not found it didn't return error. All of metadata fields except file would be None
pub(crate) fn get_metadata(file: TaggedFile) -> AudioMetadata {
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
            meta
        }
        None => meta,
    }
}
