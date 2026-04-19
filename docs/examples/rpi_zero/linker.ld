/* Minimal linker script */
ENTRY(rust_main)
SECTIONS
{
  . = 0x8000;
  .text : { *(.text*) }
  .rodata : { *(.rodata*) }
  .bss : { *(.bss*) }
}
