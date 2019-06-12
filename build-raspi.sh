#/bin/bash

TARGET=arm-unknown-linux-gnueabihf

# Cutomize this to your environment
BASE_PATH=/home/$USER/Downloads

# customize this
export OPENSSL_LIB_DIR=$BASE_PATH/openssl-1.1.1b
export SQLITE_SRC_DIR=$BASE_PATH/sqlite-autoconf-3270200

# clone https://github.com/raspberrypi/tools
# (see README.armv6.md)
export ARMV6_LINKER="$BASE_PATH/raspberrypi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc"

# needed to link crosscompiled OpenSSL and Sqlite3
export OPENSSL_INCLUDE_DIR=$OPENSSL_LIB_DIR/include
export SQLITE3_LIB_DIR=$SQLITE_SRC_DIR/.libs

if [ "$1" = "full" ]; then
    echo "*** Compiling openssl for $TARGET ***"
    cd $OPENSSL_LIB_DIR
    export MACHINE=armv6 ARCH=arm CC=$ARMV6_LINKER
    make clean && ./config shared && make

    echo "*** Compiling sqlite for $TARGET ***"
    cd $SQLITE_SRC_DIR
    ./configure --enable-shared --host=arm-linux CC=$ARMV6_LINKER
    make -j8

    # unset this, it's deadly for the rest of the process
    unset CC
fi

echo "*** Cleaning up everything ***"
cargo clean

# echo "*** Compiling open-taffeta for $TARGET ***"
# PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target $TARGET

echo "*** Compiling and building .deb for $TARGET ***"
PKG_CONFIG_ALLOW_CROSS=1 cargo deb --target $TARGET --variant=armv6

echo
echo DONE
