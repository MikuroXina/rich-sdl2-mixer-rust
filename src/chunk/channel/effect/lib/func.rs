use rich_sdl2_rust::{audio::format::AudioFormatFlag, Result, SdlError};

use super::{Effect, PositionArgs};
use crate::device::MixSpec;

mod conv;
mod rotate;

// TODO (MikuroXina): Improve u8 case with table method
macro_rules! position {
    ($target:ty, 2ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(2 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 2];
                    $crate::converter!($target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(2ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    ($target:ty, 4ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(4 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 4];
                    $crate::converter!($target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(4ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    ($target:ty, 6ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(6 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 6];
                    $crate::converter!($target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(6ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Lsb, $target:ty, 2ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(2 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 2];
                    $crate::converter!(Lsb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(2ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Lsb, $target:ty, 4ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(4 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 4];
                    $crate::converter!(Lsb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(4ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Lsb, $target:ty, 6ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(6 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 6];
                    $crate::converter!(Lsb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(6ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Msb, $target:ty, 2ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(2 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 2];
                    $crate::converter!(Msb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(2ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Msb, $target:ty, 4ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(4 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 4];
                    $crate::converter!(Msb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(4ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
    (Msb, $target:ty, 6ch, $args:ident) => {
        Box::new(move |buf: &mut [u8]| {
            buf.chunks_exact_mut(6 * std::mem::size_of::<$target>())
                .for_each(|ch| {
                    let mut gained = [0.0; 6];
                    $crate::converter!(Msb, $target, ch, gained, {
                        apply_gains(&$args, &mut gained);
                        $crate::rotate!(6ch, $args.room_angle.0, gained);
                    })
                })
        })
    };
}

pub(super) fn select_fn<'device>(format: MixSpec, args: PositionArgs) -> Result<Effect<'device>> {
    let channels = format.channels;
    let bit_size = format.format.bit_size;
    let is_signed = format.format.flag.contains(AudioFormatFlag::SIGNED);
    let is_msb = format.format.flag.contains(AudioFormatFlag::BIG_ENDIAN);
    if format.format.flag.contains(AudioFormatFlag::FLOAT) {
        return Ok(match channels {
            1 | 2 => position!(f32, 2ch, args),
            4 => position!(f32, 4ch, args),
            6 => position!(f32, 6ch, args),
            _ => return Err(SdlError::UnsupportedFeature),
        });
    }
    Ok(match (channels, bit_size, is_signed, is_msb) {
        (1 | 2, 8, false, _) => position!(u8, 2ch, args),
        (4, 8, false, _) => position!(u8, 4ch, args),
        (6, 8, false, _) => position!(u8, 6ch, args),
        (1 | 2, 8, true, _) => position!(i8, 2ch, args),
        (4, 8, true, _) => position!(i8, 4ch, args),
        (6, 8, true, _) => position!(i8, 6ch, args),
        (1 | 2, 16, false, false) => position!(Lsb, u16, 2ch, args),
        (4, 16, false, false) => position!(Lsb, u16, 4ch, args),
        (6, 16, false, false) => position!(Lsb, u16, 6ch, args),
        (1 | 2, 16, true, false) => position!(Lsb, i16, 2ch, args),
        (4, 16, true, false) => position!(Lsb, i16, 4ch, args),
        (6, 16, true, false) => position!(Lsb, i16, 6ch, args),
        (1 | 2, 16, false, true) => position!(Msb, u16, 2ch, args),
        (4, 16, false, true) => position!(Msb, u16, 4ch, args),
        (6, 16, false, true) => position!(Msb, u16, 6ch, args),
        (1 | 2, 16, true, true) => position!(Msb, i16, 2ch, args),
        (4, 16, true, true) => position!(Msb, i16, 4ch, args),
        (6, 16, true, true) => position!(Msb, i16, 6ch, args),
        (1 | 2, 32, _, true) => position!(Msb, i32, 2ch, args),
        (4, 32, _, true) => position!(Msb, i32, 4ch, args),
        (6, 32, _, true) => position!(Msb, i32, 6ch, args),
        (1 | 2, 32, _, false) => position!(Lsb, i32, 2ch, args),
        (4, 32, _, false) => position!(Lsb, i32, 4ch, args),
        (6, 32, _, false) => position!(Lsb, i32, 6ch, args),
        _ => return Err(SdlError::UnsupportedFeature),
    })
}

#[inline(always)]
fn apply_gains(args: &PositionArgs, gained: &mut [f64]) {
    for (out, &gain) in gained.iter_mut().zip(args.gains.iter()) {
        *out *= gain;
        *out *= args.distance;
    }
}
