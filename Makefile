setup_ubuntu: 
	sudo apt install -y libssl-dev pkg-config

setup_fedora:
	sudo dnf install -y openssl-devel pkg-config

setup_test:
	cargo install cargo-tarpaulin

test: setup_test
	cargo tarpaulin --all-features --workspace --timeout 120 --out Xml -- --test-threads=1
