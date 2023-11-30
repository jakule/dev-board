all:
	# run npm install only if node_modules does not exist
	[ -d node_modules ] || npm install
	npm run build
	cargo build

.PHONY: test
test:
	CI=1 npm run test
	cargo test

.PHONY: clean
clean:
	rm -rf node_modules web/node_modules web/build
	cargo clean

.PHONY: lint
lint:
	npm run lint
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: start-postgres
start-postgres:
	docker-compose up -d db