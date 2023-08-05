build: version.json
	cp version.json static/frontend-version.json
	cargo build --release

release:
	./scripts/release.sh

deb: clean version.json
	cp version.json static/frontend-version.json
	debuild

clean:
	cargo clean
	rm -rvf version.json static/frontend-version.json debian/bolas debian/bolas.postrm.debhelper debhelper-build-stamp

version.json:
	echo '{"build-timestamp": "$(shell date --utc --iso-8601=seconds)", "revision": "$(shell git rev-parse HEAD)", "version": "$(shell git tag -l | tail -n 1)"}' | jq . > version.json

run:
	cargo run -- \
		--static-file-path ./static \
		--tcp-addrs 127.0.0.1:23080 \
		--unix-addrs /tmp/bolas.sock
