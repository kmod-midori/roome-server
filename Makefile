.PHONY: build copy-tftp

all: build build-hook

build:
	cross build --target=armv7-unknown-linux-gnueabihf --release

build-hook: hook.c
	mkdir -p build
	arm-linux-gnueabihf-gcc -Wall -shared -fPIC -ldl hook.c -o build/libhook.so

copy-tftp:
	cp build/libhook.so /srv/tftp/
	cp target/armv7-unknown-linux-gnueabihf/release/roome-server /srv/tftp/
