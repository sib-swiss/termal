.PHONY: test clean install fmt manuscript

RUST_SOURCES = $(shell find src -name '*.rs')
TERMAL_BINARY = ./target/release/termal
INSTALL_DIR = /usr/local/bin
MAN_DIR = /usr/share/man
MS_DIR = ./manuscript

all: $(TERMAL_BINARY) termal.1.gz manuscript

$(TERMAL_BINARY): $(RUST_SOURCES)
	cargo build --release

WINDOWS_BINARY = ./target/x86_64-pc-windows-gnu/release/termal.exe

termal.1.gz: termal.1
	gzip -f $<

termal.1: termal.md
	pandoc --standalone --to=man $< > $@

tags: $(RUST_SOURCES)
	ctags -R --exclude='data/*' --exclude='target/*'

fmt:
	rustfmt src/**/*.rs

roadmap.pdf: roadmap.md meta.yaml
	pandoc --standalone --metadata-file meta.yaml --to=latex \
				--filter pandoc-crossref --citeproc --number-sections \
				--output $@ $<

install: 
	install -m 755 $(TERMAL_BINARY) $(INSTALL_DIR)
	install -m 644 termal.1.gz $(MAN_DIR)/man1

manuscript:
	$(MAKE) -C $(MS_DIR)

test:
	cargo test 2> /dev/null
	make -C app-tests/ test

clean:
	$(RM) termal.1

mrproper: clean
	cargo clean
