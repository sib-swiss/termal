.PHONY: test

RUST_SOURCES = $(shell find src -name '*.rs')

tags: $(RUST_SOURCES)
	ctags -R --exclude='data/*' --exclude='target/*'

roadmap.pdf: roadmap.md meta.yaml
	pandoc --standalone --metadata-file meta.yaml --to=latex \
				--filter pandoc-crossref --citeproc --number-sections \
				--output $@ $<

release:
	cargo build --release

test:
	cargo test 2> /dev/null
	make -C app-tests/ test

