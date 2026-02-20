.PHONY: all fmt fix lint test check build-native docs update clean ci strict-lint

# -- dev
all: update fmt fix lint test build-native

# --
fmt:
	@cargo fmt --all

# --
fix:
	@cargo fix --allow-dirty
	@cargo clippy --fix --all --allow-dirty

# --
lint:
	@cargo clippy --all-features

# --
strict-lint:
	@cargo clippy --all-features -- -D warnings

# --
test:
	@cargo test

# --
check:
	@cargo check --all-features


# --
build-native:
	RUSTFLAGS="-C target-cpu=native" cargo build --release

# --
docs:
	@cargo doc -p sintax_rs --open

# --
update:
	@cargo update

# --
clean:
	@cargo clean
