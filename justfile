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
release_version := "v" + version
dist := "dist"
pkg_root := bin + "-" + release_version
pkg := pkg_root + "-" + target

clean:
    rm -rf {{dist}}

release-local:
    rm -rf {{dist}}/{{pkg_root}}
    rm -f {{dist}}/{{pkg}}.tar.gz
    cargo build --release --locked
    mkdir -p {{dist}}/{{pkg_root}}/data/colormaps
    cp README.md LICENSE {{dist}}/{{pkg_root}}/
    cp CHANGELOG.md {{dist}}/{{pkg_root}}/ 2>/dev/null || true
    cp doc/manual.md {{dist}}/{{pkg_root}}/termal-manual.md 2>/dev/null || true
    cp doc/manual.pdf {{dist}}/{{pkg_root}}/termal-manual.pdf 2>/dev/null || true
    cp target/release/{{bin}} {{dist}}/{{pkg_root}}/
    cp -rL data/example-1.msa data/PF00244.26.sto data/OX_OFA-pg2.msa data/OX_OFA-pg2.order {{dist}}/{{pkg_root}}/data/ 2>/dev/null || true
    cp -rL data/colormaps/gecos_default.json data/colormaps/high-contrast.json data/colormaps/no-green.json {{dist}}/{{pkg_root}}/data/colormaps/ 2>/dev/null || true
    tar -C {{dist}} -czf {{dist}}/{{pkg}}.tar.gz {{pkg_root}}


# Detect target triple
target := `rustc -vV | sed -n 's/^host: //p'`

archive:
    cd {{dist}} && \
    for f in *; do \
        tar czf "$f".tar.gz "$f"; \
    done
