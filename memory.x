/* Linker script for the EFR32MG22 */
/* documentation about the file format is at https://docs.rs/cortex-m-rt/latest/cortex_m_rt/ */

/* memory layout is documented at https://www.silabs.com/documents/public/data-sheets/efr32mg22e-datasheet.pdf page 15 */
/* RAM0_RAM -> RAM */
/* FLASH -> FLASH */

MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 32K
}

