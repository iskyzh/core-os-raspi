#!/usr/bin/env python3
import sys
import struct
import os
import time
import serial

ser = serial.Serial('/dev/tty.usbserial-1440', 115200, timeout=0)

FILE = "kernel8.img"
size = os.path.getsize(FILE)
with open(FILE, mode='rb') as file:
    sys.stderr.write(f"<-- sending {FILE} of size {size}...\n")
    ser.write(struct.pack("<L", size))
    sys.stderr.write(f"<-- sending binary...\n")
    ser.write(file.read())
    sys.stderr.write(f"<-- done.\n")

ser.close()
