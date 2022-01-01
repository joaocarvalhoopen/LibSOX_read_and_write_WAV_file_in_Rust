# LibSOX read and write WAV file in Rust
Having fun with libSOX and Rust FFI and unsafe.

## Description
This is a simple example of the implementation and usage of a read and write WAV files to and from buffers function with libSOX in Rust. It uses unsafe and FFI all over the code, but it was fun :-) .

## You can ear the WAV files in here

* **Input WAV** <br>
  [1.wav](./1.wav)

* **Output WAV** <br>
  [out.wav](./out.wav)


# LibSOX info, install and configurations 
The following is a text that I wrote on the subject that explains many things about libSOX, where to find things, how to install and compile it in Rust, the all enchilada! Have fun!


## Part 1

Hello all, <br>
<br>
recently I tought of making some small programs in the theme of audio that used neural nets and was thinking of using PyTorch, but this time I would not use Python, I was thinking in using Rust. I would use the Rust bindings to the libs of PyTorch that are written bellow in C_plus_plus under the Python front-end. <br>

* **tch** <br>
  Rust wrappers for the PyTorch C++ api (libtorch). <br>
  [https://crates.io/crates/tch](https://crates.io/crates/tch)

* **Examples** <br>
  [https://github.com/LaurentMazare/tch-rs/tree/main/examples](https://github.com/LaurentMazare/tch-rs/tree/main/examples)

But then I would like to use in Rust, the PyTorch audio libs, although those aren't made from start in C or C_plus_plus in PyTorch, they are made as libs of Python that map the libSOX from the great program SoX - Sound eXchange, to read and write files and to implement the effects. <br>
<br>
In Python there is also one other good library for audio processing called libROSA, mas that is also a C/C_plus_plus made exclusively for Python, with objects in Python. <br>

* **libROSA** <br>
  [https://librosa.org/](https://librosa.org/)

But getting back to SOX ... I know of this program for many years, but never never have seen all of it's features that are listed in it's manual in detail. You can see here the list of features: <br>

* [http://sox.sourceforge.net/Docs/Features](http://sox.sourceforge.net/Docs/Features)

SOX makes a lot of things in a single program and in a single library, the libSOX. LibSOX, has a very simple interface with few functions and structures in C. You can execute effects and there is the concept of a chain (sequence) of effects. In which, you can put an effect or a filter after the other in a path that the audio signal runs along. The Signal enters in a side and comes out from the other side. You can also have mixing between several audio files, with multiple channels. <br>

* **SOX code project site** <br>
  [http://sox.sourceforge.net/](http://sox.sourceforge.net/)

* **The SOX manual can be found here** <br>
  [http://sox.sourceforge.net/sox.html](http://sox.sourceforge.net/sox.html)

* **The libSOX manual can be found here** <br>
  [http://sox.sourceforge.net/libsox.html](http://sox.sourceforge.net/libsox.html)

* **The libSOX .h file can be study here, it is sox.h, common to all SOX** <br>
  [https://sourceforge.net/p/sox/code/ci/master/tree/src/sox.h](https://sourceforge.net/p/sox/code/ci/master/tree/src/sox.h)

The libSOX Rust bindings are generated automatically in this crate (package). <br>

* **Crate - libsox-sys** <br>
  [https://crates.io/crates/libsox-sys](https://crates.io/crates/libsox-sys)

To use this lib in Rust, you must use unsafe code blocks or functions. A good example of how to use them in this context, including the application of effects is in here: <br>

* [https://github.com/vtavernier/libsox-sys/blob/master/examples/example1.rs](https://github.com/vtavernier/libsox-sys/blob/master/examples/example1.rs)

* **All the effects are listed in here** <br>
  [https://sourceforge.net/p/sox/code/ci/master/tree/src/effects.h](https://sourceforge.net/p/sox/code/ci/master/tree/src/effects.h)

There are 7 examples of using the libSOX in C and they can used as a starting point to understand how to use the bindings in Rust for libSOX, they are: <br>

* [https://sourceforge.net/p/sox/code/ci/master/tree/src/example0.c](https://sourceforge.net/p/sox/code/ci/master/tree/src/example0.c) <br>
  [https://sourceforge.net/p/sox/code/ci/master/tree/src/example1.c](https://sourceforge.net/p/sox/code/ci/master/tree/src/example1.c) <br>
  [https://sourceforge.net/p/sox/code/ci/master/tree/src/example2.c](https://sourceforge.net/p/sox/code/ci/master/tree/src/example2.c) <br>
  [https://sourceforge.net/p/sox/code/ci/master/tree/src/example3.c](https://sourceforge.net/p/sox/code/ci/master/tree/src/example3.c) <br>
  [https://sourceforge.net/p/sox/code/ci/master/tree/src/example4.c](https://sourceforge.net/p/sox/code/ci/master/tree/src/example4.c) <br>
  [https://sourceforge.net/p/sox/code/ci/master/tree/src/example5.c](https://sourceforge.net/p/sox/code/ci/master/tree/src/example5.c) <br>
  [https://sourceforge.net/p/sox/code/ci/master/tree/src/example6.c](https://sourceforge.net/p/sox/code/ci/master/tree/src/example6.c) <br>

A good exercise would be to re-implement all the 6 examples that aren't implemented yet in Rust. <br>

* **The documentation of the bindings in Rust can be seen here.** <br>
  [https://vtavernier.github.io/libsox-sys/libsox_sys/index.html](https://vtavernier.github.io/libsox-sys/libsox_sys/index.html)

There is also specific libraries in Rust for ReSample de buffer with libSOX ReSampler Library. <br>

* **libsoxr** <br>
  Wrapper for libsoxr (resampling library for sounds) <br>
  [https://crates.io/crates/libsoxr](https://crates.io/crates/libsoxr)

In Rust there is also other lib for audio processing, but is more focused in the low level. <br>

* **dasp** <br>
  A crate providing the fundamentals for working with audio PCM DSP. <br>
  [https://crates.io/crates/dasp](https://crates.io/crates/dasp)


## Part 2

Hello all again, <br>
<br>
Today I tried to install the Rust libsox-sys, in Linux Ubuntu 20.04 and it was giving me compilation errors. Because others may try to make the same thing and encounter the some problems, I will give here a small solution so that you can compile the example that is in the following directory: <br>

* [https://github.com/vtavernier/libsox-sys/tree/master/examples](https://github.com/vtavernier/libsox-sys/tree/master/examples)

First go to the directory were you want to put your project in and then create one project for testing the libSOX. I called this project libsox_test: <br>

```
> cargo new libsox_test
> cd libsox_test
```

Add to the file .toml the following dependencies: <br>

```
[dependencies]
libc = "0.2.106"
libsox-sys = "0.1.1"
```

Then copy the following file. <br>

* [https://github.com/vtavernier/libsox-sys/tree/master/examples/example1.rs](https://github.com/vtavernier/libsox-sys/tree/master/examples/example1.rs)

to ``` /src/main.rs ``` <br>
<br>

Then install the packages of the dependencies in Ubuntu/Debian: <br>

### 1. Install SoX.

```
> sudo apt-get update
> sudo apt-get install sox
```

### 2. Install libsox-dev.

```
> sudo apt-get install libsox-dev
```

### 3. Install clang

If you don't have clang installed, the Rust compiler and Cargo can't compile the lib.rs. To correct that erro, install the clang compiler with the linux package libclang. 

* Debian/Ubuntu:

```
> sudo apt-get install libclang-dev
```

* **Note:** To found out the solution to the second problem I followed the following tutorial: <br>
  **Fixing Rust compilation issues caused by missing packages, part 2** <br>
  [https://www.unadulterated-faff.com/articles/2020/02/04/fixing-rust-compilation-issues-caused-by-missing-packages-part-2.html](https://www.unadulterated-faff.com/articles/2020/02/04/fixing-rust-compilation-issues-caused-by-missing-packages-part-2.html)

Then you can compile, by making: <br>

```
> cargo build       or      cargo build --release
```

To execute fins one WAV file for example 1.wav and do: <br> 

```
> cargo run 1.wav out.wav
```

The example program will generate the output but you will not notice any difference in the sound, so alter the ``` line 103 in main.rs ```. <br>

```
// from:
let vol = [b"3dB\0".as_ptr() as *mut i8];
```

```
// to:
let vol = [b"-6dB\0".as_ptr() as *mut i8];
```

Execute again. <br>

```
> cargo run 1.wav out.wav
```

Now you are going to listen the second (output) WAV with a lower amplitude then the first one. <br>


## Part 3

I would like to leave here the following note: <br>
<br>
Rust unsafe code must be used so that you can use the libSOX that is written in C from a Rust program. You can compare the examples that I have listed in Part 1, you can start with exemplo1.c and exemplo1.rs. <br>
<br>
If you pay attention you will see some important differences, between the call in C and the call's in Rust. I have added here 3 of those differences. <br>
<br>
**In C:** <br>

```C
/* input and output files */
static sox_format_t * in, * out;
```

**In Rust:** <br>

```Rust
static mut IN_FILE: *mut sox_format_t = ptr::null_mut();
static mut OUT_FILE: *mut sox_format_t = ptr::null_mut();
```

**In C:** <br>

```C
char * vol[] = {"3dB"};
```

**In Rust:** <br>

```Rust
let vol = [b"3dB\0".as_ptr() as *mut i8];
```

**In C:** <br>

```C
assert(sox_add_effect(chain, e, &in->signal, &in->signal) == SOX_SUCCESS);
```

**In Rust:** <br>

```Rust
assert!(sox_add_effect(chain, e, &mut (*IN_FILE).signal, &(*IN_FILE).signal) == sox_error_t_SOX_SUCCESS);
```

You only need unsafe code to create an abstraction layer, then you can use normal Rust safe code for the rest of you program. <br>


## License
MIT Open Source License.


## Have fun!
Best regards, <br>
João Nuno Carvalho <br>
