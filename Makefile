.PHONY: test

roadmap.pdf: roadmap.md meta.yaml
	pandoc --standalone --metadata-file meta.yaml --to=latex \
				--filter pandoc-crossref --citeproc --number-sections \
				--output $@ $<

test:
	cargo test 2> /dev/null
	make -C app-tests/ test

