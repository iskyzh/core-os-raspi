TARGET = aarch64-unknown-none-softfloat
OUTPUT = kernel.img
QEMU_BINARY = qemu-system-aarch64
QEMU_MACHINE_TYPE = raspi3
QEMU_MISC_ARGS = -serial stdio -display none
LINKER_FILE = src/link.ld
RUSTC_MISC_ARGS = -C target-cpu=cortex-a53
RUSTFLAGS = -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
TARGET_TYPE = release
CARGO_OUTPUT = target/$(TARGET)/$(TARGET_TYPE)/core-os-rust
OBJCOPY_CMD = cargo objcopy \
		-- \
		--strip-all \
		-O binary

all: $(OUTPUT)

$(CARGO_OUTPUT): FORCE
	RUSTFLAGS="$(RUSTFLAGS)" cargo xbuild --target $(TARGET) --$(TARGET_TYPE)

$(OUTPUT): $(CARGO_OUTPUT)
	$(OBJCOPY_CMD) $< ./$(OUTPUT)

qemu: all
	$(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE) -kernel kernel.img $(QEMU_MISC_ARGS)
FORCE:
