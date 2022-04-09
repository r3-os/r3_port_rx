ENTRY(start);

/* R5F562N8 512KB ROM, 96KB RAM */
_device_ram_size = 96 * 1024;
_device_rom_size = 512 * 1024;

MEMORY
{
  /* Avoid placing data at PA 0 */
  OCRAM : ORIGIN = 0x00000000 + 1, LENGTH = _device_ram_size - 1,
  OCROM : ORIGIN = 0x100000000 - _device_rom_size, LENGTH = _device_rom_size
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

  /* Fixed vector table */
  .fixed_vector_table 0xffffff80 :
  {
    KEEP(*(.fixed_vector_table .fixed_vector_table.*));
  } > OCROM

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