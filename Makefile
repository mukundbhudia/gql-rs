CARGO_EXTRA_ARGS	?=

ifdef CI
CARGO_EXTRA_ARGS 	+= --release
endif

.PHONY: default
default:
	test

.PHONY: test
test:
	@echo "This will reset your DB."
	@read -p "Continue?" -n 1 -r; \
	if [[ $$REPLY =~ ^[Yy] ]]; \
	then \
		$(MAKE) test-force; \
	fi

.PHONY: test-force
test-force: export RUST_BACKTRACE=1
test-force: db-restart
	@(cargo test --all-features $(CARGO_EXTRA_ARGS));

.PHONY: fmt
fmt:
	@(cargo fmt)

.PHONY: validate
validate:
	@(cargo fmt -- --check)

.PHONY: compile
compile:
	@(cargo build $(CARGO_EXTRA_ARGS))

.PHONY: check
check:
	@(cargo check $(CARGO_EXTRA_ARGS))

.PHONY: schema
schema:
	cargo run --bin dump_schema schema/schema.graphql

.PHONY: db-restart
db-restart: db-stop db-start

.PHONY: db-start
db-start:
	@(docker-compose up --detach)

.PHONY: db-stop
db-stop:
	@(docker-compose down)

.PHONY: run
run: export RUST_LOG=debug
run: export RUST_BACKTRACE=1
run:
	@(cargo run --bin promo-api)

.PHONY: lint
lint:
	@(cargo clippy --all-targets --all-features -- -D warnings)
