default:
    just --list

format:
    rustfmt src/**/*.rs

tags:
    ctags -R --exclude='data/*' --exclude='target/*'

test:
	cargo test --color=always --no-fail-fast

# Make a PDF of the user manual

manual:
  just doc/


# Make a release for GitHub 
crate := "termal-msa"
bin := "termal"
version := `cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version'`
dist := "dist"
pkg := bin + "-" + version + "-" + target

clean:
    rm -rf {{dist}}

release-local:
    rm -rf {{dist}}/{{pkg}}
    cargo build --release --locked
    mkdir -p {{dist}}/{{pkg}}/data
    cp target/release/{{bin}} {{dist}}/{{pkg}}/
    cp data/example-1.msa {{dist}}/{{pkg}}/data/
    tar -C {{dist}} -czf {{dist}}/{{pkg}}.tar.gz {{pkg}}


# Detect target triple
target := `rustc -vV | sed -n 's/^host: //p'`

archive:
    cd {{dist}} && \
    for f in *; do \
        tar czf "$f".tar.gz "$f"; \
    done
