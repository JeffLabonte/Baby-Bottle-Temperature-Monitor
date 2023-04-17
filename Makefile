setup_ubuntu: 
	sudo apt install -y libssl-dev pkg-config

setup-test:
	cargo install cargo-tarpaulin

test:
	cargo tarpaulin --all-features --workspace --timeout 120 --out Xml
