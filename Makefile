.PHONY: all build install uninstall

DATA_DIR := $(HOME)/.local/share/aeonium
BIN_INSTALL_DIR := $(HOME)/.local/bin

all: build install

build:
	cargo build --release

install:
	install -Dm755 ./target/release/aeonium-gui $(DATA_DIR)/aeonium-gui
	install -Dm755 ./target/release/aeonium-menu $(BIN_INSTALL_DIR)/aeonium-menu
	@echo "Installed aeonium-menu to $(BIN_INSTALL_DIR)"

