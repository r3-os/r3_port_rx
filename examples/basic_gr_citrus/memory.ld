ENTRY(start);

/* RX631, 2MB ROM, 256KB RAM */
_device_ram_size = 256 * 1024;
_device_rom_size = 2 * 1024 * 1024;

MEMORY
{
  /* Avoid placing data at PA 0 */
  OCRAM : ORIGIN = 0x00000000 + 1, LENGTH = _device_ram_size - 1,
  /* The last 32KB is occupied by "Internal Flash ROM rewrite program via USB Mass Storage" */
  OCROM : ORIGIN = 0x100000000 - _device_rom_size, LENGTH = _device_rom_size - 0x8000
}

SECTIONS
{
  .text :
  {
    KEEP(*(.text.start));
    *(.text .text.*);
    __etext = .;
  } > OCROM

  .rodata :
  {
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > OCROM

  .data : ALIGN(4)
  {
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    __edata = .;
  } > OCRAM AT> OCROM

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  .bss : ALIGN(4)
  {
    __sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    __ebss = .;
  } > OCRAM

  /* Main stack */
  . += 4096;
  . = ALIGN(4);
  _stack_start = .;

  /* Place the heap right after `.uninit` */
  . = ALIGN(4);
  __sheap = .;

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}