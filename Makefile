.PHONY: format-lint-fix strict-lint


update-format-lint-fix:
	@cargo update
	@cargo fmt --all
	@cargo clippy --fix --all --allow-dirty


strict-lint:
	@cargo clippy --all-features -- -D warnings
