index:
	cd py-fixed2float/examples && jupyter-nbconvert --to html notebook.ipynb
	ln -sf py-fixed2float/examples/notebook.html index.html
