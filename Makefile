.PHONY: rust-analyzer
rust-analyzer:
	bazel run @rules_rust//tools/rust_analyzer:gen_rust_project

.PHONY: repin
repin:
	CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index

.PHONY: migrate-diff
migrate-diff:
	atlas --config file://src/piston/server/atlas.hcl --env development \
		migrate diff
