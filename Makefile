BIN_DIR = target/release
INSTALL_DIR = $(HOME)/.local/bin

TOOLS = lil_parser

build:
	cargo build --release

install: build
	mkdir -p $(INSTALL_DIR)
	for bin in $(TOOLS); do \
		cp $(BIN_DIR)/$$bin $(INSTALL_DIR)/$$bin; \
	done
