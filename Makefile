.PHONY: all build install gui menu

DATA_DIR := $(HOME)/.local/share/aeonium
BIN_INSTALL_DIR := $(HOME)/.local/bin

all: help

build: gui menu

install: build
	install -Dm755 ./target/release/aeonium-gui $(DATA_DIR)/aeonium-gui
	install -Dm755 ./target/release/aeonium-menu $(BIN_INSTALL_DIR)/aeonium-menu
	@echo "Installed aeonium-menu to $(BIN_INSTALL_DIR)"

gui:
	cargo build --release --bin aeonium-menu

menu:
	cargo build --release --bin aeonium-gui

clean:
	cargo clean

help:
	@echo  'Aeonium build targets:'
	@echo  '  all              - Build and install (default)'
	@echo  '  build            - Build both binaries'
	@echo  '  install          - Install binaries to user directories'
	@echo  '  clean            - Remove build artifacts (cargo clean)'
	@echo  ''
	@echo  'Individual components:'
	@echo  '  gui              - Build aeonium-gui'
	@echo  '  menu             - Build aeonium-menu'
	@echo  ''
	@echo  'Install destinations:'
	@echo  "  DATA_DIR         = $(DATA_DIR)"
	@echo  "  BIN_INSTALL_DIR  = $(BIN_INSTALL_DIR)"
	@echo  ''
	@echo  'Usage examples:'
	@echo  '  make              - Same as "make help" (This message)'
	@echo  '  make gui          - Only build the GUI'
	@echo  '  make install      - Install built binaries'
