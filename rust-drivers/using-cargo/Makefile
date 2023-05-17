# SPDX-License-Identifier: GPL-2.0


CLANG ?= clang
ifeq ($(origin CC),default)
CC := ${CLANG}
endif

# make KDIR=/home/noe/linux
KDIR ?= /lib/modules/`uname -r`/build
ARCH ?= x86_64
CROSS_COMPILE ?= x86_64-unknown-linux-gnu

build-ko:
	cd ext_crate && cargo build -v
	cp ext_crate/target/x86_64-unknown-linux-gnu/debug/deps/ext_crate-*.o ext_crate.o_shipped
	$(MAKE) -C $(KDIR) M=$$PWD ARCH=$(ARCH) CROSS_COMPILE=$(CROSS_COMPILE)

clean:
	@rm -rf \
		.*.cmd *.o *_shipped *.mod *.mod.c *.ko Module.symvers modules.order ext_crate/target
