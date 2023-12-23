BIN_DIR=~/bin
BIN_FILE = $(BIN_DIR)/tt

all: fmt test build
fmt:
	@cargo fmt
	@echo "Formatted rs files"
test:
	@cargo test
build:
	@echo "Bin file path: $(BIN_FILE)"
	@rm -f $(BIN_FILE)
	@echo "Removed $(BIN_FILE)"
	@cargo build
	@mv ./target/debug/token-tide $(BIN_FILE)
	@echo "Updated bin file"
clean:
	@cargo clean
	@echo "Finished Clean"
.PHONY: all fmt test build clean