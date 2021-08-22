//! The collection of audio effects.

use rich_sdl2_rust::{Result, SdlError};

use self::{
    func::select_fn,
    gain::{gains, Gain},
    room_angle::RoomAngle,
};
use super::Effect;
use crate::device::MixDevice;

mod func;
mod gain;
mod room_angle;

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

#[derive(Debug, Default)]
struct PositionArgs {
    room_angle: RoomAngle,
    gains: [Gain; 6],
    distance: Gain,
    channels: u32,
}

/// An effect that controls the playing position.
/// `angle` is clamped in `0..360`.
pub fn position<'device>(
    device: &MixDevice<'device>,
    angle: i16,
    distance: u8,
) -> Result<Effect<'device>> {
    // Original by Ryan C. Gordon (icculus@icculus.org) from SDL_mixer/src/effect_position.c
    let format = device.query();
    let channels = format.channels;
    let angle = angle.clamp(0, 359);
    let room_angle = RoomAngle::new(channels, angle);
    // flip to scale uses
    let distance = 255 - distance;
    let gains = gains(channels, angle, room_angle);
    let args = PositionArgs {
        room_angle,
        gains,
        distance: distance.into(),
        channels,
    };
    select_fn(format, args)
}
