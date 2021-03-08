serve:
	cd expand-variables && cargo build --release
	mdbook serve

build:
	cd expand-variables && cargo build --release
	mdbook build