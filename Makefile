MAKEFLAGS += -rR
.SUFFIXES:

KERNEL_ARCH ?= x86_64
ISO_NAME := $(KERNEL_ARCH)-hynix.iso

.PHONY: all
all: run-legacy

.PHONY: kernel
kernel:
	$(MAKE) -C kernel

limine/limine:
	rm -rf limine 
	git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1
	$(MAKE) -C limine

$(ISO_NAME): limine/limine kernel
	mkdir -p iso_root/{boot/limine,EFI/BOOT}
	cp -v kernel/kernel iso_root/boot/
	cp -v limine.conf iso_root/boot/limine/

	cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(ISO_NAME)
	./limine/limine bios-install $(ISO_NAME)
	rm -rf iso_root

.PHONY: build-iso
build-iso: $(ISO_NAME)

.PHONY: run-legacy
run-legacy: build-iso
	qemu-system-$(KERNEL_ARCH) \
		-M q35 \
		-cdrom $(ISO_NAME) \
		-boot d \
		-serial mon:stdio \
		-m 2G

.PHONY: clean
clean: 
	rm -rf iso_root
	rm -rf limine 
	rm $(ISO_NAME)
	$(MAKE) -C kernel clean
