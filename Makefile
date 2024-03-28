CROSS_COMPILE = riscv64-unknown-elf-

OC              := $(CROSS_COMPILE)objcopy
OD		:= $(CROSS_COMPILE)objdump

OUTDIR_RELEASE 	:= ./target/riscv64gc-unknown-none-elf/release/
OUTDIR_DEBUG 	:= ./target/riscv64gc-unknown-none-elf/debug/

OUTDIR 		:= ./output

NAME := milkv_duo_rust

.PHONY: release debug clean

release:
	cargo build --release
	$(OC) -O binary $(OUTDIR_RELEASE)$(NAME) $(NAME).bin
	$(OD) -s -d $(OUTDIR_RELEASE)$(NAME) > $(NAME).txt
	cp $(NAME).bin $(OUTDIR)
	mv $(NAME).bin ./tools/gen_fip/
	cd ./tools/gen_fip && ./run.sh cd -
	mv $(NAME).txt $(OUTDIR)
	cp ./tools/gen_fip/fip.bin $(OUTDIR)

debug:
	cargo build
	$(OC) -O binary $(OUTDIR_DEBUG)$(NAME) $(NAME).bin
	$(OD) -s -d $(OUTDIR_DEBUG)$(NAME) > $(NAME).txt
	cp $(NAME).bin $(OUTDIR)
	mv $(NAME).bin ./tools/gen_fip/
	cd ./tools/gen_fip && ./run.sh cd -
	mv $(NAME).txt $(OUTDIR)
	cp ./tools/gen_fip/fip.bin $(OUTDIR)

clean:
	cargo clean
	rm $(OUTDIR)/*
	rm ./tools/gen_fip/$(NAME).bin
	rm ./tools/gen_fip/fip.bin
