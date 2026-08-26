/* Placeholder linker script for a generic Cortex-M4F chip (e.g. STM32F411).
 * Adjust ORIGIN/LENGTH to match your actual chip's datasheet before flashing. */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
