# SPDX-License-Identifier: GPL-2.0

#KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) LLVM=1 V=1 -C $(KDIR) M=$$PWD

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD modules_install
