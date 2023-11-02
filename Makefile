PHONY := generate-% build

SHELL := /bin/bash

CARGO_ARGUMENTS := --release --all-features

ifneq ($(CARGO_TARGET),)
CARGO_ARGUMENTS += --target $(CARGO_TARGET)
endif

generate-env-from-env-sample:
	@${SHELL} -c ". .make-helpers; generate_env_from_env_sample .devcontainer"

build:
	cargo build $(CARGO_ARGUMENTS)

test:
	cargo test $(CARGO_ARGUMENTS)

.PHONY: $(PHONY)
