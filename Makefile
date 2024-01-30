build: version.json
	cp version.json static/frontend-version.json
	cargo build --release

release:
	./scripts/release.sh

deb: clean version.json
	cp version.json static/frontend-version.json
	debuild

clean:
	rm -rvf version.json static/frontend-version.json debian/bolas debian/bolas.postrm.debhelper debhelper-build-stamp bolas.yaml

version.json:
	echo '{"build_timestamp": "$(shell date --utc --iso-8601=seconds)", "git_revision": "$(shell git rev-parse HEAD)", "version": "$(shell git describe)"}' | jq . > version.json

bolas.yaml:
	cargo run -- -g bolas.yaml

run: bolas.yaml version.json
	cp version.json static/frontend-version.json
	cargo run -- -c bolas.yaml
