//! The collection of audio effects.

use rich_sdl2_rust::{Result, SdlError};

use super::Effect;
use crate::device::MixDevice;

/// An effect that reverses channels of left and right.
pub fn stereo_reverse<'device>(device: &MixDevice<'device>) -> Result<Effect<'device>> {
    // Original by Ryan C. Gordon (icculus@icculus.org) from SDL_mixer/src/effect_stereoreverse.c
    let spec = device.query();
    if spec.channels != 2 {
        return Err(SdlError::Others {
            msg: "non-stereo stream cannot reverse".into(),
        });
    }
    let swap_before_after = |ch: &mut [u8]| {
        let (a, b) = ch.split_at_mut(ch.len() / 2);
        a.swap_with_slice(b);
    };
    Ok(match spec.format.bit_size {
        8 => Box::new(move |buf: &mut [u8]| buf.chunks_exact_mut(2).for_each(swap_before_after)),
        16 => Box::new(move |buf: &mut [u8]| buf.chunks_exact_mut(4).for_each(swap_before_after)),
        32 => Box::new(move |buf: &mut [u8]| buf.chunks_exact_mut(8).for_each(swap_before_after)),
        _ => return Err(SdlError::UnsupportedFeature),
    })
}
