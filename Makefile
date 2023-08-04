build:
	cargo build

release:
	./release.sh

deb:
	debuild

run:
	cargo run -- \
		--static-file-path ./static \
		--tcp-addrs 127.0.0.1:8080 \
		--unix-addrs /tmp/bolas.sock
