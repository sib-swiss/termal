.PHONY: clean

termal.pdf: termal.tex termal.bib 
	xelatex $<
	bibtex termal
	xelatex $<
	xelatex $<

clean:
	$(RM) *.aux *.log *.bbl *.blg *.out *.toc *.lof *.lot 
