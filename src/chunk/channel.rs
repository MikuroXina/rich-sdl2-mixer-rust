//! Channels to play a [`MixChunk`].

use std::marker::PhantomData;

use crate::{bind, device::MixDevice};

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
}
