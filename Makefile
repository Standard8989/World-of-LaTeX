test: test.tex
	ptex2pdf -l -ot -kanji=utf8 -synctex=1 test.tex -output-directory output