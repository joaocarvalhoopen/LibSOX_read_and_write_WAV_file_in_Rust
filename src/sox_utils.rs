/// LibSOX read and write WAV file in Rust.
/// 
/// Author: João Nuno Carvalho
/// Date: 2022.01.01
/// 
/// Description: This is a simple example of the implementation and usage of a
///              read and write WAV files to and from buffers function with
///              libSOX in Rust.
///              It uses unsafe and FFI all over the code, but it was fun :-) .
/// 
/// License: MIT Open Source License.
/// 
/// LibSOX info, install and configurations:
///  
/// The following is a text that I wrote on the subject that explains many things
/// about libSOX, where to find things, how to install and compile it in Rust, the
/// all enchilada! Have fun!
///   (See README.md in the project page for full text)
/// 


use std::ptr;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};

use libc::free;

use libsox_sys::*;

pub struct Signal {
    channels:  u32,    // 1 ou 2 channels.
    length:    u64, 
    precision: u32,    // Is the number of bit's.
    rate:      f64,    // ex: 8_000 as f64
}

// TODO: Add a Result with the Error.
pub fn read_file_to_buffer(filename_in: & str) -> Result<(Signal, Vec<f32>), String> {
    let mut buf_vec: Vec<f32> = Vec::new();

    let signal: Signal;

    let c_filename: CString = CString::new(filename_in).unwrap();

    let mut c_in_file: *mut sox_format_t = ptr::null_mut();

    let res: c_int;
    unsafe {
        // All libSoX applications must start by initialising the SoX library
        assert!(sox_init() == sox_error_t_SOX_SUCCESS);

        // Open the input file (with default parameters)
        c_in_file = sox_open_read(c_filename.as_ptr(), ptr::null(), ptr::null(), ptr::null());

        assert!(c_in_file != ptr::null_mut());

        signal = Signal {
            channels:  (*c_in_file).signal.channels,
            length:    (*c_in_file).signal.length,
            precision: (*c_in_file).signal.precision,
            rate:      (*c_in_file).signal.rate,
        };

        let osamp: *mut usize = & mut 0_usize; 

        // TODO: Lidar com o caso de o read ser só parcial e necessitar-se de fazer novo read.
        *osamp = ((*c_in_file).signal.length) as usize;


        // We need to allocate an array or a block of memory of the type "sox_sample_t", so that C can fill it with values.
        let array_size = *osamp;
        let obuf: *mut sox_sample_t = libc::malloc(std::mem::size_of::<sox_sample_t>() * array_size) as *mut sox_sample_t;

        // ensure that *osamp is a multiple of the number of channels.
        // *osamp -= *osamp % (*effp).out_signal.channels as usize;
        *osamp -= *osamp % (*c_in_file).signal.channels as usize;

        // Read up to *osamp samples into obuf; store the actual number read
        // back to *osamp
        *osamp = sox_read(c_in_file, obuf, *osamp);
        
        // sox_read may return a number that is less than was requested; only if
        // 0 samples is returned does it indicate that end-of-file has been reached
        // or an error has occurred
        if *osamp == 0 && (*c_in_file).sox_errno != 0 {
            eprintln!("{:?}: {:?}", CStr::from_ptr((*c_in_file).filename), CStr::from_ptr((*c_in_file).sox_errstr.as_ptr()));
        }
        res = if *osamp != 0 { sox_error_t_SOX_SUCCESS } else { sox_error_t_SOX_EOF };
        
        // Copy the C buffer to the Rust buffer.
        for i in 0..(*osamp) {

            let val_f64 = *obuf.offset(i as isize) as f64;
            // Convert and normalize [-i32, +i32] to between [-1, 1] .
            let val_f32: f32 = if val_f64 >= 0.0 {
                    ( val_f64 / (i32::max_value() as f64) ) as f32 
                } else {
                    ( -val_f64 / (i32::min_value() as f64) ) as f32
                };    

            buf_vec.push(val_f32);
        }

        sox_close(c_in_file);
        sox_quit();
    }

    match res {
        sox_error_t_SOX_SUCCESS => Ok((signal, buf_vec)),
        sox_error_t_SOX_EOF => Err("Error: EOF reached!".to_string()),
        _ => Err("Error: Undetermined error".to_string()),    
    }
}

// TODO: Add a Result with the Error.
pub fn write_file_from_buffer(filename_out: & str, signal: & Signal, buffer: & Vec<f32>) -> Result<(), String> {

    let res:c_int;

    let c_filename: CString = CString::new(filename_out).unwrap();

    let mut c_out_file: *mut sox_format_t = ptr::null_mut(); 

    let res: c_int;
    unsafe {
        // All libSoX applications must start by initialising the SoX library
        assert!(sox_init() == sox_error_t_SOX_SUCCESS);

        // Create and allocate the signal structure.
        let array_size = 1;
        let mut signal_struct: * mut sox_signalinfo_t = libc::malloc(std::mem::size_of::<sox_signalinfo_t>() * array_size) as *mut sox_signalinfo_t;

        // // Fill the signal structure.
        // (*signal_struct).channels  = 1;
        // (*signal_struct).length    = 0;               // Note: I think that this value is filled after writing.
        // (*signal_struct).precision = 16;              // Note: I think is the number of bit's.
        // (*signal_struct).rate      = 8_000 as f64;    // This depends on the buffer that was reeded.
        // (*signal_struct).mult = ptr::null_mut();   // as *mut f64;

        // Fill the signal structure.
        (*signal_struct).channels  = signal.channels;
        (*signal_struct).length    = signal.length;      // Note: I think that this value is filled after writing.
        (*signal_struct).precision = signal.precision;   // Note: I think is the number of bit's.
        (*signal_struct).rate      = signal.rate;        // This depends on the buffer that was reeded.
        
        (*signal_struct).mult = ptr::null_mut();   // as *mut f64;

        // Open the output file; we must specify the output signal characteristics.
        // Since we are using only simple effects, they are the same as the input
        // file characteristics 
        c_out_file = sox_open_write(c_filename.as_ptr(), signal_struct, ptr::null(), ptr::null(), ptr::null(), None);
        // assert!(c_out_file != ptr::null_mut());
        if c_out_file == ptr::null_mut() { 
            return Err(format!("Error: Undetermined error writing {:?}", CStr::from_ptr(c_filename.as_ptr()) ));
        }

        // Allocate the inner buffer.
        let mut isamp = buffer.len();
        let ibuf: *mut sox_sample_t = libc::malloc(std::mem::size_of::<sox_sample_t>() * isamp) as *mut sox_sample_t;

        // ensure that *osamp is a multiple of the number of channels.
        // *osamp -= *osamp % (*effp).out_signal.channels as usize;
        isamp -= isamp % (* signal_struct).channels as usize;

        // Copy the Rust buffer to the C buffer.
        for i in 0..buffer.len() {
            let val_f64 = buffer[i] as f64;

            if val_f64 > 1.0 || val_f64 < -1.0 {
                return Err("Error: write_file_from_buffer, buffer out of bounds [-1, 1] .".to_string());
            }
            
            // Convert and des-normalize [-1, 1] to between [-i32, +i32] .
            let val_i32: i32 = if val_f64 >= 0.0 {
                    ( val_f64 * (i32::max_value() as f64) ) as i32 
                } else {
                    ( -val_f64 * (i32::min_value() as f64) ) as i32
                };

            *ibuf.offset(i as isize) = val_i32;
        }

        // IMPORTANT: There should be a way to do a memcpy() in Rust, or something like it.
        //            I can always do a memcpy() by calling the libc :-)
        //            TODO: See this later!
        //                  I can also get a pointer to the underling array of the vec<f32> if it was a vec<i32>
        //                  and cast the pointer and send the pointer directly to the sox_write() function
        //                  as the buffer.  

        // Write out *isamp samples
        let len = sox_write(c_out_file, ibuf, isamp);

        // len is the number of samples that were actually written out; if this is
        // different to *isamp, then something has gone wrong--most often, it's
        // out of disc space
        if len != isamp {
            eprintln!("{:?}: {:?}", CStr::from_ptr((* c_out_file).filename), CStr::from_ptr((* c_out_file).sox_errstr.as_ptr()));
            res = sox_error_t_SOX_EOF;
        } else {
            res = sox_error_t_SOX_SUCCESS; // All samples output successfully
        }

        sox_close(c_out_file);
        sox_quit();
    }

    match res {
        sox_error_t_SOX_SUCCESS => Ok(()),
        sox_error_t_SOX_EOF => Err("Error: Writing reached the end of file!".to_string()),
        _ => Err("Error: Undetermined error writing!".to_string()),
    }
}