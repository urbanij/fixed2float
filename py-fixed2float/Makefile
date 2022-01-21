UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	EXTENSION := dylib
endif
ifeq ($(UNAME_S),Linux)
	EXTENSION := so
endif

all:
	cargo build
	cp $(CARGO_TARGET_DIR)/debug/libfixed2float.$(EXT) fixed2float.so
