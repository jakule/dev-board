all:
	# run npm install only if node_modules does not exist
	[ -d node_modules ] || npm install
	npm run build
	cargo build

test:
	CI=1 npm run test
	cargo test

clean:
	rm -rf node_modules web/node_modules web/build
	cargo clean

fmt:
	cargo fmt

lint:
	npm run lint
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features

start-postgres:
	docker-compose up -d db

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

migrate-create:
	sqlx migrate add $(name)

sqlx-prepare:
	cargo sqlx prepare
