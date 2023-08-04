build:
	cargo build --release

release:
	./release.sh

deb: clean
	debuild

clean:
	cargo clean
	rm -rvf debian/bolas debian/bolas.postrm.debhelper debhelper-build-stamp

run:
	cargo run -- \
		--static-file-path ./static \
		--tcp-addrs 127.0.0.1:8080 \
		--unix-addrs /tmp/bolas.sock
