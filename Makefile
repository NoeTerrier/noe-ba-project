DISK_NAME = disk.img
DEBIAN_IMG = debian-11.6.0-amd64-netinst.iso

all: boot-dev-vm-nographic

disk-img:
	qemu-img create -f qcow2 ${DISK_NAME} 20G

install-debian:
	kvm -hda ${DISK_NAME} -cdrom ${DEBIAN_IMG} -m 4g -net nic -net user -soundhw all


boot-dev-vm-nographic:
	kvm \
	-soundhw all \
	-hda ${DISK_NAME} \
	-m 8G \
	-nographic \
	-nic user,model=virtio-net-pci,hostfwd=tcp::2222-:22

boot-dev-vm:
	kvm \
	-soundhw all \
	-hda ${DISK_NAME} \
	-m 8G \
	-cpu host

connect-vm:
	ssh -p 2222 user@localhost
