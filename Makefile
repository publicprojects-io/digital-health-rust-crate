##
# help: Display this help message.
##
.PHONY: help
help:
        @awk '/^##/{a=1-a}a' $(MAKEFILE_LIST) | cut -c3-

# newline: This is a print helper when you want to embed a newline in a message.
define newline


endef

##
# make test: test everything
##
.PHONY: test
test:
	bin/test

##
# make ci-minimum-supported-rust-version: test with the minimum supported Rust version
##
.PHONY: ci-minimum-supported-rust-version
ci-minimum-supported-rust-version:
	bin/make-ci-minimum-supported-rust-version

##
# make test-benchmarks: run bin/test-benchmarks to generate code benchmarks with criterion.
##
.PHONY: test-benchmarks
test-benchmarks:
	bin/test-benchmarks

##
# make ci: run bin/ci to do continuous integration.
##
.PHONY: ci
ci:
	bin/ci

##
# make test-coverage: run bin/test-coverage to generate code coverage analysis with tarpaulin.
##
.PHONY: test-coverage
test-coverage:
	bin/test-coverage
