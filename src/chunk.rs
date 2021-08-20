//! Audio sampling buffer controls.

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ptr::NonNull,
};

use crate::{bind, device::MixDevice};

/// A chunk buffer of the audio data.
pub struct MixChunk<'device> {
    ptr: NonNull<bind::Mix_Chunk>,
    _phantom: PhantomData<&'device MixDevice<'device>>,
}

impl<'device> MixChunk<'device> {
    /// Constructs a chunk from the file name, or `Err` on failure.
    ///
    /// # Panics
    ///
    /// Panics if `file_name` is empty.
    pub fn new(_device: &'device MixDevice<'device>, file_name: &str) -> Result<Self> {
        let read_binary_mode = CStr::from_bytes_with_nul(b"rb\0").unwrap();
        let cstr = CString::new(file_name).expect("file_name must no be empty");
        let ptr = unsafe {
            bind::Mix_LoadWAV_RW(
                bind::SDL_RWFromFile(cstr.as_ptr(), read_binary_mode.as_ptr()),
                1,
            )
        };
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

impl Drop for MixChunk<'_> {
    fn drop(&mut self) {
        unsafe { bind::Mix_FreeChunk(self.ptr.as_ptr()) }
    }
}
