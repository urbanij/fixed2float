UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	EXT := dylib
endif
ifeq ($(UNAME_S),Linux)
	EXT := so
endif

all:
	cargo build --release
	cp $(CARGO_TARGET_DIR)/release/libfixed2float.$(EXT) fixed2float/fixed2float.so


# pre-commit:
	

clean:
	rm -rf dist *.so __pycache__ .pytest_cache
	cd fixed2float && rm -rf *.so __pycache__
