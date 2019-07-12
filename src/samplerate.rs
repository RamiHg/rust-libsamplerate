use libc;
 pub struct SRC_STATE_tag {
        _unused: [u8; 0],
    }
extern "C" {
    /*
** Copyright (c) 2002-2016, Erik de Castro Lopo <erikd@mega-nerd.com>
** All rights reserved.
**
** This code is released under 2-clause BSD license. Please see the
** file at : https://github.com/erikd/libsamplerate/blob/master/COPYING
*/
    /*
** API documentation is available here:
**     http://www.mega-nerd.com/SRC/api.html
*/
    /* __cplusplus */
    /* Opaque data type SRC_STATE. */
   
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn linear_set_converter(psrc: *mut SRC_PRIVATE, src_enum: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn zoh_set_converter(psrc: *mut SRC_PRIVATE, src_enum: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn sinc_set_converter(psrc: *mut SRC_PRIVATE, src_enum: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    /* In src_linear.c */
    #[no_mangle]
    fn linear_get_name(src_enum: libc::c_int) -> *const libc::c_char;
    /* In src_zoh.c */
    #[no_mangle]
    fn zoh_get_name(src_enum: libc::c_int) -> *const libc::c_char;
    /* In src_sinc.c */
    #[no_mangle]
    fn sinc_get_name(src_enum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn linear_get_description(src_enum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn zoh_get_description(src_enum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn sinc_get_description(src_enum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn lrint(_: libc::c_double) -> libc::c_long;
}
pub type SRC_STATE = SRC_STATE_tag;
/* SRC_DATA is used to pass data to src_simple() and src_process(). */
#[derive ( Copy , Clone )]
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
pub type src_callback_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                _: *mut *mut libc::c_float) -> libc::c_long>;
pub type SRC_PRIVATE = SRC_PRIVATE_tag;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SRC_PRIVATE_tag {
    pub last_ratio: libc::c_double,
    pub last_position: libc::c_double,
    pub error: libc::c_int,
    pub channels: libc::c_int,
    pub mode: libc::c_int,
    pub private_data: *mut libc::c_void,
    pub vari_process: Option<unsafe extern "C" fn(_: *mut SRC_PRIVATE_tag,
                                                  _: *mut SRC_DATA)
                                 -> libc::c_int>,
    pub const_process: Option<unsafe extern "C" fn(_: *mut SRC_PRIVATE_tag,
                                                   _: *mut SRC_DATA)
                                  -> libc::c_int>,
    pub reset: Option<unsafe extern "C" fn(_: *mut SRC_PRIVATE_tag) -> ()>,
    pub copy: Option<unsafe extern "C" fn(_: *mut SRC_PRIVATE_tag,
                                          _: *mut SRC_PRIVATE_tag)
                         -> libc::c_int>,
    pub callback_func: src_callback_t,
    pub user_callback_data: *mut libc::c_void,
    pub saved_frames: libc::c_long,
    pub saved_data: *const libc::c_float,
}
pub const SRC_ERR_NO_ERROR: unnamed_0 = 0;
pub const SRC_ERR_BAD_STATE: unnamed_0 = 2;
pub const SRC_ERR_BAD_CONVERTER: unnamed_0 = 10;
pub const SRC_MODE_PROCESS: unnamed = 555;
pub const SRC_ERR_MALLOC_FAILED: unnamed_0 = 1;
pub const SRC_ERR_BAD_CHANNEL_COUNT: unnamed_0 = 11;
pub const SRC_MODE_CALLBACK: unnamed = 556;
pub const SRC_ERR_BAD_CALLBACK: unnamed_0 = 17;
pub const SRC_ERR_DATA_OVERLAP: unnamed_0 = 16;
pub const SRC_ERR_BAD_SRC_RATIO: unnamed_0 = 6;
pub const SRC_ERR_BAD_DATA_PTR: unnamed_0 = 4;
pub const SRC_ERR_BAD_DATA: unnamed_0 = 3;
pub const SRC_ERR_BAD_MODE: unnamed_0 = 18;
pub const SRC_ERR_BAD_PROC_PTR: unnamed_0 = 7;
pub const SRC_TRUE: unnamed = 1;
pub const SRC_ERR_NULL_CALLBACK: unnamed_0 = 19;
pub const SRC_FALSE: unnamed = 0;
/* This must be the last error number. */
pub const SRC_ERR_MAX_ERROR: unnamed_0 = 23;
pub const SRC_ERR_BAD_INTERNAL_STATE: unnamed_0 = 22;
pub const SRC_ERR_SINC_PREPARE_DATA_BAD_LEN: unnamed_0 = 21;
pub const SRC_ERR_NO_VARIABLE_RATIO: unnamed_0 = 20;
pub const SRC_ERR_BAD_PRIV_PTR: unnamed_0 = 14;
pub const SRC_ERR_SIZE_INCOMPATIBILITY: unnamed_0 = 13;
pub const SRC_ERR_SINC_BAD_BUFFER_LEN: unnamed_0 = 12;
pub const SRC_ERR_FILTER_LEN: unnamed_0 = 9;
pub const SRC_ERR_SHIFT_BITS: unnamed_0 = 8;
pub const SRC_ERR_BAD_SINC_STATE: unnamed_0 = 15;
pub const SRC_ERR_NO_PRIVATE: unnamed_0 = 5;
/*
** Copyright (c) 2002-2016, Erik de Castro Lopo <erikd@mega-nerd.com>
** All rights reserved.
**
** This code is released under 2-clause BSD license. Please see the
** file at : https://github.com/erikd/libsamplerate/blob/master/COPYING
*/
/*
** Inspiration : http://sourcefrog.net/weblog/software/languages/C/unused.html
*/
pub type unnamed = libc::c_uint;
pub type unnamed_0 = libc::c_uint;
/*
**	Standard initialisation function : return an anonymous pointer to the
**	internal state of the converter. Choose a converter from the enums below.
**	Error returned in *error.
*/
#[no_mangle]
pub unsafe extern "C" fn src_new(mut converter_type: libc::c_int,
                                 mut channels: libc::c_int,
                                 mut error: *mut libc::c_int)
 -> *mut SRC_STATE {
    let mut psrc: *mut SRC_PRIVATE = 0 as *mut SRC_PRIVATE;
    if !error.is_null() { *error = SRC_ERR_NO_ERROR as libc::c_int }
    if channels < 1i32 {
        if !error.is_null() {
            *error = SRC_ERR_BAD_CHANNEL_COUNT as libc::c_int
        }
        return 0 as *mut SRC_STATE
    }
    psrc =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<SRC_PRIVATE>() as libc::c_ulong) as
            *mut SRC_PRIVATE;
    if psrc.is_null() {
        if !error.is_null() { *error = SRC_ERR_MALLOC_FAILED as libc::c_int }
        return 0 as *mut SRC_STATE
    }
    (*psrc).channels = channels;
    (*psrc).mode = SRC_MODE_PROCESS as libc::c_int;
    if psrc_set_converter(psrc, converter_type) !=
           SRC_ERR_NO_ERROR as libc::c_int {
        if !error.is_null() { *error = SRC_ERR_BAD_CONVERTER as libc::c_int }
        free(psrc as *mut libc::c_void);
        psrc = 0 as *mut SRC_PRIVATE
    }
    src_reset(psrc as *mut SRC_STATE);
    return psrc as *mut SRC_STATE;
}
/*
**	Reset the internal SRC state.
**	Does not modify the quality settings.
**	Does not free any memory allocations.
**	Returns non zero on error.
*/
#[no_mangle]
pub unsafe extern "C" fn src_reset(mut state: *mut SRC_STATE) -> libc::c_int {
    let mut psrc: *mut SRC_PRIVATE = 0 as *mut SRC_PRIVATE;
    psrc = state as *mut SRC_PRIVATE;
    if psrc.is_null() { return SRC_ERR_BAD_STATE as libc::c_int }
    if (*psrc).reset.is_some() {
        (*psrc).reset.expect("non-null function pointer")(psrc);
    }
    (*psrc).last_position = 0.0f64;
    (*psrc).last_ratio = 0.0f64;
    (*psrc).saved_data = 0 as *const libc::c_float;
    (*psrc).saved_frames = 0i32 as libc::c_long;
    (*psrc).error = SRC_ERR_NO_ERROR as libc::c_int;
    return SRC_ERR_NO_ERROR as libc::c_int;
}
/*
** Copyright (c) 2002-2016, Erik de Castro Lopo <erikd@mega-nerd.com>
** All rights reserved.
**
** This code is released under 2-clause BSD license. Please see the
** file at : https://github.com/erikd/libsamplerate/blob/master/COPYING
*/
unsafe extern "C" fn psrc_set_converter(mut psrc: *mut SRC_PRIVATE,
                                        mut converter_type: libc::c_int)
 -> libc::c_int {
    if sinc_set_converter(psrc, converter_type) ==
           SRC_ERR_NO_ERROR as libc::c_int {
        return SRC_ERR_NO_ERROR as libc::c_int
    }
    if zoh_set_converter(psrc, converter_type) ==
           SRC_ERR_NO_ERROR as libc::c_int {
        return SRC_ERR_NO_ERROR as libc::c_int
    }
    if linear_set_converter(psrc, converter_type) ==
           SRC_ERR_NO_ERROR as libc::c_int {
        return SRC_ERR_NO_ERROR as libc::c_int
    }
    return SRC_ERR_BAD_CONVERTER as libc::c_int;
}
/*
** Clone a handle : return an anonymous pointer to a new converter
** containing the same internal state as orig. Error returned in *error.
*/
#[no_mangle]
pub unsafe extern "C" fn src_clone(mut orig: *mut SRC_STATE,
                                   mut error: *mut libc::c_int)
 -> *mut SRC_STATE {
    let mut psrc: *mut SRC_PRIVATE = 0 as *mut SRC_PRIVATE;
    let mut copy_error: libc::c_int = 0;
    if !error.is_null() { *error = SRC_ERR_NO_ERROR as libc::c_int }
    psrc =
        calloc(1i32 as libc::c_ulong,
               ::std::mem::size_of::<SRC_PRIVATE>() as libc::c_ulong) as
            *mut SRC_PRIVATE;
    if psrc.is_null() {
        if !error.is_null() { *error = SRC_ERR_MALLOC_FAILED as libc::c_int }
        return 0 as *mut SRC_STATE
    }
    let mut orig_priv: *mut SRC_PRIVATE = orig as *mut SRC_PRIVATE;
    memcpy(psrc as *mut libc::c_void, orig_priv as *const libc::c_void,
           ::std::mem::size_of::<SRC_PRIVATE>() as libc::c_ulong);
    copy_error =
        (*orig_priv).copy.expect("non-null function pointer")(orig_priv,
                                                              psrc);
    if copy_error != SRC_ERR_NO_ERROR as libc::c_int {
        if !error.is_null() { *error = copy_error }
        free(psrc as *mut libc::c_void);
        psrc = 0 as *mut SRC_PRIVATE
    }
    return psrc as *mut SRC_STATE;
}
/*
**	Initilisation for callback based API : return an anonymous pointer to the
**	internal state of the converter. Choose a converter from the enums below.
**	The cb_data pointer can point to any data or be set to NULL. Whatever the
**	value, when processing, user supplied function "func" gets called with
**	cb_data as first parameter.
*/
#[no_mangle]
pub unsafe extern "C" fn src_callback_new(mut func: src_callback_t,
                                          mut converter_type: libc::c_int,
                                          mut channels: libc::c_int,
                                          mut error: *mut libc::c_int,
                                          mut cb_data: *mut libc::c_void)
 -> *mut SRC_STATE {
    let mut src_state: *mut SRC_STATE = 0 as *mut SRC_STATE;
    if func.is_none() {
        if !error.is_null() { *error = SRC_ERR_BAD_CALLBACK as libc::c_int }
        return 0 as *mut SRC_STATE
    }
    if !error.is_null() { *error = 0i32 }
    src_state = src_new(converter_type, channels, error);
    if src_state.is_null() { return 0 as *mut SRC_STATE }
    src_reset(src_state);
    (*(src_state as *mut SRC_PRIVATE)).mode =
        SRC_MODE_CALLBACK as libc::c_int;
    let ref mut fresh0 = (*(src_state as *mut SRC_PRIVATE)).callback_func;
    *fresh0 = func;
    let ref mut fresh1 =
        (*(src_state as *mut SRC_PRIVATE)).user_callback_data;
    *fresh1 = cb_data;
    return src_state;
}
/*
**	Cleanup all internal allocations.
**	Always returns NULL.
*/
#[no_mangle]
pub unsafe extern "C" fn src_delete(mut state: *mut SRC_STATE)
 -> *mut SRC_STATE {
    let mut psrc: *mut SRC_PRIVATE = 0 as *mut SRC_PRIVATE;
    psrc = state as *mut SRC_PRIVATE;
    if !psrc.is_null() {
        if !(*psrc).private_data.is_null() { free((*psrc).private_data); }
        memset(psrc as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<SRC_PRIVATE>() as libc::c_ulong);
        free(psrc as *mut libc::c_void);
    }
    return 0 as *mut SRC_STATE;
}
/*
**	Standard processing function.
**	Returns non zero on error.
*/
#[no_mangle]
pub unsafe extern "C" fn src_process(mut state: *mut SRC_STATE,
                                     mut data: *mut SRC_DATA) -> libc::c_int {
    let mut psrc: *mut SRC_PRIVATE = 0 as *mut SRC_PRIVATE;
    let mut error: libc::c_int = 0;
    psrc = state as *mut SRC_PRIVATE;
    if psrc.is_null() { return SRC_ERR_BAD_STATE as libc::c_int }
    if (*psrc).vari_process.is_none() || (*psrc).const_process.is_none() {
        return SRC_ERR_BAD_PROC_PTR as libc::c_int
    }
    if (*psrc).mode != SRC_MODE_PROCESS as libc::c_int {
        return SRC_ERR_BAD_MODE as libc::c_int
    }
    if data.is_null() { return SRC_ERR_BAD_DATA as libc::c_int }
    if (*data).data_in.is_null() &&
           (*data).input_frames > 0i32 as libc::c_long ||
           (*data).data_out.is_null() &&
               (*data).output_frames > 0i32 as libc::c_long {
        return SRC_ERR_BAD_DATA_PTR as libc::c_int
    }
    if 0 != is_bad_src_ratio((*data).src_ratio) {
        return SRC_ERR_BAD_SRC_RATIO as libc::c_int
    }
    if (*data).input_frames < 0i32 as libc::c_long {
        (*data).input_frames = 0i32 as libc::c_long
    }
    if (*data).output_frames < 0i32 as libc::c_long {
        (*data).output_frames = 0i32 as libc::c_long
    }
    if (*data).data_in < (*data).data_out as *const libc::c_float {
        if (*data).data_in.offset(((*data).input_frames *
                                       (*psrc).channels as libc::c_long) as
                                      isize) >
               (*data).data_out as *const libc::c_float {
            return SRC_ERR_DATA_OVERLAP as libc::c_int
        }
    } else if (*data).data_out.offset(((*data).output_frames *
                                           (*psrc).channels as libc::c_long)
                                          as isize) >
                  (*data).data_in as *mut libc::c_float {
        return SRC_ERR_DATA_OVERLAP as libc::c_int
    }
    (*data).input_frames_used = 0i32 as libc::c_long;
    (*data).output_frames_gen = 0i32 as libc::c_long;
    if (*psrc).last_ratio < 1.0f64 / 256i32 as libc::c_double {
        (*psrc).last_ratio = (*data).src_ratio
    }
    if fabs((*psrc).last_ratio - (*data).src_ratio) < 1e-15f64 {
        error =
            (*psrc).const_process.expect("non-null function pointer")(psrc,
                                                                      data)
    } else {
        error =
            (*psrc).vari_process.expect("non-null function pointer")(psrc,
                                                                     data)
    }
    return error;
}
/* fmod_one */
#[inline]
unsafe extern "C" fn is_bad_src_ratio(mut ratio: libc::c_double)
 -> libc::c_int {
    return (ratio < 1.0f64 / 256i32 as libc::c_double ||
                ratio > 1.0f64 * 256i32 as libc::c_double) as libc::c_int;
}
/*
**	Callback based processing function. Read up to frames worth of data from
**	the converter int *data and return frames read or -1 on error.
*/
#[no_mangle]
pub unsafe extern "C" fn src_callback_read(mut state: *mut SRC_STATE,
                                           mut src_ratio: libc::c_double,
                                           mut frames: libc::c_long,
                                           mut data: *mut libc::c_float)
 -> libc::c_long {
    let mut psrc: *mut SRC_PRIVATE = 0 as *mut SRC_PRIVATE;
    let mut src_data: SRC_DATA =
        SRC_DATA{data_in: 0 as *const libc::c_float,
                 data_out: 0 as *mut libc::c_float,
                 input_frames: 0,
                 output_frames: 0,
                 input_frames_used: 0,
                 output_frames_gen: 0,
                 end_of_input: 0,
                 src_ratio: 0.,};
    let mut output_frames_gen: libc::c_long = 0;
    let mut error: libc::c_int = 0i32;
    if state.is_null() { return 0i32 as libc::c_long }
    if frames <= 0i32 as libc::c_long { return 0i32 as libc::c_long }
    psrc = state as *mut SRC_PRIVATE;
    if (*psrc).mode != SRC_MODE_CALLBACK as libc::c_int {
        (*psrc).error = SRC_ERR_BAD_MODE as libc::c_int;
        return 0i32 as libc::c_long
    }
    if (*psrc).callback_func.is_none() {
        (*psrc).error = SRC_ERR_NULL_CALLBACK as libc::c_int;
        return 0i32 as libc::c_long
    }
    memset(&mut src_data as *mut SRC_DATA as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<SRC_DATA>() as libc::c_ulong);
    if 0 != is_bad_src_ratio(src_ratio) {
        (*psrc).error = SRC_ERR_BAD_SRC_RATIO as libc::c_int;
        return 0i32 as libc::c_long
    }
    src_data.src_ratio = src_ratio;
    src_data.data_out = data;
    src_data.output_frames = frames;
    src_data.data_in = (*psrc).saved_data;
    src_data.input_frames = (*psrc).saved_frames;
    output_frames_gen = 0i32 as libc::c_long;
    while output_frames_gen < frames {
        /*	Use a dummy array for the case where the callback function
		**	returns without setting the ptr.
		*/
        let mut dummy: [libc::c_float; 1] = [0.; 1];
        if src_data.input_frames == 0i32 as libc::c_long {
            let mut ptr: *mut libc::c_float = dummy.as_mut_ptr();
            src_data.input_frames =
                (*psrc).callback_func.expect("non-null function pointer")((*psrc).user_callback_data,
                                                                          &mut ptr);
            src_data.data_in = ptr;
            if src_data.input_frames == 0i32 as libc::c_long {
                src_data.end_of_input = 1i32
            }
        }
        (*psrc).mode = SRC_MODE_PROCESS as libc::c_int;
        error = src_process(state, &mut src_data);
        (*psrc).mode = SRC_MODE_CALLBACK as libc::c_int;
        if error != 0i32 { break ; }
        src_data.data_in =
            src_data.data_in.offset((src_data.input_frames_used *
                                         (*psrc).channels as libc::c_long) as
                                        isize);
        src_data.input_frames -= src_data.input_frames_used;
        src_data.data_out =
            src_data.data_out.offset((src_data.output_frames_gen *
                                          (*psrc).channels as libc::c_long) as
                                         isize);
        src_data.output_frames -= src_data.output_frames_gen;
        output_frames_gen += src_data.output_frames_gen;
        if src_data.end_of_input == SRC_TRUE as libc::c_int &&
               src_data.output_frames_gen == 0i32 as libc::c_long {
            break ;
        }
    }
    (*psrc).saved_data = src_data.data_in;
    (*psrc).saved_frames = src_data.input_frames;
    if error != 0i32 { (*psrc).error = error; return 0i32 as libc::c_long }
    return output_frames_gen;
}
/*
**	Simple interface for performing a single conversion from input buffer to
**	output buffer at a fixed conversion ratio.
**	Simple interface does not require initialisation as it can only operate on
**	a single buffer worth of audio.
*/
#[no_mangle]
pub unsafe extern "C" fn src_simple(mut src_data: *mut SRC_DATA,
                                    mut converter: libc::c_int,
                                    mut channels: libc::c_int)
 -> libc::c_int {
    let mut src_state: *mut SRC_STATE = 0 as *mut SRC_STATE;
    let mut error: libc::c_int = 0;
    src_state = src_new(converter, channels, &mut error);
    if src_state.is_null() { return error }
    (*src_data).end_of_input = 1i32;
    error = src_process(src_state, src_data);
    src_delete(src_state);
    return error;
}
/*
** This library contains a number of different sample rate converters,
** numbered 0 through N.
**
** Return a string giving either a name or a more full description of each
** sample rate converter or NULL if no sample rate converter exists for
** the given value. The converters are sequentially numbered from 0 to N.
*/
#[no_mangle]
pub unsafe extern "C" fn src_get_name(mut converter_type: libc::c_int)
 -> *const libc::c_char {
    let mut desc: *const libc::c_char = 0 as *const libc::c_char;
    desc = sinc_get_name(converter_type);
    if !desc.is_null() { return desc }
    desc = zoh_get_name(converter_type);
    if !desc.is_null() { return desc }
    desc = linear_get_name(converter_type);
    if !desc.is_null() { return desc }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn src_get_description(mut converter_type: libc::c_int)
 -> *const libc::c_char {
    let mut desc: *const libc::c_char = 0 as *const libc::c_char;
    desc = sinc_get_description(converter_type);
    if !desc.is_null() { return desc }
    desc = zoh_get_description(converter_type);
    if !desc.is_null() { return desc }
    desc = linear_get_description(converter_type);
    if !desc.is_null() { return desc }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn src_get_version() -> *const libc::c_char {
    return b"libsamplerate-0.1.9 (c) 2002-2008 Erik de Castro Lopo\x00" as
               *const u8 as *const libc::c_char;
}
/*
**	Set a new SRC ratio. This allows step responses
**	in the conversion ratio.
**	Returns non zero on error.
*/
#[no_mangle]
pub unsafe extern "C" fn src_set_ratio(mut state: *mut SRC_STATE,
                                       mut new_ratio: libc::c_double)
 -> libc::c_int {
    let mut psrc: *mut SRC_PRIVATE = 0 as *mut SRC_PRIVATE;
    psrc = state as *mut SRC_PRIVATE;
    if psrc.is_null() { return SRC_ERR_BAD_STATE as libc::c_int }
    if (*psrc).vari_process.is_none() || (*psrc).const_process.is_none() {
        return SRC_ERR_BAD_PROC_PTR as libc::c_int
    }
    if 0 != is_bad_src_ratio(new_ratio) {
        return SRC_ERR_BAD_SRC_RATIO as libc::c_int
    }
    (*psrc).last_ratio = new_ratio;
    return SRC_ERR_NO_ERROR as libc::c_int;
}
/*
**	Get the current channel count.
**	Returns negative on error, positive channel count otherwise
*/
#[no_mangle]
pub unsafe extern "C" fn src_get_channels(mut state: *mut SRC_STATE)
 -> libc::c_int {
    let mut psrc: *mut SRC_PRIVATE = 0 as *mut SRC_PRIVATE;
    psrc = state as *mut SRC_PRIVATE;
    if psrc.is_null() { return SRC_ERR_BAD_STATE as libc::c_int }
    if (*psrc).vari_process.is_none() || (*psrc).const_process.is_none() {
        return SRC_ERR_BAD_PROC_PTR as libc::c_int
    }
    return (*psrc).channels;
}
/*
** Return TRUE if ratio is a valid conversion ratio, FALSE
** otherwise.
*/
#[no_mangle]
pub unsafe extern "C" fn src_is_valid_ratio(mut ratio: libc::c_double)
 -> libc::c_int {
    if 0 != is_bad_src_ratio(ratio) { return SRC_FALSE as libc::c_int }
    return SRC_TRUE as libc::c_int;
}
/*
**	Return an error number.
*/
#[no_mangle]
pub unsafe extern "C" fn src_error(mut state: *mut SRC_STATE) -> libc::c_int {
    if !state.is_null() { return (*(state as *mut SRC_PRIVATE)).error }
    return SRC_ERR_NO_ERROR as libc::c_int;
}
/*
**	Convert the error number into a string.
*/
#[no_mangle]
pub unsafe extern "C" fn src_strerror(mut error: libc::c_int)
 -> *const libc::c_char {
    match error {
        0 => { return b"No error.\x00" as *const u8 as *const libc::c_char }
        1 => {
            return b"Malloc failed.\x00" as *const u8 as *const libc::c_char
        }
        2 => {
            return b"SRC_STATE pointer is NULL.\x00" as *const u8 as
                       *const libc::c_char
        }
        3 => {
            return b"SRC_DATA pointer is NULL.\x00" as *const u8 as
                       *const libc::c_char
        }
        4 => {
            return b"SRC_DATA->data_out or SRC_DATA->data_in is NULL.\x00" as
                       *const u8 as *const libc::c_char
        }
        5 => {
            return b"Internal error. No private data.\x00" as *const u8 as
                       *const libc::c_char
        }
        6 => {
            return b"SRC ratio outside [1/256, 256] range.\x00" as *const u8
                       as *const libc::c_char
        }
        15 => {
            return b"src_process() called without reset after end_of_input.\x00"
                       as *const u8 as *const libc::c_char
        }
        7 => {
            return b"Internal error. No process pointer.\x00" as *const u8 as
                       *const libc::c_char
        }
        8 => {
            return b"Internal error. SHIFT_BITS too large.\x00" as *const u8
                       as *const libc::c_char
        }
        9 => {
            return b"Internal error. Filter length too large.\x00" as
                       *const u8 as *const libc::c_char
        }
        10 => {
            return b"Bad converter number.\x00" as *const u8 as
                       *const libc::c_char
        }
        11 => {
            return b"Channel count must be >= 1.\x00" as *const u8 as
                       *const libc::c_char
        }
        12 => {
            return b"Internal error. Bad buffer length. Please report this.\x00"
                       as *const u8 as *const libc::c_char
        }
        13 => {
            return b"Internal error. Input data / internal buffer size difference. Please report this.\x00"
                       as *const u8 as *const libc::c_char
        }
        14 => {
            return b"Internal error. Private pointer is NULL. Please report this.\x00"
                       as *const u8 as *const libc::c_char
        }
        16 => {
            return b"Input and output data arrays overlap.\x00" as *const u8
                       as *const libc::c_char
        }
        17 => {
            return b"Supplied callback function pointer is NULL.\x00" as
                       *const u8 as *const libc::c_char
        }
        18 => {
            return b"Calling mode differs from initialisation mode (ie process v callback).\x00"
                       as *const u8 as *const libc::c_char
        }
        19 => {
            return b"Callback function pointer is NULL in src_callback_read ().\x00"
                       as *const u8 as *const libc::c_char
        }
        20 => {
            return b"This converter only allows constant conversion ratios.\x00"
                       as *const u8 as *const libc::c_char
        }
        21 => {
            return b"Internal error : Bad length in prepare_data ().\x00" as
                       *const u8 as *const libc::c_char
        }
        22 => {
            return b"Error : Someone is trampling on my internal state.\x00"
                       as *const u8 as *const libc::c_char
        }
        23 => {
            return b"Placeholder. No error defined for this error number.\x00"
                       as *const u8 as *const libc::c_char
        }
        _ => { }
    }
    return 0 as *const libc::c_char;
}
/*
** Extra helper functions for converting from short to float and
** back again.
*/
#[no_mangle]
pub unsafe extern "C" fn src_short_to_float_array(mut in_0:
                                                      *const libc::c_short,
                                                  mut out: *mut libc::c_float,
                                                  mut len: libc::c_int) {
    while 0 != len {
        len -= 1;
        *out.offset(len as isize) =
            (*in_0.offset(len as isize) as libc::c_int as libc::c_double /
                 (1.0f64 * 0x8000i32 as libc::c_double)) as libc::c_float
    };
}
#[no_mangle]
pub unsafe extern "C" fn src_float_to_short_array(mut in_0:
                                                      *const libc::c_float,
                                                  mut out: *mut libc::c_short,
                                                  mut len: libc::c_int) {
    let mut scaled_value: libc::c_double = 0.;
    while 0 != len {
        len -= 1;
        scaled_value =
            *in_0.offset(len as isize) as libc::c_double *
                (8.0f64 * 0x10000000i32 as libc::c_double);
        if 0i32 == 0i32 &&
               scaled_value >= 1.0f64 * 0x7fffffffi32 as libc::c_double {
            *out.offset(len as isize) = 32767i32 as libc::c_short
        } else if 0i32 == 0i32 &&
                      scaled_value <=
                          -8.0f64 * 0x10000000i32 as libc::c_double {
            *out.offset(len as isize) = -32768i32 as libc::c_short
        } else {
            *out.offset(len as isize) =
                (lrint(scaled_value) >> 16i32) as libc::c_short
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn src_int_to_float_array(mut in_0: *const libc::c_int,
                                                mut out: *mut libc::c_float,
                                                mut len: libc::c_int) {
    while 0 != len {
        len -= 1;
        *out.offset(len as isize) =
            (*in_0.offset(len as isize) as libc::c_double /
                 (8.0f64 * 0x10000000i32 as libc::c_double)) as libc::c_float
    };
}
#[no_mangle]
pub unsafe extern "C" fn src_float_to_int_array(mut in_0:
                                                    *const libc::c_float,
                                                mut out: *mut libc::c_int,
                                                mut len: libc::c_int) {
    let mut scaled_value: libc::c_double = 0.;
    while 0 != len {
        len -= 1;
        scaled_value =
            *in_0.offset(len as isize) as libc::c_double *
                (8.0f64 * 0x10000000i32 as libc::c_double);
        if 0i32 == 0i32 &&
               scaled_value >= 1.0f64 * 0x7fffffffi32 as libc::c_double {
            *out.offset(len as isize) = 0x7fffffffi32
        } else if 0i32 == 0i32 &&
                      scaled_value <=
                          -8.0f64 * 0x10000000i32 as libc::c_double {
            *out.offset(len as isize) = -1i32 - 0x7fffffffi32
        } else {
            *out.offset(len as isize) = lrint(scaled_value) as libc::c_int
        }
    };
}