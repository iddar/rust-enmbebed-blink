GDB := arm-none-eabi-gdb
BMP_PORT ?= /dev/ttyBmpGdb
BASENAME = $(shell basename $(CURDIR))
FIRMWARE = target/thumbv7em-none-eabihf/release/$(BASENAME)

.PHONY: build
build:
	@echo "Compile bin"
	cargo build --release

.PHONY: flash 
flash: build
	@printf "  BMP $(BMP_PORT) $(FIRMWARE) (flash)\n"

	$(GDB) -nx --batch \
	           -ex 'target extended-remote $(BMP_PORT)' \
	           -x flash.scr \
	           $(FIRMWARE)

