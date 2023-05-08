UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	EXT := dylib
endif
ifeq ($(UNAME_S),Linux)
	EXT := so
endif

all: build install demo
	# cargo build --release
	# cp $(CARGO_TARGET_DIR)/release/libfixed2float.$(EXT) fixed2float/fixed2float.so

build: pre-commit
	maturin build
	
install:
	pip install --force-reinstall $(CARGO_TARGET_DIR)/wheels/fixed2float-*.whl
	

pre-commit:
	cargo fmt
	cargo test

demo:
	@cd && python -c '''\
	import fixed2float; \
	print(f"fixed2float v{fixed2float.__version__}"); \
	print(f"53.123 in Fx<12,15> = {fixed2float.to_Fx(53.123, 12, 15, True)}")\
	'''

clean:
	rm -rf dist *.so __pycache__ .pytest_cache $(CARGO_TARGET_DIR)/wheels/fixed2float-*
	cd fixed2float && rm -rf *.so __pycache__
	cd examples && rm -rf .ipynb_checkpoints
