PHONY := generate-% build

SHELL := /bin/bash

generate-env-from-env-sample:
	@${SHELL} -c ". .make-helpers; generate_env_from_env_sample .devcontainer"

build:
	cargo build --release

.PHONY: $(PHONY)
