use libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn lrint(_: libc::c_double) -> libc::c_long;
}
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
/* SRC_DATA is used to pass data to src_simple() and src_process(). */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRC_DATA {
    pub data_in: *const libc::c_float,
    pub data_out: *mut libc::c_float,
    pub input_frames: libc::c_long,
    pub output_frames: libc::c_long,
    pub input_frames_used: libc::c_long,
    pub output_frames_gen: libc::c_long,
    pub end_of_input: libc::c_int,
    pub src_ratio: libc::c_double,
}
/*
** User supplied callback function type for use with src_callback_new()
** and src_callback_read(). First parameter is the same pointer that was
** passed into src_callback_new(). Second parameter is pointer to a
** pointer. The user supplied callback function must modify *data to
** point to the start of the user supplied float array. The user supplied
** function must return the number of frames that **data points to.
*/
pub type src_callback_t =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut *mut libc::c_float) -> libc::c_long>;
/*
** The following enums can be used to set the interpolator type
** using the function src_set_converter().
*/
pub type unnamed = libc::c_uint;
pub const SRC_LINEAR: unnamed = 4;
pub const SRC_ZERO_ORDER_HOLD: unnamed = 3;
pub const SRC_SINC_FASTEST: unnamed = 2;
pub const SRC_SINC_MEDIUM_QUALITY: unnamed = 1;
pub const SRC_SINC_BEST_QUALITY: unnamed = 0;
pub type unnamed_0 = libc::c_uint;
/* This must be the last error number. */
pub const SRC_ERR_MAX_ERROR: unnamed_0 = 23;
pub const SRC_ERR_BAD_INTERNAL_STATE: unnamed_0 = 22;
pub const SRC_ERR_SINC_PREPARE_DATA_BAD_LEN: unnamed_0 = 21;
pub const SRC_ERR_NO_VARIABLE_RATIO: unnamed_0 = 20;
pub const SRC_ERR_NULL_CALLBACK: unnamed_0 = 19;
pub const SRC_ERR_BAD_MODE: unnamed_0 = 18;
pub const SRC_ERR_BAD_CALLBACK: unnamed_0 = 17;
pub const SRC_ERR_DATA_OVERLAP: unnamed_0 = 16;
pub const SRC_ERR_BAD_SINC_STATE: unnamed_0 = 15;
pub const SRC_ERR_BAD_PRIV_PTR: unnamed_0 = 14;
pub const SRC_ERR_SIZE_INCOMPATIBILITY: unnamed_0 = 13;
pub const SRC_ERR_SINC_BAD_BUFFER_LEN: unnamed_0 = 12;
pub const SRC_ERR_BAD_CHANNEL_COUNT: unnamed_0 = 11;
pub const SRC_ERR_BAD_CONVERTER: unnamed_0 = 10;
pub const SRC_ERR_FILTER_LEN: unnamed_0 = 9;
pub const SRC_ERR_SHIFT_BITS: unnamed_0 = 8;
pub const SRC_ERR_BAD_PROC_PTR: unnamed_0 = 7;
pub const SRC_ERR_BAD_SRC_RATIO: unnamed_0 = 6;
pub const SRC_ERR_NO_PRIVATE: unnamed_0 = 5;
pub const SRC_ERR_BAD_DATA_PTR: unnamed_0 = 4;
pub const SRC_ERR_BAD_DATA: unnamed_0 = 3;
pub const SRC_ERR_BAD_STATE: unnamed_0 = 2;
pub const SRC_ERR_MALLOC_FAILED: unnamed_0 = 1;
pub const SRC_ERR_NO_ERROR: unnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRC_PRIVATE_tag {
    pub last_ratio: libc::c_double,
    pub last_position: libc::c_double,
    pub error: libc::c_int,
    pub channels: libc::c_int,
    pub mode: libc::c_int,
    pub private_data: *mut libc::c_void,
    pub vari_process:
        Option<unsafe extern "C" fn(_: *mut SRC_PRIVATE_tag, _: *mut SRC_DATA) -> libc::c_int>,
    pub const_process:
        Option<unsafe extern "C" fn(_: *mut SRC_PRIVATE_tag, _: *mut SRC_DATA) -> libc::c_int>,
    pub reset: Option<unsafe extern "C" fn(_: *mut SRC_PRIVATE_tag) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(_: *mut SRC_PRIVATE_tag, _: *mut SRC_PRIVATE_tag) -> libc::c_int,
    >,
    pub callback_func: src_callback_t,
    pub user_callback_data: *mut libc::c_void,
    pub saved_frames: libc::c_long,
    pub saved_data: *const libc::c_float,
}
pub type SRC_PRIVATE = SRC_PRIVATE_tag;
/*========================================================================================
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINEAR_DATA {
    pub linear_magic_marker: libc::c_int,
    pub channels: libc::c_int,
    pub reset: libc::c_int,
    pub in_count: libc::c_long,
    pub in_used: libc::c_long,
    pub out_count: libc::c_long,
    pub out_gen: libc::c_long,
    pub last_value: [libc::c_float; 1],
}
/* In src_linear.c */
#[no_mangle]
pub unsafe extern "C" fn linear_get_name(mut src_enum: libc::c_int) -> *const libc::c_char {
    if src_enum == SRC_LINEAR as libc::c_int {
        return b"Linear Interpolator\x00" as *const u8 as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn linear_get_description(mut src_enum: libc::c_int) -> *const libc::c_char {
    if src_enum == SRC_LINEAR as libc::c_int {
        return b"Linear interpolator, very fast, poor quality.\x00" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn linear_set_converter(
    mut psrc: *mut SRC_PRIVATE,
    mut src_enum: libc::c_int,
) -> libc::c_int {
    let mut priv_0: *mut LINEAR_DATA = 0 as *mut LINEAR_DATA;
    if src_enum != SRC_LINEAR as libc::c_int {
        return SRC_ERR_BAD_CONVERTER as libc::c_int;
    }
    if !(*psrc).private_data.is_null() {
        free((*psrc).private_data);
        (*psrc).private_data = 0 as *mut libc::c_void
    }
    if (*psrc).private_data.is_null() {
        priv_0 = calloc(
            1i32 as libc::c_ulong,
            (::std::mem::size_of::<LINEAR_DATA>() as libc::c_ulong).wrapping_add(
                ((*psrc).channels as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            ),
        ) as *mut LINEAR_DATA;
        (*psrc).private_data = priv_0 as *mut libc::c_void
    }
    if priv_0.is_null() {
        return SRC_ERR_MALLOC_FAILED as libc::c_int;
    }
    (*priv_0).linear_magic_marker = 'l' as i32
        + (('i' as i32) << 4i32)
        + (('n' as i32) << 8i32)
        + (('e' as i32) << 12i32)
        + (('a' as i32) << 16i32)
        + (('r' as i32) << 20i32);
    (*priv_0).channels = (*psrc).channels;
    (*psrc).const_process = Some(linear_vari_process);
    (*psrc).vari_process = Some(linear_vari_process);
    (*psrc).reset = Some(linear_reset);
    (*psrc).copy = Some(linear_copy);
    linear_reset(psrc);
    return SRC_ERR_NO_ERROR as libc::c_int;
}
unsafe extern "C" fn linear_reset(mut psrc: *mut SRC_PRIVATE) {
    let mut priv_0: *mut LINEAR_DATA = 0 as *mut LINEAR_DATA;
    priv_0 = (*psrc).private_data as *mut LINEAR_DATA;
    if priv_0.is_null() {
        return;
    }
    (*priv_0).channels = (*psrc).channels;
    (*priv_0).reset = 1i32;
    memset(
        (*priv_0).last_value.as_mut_ptr() as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((*priv_0).channels as libc::c_ulong),
    );
}
unsafe extern "C" fn linear_copy(
    mut from: *mut SRC_PRIVATE,
    mut to: *mut SRC_PRIVATE,
) -> libc::c_int {
    if (*from).private_data.is_null() {
        return SRC_ERR_NO_PRIVATE as libc::c_int;
    }
    let mut to_priv: *mut LINEAR_DATA = 0 as *mut LINEAR_DATA;
    let mut from_priv: *mut LINEAR_DATA = (*from).private_data as *mut LINEAR_DATA;
    let mut private_size: size_t = (::std::mem::size_of::<LINEAR_DATA>() as libc::c_ulong)
        .wrapping_add(
            ((*from_priv).channels as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
    to_priv = calloc(1i32 as libc::c_ulong, private_size) as *mut LINEAR_DATA;
    if to_priv.is_null() {
        return SRC_ERR_MALLOC_FAILED as libc::c_int;
    }
    memcpy(
        to_priv as *mut libc::c_void,
        from_priv as *const libc::c_void,
        private_size,
    );
    (*to).private_data = to_priv as *mut libc::c_void;
    return SRC_ERR_NO_ERROR as libc::c_int;
}
/*
** Copyright (c) 2002-2016, Erik de Castro Lopo <erikd@mega-nerd.com>
** All rights reserved.
**
** This code is released under 2-clause BSD license. Please see the
** file at : https://github.com/erikd/libsamplerate/blob/master/COPYING
*/
unsafe extern "C" fn linear_vari_process(
    mut psrc: *mut SRC_PRIVATE,
    mut data: *mut SRC_DATA,
) -> libc::c_int {
    let mut priv_0: *mut LINEAR_DATA = 0 as *mut LINEAR_DATA;
    let mut src_ratio: libc::c_double = 0.;
    let mut input_index: libc::c_double = 0.;
    let mut rem: libc::c_double = 0.;
    let mut ch: libc::c_int = 0;
    if (*data).input_frames <= 0i32 as libc::c_long {
        return SRC_ERR_NO_ERROR as libc::c_int;
    }
    if (*psrc).private_data.is_null() {
        return SRC_ERR_NO_PRIVATE as libc::c_int;
    }
    priv_0 = (*psrc).private_data as *mut LINEAR_DATA;
    if 0 != (*priv_0).reset {
        ch = 0i32;
        while ch < (*priv_0).channels {
            *(*priv_0).last_value.get_unchecked_mut(ch as usize) = *(*data).data_in.offset(ch as isize);
            ch += 1
        }
        (*priv_0).reset = 0i32
    }
    (*priv_0).in_count = (*data).input_frames * (*priv_0).channels as libc::c_long;
    (*priv_0).out_count = (*data).output_frames * (*priv_0).channels as libc::c_long;
    (*priv_0).out_gen = 0i32 as libc::c_long;
    (*priv_0).in_used = (*priv_0).out_gen;
    src_ratio = (*psrc).last_ratio;
    if 0 != is_bad_src_ratio(src_ratio) {
        return SRC_ERR_BAD_INTERNAL_STATE as libc::c_int;
    }
    input_index = (*psrc).last_position;
    while input_index < 1.0f64 && (*priv_0).out_gen < (*priv_0).out_count {
        if (*priv_0).in_used as libc::c_double
            + (*priv_0).channels as libc::c_double * (1.0f64 + input_index)
            >= (*priv_0).in_count as libc::c_double
        {
            break;
        }
        if (*priv_0).out_count > 0i32 as libc::c_long
            && fabs((*psrc).last_ratio - (*data).src_ratio) > 1e-20f64
        {
            src_ratio = (*psrc).last_ratio
                + (*priv_0).out_gen as libc::c_double * ((*data).src_ratio - (*psrc).last_ratio)
                    / (*priv_0).out_count as libc::c_double
        }
        ch = 0i32;
        while ch < (*priv_0).channels {
            *(*data).data_out.offset((*priv_0).out_gen as isize) =
                (*(*priv_0).last_value.get_unchecked(ch as usize) as libc::c_double
                    + input_index
                        * (*(*data).data_in.offset(ch as isize) - *(*priv_0).last_value.get_unchecked(ch as usize))
                            as libc::c_double) as libc::c_float;
            (*priv_0).out_gen += 1;
            ch += 1
        }
        input_index += 1.0f64 / src_ratio
    }
    rem = fmod_one(input_index);
    (*priv_0).in_used += (*priv_0).channels as libc::c_long * lrint(input_index - rem);
    input_index = rem;
    while (*priv_0).out_gen < (*priv_0).out_count
        && (*priv_0).in_used as libc::c_double + (*priv_0).channels as libc::c_double * input_index
            < (*priv_0).in_count as libc::c_double
    {
        if (*priv_0).out_count > 0i32 as libc::c_long
            && fabs((*psrc).last_ratio - (*data).src_ratio) > 1e-20f64
        {
            src_ratio = (*psrc).last_ratio
                + (*priv_0).out_gen as libc::c_double * ((*data).src_ratio - (*psrc).last_ratio)
                    / (*priv_0).out_count as libc::c_double
        }
        if 0 != 0i32
            && (*priv_0).in_used < (*priv_0).channels as libc::c_long
            && input_index < 1.0f64
        {
            exit(1i32);
        }
        ch = 0i32;
        while ch < (*priv_0).channels {
            *(*data).data_out.offset((*priv_0).out_gen as isize) = (*(*data).data_in.offset(
                ((*priv_0).in_used - (*priv_0).channels as libc::c_long + ch as libc::c_long)
                    as isize,
            ) as libc::c_double
                + input_index
                    * (*(*data)
                        .data_in
                        .offset(((*priv_0).in_used + ch as libc::c_long) as isize)
                        - *(*data).data_in.offset(
                            ((*priv_0).in_used - (*priv_0).channels as libc::c_long
                                + ch as libc::c_long) as isize,
                        )) as libc::c_double)
                as libc::c_float;
            (*priv_0).out_gen += 1;
            ch += 1
        }
        input_index += 1.0f64 / src_ratio;
        rem = fmod_one(input_index);
        (*priv_0).in_used += (*priv_0).channels as libc::c_long * lrint(input_index - rem);
        input_index = rem
    }
    if (*priv_0).in_used > (*priv_0).in_count {
        input_index += (((*priv_0).in_used - (*priv_0).in_count)
            / (*priv_0).channels as libc::c_long) as libc::c_double;
        (*priv_0).in_used = (*priv_0).in_count
    }
    (*psrc).last_position = input_index;
    if (*priv_0).in_used > 0i32 as libc::c_long {
        ch = 0i32;
        while ch < (*priv_0).channels {
            *(*priv_0).last_value.get_unchecked_mut(ch as usize) = *(*data).data_in.offset(
                ((*priv_0).in_used - (*priv_0).channels as libc::c_long + ch as libc::c_long)
                    as isize,
            );
            ch += 1
        }
    }
    (*psrc).last_ratio = src_ratio;
    (*data).input_frames_used = (*priv_0).in_used / (*priv_0).channels as libc::c_long;
    (*data).output_frames_gen = (*priv_0).out_gen / (*priv_0).channels as libc::c_long;
    return SRC_ERR_NO_ERROR as libc::c_int;
}
/*----------------------------------------------------------
**	Common static inline functions.
*/
#[inline]
unsafe extern "C" fn fmod_one(mut x: libc::c_double) -> libc::c_double {
    let mut res: libc::c_double = 0.;
    res = x - lrint(x) as libc::c_double;
    if res < 0.0f64 {
        return res + 1.0f64;
    }
    return res;
}
/* fmod_one */
#[inline]
unsafe extern "C" fn is_bad_src_ratio(mut ratio: libc::c_double) -> libc::c_int {
    return (ratio < 1.0f64 / 256i32 as libc::c_double || ratio > 1.0f64 * 256i32 as libc::c_double)
        as libc::c_int;
}
