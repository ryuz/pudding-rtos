#!/bin/bash -e

NAME=zynqmp_rpu_no_kernel

PROC=remoteproc${1:-0}
TARGET=${2:-debug}
OPTION=${2:+--}$2

cargo build ${OPTION}

if grep -q running /sys/class/remoteproc/${PROC}/state; then
    sudo sh -c "echo stop > /sys/class/remoteproc/${PROC}/state"
fi

sudo cp target/armv7r-none-eabi/${TARGET}/${NAME} /lib/firmware
sudo sh -c "echo ${NAME} > /sys/class/remoteproc/${PROC}/firmware"
sudo sh -c "echo start > /sys/class/remoteproc/${PROC}/state"
