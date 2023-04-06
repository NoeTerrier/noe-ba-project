# noe-ba-project
Bachelor project 2023 at DCSL @ EPFL

# Linux drivers inside kernel 6.2.x

## Setup a dev VM

1.  install `qemu` and `qemu-system`
    
2.  download a debian iso (https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-11.6.0-amd64-netinst.iso) (update DEBIAN_IMG in the following Makefile to be the path of the iso)
    
3.  do commands:
    
    1.  `make disk-img` it will create a disk.img file of size 20Go
    2.  `make install-debian` and proceed to installation of debian (with ssh-server if you don't want to use the vm in graphical mode but via ssh connection)

## Launch (and connect) to dev VM

1.  launch vm:
    `make boot-dev-vm` or `make boot-dev-vm-nographical` for non graphical mode
    
2.  (in non graphical mode) ssh into vm:
    `make connect-vm`
    

## Setting up  the toolchain

Rust in kernel intro explains all the required components: https://www.kernel.org/doc/html/latest/rust/quick-start.html

### rustc[](https://www.kernel.org/doc/html/latest/rust/quick-start.html#rustc "Permalink to this heading")

> for the moment, the kernel depends on some unstable Rust features

### Rust standard library source

> The Rust standard library source is required because the build system will cross-compile `core` and `alloc`.

### libclang

Use of [LLVM](https://fr.wikipedia.org/wiki/LLVM) for understand C code in the kernel:

> `libclang` (part of LLVM) is used by `bindgen` to understand the C code in the kernel, which means LLVM needs to be installed; like when the kernel is compiled with `CC=clang` or `LLVM=1`.

### bindgen

> The bindings to the C side of the kernel are generated at build time using the `bindgen` tool.

### Building

> Building a kernel with a complete LLVM toolchain is the best supported setup at the moment.

## Inside the dev VM: build and install the kernel

1.  Install dependencies for compiling the kernel
    `apt install git build-essential fakeroot libncurses5-dev libssl-dev flex bison libelf-dev`
    
2.  Install Rust: https://www.rust-lang.org/tools/install and setup the tools chain as described in previous section
    
3.  Download the linux kernel: (works the [Rust-for-linux](https://github.com/Rust-for-Linux/linux) kernel too)
    `git clone --depth=1 --branch=v6.2 http://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git`
    
4.  Go inside the linux-6.2.2 folder:
    `cd linux`
    
5.  Check if rust is available in the config
    `make LLVM=1 rustavailable`
    
6.  Activate rust support in the kernel, the option will be visible if a compatible toolchain is available
    `make menuconfig`
    
    ```bash
    General setup --->
       L[*] Rust support
    ```
    
7.  Compile the kernel (~20min) it will produce a `linux-image-6.2.0_6.2.0-6_amd64.deb` image
    `make bindeb-pkg ARCH=x86_64 CC=clang`
    
8.  Install the kernel
    `sudo apt install ./linux-image-6.2.0_6.2.0-6_amd64.deb`
    
9.  verify the installation, it must match the image version:
    `uname -a`
    

## Write, build and insert a rust kernel out-of-tree

Sample of rust drivers can be found in the folder [rust-drivers](https://github.com/NoeTerrier/noe-ba-project/tree/main/rust-drivers)

Build the module with command:

`make KDIR=<PATH_TO_LINUX> CC=clang`

The following commands are useful to manage modules:

```bash
sudo insmod hello_world.ko    	# insert the module in the kernel 
sudo rmmod hello_world        	# delete module
sudo modinfo hello_world.ko   	# print info about a module file
sudo lsmod			# list all installed modules
dmesg  				# print the kernel logs
```

# IOCLT in a Rust driver

Kernel documentation reference: https://www.kernel.org/doc/html/latest/driver-api/ioctl.html

IOCTL can't be implemented now in the official linux kernel version. But we can use the version of the [Rust-for-Linux](https://github.com/Rust-for-Linux/linux) project in order to use a more rust-friendly version of the kernel.
