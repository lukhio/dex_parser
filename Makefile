all:
	cargo build
	./target/debug/dex_parser lumen.apk
