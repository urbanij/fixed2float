all: fmt test py

py:
	cd py-fixed2float && make

fmt:
	cargo fmt

test:
	cargo test

index:
	cd py-fixed2float/examples && jupyter-nbconvert --to html notebook.ipynb && mv notebook.html .notebook.html
	cd docs && ln -sf ../py-fixed2float/examples/.notebook.html index.html

clean:
	cd py-fixed2float && make clean
	rm -rf examples/.ipynb_checkpoints