SECTIONS
{
	_start = .;
	. = 0;
	.head : {
		KEEP(*(.head))
 	}
	. = 0x1000;
	.text : { *(.text) }
	. = 0xf000;
	.data : { *(.data) }
	.bss : { *(.bss) }
}
