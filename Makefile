TARGET = aarch64-unknown-none-softfloat
OUTPUT = kernel8.img
QEMU_BINARY = qemu-system-aarch64
QEMU_MACHINE_TYPE = raspi3
QEMU_MISC_ARGS = -serial mon:stdio -display none
QEMU_MISC_ARGS_BOOTLOADER = -serial stdio -display none
LINKER_FILE = src/bsp/rpi/link.ld
RUSTC_MISC_ARGS = -C target-cpu=cortex-a53
RUSTFLAGS = -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
TARGET_TYPE = release
CARGO_OUTPUT = target/$(TARGET)/$(TARGET_TYPE)/core-os-rust
OBJCOPY_CMD = cargo objcopy \
		-- \
		--strip-all \
		-O binary
DEMO_PAYLOAD = demo_payload_rpi3.img

all: $(OUTPUT)

$(CARGO_OUTPUT): FORCE
	RUSTFLAGS="$(RUSTFLAGS)" cargo xbuild --target $(TARGET) --bin core-os-rust --release

$(OUTPUT): $(CARGO_OUTPUT)
	$(OBJCOPY_CMD) $< ./$(OUTPUT)

qemu: all
	$(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE) -kernel $(OUTPUT) $(QEMU_MISC_ARGS)

qemudemo: all
	$(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE) -kernel $(DEMO_PAYLOAD) $(QEMU_MISC_ARGS)

qemuboot: all
	./boot.py | $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE) -kernel bootloader.alex_chi.img $(QEMU_MISC_ARGS_BOOTLOADER)

qemuasm: all
	$(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE) -kernel $(OUTPUT) $(QEMU_MISC_ARGS) -d in_asm

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -no-show-raw-insn -print-imm-hex $(CARGO_OUTPUT)

readelf:
	readelf -a $(CARGO_OUTPUT)

raspbootcom:
	cd vendor/raspbootcom/raspbootcom && make

raspi: $(OUTPUT)
	./serialboot.py
clean:
	rm -rf target
FORCE:
