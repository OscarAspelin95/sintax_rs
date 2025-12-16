.PHONY: format-lint-fix strict-lint


format-lint-fix:
	@cargo fmt --all
	@cargo clippy --fix --all --allow-dirty


strict-lint:
	@cargo clippy --all-features -- -D warnings
