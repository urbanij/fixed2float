UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	EXT := dylib
endif
ifeq ($(UNAME_S),Linux)
	EXT := so
endif

all:
	cargo build
	cp $(CARGO_TARGET_DIR)/debug/libfixed2float.$(EXT) fixed2float.so


pre-commit:
	cp ../README.md .
	