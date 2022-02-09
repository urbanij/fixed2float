index:
	cd py-fixed2float/examples && jupyter-nbconvert --to html notebook.ipynb && mv notebook.html .notebook.html
	cd docs && ln -sf ../py-fixed2float/examples/.notebook.html index.html
