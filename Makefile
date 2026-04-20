RUST_DIR=tools/lil_parser
BIN_NAME=lil_parser
INSTALL_DIR=~/.local/bin

DOOM_DIR=~/.config/doom/lisp
EL_FILE=emacs/doomed_ida.el

build:
	cd $(RUST_DIR) && cargo build --release

install-bin: build
	mkdir -p $(INSTALL_DIR)
	cp $(RUST_DIR)/target/release/$(BIN_NAME) $(INSTALL_DIR)/$(BIN_NAME)

install-el:
	mkdir -p $(DOOM_DIR)
	cp $(EL_FILE) $(DOOM_DIR)/doomed_ida.el

install: install-bin install-el
