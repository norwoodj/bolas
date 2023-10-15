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
	echo '{"build_timestamp": "$(shell date --utc --iso-8601=seconds)", "git_revision": "$(shell git rev-parse HEAD)", "version": "$(shell git describe)"}' | jq . > version.json

run: version.json
	cp version.json static/frontend-version.json
	cargo run -- \
		--static-file-path ./static \
		--tcp-addrs 0.0.0.0:23080 \
		--unix-addrs /tmp/bolas.sock \
		--management-tcp-addrs 0.0.0.0:23090
