#!/bin/bash

TARGET=zynqmp_rpu_rust_2

sudo cp target/armv7r-none-eabi/debug/$TARGET /lib/firmware
#sudo cp target/armv7r-none-eabi/release/$TARGET /lib/firmware
sudo sh -c "echo $TARGET > /sys/class/remoteproc/remoteproc0/firmware"
sudo sh -c "echo start > /sys/class/remoteproc/remoteproc0/state"
