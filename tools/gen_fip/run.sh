./fiptool.py -v genfip \
    'fip.bin' \
    --MONITOR_RUNADDR="0x0000000080000000" \
    --CHIP_CONF='chip_conf.bin' \
    --NOR_INFO='FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF' \
    --NAND_INFO='00000000'\
    --BL2='bl2.bin' \
    --DDR_PARAM='ddr_param.bin' \
    --MONITOR='fw_dynamic.bin' \
    --LOADER_2ND='bl33.bin' \
    --compress='lzma'
