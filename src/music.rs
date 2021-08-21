//! The music control that are suitable to background music.

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{ffi::CString, marker::PhantomData, ptr::NonNull};

use self::ty::MusicType;
use crate::{bind, device::MixDevice};

pub mod ty;

/// A music buffer of the audio data.
pub struct MixMusic<'device> {
    ptr: NonNull<bind::Mix_Music>,
    _phantom: PhantomData<&'device MixDevice<'device>>,
}

impl<'device> MixMusic<'device> {
    /// Constructs a music from the file, or `Err` on failure.
    ///
    /// # Panics
    ///
    /// Panics if `file_name` is empty.
    pub fn new(_device: &'device MixDevice<'device>, file_name: &str) -> Result<Self> {
        let cstr = CString::new(file_name).expect("file_name must not be empty");
        let ptr = unsafe { bind::Mix_LoadMUS(cstr.as_ptr()) };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the type of the music.
    pub fn music_type(&self) -> MusicType {
        let raw = unsafe { bind::Mix_GetMusicType(self.ptr.as_ptr()) };
        MusicType::from_raw(raw)
    }

    /// Constructs a music from the file with the custom player command, or `Err` on failure.
    ///
    /// # Panics
    ///
    /// Panics if `file_name` or `command` is empty.
    pub fn with_cmd(
        device: &'device MixDevice<'device>,
        file_name: &str,
        command: &str,
    ) -> Result<Self> {
        let cmd_cstr = CString::new(command).expect("cmd must not be empty");
        let ret = unsafe { bind::Mix_SetMusicCMD(cmd_cstr.as_ptr()) };
        if ret == -1 {
            return Err(SdlError::Others { msg: Sdl::error() });
        }
        Self::new(device, file_name)
    }

    /// Plays the music. If a music is already playing, it synchronously waits until the music ends. If `loops` is `None`, the play continues infinitely.
    pub fn play(&self, loops: Option<u32>) -> Result<()> {
        let ret = unsafe { bind::Mix_PlayMusic(self.ptr.as_ptr(), loops.map_or(-1, |n| n as _)) };
        if ret == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(())
        }
    }
}

impl Drop for MixMusic<'_> {
    fn drop(&mut self) {
        unsafe {
            bind::Mix_SetMusicCMD(std::ptr::null());
            bind::Mix_FreeMusic(self.ptr.as_ptr());
        }
    }
}
