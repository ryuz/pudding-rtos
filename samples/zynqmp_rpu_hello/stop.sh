#!/bin/bash

sudo sh -c "echo stop > /sys/class/remoteproc/remoteproc${1:-0}/state"
