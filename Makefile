.PHONY: test

RUST_SOURCES = $(shell find src -name '*.rs')
TERMAL_BINARY = ./target/release/termal

$(TERMAL_BINARY):
	cargo build --release

tags: $(RUST_SOURCES)
	ctags -R --exclude='data/*' --exclude='target/*'

roadmap.pdf: roadmap.md meta.yaml
	pandoc --standalone --metadata-file meta.yaml --to=latex \
				--filter pandoc-crossref --citeproc --number-sections \
				--output $@ $<

install: $(TERMAL_BINARY)
	install -m 755 $(TERMAL_BINARY) /usr/local/bin

test:
	cargo test 2> /dev/null
	make -C app-tests/ test

