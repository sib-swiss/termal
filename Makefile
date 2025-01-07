.PHONY: test clean install

RUST_SOURCES = $(shell find src -name '*.rs')
TERMAL_BINARY = ./target/release/termal

all: $(TERMAL_BINARY) termal.1.gz

$(TERMAL_BINARY): $(RUST_SOURCES)
	cargo build --release

termal.1.gz: termal.1
	gzip $<

termal.1: termal.md
	pandoc --standalone --to=man $< > $@

tags: $(RUST_SOURCES)
	ctags -R --exclude='data/*' --exclude='target/*'

roadmap.pdf: roadmap.md meta.yaml
	pandoc --standalone --metadata-file meta.yaml --to=latex \
				--filter pandoc-crossref --citeproc --number-sections \
				--output $@ $<

install: $(TERMAL_BINARY)
	install -m 755 $(TERMAL_BINARY) /usr/local/bin
	install -m 644 termal.1.gz /usr/share/man/man1

test:
	cargo test 2> /dev/null
	make -C app-tests/ test

clean:
	$(RM) termal.1
