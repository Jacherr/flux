# flux

High-level command-line media processing tool written in Rust. Designed to inter-operate with the [Assyst Discord bot](https://github.com/jacherr/assyst2).

## Building

### Prerequisites
- You must have [Rust](https://rust-lang.org) installed. The toolchain is selected using the `rust-toolchain` file when compiling and you do not need to install a specify one manually.
- You ideally should have Git installed.
- Building Flux is only tested on, and written for, Linux. It may or may not work on other platforms.
- You will need gcc to compile the native libvips objects.
- You must have [libvips](https://github.com/libvips/libvips) installed. Flux will not build without libvips.
---
- (Optional) You may have [gegl](https://www.gegl.org/) installed. Some operations require this.
- (Optional) You may have [ffmpeg](https://ffmpeg.org) installed. Video processing requires this.
- (Optional) You may have [Docker](https://www.docker.com/) installed. Makesweet operations require this.

---
### Build Steps
- Clone the repository: `git clone https://github.com/jacherr/flux --recurse-submodules`
- `cd` to the project: `cd flux`
- Build flux: `cargo build --release` (You may omit the `--release` flag to build with debug symbols)
- Once built, Flux can be installed in `/usr/local` by using the `./install.sh` script (must be superuser). To uninstall, use `./uninstall.sh` (also as superuser).

**NOTE**: If you get a linker error during compile, the most likely cause is that the libvips objects failed to compile. Make sure libvips is up to date. You can also try manually compiling these objects to see if this is the cause, e.g., ``gcc -fPIC -Wall -O2 -shared ./vips/flux_v_text.c -g -o ./build/libflux_v_text.so `pkg-config vips --cflags --libs` ``

Once built, you may need to add `./build` to your ldconfig or move the compiled shared objects to somewhere in `/usr` (such as `/usr/local/lib`) where they are searched for.
Alternatively, you can pass the env variable `LD_LIBRARY_PATH=./build` when running flux. Otherwise, you will get an error on execution saying that the shared objects cannot be found.

## Usage
Basic usage is `flux -i [input image file path] -o [operation name] [output image file path]`.

For example, `flux -i input.gif -o reverse reverse.gif`.

For operations that take parameters, such as `ghost` (with takes a `depth` parameter), you can provide these in the operation e.g.: `ghost[depth=10]`.

Operations can be chained. When an operation is complete, its output is pushed to the input queue. When an operation runs, it pops from the queue however many inputs it needs. When outputting the final file, the input queue must be empty or flux will exit with an error.

A full list of operations and other flags will come later, once Flux is more complete.