# Makefile for the gotp TOTP CLI.
# Run `make` or `make help` to see the available targets.

# Extra arguments passed through to `make run`, e.g.
#   make run ARGS="generate -a gmail"
ARGS ?=

.DEFAULT_GOAL := help

.PHONY: help build release run test fmt fmt-check lint check install uninstall clean ci

help: ## Show this help.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

build: ## Build the debug binary.
	cargo build

release: ## Build the optimized release binary (target/release/gotp).
	cargo build --release

run: ## Run the CLI; pass arguments via ARGS, e.g. make run ARGS="generate -a gmail".
	cargo run -- $(ARGS)

test: ## Run the test suite.
	cargo test

fmt: ## Format the source with rustfmt.
	cargo fmt

fmt-check: ## Check formatting without modifying files.
	cargo fmt --check

lint: ## Run clippy and fail on warnings.
	cargo clippy --all-targets -- -D warnings

check: ## Type-check without producing a binary.
	cargo check

install: ## Install the gotp binary into ~/.cargo/bin.
	cargo install --path .

uninstall: ## Remove the installed gotp binary.
	cargo uninstall gotp

clean: ## Remove build artifacts.
	cargo clean

ci: fmt-check lint test ## Run the full pre-commit gate (format check, lint, tests).
