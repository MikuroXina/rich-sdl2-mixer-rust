//! Audio formats for [`super::MixMusic`].

use crate::bind;

/// A type of audio format supported by [`MixMusic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicType {
    /// A player from the command specified by [`MixMusic::with_cmd`].
    Command,
    /// A wave format.
    Wave,
    /// A mod format.
    Mod,
    /// A midi format.
    Midi,
    /// A ogg vorbis format.
    Ogg,
    /// A mpeg-1 audio layer-3 format.
    Mp3,
    /// A flac format.
    Flac,
    /// A opus format.
    Opus,
    /// An unknown format.
    Unknown,
}

impl MusicType {
    pub(super) fn from_raw(raw: bind::Mix_MusicType) -> Self {
        use MusicType::*;
        match raw {
            bind::Mix_MusicType_MUS_CMD => Command,
            bind::Mix_MusicType_MUS_WAV => Wave,
            bind::Mix_MusicType_MUS_MOD => Mod,
            bind::Mix_MusicType_MUS_MID => Midi,
            bind::Mix_MusicType_MUS_OGG => Ogg,
            bind::Mix_MusicType_MUS_MP3 => Mp3,
            bind::Mix_MusicType_MUS_FLAC => Flac,
            bind::Mix_MusicType_MUS_OPUS => Opus,
            _ => Unknown,
        }
    }

    pub(super) fn convert_pos(&self, pos: f64) -> f64 {
        match *self {
            MusicType::Mod => pos.floor(),
            MusicType::Ogg => pos,
            MusicType::Mp3 => {
                unsafe {
                    bind::Mix_RewindMusic();
                }
                pos
            }
            _ => 0.0,
        }
    }
}
