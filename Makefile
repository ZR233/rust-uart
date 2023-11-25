# SPDX-License-Identifier: GPL-2.0

KDIR ?= /mnt/sdb/dev/linux-rust
ARCH ?= riscv
fs_name := busybox.ext4
BUSYBOX ?= /mnt/sdb/dev/busybox-1.36.1/build_riscv/_install

default:
	$(MAKE) ARCH=$(ARCH) -C $(KDIR) M=$$PWD LLVM=1

rust-analyzer: 
	$(MAKE) ARCH=$(ARCH) -C $(KDIR) M=$$PWD rust-analyzer

modules_install: default
	$(MAKE) ARCH=$(ARCH) -C $(KDIR) M=$$PWD LLVM=1 modules_install

rootfs: default
	rm -f $(fs_name)
	rm -rf mnt
	dd if=/dev/zero of=$(fs_name) bs=1M count=32	
	mkfs.ext4 $(fs_name)
	mkdir -p mnt
	sudo mount $(fs_name) mnt
	sudo cp -arf $(BUSYBOX)/* mnt
	sudo mkdir -p mnt/proc mnt/sys mnt/dev
	sudo mkdir -p mnt/etc
	sudo touch mnt/etc/fstab
	sudo mkdir -p mnt/etc/init.d
	sudo cp rcS mnt/etc/init.d 
	sudo chmod +x mnt/etc/init.d/rcS
	sudo cp rust_uart.ko mnt
	sudo umount mnt


run: rootfs
	qemu-system-riscv64 \
 		-m 2G \
		-machine 'virt' \
 		-kernel $(KDIR)/arch/$(ARCH)/boot/Image \
 		-display none \
    	-append "root=/dev/vda ro console=ttyS0" \
    	-drive file=$(fs_name),format=raw,id=hd0 \
    	-device virtio-blk-device,drive=hd0 \
		-serial stdio
