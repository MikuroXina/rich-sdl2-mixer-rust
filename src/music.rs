//! The music control that are suitable to background music.

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{ffi::CString, marker::PhantomData, ptr::NonNull};

use crate::{bind, device::MixDevice};

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
}

impl Drop for MixMusic<'_> {
    fn drop(&mut self) {
        unsafe { bind::Mix_FreeMusic(self.ptr.as_ptr()) }
    }
}
