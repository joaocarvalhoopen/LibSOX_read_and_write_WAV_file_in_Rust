// Note: I think this is an example from the crate github, the other files I made them.

// This example is a straight port from the corresponding example in the SoX source code repository
// https://sourceforge.net/p/sox/code/ci/master/tree/src/example1.c

// use std::env;
use std::ptr;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};

use libc::free;

use libsox_sys::*;

static mut IN_FILE: *mut sox_format_t = ptr::null_mut();
static mut OUT_FILE: *mut sox_format_t = ptr::null_mut();

// The function that will be called to input samples into the effects chain.
// In this example, we get samples to process from a SoX-openned audio file.
// In a different application, they might be generated or come from a different
// part of the application.
unsafe extern "C" fn input_drain(effp: *mut sox_effect_t, obuf: *mut sox_sample_t, osamp: *mut usize) -> c_int {
    // ensure that *osamp is a multiple of the number of channels.
    *osamp -= *osamp % (*effp).out_signal.channels as usize;

    // Read up to *osamp samples into obuf; store the actual number read
    // back to *osamp
    *osamp = sox_read(IN_FILE, obuf, *osamp);

    // sox_read may return a number that is less than was requested; only if
    // 0 samples is returned does it indicate that end-of-file has been reached
    // or an error has occurred
    if *osamp == 0 && (*IN_FILE).sox_errno != 0 {
        eprintln!("{:?}: {:?}", CStr::from_ptr((*IN_FILE).filename), CStr::from_ptr((*IN_FILE).sox_errstr.as_ptr()));
    }
    return if *osamp != 0 { sox_error_t_SOX_SUCCESS } else { sox_error_t_SOX_EOF };
}

// The function that will be called to output samples from the effects chain.
// In this example, we store the samples in a SoX-opened audio file.
// In a different application, they might perhaps be analysed in some way,
// or displayed as a wave-form
unsafe extern "C" fn output_flow(_effp: *mut sox_effect_t, ibuf: *const sox_sample_t, _obuf: *mut sox_sample_t, isamp: *mut usize, osamp: *mut usize) -> c_int {
    // Write out *isamp samples
    let len = sox_write(OUT_FILE, ibuf, *isamp);

    // len is the number of samples that were actually written out; if this is
    // different to *isamp, then something has gone wrong--most often, it's
    // out of disc space
    if len != *isamp {
        eprintln!("{:?}: {:?}", CStr::from_ptr((*OUT_FILE).filename), CStr::from_ptr((*OUT_FILE).sox_errstr.as_ptr()));
        return sox_error_t_SOX_EOF;
    }

    // Outputting is the last `effect' in the effect chain so always passes
    // 0 samples on to the next effect (as there isn't one!)
    *osamp = 0;

    return sox_error_t_SOX_SUCCESS; // All samples output successfully
}

// Reads input file, applies vol & flanger effects, stores in output file.
// E.g. example1 monkey.au monkey.aiff
pub unsafe fn unsafe_main(args: Vec<CString>) {
    // All libSoX applications must start by initialising the SoX library
    assert!(sox_init() == sox_error_t_SOX_SUCCESS);

    // Open the input file (with default parameters)
    IN_FILE = sox_open_read(args[1].as_ptr(), ptr::null(), ptr::null(), ptr::null());
    assert!(IN_FILE != ptr::null_mut());

    // Open the output file; we must specify the output signal characteristics.
    // Since we are using only simple effects, they are the same as the input
    // file characteristics 
    OUT_FILE = sox_open_write(args[2].as_ptr(), &(*IN_FILE).signal, ptr::null(), ptr::null(), ptr::null(), None);
    assert!(OUT_FILE != ptr::null_mut());

    // Create an effects chain; some effects need to know about the input
    // or output file encoding so we provide that information here 
    let chain = sox_create_effects_chain(&(*IN_FILE).encoding, &(*OUT_FILE).encoding);

    // A `stub' effect handler to handle inputting samples to the effects
    // chain; the only function needed for this example is `drain'
    let input_handler = sox_effect_handler_t {
        name: b"input\0".as_ptr() as *const i8,
        usage: ptr::null(),
        flags: SOX_EFF_MCHAN,
        getopts: None,
        start: None,
        flow: None,
        drain: Some(input_drain),
        stop: None,
        kill: None,
        priv_size: 0
    };

    // The first effect in the effect chain must be something that can source
    // samples; in this case, we have defined an input handler that inputs
    // data from an audio file 
    let e = sox_create_effect(&input_handler);
    // This becomes the first `effect' in the chain 
    assert!(sox_add_effect(chain, e, &mut (*IN_FILE).signal, &(*IN_FILE).signal) == sox_error_t_SOX_SUCCESS);
    free(e as *mut c_void);

    // let vol = [b"3dB\0".as_ptr() as *mut i8];
    let vol = [b"-6dB\0".as_ptr() as *mut i8];

    // Create the `vol' effect, and initialise it with the desired parameters: 
    let e = sox_create_effect(sox_find_effect(b"vol\0".as_ptr() as *const i8));
    assert!(sox_effect_options(e, 1, vol.as_ptr()) == sox_error_t_SOX_SUCCESS);
    // Add the effect to the end of the effects processing chain: 
    assert!(sox_add_effect(chain, e, &mut (*IN_FILE).signal, &(*IN_FILE).signal) == sox_error_t_SOX_SUCCESS);
    free(e as *mut c_void);

    // Create the `flanger' effect, and initialise it with default parameters: 
    let e = sox_create_effect(sox_find_effect(b"flanger\0".as_ptr() as *const i8));
    assert!(sox_effect_options(e, 0, ptr::null()) == sox_error_t_SOX_SUCCESS);
    // Add the effect to the end of the effects processing chain: 
    assert!(sox_add_effect(chain, e, &mut (*IN_FILE).signal, &(*IN_FILE).signal) == sox_error_t_SOX_SUCCESS);
    free(e as *mut c_void);

    // A `stub' effect handler to handle outputting samples from the effects
    // chain; the only function needed for this example is `flow'
    let output_handler = sox_effect_handler_t {
        name: b"output\0".as_ptr() as *const i8,
        usage: ptr::null(),
        flags: SOX_EFF_MCHAN,
        getopts: None,
        start: None,
        flow: Some(output_flow),
        drain: None,
        stop: None,
        kill: None,
        priv_size: 0
    };

    // The last effect in the effect chain must be something that only consumes
    // samples; in this case, we have defined an output handler that outputs
    // data to an audio file 
    let e = sox_create_effect(&output_handler);
    assert!(sox_add_effect(chain, e, &mut (*IN_FILE).signal, &(*IN_FILE).signal) == sox_error_t_SOX_SUCCESS);
    free(e as *mut c_void);

    // Flow samples through the effects processing chain until EOF is reached 
    sox_flow_effects(chain, None, ptr::null_mut());

    // All done; tidy up:
    sox_delete_effects_chain(chain);
    sox_close(OUT_FILE);
    sox_close(IN_FILE);
    sox_quit();
}