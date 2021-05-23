all:
	cargo build
	RUST_BACKTRACE=1 ./target/debug/dex_parser out.dex
	# ./target/debug/dex_parser app-release.apk
