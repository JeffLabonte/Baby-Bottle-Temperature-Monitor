setup_ubuntu: 
	sudo apt install -y libssl-dev pkg-config

setup_fedora:
	sudo dnf install -y openssl-devel pkg-config

setup_test:
	cargo install cargo-tarpaulin

test: setup_test
	cargo tarpaulin --all-features --workspace --timeout 120 --out Xml -- --test-threads=1

install:
	sudo install -m 644 -D etc/baby_bottle/configs.conf /etc/baby_bottle/configs.conf
	cargo build --release
	sudo install -m 755 target/release/baby-bottle-temperature-monitor /usr/bin/baby-bottle-temperature-monitor
	sudo install -m 644 etc/systemd/system/baby-bottle.service /etc/systemd/system/baby-bottle.service
	sudo systemctl daemon-reload
	sudo systemctl enable --now baby-bottle.service
