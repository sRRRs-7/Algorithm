bench:
	cargo bench > bench.json

release:
	cargo run --release

qr:
	./target/release/algo -o qr-code.png "https://doc.rust-lang.org/std/index.html"