#!/usr/bin/env python3
import sys
import struct
import os
import time

time.sleep(1)

FILE = "kernel8.img"
size = os.path.getsize(FILE)
with open(FILE, mode='rb') as file:
    sys.stderr.write(f"<-- sending {FILE} of size {size}...\n")
    sys.stdout.buffer.write(struct.pack("<L", size))
    sys.stderr.write(f"<-- sending binary...\n")
    sys.stdout.buffer.write(file.read())
    sys.stderr.write(f"<-- done.\n")