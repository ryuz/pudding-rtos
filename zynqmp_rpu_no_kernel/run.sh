#!/bin/bash -e

cargo build

TARGET=zynqmp_rpu_no_kernel
PROC=remoteproc0

if grep -q running /sys/class/remoteproc/$PROC/state; then
    sudo sh -c "echo stop > /sys/class/remoteproc/$PROC/state"
fi

sudo cp target/armv7r-none-eabi/debug/$TARGET /lib/firmware
#sudo cp target/armv7r-none-eabi/release/$TARGET /lib/firmware
sudo sh -c "echo $TARGET > /sys/class/remoteproc/$PROC/firmware"
sudo sh -c "echo start > /sys/class/remoteproc/$PROC/state"
