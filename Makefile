PACKAGE = adequate

# vet
vet\:check: # vet code syntax [synonym: check]
	@cargo check --all --verbose
.PHONY: vet\:check

check: vet\:check
.PHONY: check

vet\:format: # vet format without changes [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: vet\:format

format: vet\:format
.PHONY: format

fmt: vet\:format
.PHONY: fmt

vet\:lint: # vet coding style using clippy [synonym: lint]
	@cargo clippy --all-targets
.PHONY: vet\:lint

lint: vet\:lint
.PHONY: lint

vet\:all: vet\:check vet\:format vet\:lint # Check code using all vet targets
.PHONY: vet\:all

vet: vet\:check # Alias for vet:check
.PHONY: vet

# test
test\:doc: # Run only doc tests
	@cargo test --doc
.PHONY: test\:doc

test\:lib: # Run unit tests
	@cargo test --tests
.PHONY: test\:lib

test: test\:lib # Alias for test:lib
.PHONY: test

test\:all: test\:doc test\:lib # Run all tests
.PHONY: test\:all

# coverage
coverage\:lib: # Generate a coverage report of tests for library [synonym: cov:lib]
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/lib"; \
	cargo test --lib --no-run --target-dir=$${target_dir}; \
	result=($${target_dir}/index.js*); \
	if [ -f $${result}[0] ]; then \
		rm "$${target_dir}/index.js*"; \
	fi; \
	file=($$target_dir/debug/deps/$(PACKAGE)-*); \
	kcov --verify --include-path=$$dir/src $$target_dir $${file[0]}; \
	grep 'index.html' $$target_dir/index.js* | \
		grep --only-matching --extended-regexp \
		'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'
.PHONY: coverage\:lib

cov\:lib: coverage\:lib
.PHONY: cov\:lib

coverage: coverage\:lib # Alias for coverage:lib [synonym: cov]
.PHONY: coverage

cov: coverage
.PHONY: cov

# documentation
document: # Generate documentation files [synonym: doc]
	cargo rustdoc --package $(PACKAGE)
.PHONY: document

doc: document
.PHONY: doc
# }}}

# build
build\:debug: # Build in debug mode
	cargo build
.PHONY: build\:debug

build: build\:debug # Alias for build:debug
.PHONY: build

build\:release: # Create release build
	cargo build --release
.PHONY: build\:release

# utility
clean: # Clean up
	@cargo clean
.PHONY: clean

# NOTE:
# This depends on environment variables from .env.ci, and requires
# the gitlab-runner command.
runner-%: # Run a CI job on local (on Docker)
	@set -uo pipefail; \
	job=$(subst runner-,,$@); \
	opt=""; \
	while read line; do \
		opt+=" --env $$(echo $$line | sed -E 's/^export //')"; \
	done < .env.ci; \
	gitlab-runner exec docker \
		--executor docker \
		--cache-dir /cache \
		--docker-volumes $$(pwd)/.cache/gitlab-runner:/cache \
		--docker-volumes /var/run/docker.sock:/var/run/docker.sock \
		$${opt} $${job}
.PHONY: runner

package: # Create package
	@cargo package
.PHONY: package

publish: # Publish package
	@cargo publish
.PHONY: publish

help: # Display this message
	@grep --extended-regexp '^[0-9a-z\:\\\%]+: ' $(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = test
default: test
