.DEFAULT_GOAL := help

SRC_FILES := $(shell find src -type f -name "*.rs")
CARGO_TOML := Cargo.toml

DEBUG_TARGET := target/debug/quick_fox_status
RELEASE_TARGET := target/release/quick_fox_status
INSTALL_PATH := /usr/local/bin/quick_fox_status
DEBUG_TARGET2 := target/debug/quick_fox_status_run
RELEASE_TARGET2 := target/release/quick_fox_status_run
INSTALL_PATH2 := /usr/local/bin/quick_fox_status_run
SPOOL_DIR := /var/spool/quick_fox_status
ZABBIX_USER ?= zabbix

.PHONY: all debug release clean help install install_suid

all: debug

debug: $(DEBUG_TARGET) ${DEBUG_TARGET2}

release: $(RELEASE_TARGET) ${RELEASE_TARGET2}

$(DEBUG_TARGET): $(SRC_FILES) $(CARGO_TOML)
	cargo build

$(DEBUG_TARGET2): $(SRC_FILES) $(CARGO_TOML)
	cargo build

$(RELEASE_TARGET): $(SRC_FILES) $(CARGO_TOML)
	cargo build --release

$(RELEASE_TARGET2): $(SRC_FILES) $(CARGO_TOML)
	cargo build --release

install: $(RELEASE_TARGET) $(RELEASE_TARGET2)
	sudo install -m 755 $(RELEASE_TARGET) $(INSTALL_PATH)
	sudo install -m 755 $(RELEASE_TARGET2) $(INSTALL_PATH2)

install_suid: $(RELEASE_TARGET) $(RELEASE_TARGET2)
	sudo install -m 4755 -o $(ZABBIX_USER) $(RELEASE_TARGET) $(INSTALL_PATH)
	sudo install -m 755 $(RELEASE_TARGET2) $(INSTALL_PATH2)
	sudo mkdir -p $(SPOOL_DIR)
	sudo chown $(ZABBIX_USER):$(ZABBIX_USER) $(SPOOL_DIR)
	sudo chmod 700 $(SPOOL_DIR)

clean:
	cargo clean

bumppatch:
	bumpversion --allow-dirty --verbose patch

help:
	@echo "Usage: make [target]"
	@echo "Targets:"
	@echo "  all          - Build in debug mode (default)"
	@echo "  debug        - Build in debug mode"
	@echo "  release      - Build in release mode"
	@echo "  install      - Install the release binary to /usr/local/bin/ (requires sudo)"
	@echo "  install_suid - Install the release binary with SUID as $(ZABBIX_USER) and set up /var/spool/quick_fox_status (requires sudo)"
	@echo "  clean        - Remove build artifacts"
	@echo "  bumppatch    - Bump patch version"
	@echo "  help         - Show this help message"
	@echo "Variables:"
	@echo "  ZABBIX_USER  - User to own the installed binary and spool directory (default: zabbix)"

