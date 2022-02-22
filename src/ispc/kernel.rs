#[allow(non_camel_case_types,dead_code,non_upper_case_globals,non_snake_case,improper_ctypes)]
pub mod kernel {
/* automatically generated by rust-bindgen 0.59.2 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type size_t = ::std::os::raw::c_ulonglong;
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rgba_surface {
    pub ptr: *mut u8,
    pub width: i32,
    pub height: i32,
    pub stride: i32,
}
#[test]
fn bindgen_test_layout_rgba_surface() {
    assert_eq!(
        ::std::mem::size_of::<rgba_surface>(),
        24usize,
        concat!("Size of: ", stringify!(rgba_surface))
    );
    assert_eq!(
        ::std::mem::align_of::<rgba_surface>(),
        8usize,
        concat!("Alignment of ", stringify!(rgba_surface))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rgba_surface>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rgba_surface),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rgba_surface>())).width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rgba_surface),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rgba_surface>())).height as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(rgba_surface),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rgba_surface>())).stride as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rgba_surface),
            "::",
            stringify!(stride)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bc6h_enc_settings {
    pub slow_mode: bool,
    pub fast_mode: bool,
    pub refineIterations_1p: i32,
    pub refineIterations_2p: i32,
    pub fastSkipTreshold: i32,
}
#[test]
fn bindgen_test_layout_bc6h_enc_settings() {
    assert_eq!(
        ::std::mem::size_of::<bc6h_enc_settings>(),
        16usize,
        concat!("Size of: ", stringify!(bc6h_enc_settings))
    );
    assert_eq!(
        ::std::mem::align_of::<bc6h_enc_settings>(),
        4usize,
        concat!("Alignment of ", stringify!(bc6h_enc_settings))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bc6h_enc_settings>())).slow_mode as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bc6h_enc_settings),
            "::",
            stringify!(slow_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bc6h_enc_settings>())).fast_mode as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(bc6h_enc_settings),
            "::",
            stringify!(fast_mode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc6h_enc_settings>())).refineIterations_1p as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bc6h_enc_settings),
            "::",
            stringify!(refineIterations_1p)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc6h_enc_settings>())).refineIterations_2p as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bc6h_enc_settings),
            "::",
            stringify!(refineIterations_2p)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc6h_enc_settings>())).fastSkipTreshold as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(bc6h_enc_settings),
            "::",
            stringify!(fastSkipTreshold)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bc7_enc_settings {
    pub mode_selection: [bool; 4usize],
    pub refineIterations: [i32; 8usize],
    pub skip_mode2: bool,
    pub fastSkipTreshold_mode1: i32,
    pub fastSkipTreshold_mode3: i32,
    pub fastSkipTreshold_mode7: i32,
    pub mode45_channel0: i32,
    pub refineIterations_channel: i32,
    pub channels: i32,
}
#[test]
fn bindgen_test_layout_bc7_enc_settings() {
    assert_eq!(
        ::std::mem::size_of::<bc7_enc_settings>(),
        64usize,
        concat!("Size of: ", stringify!(bc7_enc_settings))
    );
    assert_eq!(
        ::std::mem::align_of::<bc7_enc_settings>(),
        4usize,
        concat!("Alignment of ", stringify!(bc7_enc_settings))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bc7_enc_settings>())).mode_selection as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(mode_selection)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc7_enc_settings>())).refineIterations as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(refineIterations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bc7_enc_settings>())).skip_mode2 as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(skip_mode2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc7_enc_settings>())).fastSkipTreshold_mode1 as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(fastSkipTreshold_mode1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc7_enc_settings>())).fastSkipTreshold_mode3 as *const _ as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(fastSkipTreshold_mode3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc7_enc_settings>())).fastSkipTreshold_mode7 as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(fastSkipTreshold_mode7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc7_enc_settings>())).mode45_channel0 as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(mode45_channel0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bc7_enc_settings>())).refineIterations_channel as *const _
                as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(refineIterations_channel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bc7_enc_settings>())).channels as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(bc7_enc_settings),
            "::",
            stringify!(channels)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct etc_enc_settings {
    pub fastSkipTreshold: i32,
}
#[test]
fn bindgen_test_layout_etc_enc_settings() {
    assert_eq!(
        ::std::mem::size_of::<etc_enc_settings>(),
        4usize,
        concat!("Size of: ", stringify!(etc_enc_settings))
    );
    assert_eq!(
        ::std::mem::align_of::<etc_enc_settings>(),
        4usize,
        concat!("Alignment of ", stringify!(etc_enc_settings))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<etc_enc_settings>())).fastSkipTreshold as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(etc_enc_settings),
            "::",
            stringify!(fastSkipTreshold)
        )
    );
}
extern "C" {
    pub fn CompressBlocksBC1_ispc(src: *mut rgba_surface, dst: *mut u8);
}
extern "C" {
    pub fn CompressBlocksBC3_ispc(src: *mut rgba_surface, dst: *mut u8);
}
extern "C" {
    pub fn CompressBlocksBC4_ispc(src: *mut rgba_surface, dst: *mut u8);
}
extern "C" {
    pub fn CompressBlocksBC5_ispc(src: *mut rgba_surface, dst: *mut u8);
}
extern "C" {
    pub fn CompressBlocksBC6H_ispc(
        src: *mut rgba_surface,
        dst: *mut u8,
        settings: *mut bc6h_enc_settings,
    );
}
extern "C" {
    pub fn CompressBlocksBC7_ispc(
        src: *mut rgba_surface,
        dst: *mut u8,
        settings: *mut bc7_enc_settings,
    );
}
extern "C" {
    pub fn CompressBlocksETC1_ispc(
        src: *mut rgba_surface,
        dst: *mut u8,
        settings: *mut etc_enc_settings,
    );
}
}