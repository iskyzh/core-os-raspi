TARGET = aarch64-unknown-none-softfloat
OUTPUT = kernel.img
QEMU_BINARY = qemu-system-aarch64
QEMU_MACHINE_TYPE = raspi3
QEMU_MISC_ARGS = -d in_asm
LINKER_FILE = src/link.ld
RUSTC_MISC_ARGS = -C target-cpu=cortex-a53
RUSTFLAGS = -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
CARGO_OUTPUT = target/$(TARGET)/release/core-os-rust
OBJCOPY_CMD = cargo objcopy \
		-- \
		--strip-all \
		-O binary

all: $(OUTPUT)

$(CARGO_OUTPUT): FORCE
	RUSTFLAGS="$(RUSTFLAGS)" cargo xbuild --target $(TARGET) --release

$(OUTPUT): $(CARGO_OUTPUT)
	$(OBJCOPY_CMD) $< ./$(OUTPUT)

qemu: all
	qemu-system-aarch64 -M raspi3 -kernel kernel.img -d in_asm
FORCE:
