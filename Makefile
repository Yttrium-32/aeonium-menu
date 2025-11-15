.PHONY: all build install gui menu

DATA_DIR := $(HOME)/.local/share/aeonium
BIN_INSTALL_DIR := $(HOME)/.local/bin

all: build install

build: gui menu

install:
	install -Dm755 ./target/release/aeonium-gui $(DATA_DIR)/aeonium-gui
	install -Dm755 ./target/release/aeonium-menu $(BIN_INSTALL_DIR)/aeonium-menu
	@echo "Installed aeonium-menu to $(BIN_INSTALL_DIR)"

gui:
	cargo build --release -v --bin aeonium-menu

menu:
	cargo build --release -v --bin aeonium-gui

clean:
	cargo clean

