run:
	cargo run --package hawk-terminal
	
test:
	cargo test --package hawk-terminal

test-run: test run
