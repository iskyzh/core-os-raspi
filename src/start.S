.section ".text.boot"

.global _start

_start:
    mrs x1, mpidr_el1
    and x1, x1, #3
    cbz x1, 2f
1:  wfe                 // Core != 0
    b 1b
2:
    ldr x1, =_start     // Core 0
    mov sp, x1
    bl runtime_init
    b 1b
