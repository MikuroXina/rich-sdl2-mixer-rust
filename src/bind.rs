/* automatically generated by rust-bindgen 0.59.1 */

//! Rust FFI to `SDL_mixer.h`

#![allow(warnings)]

pub const MIX_MAJOR_VERSION: u32 = 2;
pub const MIX_MINOR_VERSION: u32 = 0;
pub const MIX_PATCHLEVEL: u32 = 4;
pub const MIX_CHANNELS: u32 = 8;
pub const MIX_DEFAULT_FREQUENCY: u32 = 44100;
pub const MIX_DEFAULT_FORMAT: u32 = 32784;
pub const MIX_DEFAULT_CHANNELS: u32 = 2;
pub const MIX_MAX_VOLUME: u32 = 128;
pub const MIX_CHANNEL_POST: i32 = -2;
pub const MIX_EFFECTSMAXSPEED: &'static [u8; 20usize] = b"MIX_EFFECTSMAXSPEED\0";
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___sbuf() {
    assert_eq!(
        ::std::mem::size_of::<__sbuf>(),
        16usize,
        concat!("Size of: ", stringify!(__sbuf))
    );
    assert_eq!(
        ::std::mem::align_of::<__sbuf>(),
        8usize,
        concat!("Alignment of ", stringify!(__sbuf))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sbuf>()))._base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sbuf),
            "::",
            stringify!(_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sbuf>()))._size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sbuf),
            "::",
            stringify!(_size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
#[test]
fn bindgen_test_layout___sFILE() {
    assert_eq!(
        ::std::mem::size_of::<__sFILE>(),
        152usize,
        concat!("Size of: ", stringify!(__sFILE))
    );
    assert_eq!(
        ::std::mem::align_of::<__sFILE>(),
        8usize,
        concat!("Alignment of ", stringify!(__sFILE))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._p as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_p)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._r as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_r)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._w as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_w)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._file as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_file)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._bf as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_bf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._lbfsize as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_lbfsize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._cookie as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_cookie)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._close as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_close)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._read as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_read)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._seek as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_seek)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._write as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_write)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._ub as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ub)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._extra as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_extra)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._ur as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ur)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._ubuf as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ubuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._nbuf as *const _ as usize },
        119usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_nbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._lb as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_lb)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._blksize as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_blksize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sFILE>()))._offset as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_offset)
        )
    );
}
pub type FILE = __sFILE;
pub const SDL_bool_SDL_FALSE: SDL_bool = 0;
pub const SDL_bool_SDL_TRUE: SDL_bool = 1;
pub type SDL_bool = ::std::os::raw::c_uint;
pub type Uint8 = u8;
pub type Sint16 = i16;
pub type Uint16 = u16;
pub type Uint32 = u32;
pub type Sint64 = i64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_RWops {
    pub size: ::std::option::Option<unsafe extern "C" fn(context: *mut SDL_RWops) -> Sint64>,
    pub seek: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut SDL_RWops,
            offset: Sint64,
            whence: ::std::os::raw::c_int,
        ) -> Sint64,
    >,
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut SDL_RWops,
            ptr: *mut ::std::os::raw::c_void,
            size: size_t,
            maxnum: size_t,
        ) -> size_t,
    >,
    pub write: ::std::option::Option<
        unsafe extern "C" fn(
            context: *mut SDL_RWops,
            ptr: *const ::std::os::raw::c_void,
            size: size_t,
            num: size_t,
        ) -> size_t,
    >,
    pub close: ::std::option::Option<
        unsafe extern "C" fn(context: *mut SDL_RWops) -> ::std::os::raw::c_int,
    >,
    pub type_: Uint32,
    pub hidden: SDL_RWops__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SDL_RWops__bindgen_ty_1 {
    pub stdio: SDL_RWops__bindgen_ty_1__bindgen_ty_1,
    pub mem: SDL_RWops__bindgen_ty_1__bindgen_ty_2,
    pub unknown: SDL_RWops__bindgen_ty_1__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_RWops__bindgen_ty_1__bindgen_ty_1 {
    pub autoclose: SDL_bool,
    pub fp: *mut FILE,
}
#[test]
fn bindgen_test_layout_SDL_RWops__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<SDL_RWops__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<SDL_RWops__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1__bindgen_ty_1>())).autoclose as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(autoclose)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1__bindgen_ty_1>())).fp as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(fp)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_RWops__bindgen_ty_1__bindgen_ty_2 {
    pub base: *mut Uint8,
    pub here: *mut Uint8,
    pub stop: *mut Uint8,
}
#[test]
fn bindgen_test_layout_SDL_RWops__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<SDL_RWops__bindgen_ty_1__bindgen_ty_2>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<SDL_RWops__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1__bindgen_ty_2>())).base as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1__bindgen_ty_2>())).here as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(here)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1__bindgen_ty_2>())).stop as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(stop)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_RWops__bindgen_ty_1__bindgen_ty_3 {
    pub data1: *mut ::std::os::raw::c_void,
    pub data2: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_SDL_RWops__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<SDL_RWops__bindgen_ty_1__bindgen_ty_3>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<SDL_RWops__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1__bindgen_ty_3>())).data1 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(data1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1__bindgen_ty_3>())).data2 as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(data2)
        )
    );
}
#[test]
fn bindgen_test_layout_SDL_RWops__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<SDL_RWops__bindgen_ty_1>(),
        24usize,
        concat!("Size of: ", stringify!(SDL_RWops__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<SDL_RWops__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(SDL_RWops__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1>())).stdio as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1),
            "::",
            stringify!(stdio)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1>())).mem as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1),
            "::",
            stringify!(mem)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops__bindgen_ty_1>())).unknown as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops__bindgen_ty_1),
            "::",
            stringify!(unknown)
        )
    );
}
#[test]
fn bindgen_test_layout_SDL_RWops() {
    assert_eq!(
        ::std::mem::size_of::<SDL_RWops>(),
        72usize,
        concat!("Size of: ", stringify!(SDL_RWops))
    );
    assert_eq!(
        ::std::mem::align_of::<SDL_RWops>(),
        8usize,
        concat!("Alignment of ", stringify!(SDL_RWops))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops>())).size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops>())).seek as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops),
            "::",
            stringify!(seek)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops>())).read as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops),
            "::",
            stringify!(read)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops>())).write as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops),
            "::",
            stringify!(write)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops>())).close as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops),
            "::",
            stringify!(close)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops>())).type_ as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_RWops>())).hidden as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_RWops),
            "::",
            stringify!(hidden)
        )
    );
}
extern "C" {
    pub fn SDL_RWFromFile(
        file: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
    ) -> *mut SDL_RWops;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDL_version {
    pub major: Uint8,
    pub minor: Uint8,
    pub patch: Uint8,
}
#[test]
fn bindgen_test_layout_SDL_version() {
    assert_eq!(
        ::std::mem::size_of::<SDL_version>(),
        3usize,
        concat!("Size of: ", stringify!(SDL_version))
    );
    assert_eq!(
        ::std::mem::align_of::<SDL_version>(),
        1usize,
        concat!("Alignment of ", stringify!(SDL_version))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_version>())).major as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_version),
            "::",
            stringify!(major)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_version>())).minor as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_version),
            "::",
            stringify!(minor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDL_version>())).patch as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(SDL_version),
            "::",
            stringify!(patch)
        )
    );
}
extern "C" {
    pub fn Mix_Linked_Version() -> *const SDL_version;
}
pub const MIX_InitFlags_MIX_INIT_FLAC: MIX_InitFlags = 1;
pub const MIX_InitFlags_MIX_INIT_MOD: MIX_InitFlags = 2;
pub const MIX_InitFlags_MIX_INIT_MP3: MIX_InitFlags = 8;
pub const MIX_InitFlags_MIX_INIT_OGG: MIX_InitFlags = 16;
pub const MIX_InitFlags_MIX_INIT_MID: MIX_InitFlags = 32;
pub const MIX_InitFlags_MIX_INIT_OPUS: MIX_InitFlags = 64;
pub type MIX_InitFlags = ::std::os::raw::c_uint;
extern "C" {
    pub fn Mix_Init(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_Quit();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mix_Chunk {
    pub allocated: ::std::os::raw::c_int,
    pub abuf: *mut Uint8,
    pub alen: Uint32,
    pub volume: Uint8,
}
#[test]
fn bindgen_test_layout_Mix_Chunk() {
    assert_eq!(
        ::std::mem::size_of::<Mix_Chunk>(),
        24usize,
        concat!("Size of: ", stringify!(Mix_Chunk))
    );
    assert_eq!(
        ::std::mem::align_of::<Mix_Chunk>(),
        8usize,
        concat!("Alignment of ", stringify!(Mix_Chunk))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Mix_Chunk>())).allocated as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Mix_Chunk),
            "::",
            stringify!(allocated)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Mix_Chunk>())).abuf as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Mix_Chunk),
            "::",
            stringify!(abuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Mix_Chunk>())).alen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Mix_Chunk),
            "::",
            stringify!(alen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Mix_Chunk>())).volume as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Mix_Chunk),
            "::",
            stringify!(volume)
        )
    );
}
pub const Mix_Fading_MIX_NO_FADING: Mix_Fading = 0;
pub const Mix_Fading_MIX_FADING_OUT: Mix_Fading = 1;
pub const Mix_Fading_MIX_FADING_IN: Mix_Fading = 2;
pub type Mix_Fading = ::std::os::raw::c_uint;
pub const Mix_MusicType_MUS_NONE: Mix_MusicType = 0;
pub const Mix_MusicType_MUS_CMD: Mix_MusicType = 1;
pub const Mix_MusicType_MUS_WAV: Mix_MusicType = 2;
pub const Mix_MusicType_MUS_MOD: Mix_MusicType = 3;
pub const Mix_MusicType_MUS_MID: Mix_MusicType = 4;
pub const Mix_MusicType_MUS_OGG: Mix_MusicType = 5;
pub const Mix_MusicType_MUS_MP3: Mix_MusicType = 6;
pub const Mix_MusicType_MUS_MP3_MAD_UNUSED: Mix_MusicType = 7;
pub const Mix_MusicType_MUS_FLAC: Mix_MusicType = 8;
pub const Mix_MusicType_MUS_MODPLUG_UNUSED: Mix_MusicType = 9;
pub const Mix_MusicType_MUS_OPUS: Mix_MusicType = 10;
pub type Mix_MusicType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mix_Music {
    _unused: [u8; 0],
}
pub type Mix_Music = _Mix_Music;
extern "C" {
    pub fn Mix_OpenAudio(
        frequency: ::std::os::raw::c_int,
        format: Uint16,
        channels: ::std::os::raw::c_int,
        chunksize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_OpenAudioDevice(
        frequency: ::std::os::raw::c_int,
        format: Uint16,
        channels: ::std::os::raw::c_int,
        chunksize: ::std::os::raw::c_int,
        device: *const ::std::os::raw::c_char,
        allowed_changes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_AllocateChannels(numchans: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_QuerySpec(
        frequency: *mut ::std::os::raw::c_int,
        format: *mut Uint16,
        channels: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_LoadWAV_RW(src: *mut SDL_RWops, freesrc: ::std::os::raw::c_int) -> *mut Mix_Chunk;
}
extern "C" {
    pub fn Mix_LoadMUS(file: *const ::std::os::raw::c_char) -> *mut Mix_Music;
}
extern "C" {
    pub fn Mix_LoadMUS_RW(src: *mut SDL_RWops, freesrc: ::std::os::raw::c_int) -> *mut Mix_Music;
}
extern "C" {
    pub fn Mix_LoadMUSType_RW(
        src: *mut SDL_RWops,
        type_: Mix_MusicType,
        freesrc: ::std::os::raw::c_int,
    ) -> *mut Mix_Music;
}
extern "C" {
    pub fn Mix_QuickLoad_WAV(mem: *mut Uint8) -> *mut Mix_Chunk;
}
extern "C" {
    pub fn Mix_QuickLoad_RAW(mem: *mut Uint8, len: Uint32) -> *mut Mix_Chunk;
}
extern "C" {
    pub fn Mix_FreeChunk(chunk: *mut Mix_Chunk);
}
extern "C" {
    pub fn Mix_FreeMusic(music: *mut Mix_Music);
}
extern "C" {
    pub fn Mix_GetNumChunkDecoders() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetChunkDecoder(index: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_HasChunkDecoder(name: *const ::std::os::raw::c_char) -> SDL_bool;
}
extern "C" {
    pub fn Mix_GetNumMusicDecoders() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetMusicDecoder(index: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_HasMusicDecoder(name: *const ::std::os::raw::c_char) -> SDL_bool;
}
extern "C" {
    pub fn Mix_GetMusicType(music: *const Mix_Music) -> Mix_MusicType;
}
extern "C" {
    pub fn Mix_GetMusicTitle(music: *const Mix_Music) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_GetMusicTitleTag(music: *const Mix_Music) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_GetMusicArtistTag(music: *const Mix_Music) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_GetMusicAlbumTag(music: *const Mix_Music) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_GetMusicCopyrightTag(music: *const Mix_Music) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_SetPostMix(
        mix_func: ::std::option::Option<
            unsafe extern "C" fn(
                udata: *mut ::std::os::raw::c_void,
                stream: *mut Uint8,
                len: ::std::os::raw::c_int,
            ),
        >,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Mix_HookMusic(
        mix_func: ::std::option::Option<
            unsafe extern "C" fn(
                udata: *mut ::std::os::raw::c_void,
                stream: *mut Uint8,
                len: ::std::os::raw::c_int,
            ),
        >,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn Mix_HookMusicFinished(music_finished: ::std::option::Option<unsafe extern "C" fn()>);
}
extern "C" {
    pub fn Mix_GetMusicHookData() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn Mix_ChannelFinished(
        channel_finished: ::std::option::Option<
            unsafe extern "C" fn(channel: ::std::os::raw::c_int),
        >,
    );
}
pub type Mix_EffectFunc_t = ::std::option::Option<
    unsafe extern "C" fn(
        chan: ::std::os::raw::c_int,
        stream: *mut ::std::os::raw::c_void,
        len: ::std::os::raw::c_int,
        udata: *mut ::std::os::raw::c_void,
    ),
>;
pub type Mix_EffectDone_t = ::std::option::Option<
    unsafe extern "C" fn(chan: ::std::os::raw::c_int, udata: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn Mix_RegisterEffect(
        chan: ::std::os::raw::c_int,
        f: Mix_EffectFunc_t,
        d: Mix_EffectDone_t,
        arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_UnregisterEffect(
        channel: ::std::os::raw::c_int,
        f: Mix_EffectFunc_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_UnregisterAllEffects(channel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetPanning(
        channel: ::std::os::raw::c_int,
        left: Uint8,
        right: Uint8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetPosition(
        channel: ::std::os::raw::c_int,
        angle: Sint16,
        distance: Uint8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetDistance(
        channel: ::std::os::raw::c_int,
        distance: Uint8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetReverseStereo(
        channel: ::std::os::raw::c_int,
        flip: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_ReserveChannels(num: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupChannel(
        which: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupChannels(
        from: ::std::os::raw::c_int,
        to: ::std::os::raw::c_int,
        tag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupAvailable(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupCount(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupOldest(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GroupNewer(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_PlayChannelTimed(
        channel: ::std::os::raw::c_int,
        chunk: *mut Mix_Chunk,
        loops: ::std::os::raw::c_int,
        ticks: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_PlayMusic(
        music: *mut Mix_Music,
        loops: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeInMusic(
        music: *mut Mix_Music,
        loops: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeInMusicPos(
        music: *mut Mix_Music,
        loops: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
        position: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeInChannelTimed(
        channel: ::std::os::raw::c_int,
        chunk: *mut Mix_Chunk,
        loops: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
        ticks: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_Volume(
        channel: ::std::os::raw::c_int,
        volume: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_VolumeChunk(
        chunk: *mut Mix_Chunk,
        volume: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_VolumeMusic(volume: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetMusicVolume(music: *mut Mix_Music) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_HaltChannel(channel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_HaltGroup(tag: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_HaltMusic() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_ExpireChannel(
        channel: ::std::os::raw::c_int,
        ticks: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeOutChannel(
        which: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeOutGroup(
        tag: ::std::os::raw::c_int,
        ms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadeOutMusic(ms: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_FadingMusic() -> Mix_Fading;
}
extern "C" {
    pub fn Mix_FadingChannel(which: ::std::os::raw::c_int) -> Mix_Fading;
}
extern "C" {
    pub fn Mix_Pause(channel: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Mix_Resume(channel: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Mix_Paused(channel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_PauseMusic();
}
extern "C" {
    pub fn Mix_ResumeMusic();
}
extern "C" {
    pub fn Mix_RewindMusic();
}
extern "C" {
    pub fn Mix_PausedMusic() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_ModMusicJumpToOrder(order: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetMusicPosition(position: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetMusicPosition(music: *mut Mix_Music) -> f64;
}
extern "C" {
    pub fn Mix_MusicDuration(music: *mut Mix_Music) -> f64;
}
extern "C" {
    pub fn Mix_GetMusicLoopStartTime(music: *mut Mix_Music) -> f64;
}
extern "C" {
    pub fn Mix_GetMusicLoopEndTime(music: *mut Mix_Music) -> f64;
}
extern "C" {
    pub fn Mix_GetMusicLoopLengthTime(music: *mut Mix_Music) -> f64;
}
extern "C" {
    pub fn Mix_Playing(channel: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_PlayingMusic() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetMusicCMD(command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetSynchroValue(value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetSynchroValue() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetSoundFonts(paths: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetSoundFonts() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_EachSoundFont(
        function: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_char,
                arg2: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_SetTimidityCfg(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Mix_GetTimidityCfg() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Mix_GetChunk(channel: ::std::os::raw::c_int) -> *mut Mix_Chunk;
}
extern "C" {
    pub fn Mix_CloseAudio();
}
