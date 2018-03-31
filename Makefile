# Examples (uncomment one)
#EXAMPLE      := blinky_delay
#EXAMPLE      := blinky_clint
#EXAMPLE      := blinky_pwm
#EXAMPLE      := blinky_plic
#EXAMPLE      := hello_world
#EXAMPLE      := panicking
#EXAMPLE      := pll
#EXAMPLE      := semihosting
#EXAMPLE      := stlog
EXAMPLE      := rtfm
#EXAMPLE      := static_mut

# Board crate (uncomment one)
BOARD        := hifive
#BOARD        := lofive

# OpenOCD configuration (uncomment one)
OPENOCD_CFG  := hifive-openocd.cfg
#OPENOCD_CFG  := lofive-openocd.cfg

TARGET       := riscv32ia-unknown-none
TARGET_DIR   := $(abspath ./target/$(TARGET)/debug)
EXAMPLE_DIR  := $(TARGET_DIR)/examples
EXAMPLE_BIN  := $(EXAMPLE_DIR)/$(EXAMPLE)

BAUD_RATE := 115200
TTY := /dev/ttyUSB2

build:
	xargo build --examples $(ARGS)

test:
	xargo test --all $(ARGS)

clean:
	xargo clean $(ARGS)

readelf:
	llvm-readelf -a -h -s -r -symbols $(EXAMPLE_BIN) $(ARGS)

size:
	llvm-size $(EXAMPLE_BIN) $(ARGS)

objdump:
	llvm-objdump -d -S $(EXAMPLE_BIN) $(ARGS)

dwarfdump:
	llvm-dwarfdump -verify $(EXAMPLE_BIN) $(ARGS) | grep error | wc -l

stcat:
	stty -F $(TTY) $(BAUD_RATE) sane -opost -brkint -icrnl -isig -icanon -iexten -echo
	cat $(TTY) | stcat -e $(EXAMPLE_BIN)

# .gdbinit adds a upload command to gdb
gdb:
	riscv32-unknown-elf-gdb $(EXAMPLE_BIN) $(ARGS)

openocd:
	openocd -f $(OPENOCD_CFG) $(ARGS)

upload:
	openocd -f $(OPENOCD_CFG) \
		-c "flash protect 0 64 last off; program ${EXAMPLE_BIN}; resume 0x20400000; exit"

.PHONY: build test clean readelf size objdump dwarfdump gdb openocd upload
