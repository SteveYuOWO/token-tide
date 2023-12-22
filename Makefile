BIN_DIR=~/bin
BIN_FILE = $(BIN_DIR)/tt

all: build

build:
	@echo "Bin file path: $(BIN_FILE)"
	@echo "Removing $(BIN_FILE)"
	@rm -f $(BIN_FILE)
	@echo "Removed $(BIN_FILE)"
	@cargo build
	@mv ./target/debug/token-tide $(BIN_FILE)
	@echo "Update bin file"