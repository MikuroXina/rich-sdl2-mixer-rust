//! Effect processing for the channel, such as panning, decaying, etc.

use std::{ffi::c_void, os::raw::c_int};

use super::Channel;
use crate::bind;

pub mod lib;

/// An effect to process the audio buffer in the channel.
pub type Effect = Box<dyn FnMut(&mut [u8])>;

/// An extension for attaching an effect.
pub trait EffectAttachExt {
    /// Attaches the effect to the channel.
    fn attach(&self, effect: Effect);

    /// Detaches all the effect from the channel.
    fn detach_all(&self);
}

impl EffectAttachExt for Channel<'_> {
    fn attach(&self, effect: Effect) {
        let wrapped = Box::new(effect);
        let raw = Box::into_raw(wrapped);
        let _ = unsafe {
            bind::Mix_RegisterEffect(
                self.0,
                Some(effect_attach_effect_handler),
                Some(effect_attach_done_handler),
                raw.cast(),
            )
        };
    }

    fn detach_all(&self) {
        let _ = unsafe { bind::Mix_UnregisterAllEffects(self.0) };
    }
}

/// Attaches the effect to the special, post effect channel.
pub fn attach_post_effect(effect: Effect) {
    let wrapped = Box::new(effect);
    let raw = Box::into_raw(wrapped);
    let _ = unsafe {
        bind::Mix_RegisterEffect(
            bind::MIX_CHANNEL_POST,
            Some(effect_attach_effect_handler),
            Some(effect_attach_done_handler),
            raw.cast(),
        )
    };
}

/// Detaches all the effect from the special, post effect channel.
pub fn detach_all_post_effect() {
    let _ = unsafe { bind::Mix_UnregisterAllEffects(bind::MIX_CHANNEL_POST) };
}

extern "C" fn effect_attach_effect_handler(
    _: c_int,
    stream: *mut c_void,
    len: c_int,
    userdata: *mut c_void,
) {
    let callback = unsafe { &mut *(userdata as *mut Effect) };
    let stream = unsafe { std::slice::from_raw_parts_mut(stream.cast(), len as _) };
    callback(stream);
}

extern "C" fn effect_attach_done_handler(_: c_int, userdata: *mut c_void) {
    let _ = unsafe { Box::from_raw(userdata as *mut Effect) };
}
