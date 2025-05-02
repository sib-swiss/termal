.PHONY: test clean install fmt manuscript

RUST_SOURCES = $(shell find src -name '*.rs')
LINUX_BINARY = ./target/release/termal
LINUX_STATIC_BINARY = target/x86_64-unknown-linux-musl/release/termal
WINDOWS_BINARY = ./target/x86_64-pc-windows-gnu/release/termal.exe
INSTALL_DIR = /usr/local/bin
MAN_DIR = /usr/share/man
MS_DIR = ./manuscript
BINARIES = $(LINUX_BINARY) $(LINUX_STATIC_BINARY) $(WINDOWS_BINARY) 

all: $(BINARIES) termal.1.gz

$(LINUX_BINARY): $(RUST_SOURCES)
	cargo build --release

$(LINUX_STATIC_BINARY): $(RUST_SOURCES)
	cargo build --release --target x86_64-unknown-linux-musl

$(WINDOWS_BINARY): $(RUST_SOURCES)
	cargo build --release --target x86_64-pc-windows-gnu

termal.1.gz: termal.1
	gzip -kf $<

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
	install -m 755 $(LINUX_BINARY) $(INSTALL_DIR)
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
	$(RM) $(BINARIES)
