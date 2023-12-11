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

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	npm run lint
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features

.PHONY: start-postgres
start-postgres:
	docker-compose up -d db


.PHONY: migrate-up
migrate-up:
	sqlx migrate run

.PHONY: migrate-down
migrate-down:
	sqlx migrate revert

.PHONY: migrate-create
migrate-create:
	sqlx migrate add $(name)

.PHONY: sqlx-prepare
sqlx-prepare:
	cargo sqlx prepare
