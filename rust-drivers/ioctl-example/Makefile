# SPDX-License-Identifier: GPL-2.0

KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD CC=clang

clean:
	@rm -f *.mod *.c *.ko *.o *.order *.symvers *.d .*.cmd app/ioctl_app
