//! Channels to play a [`MixChunk`].

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{marker::PhantomData, os::raw::c_int};

use super::MixChunk;
use crate::{bind, device::MixDevice};

pub use pause::*;

mod pause;

/// Loops on playing in [`PlayOptions`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayLoops {
    /// Playing infinitely.
    Infinite,
    /// Playing only once.
    OneShot,
    /// Playing the specified number of times.
    Times(u32),
}

impl PlayLoops {
    fn into_raw(self) -> c_int {
        match self {
            PlayLoops::Infinite => -1,
            PlayLoops::OneShot => 0,
            PlayLoops::Times(n) => (n - 1) as _,
        }
    }
}

impl Default for PlayLoops {
    fn default() -> Self {
        Self::OneShot
    }
}

/// Options to play a chunk by [`Channel::play`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlayOptions {
    /// Loops on playing.
    pub loops: PlayLoops,
    /// The maximum duration of playing in milliseconds. Stopping by `loops` is prior to by `duration`.
    pub duration: Option<u32>,
}

/// A mixing channel for playing a [`MixChunk`].
#[derive(Debug, PartialEq, Eq)]
pub struct Channel<'device>(i32, PhantomData<&'device MixDevice<'device>>);

impl<'device> Channel<'device> {
    /// Allocates and returns the audio channels.
    pub fn allocate(_device: &'device MixDevice<'device>, num: usize) -> Vec<Self> {
        let allocated = unsafe { bind::Mix_AllocateChannels(num as _) as i32 };
        (0..allocated).map(|id| Self(id, PhantomData)).collect()
    }

    /// Returns the first free mixing channel.
    ///
    /// # Panics
    ///
    /// Panics if the mixing channels are not allocated. Please allocate the channels from [`Channel::allocate`] at first.
    pub fn first_free(_device: &'device MixDevice<'device>) -> Self {
        let allocated = unsafe { bind::Mix_AllocateChannels(-1) as i32 };
        if allocated == 0 {
            panic!("must allocate channels at first");
        }
        Self(-1, PhantomData)
    }

    /// Returns the output volume of the channel. The volume is in `0..=128`.
    pub fn volume(&self) -> u32 {
        unsafe { bind::Mix_Volume(self.0, -1) as _ }
    }

    /// Sets the output volume of the channel. The volume is clamped to `0..=128`.
    pub fn set_volume(&self, volume: u32) {
        let _ = unsafe { bind::Mix_Volume(self.0, volume.clamp(0, 128) as _) };
    }

    /// Starts to play a chunk.
    pub fn play(self, chunk: &MixChunk, options: PlayOptions) -> Result<Self> {
        let id = unsafe {
            bind::Mix_PlayChannelTimed(
                self.0,
                chunk.ptr.as_ptr(),
                options.loops.into_raw(),
                options.duration.map_or(-1, |d| d as _),
            )
        };
        if id == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self(id, PhantomData))
        }
    }

    /// Starts to play a chunk with fade-in time in milliseconds.
    pub fn play_fade_in(
        self,
        chunk: &MixChunk,
        fade_in: u32,
        options: PlayOptions,
    ) -> Result<Self> {
        let id = unsafe {
            bind::Mix_FadeInChannelTimed(
                self.0,
                chunk.ptr.as_ptr(),
                options.loops.into_raw(),
                fade_in as _,
                options.duration.map_or(-1, |d| d as _),
            )
        };
        if id == -1 {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self(id, PhantomData))
        }
    }

    /// Pauses playing and returns the [`Pauser`], or `None` if it is free.
    pub fn pause(&'device mut self) -> Option<Pauser<'device>> {
        Pauser::pause(self)
    }

    /// Halts playing on the channel.
    pub fn halt(&self) {
        let _ = unsafe { bind::Mix_HaltChannel(self.0) };
    }

    /// Halts all the playing channel.
    pub fn halt_all(_device: &'device MixDevice<'device>) {
        let _ = unsafe { bind::Mix_HaltChannel(-1) };
    }
}
