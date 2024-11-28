set dotenv-load := false

CR_YEAR := "2021"
CR_NAME := "Mohamed Seleem"
CR_REPO := "https://github.com/mselee/tuid"
CR_PROJ := "tuid"

@maturin *args:
	cd tuid-py && poetry run maturin {{args}}

@cargo *args:
	cargo {{args}}

@init:
	pipx install git+https://github.com/mselee/licenseheaders.git
	rustup update
	rustup component add clippy
	rustup component add rustfmt
	rustup component add miri
	cargo install cargo-audit cargo-criterion committed cargo-release git-cliff cargo-release
	lefthook install

@audit:
	cargo audit

@bench *args:
	cargo criterion -p tuid-bench {{args}}

@clean:
	cargo clean

@lint *args:
	cargo clippy {{args}}
	cargo fmt --check {{args}}

@fmt *args:
	cargo fmt {{args}}

@fix *args:
	cargo clippy --fix {{args}}

@test *args='--no-default-features --features fastrand':
	cargo test -p tuid {{args}}

@copyright *args:
	just _do_copyright tuid {{args}}
	just _do_copyright tuid-bench {{args}}

@commit-msg msg:
	committed --config .committed.toml --no-merge-commit --commit-file {{msg}}

@commits:
	committed --config .committed.toml --no-merge-commit master..HEAD

@changelog  *args:
	git cliff -c .cliff.toml --strip footer --unreleased {{args}} > ./CHANGELOG.md

@miri: clean
	MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test -p tuid --no-default-features --features fastrand --target mips64-unknown-linux-gnuabi64

@_do_copyright dir *args:
	licenseheaders -x '.*/*' '*.md' -d "{{dir}}" -t .copyright.tmpl -o "{{CR_NAME}}" -y "{{CR_YEAR}}" -n "{{CR_PROJ}}" -u "{{CR_REPO}}" {{args}}
