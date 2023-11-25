#!/bin/bash

MOUNT_DIR=./mnt
CURR_DIR=`pwd`
image=rootfs_riscv.ext4
busybox=/mnt/sdb/dev/busybox-1.36.1/build_riscv/_install

rm -rf $MOUNT_DIR
rm $image
dd if=/dev/zero of=./$image bs=1M count=32
mkfs.ext4 $image

mkdir -p $MOUNT_DIR
sudo mount -o loop $image $MOUNT_DIR

sudo cp -arf $busybox/* $MOUNT_DIR

cd $MOUNT_DIR
sudo mkdir -p etc dev mnt proc sys tmp mnt etc/init.d/ lib lib/module
echo "proc /proc proc defaults 0 0" > etc/fstab
echo "tmpfs /tmp tmpfs defaults 0 0" >> etc/fstab
echo "sysfs /sys sysfs defaults 0 0" >> etc/fstab

echo "#!/bin/sh" > etc/init.d/rcS
echo "mount -a" >> etc/init.d/rcS
echo "mount -o remount,rw /" >> etc/init.d/rcS
echo "echo -e \"Welcome to ARM64 Linux\"" >> etc/init.d/rcS
chmod 755 etc/init.d/rcS

echo "::sysinit:/etc/init.d/rcS" > etc/inittab
echo "::respawn:-/bin/sh" >> etc/inittab
echo "::askfirst:-/bin/sh" >> etc/inittab
chmod 755 etc/inittab

cd dev
mknod console c 5 1
mknod null c 1 3
mknod tty1 c 4 1


cd $CURR_DIR
sudo umount $MOUNT_DIR
echo "make initrd ok!"
chmod 777 $image