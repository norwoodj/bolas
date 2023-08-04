build:
	cargo build

release:
	./release.sh

deb:
	debuild

run:
	cargo run -- --unix-addrs /tmp/bolas.sock --tcp-addrs 127.0.0.1:8080
