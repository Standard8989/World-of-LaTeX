test.pdf: test.tex
	mkdir -p output && \
	ptex2pdf -l -ot -kanji=utf8 -synctex=1 test.tex -output-directory output

image: test.pdf
	cd output && \
	mkdir -p image && \
	cd image && \
	rm -f *.jpg && \
	pdftoppm -r 400 -jpeg ../test.pdf World-of-LaTeX

image.zip: image
	cd output && \
	rm -f image.zip && \
	zip -r image.zip image

clean: 
	rm -rf output
