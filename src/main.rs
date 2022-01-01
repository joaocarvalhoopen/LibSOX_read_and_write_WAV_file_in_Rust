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

mod example1;

mod sox_utils;

use std::env;
use std::ffi::CString;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    eprintln!("{}", args.len());
    assert!(args.len() == 3);

    // call_example1(& args);

    call_read_write_buffer(& args);
}

//*********************
// example1.rs
fn call_example1(args: & Vec<String>) {
    // Converts the Vec<String> in Vec<CString>.
    let cstr_argv: Vec<CString> = args.iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    // Pointer to arg_v  (p_argv) .
    //
    // let mut p_argv: Vec<_> = cstr_argv.iter() // do NOT into_iter()
    //     .map(|arg| arg.as_ptr())
    //     .collect();

    // p_argv.push(std::ptr::null());

    unsafe {
        // let args: Vec<CString> = args.map(|string_tmp| { CString::new(string_tmp)}).map(Result::unwrap).collect();  
        // example1::unsafe_main(args);

        example1::unsafe_main(cstr_argv); 
    }
}

//**********************

//*********************
// Read buffer
fn call_read_write_buffer(args: & Vec<String>) {
    // Read buffer.
    let filename_in: String = args[1].clone(); 
    let mut buffer: Vec<f32>;
    let res = sox_utils::read_file_to_buffer(& filename_in);
    if let Err(error) = res {
        println!("Error: reading file {}\n {}", filename_in, error);
        process::exit(0);
    }

    let ( signal, mut buffer) = res.unwrap();

    // Half the amplitude (8 times less).
    for val in & mut buffer {
       *val = *val / 8.0_f32;
    }

    // Write buffer.
    let filename_out: String = args[2].clone(); 
    if let Err(error) = sox_utils::write_file_from_buffer(& filename_out, & signal, & buffer) {
        println!("Error: writing file {}\n {}", filename_out, error);
        process::exit(0);
    }

}


