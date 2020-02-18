build: src/*
	cargo bootimage
run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-hello_os/debug/bootimage-hello_os.bin