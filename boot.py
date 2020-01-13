#!/usr/bin/env python3
import sys
import struct
import os
import time

time.sleep(1)

FILE = "kernel8.img"
size = os.path.getsize(FILE)
with open(FILE, mode='rb') as file:
    sys.stdout.buffer.write(struct.pack("<L", size))
    sys.stdout.buffer.write(file.read())
