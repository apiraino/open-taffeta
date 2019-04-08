#!/bin/bash

### Install Raspi1 compile toolchain and Rust target

$ sudo apt install gcc-arm-linux-gnueabihf
```
$ rustup target add arm-unknown-linux-gnueabihf
$ git clone --depth=1 https://github.com/raspberrypi/tools raspberrypi-tools
$ echo "[target.arm-unknown-linux-gnueabihf]" >> ~/.cargo/config
$ echo "linker = \"$(pwd)/raspberrypi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc\"" >> ~/.cargo/config
```
(source: https://stackoverflow.com/a/48087233)

From now on use this linker to xcompile everything!
```
export ARMV6_LINKER=/path/to/raspberrypi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc
```

## Crosscompiling for RaspberryPI (armv6)

### Compile OpenSSL for armv6
```
$ wget https://www.openssl.org/source/openssl-1.1.1b.tar.gz
$ tar xvzf openssl-1.1.1b.tar.gz && cd openssl-1.1.1b
$ export MACHINE=armv6 ARCH=arm CC=$ARMV6_LINKER
$ make clean && ./config shared && make
```

### Compile libsqlite3 for armv6
```
$ wget https://www.sqlite.org/2019/sqlite-autoconf-3270200.tar.gz
$ tar xvzf sqlite-autoconf-3270200.tar.gz && cd sqlite-autoconf-3270200
$ ./configure --enable-shared --host=arm-linux CC=$ARMV6_LINKER
$ make -j8
```

### Compile OpenTaffeta for armv6
```
PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target $TARGET
```

### Make .deb package
```
export OPENSSL_LIB_DIR=/path/to/compile/openssl-1.1.1b
export OPENSSL_INCLUDE_DIR=/path/to/compiled/openssl-1.1.1b/include
export TARGET="arm-unknown-linux-gnueabihf"
PKG_CONFIG_ALLOW_CROSS=1 cargo deb --target=$TARGET --variant=armv6
```

### (Optional) deploy on the Raspi with SSL certs
```
# src: https://github.com/FiloSottile/mkcert
# aaac.bbbb.cccc.ddd = (W)LAN ip for the device
# ./mkcert-v1.2.0-linux-amd64 localhost 127.0.0.1 aaa.bbb.ccc.ddd
# curl https://ip.device:8080/
```

## Issues! Issues everywhere!
```
$ file target/arm-unknown-linux-gnueabi/release/open_taffeta_bin
target/arm-unknown-linux-gnueabi/release/open_taffeta_bin: ELF 32-bit LSB shared object, ARM, EABI5 version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux.so.3, for GNU/Linux 3.2.0, BuildID[sha1]=3f649be3c815967715225c9aa34193c1adbcb42b, with debug_info, not stripped
```

pi@raspberrypi:~ $ sudo dpkg -i open-taffeta-armv6_0.2.0_armel.deb
dpkg: error processing archive open-taffeta-armv6_0.2.0_armel.deb (--install):
package architecture (armel) does not match system (armhf)
Errors were encountered while processing:
open-taffeta-armv6_0.2.0_armel.deb

Recompile for `arm-unknown-linux-gnueabihf`

/home/user/open-taffeta/target/arm-unknown-linux-gnueabihf/release/deps/open_taffeta_bin-0649e7dd72beaf7e.open_taffeta_bin.8268kb7c-cgu.0.rcgu.o: error adding symbols: file in wrong format

#### Error
```
$ ./open_taffeta_bin
./open_taffeta_bin: error while loading shared libraries: libssl.so.1.0.0: cannot open shared object file: No such file or directory
```

Solution:

Compile against openssl-1.1.1b

#### Error

Cannot link openssl libs with target `arm-unknown-linux-gnueabihf` with using the Raspi toolchain:
```
$ cat .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "/path/to/raspberrypi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc"

export OPENSSL_LIB_DIR=/path/to/compile/openssl-1.1.1b
export OPENSSL_INCLUDE_DIR=/path/to/compiled/openssl-1.1.1b/include
$ export TARGET="arm-unknown-linux-gnueabihf"
$ PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target=$TARGET
...
= note:  /path/to/compile/openssl-1.1.1b/libcrypto.so: undefined reference to `clock_gettime@GLIBC_2.17'
          /path/to/compile/openssl-1.1.1b/libcrypto.so: undefined reference to `getauxval@GLIBC_2.16'
/path/to/openssl-1.1.1b//libcrypto.so: undefined reference to `secure_getenv@GLIBC_2.17'
          collect2: error: ld returned 1 exit status
```

Solution:

Ensure openssl is compiled with the same gcc version as the Rust project, f.e.:
```
$ export MACHINE=armv6 ARCH=arm CC=$ARMV6_LINKER
$ make clean && ./config shared && make
```

#### Error

Cannot link libsqlite3

```
= note: /home/user/open-taffeta/target/arm-unknown-linux-gnueabihf/release/deps/liblibsqlite3_sys-01652433cf394b13.rlib(sqlite3.o):(.data.rel.aSyscall+0x58): undefined reference to `fcntl64'
          collect2: error: ld returned 1 exit status
```

Solution:

Again: compile libsqlite3 and link with the same toolchain!

### Other links

- Run ARM binaries on x86_64 host
https://ownyourbits.com/2018/06/13/transparently-running-binaries-from-any-architecture-in-linux-with-qemu-and-binfmt_misc/

#### Error

```
$ PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target $TARGET
error: failed to run custom build command for `ring v0.13.5`
process didn't exit successfully: `/home/user/Projects/open-taffeta/target/release/build/ring-e093b000b38364ec/build-script-build` (exit code: 101)
...
arm-linux-gnueabihf-gcc: error: unrecognized command line option ‘-m64’
thread 'main' panicked at 'execution failed', /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/ring-0.13.5/build.rs:645:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

Solution:

Buddy, you mixed up the env variables. Ensure that `CC` remains untouched while compiling with `rustc`.
