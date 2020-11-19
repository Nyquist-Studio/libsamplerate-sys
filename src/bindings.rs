/* automatically generated by rust-bindgen 0.55.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SRC_STATE_tag {
    _unused: [u8; 0],
}
pub type SRC_STATE = SRC_STATE_tag;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SRC_DATA {
    pub data_in: *const f32,
    pub data_out: *mut f32,
    pub input_frames: ::std::os::raw::c_long,
    pub output_frames: ::std::os::raw::c_long,
    pub input_frames_used: ::std::os::raw::c_long,
    pub output_frames_gen: ::std::os::raw::c_long,
    pub end_of_input: ::std::os::raw::c_int,
    pub src_ratio: f64,
}
pub type src_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        cb_data: *mut ::std::os::raw::c_void,
        data: *mut *mut f32,
    ) -> ::std::os::raw::c_long,
>;
extern "C" {
    pub fn src_new(
        converter_type: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut SRC_STATE;
}
extern "C" {
    pub fn src_clone(orig: *mut SRC_STATE, error: *mut ::std::os::raw::c_int) -> *mut SRC_STATE;
}
extern "C" {
    pub fn src_callback_new(
        func: src_callback_t,
        converter_type: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
        error: *mut ::std::os::raw::c_int,
        cb_data: *mut ::std::os::raw::c_void,
    ) -> *mut SRC_STATE;
}
extern "C" {
    pub fn src_delete(state: *mut SRC_STATE) -> *mut SRC_STATE;
}
extern "C" {
    pub fn src_process(state: *mut SRC_STATE, data: *mut SRC_DATA) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_callback_read(
        state: *mut SRC_STATE,
        src_ratio: f64,
        frames: ::std::os::raw::c_long,
        data: *mut f32,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn src_simple(
        data: *mut SRC_DATA,
        converter_type: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_get_name(converter_type: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn src_get_description(
        converter_type: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn src_get_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn src_set_ratio(state: *mut SRC_STATE, new_ratio: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_get_channels(state: *mut SRC_STATE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_reset(state: *mut SRC_STATE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_is_valid_ratio(ratio: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_error(state: *mut SRC_STATE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn src_strerror(error: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
pub const SRC_SINC_BEST_QUALITY: ::std::os::raw::c_uint = 0;
pub const SRC_SINC_MEDIUM_QUALITY: ::std::os::raw::c_uint = 1;
pub const SRC_SINC_FASTEST: ::std::os::raw::c_uint = 2;
pub const SRC_ZERO_ORDER_HOLD: ::std::os::raw::c_uint = 3;
pub const SRC_LINEAR: ::std::os::raw::c_uint = 4;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
extern "C" {
    pub fn src_short_to_float_array(
        in_: *const ::std::os::raw::c_short,
        out: *mut f32,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn src_float_to_short_array(
        in_: *const f32,
        out: *mut ::std::os::raw::c_short,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn src_int_to_float_array(
        in_: *const ::std::os::raw::c_int,
        out: *mut f32,
        len: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn src_float_to_int_array(
        in_: *const f32,
        out: *mut ::std::os::raw::c_int,
        len: ::std::os::raw::c_int,
    );
}