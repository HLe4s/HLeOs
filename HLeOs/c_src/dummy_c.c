typedef unsigned short uint16_t;

void dummy_c(void)
{
	int a= 1, b = 0, c = a/b;
	uint16_t * vgr = 0xb8000;
	asm volatile ("sti"); // to enable interrupt.

	vgr[0] = 0x4048;
	vgr[1] = 0x4049;
	vgr[2] = 0x402C;
	vgr[3] = 0x4049;
	vgr[4] = 0x4027;
	vgr[5] = 0x404D;
	vgr[6] = 0x4020;
	vgr[7] = 0x4044;
	vgr[8] = 0x4055;
	vgr[9] = 0x404D;
	vgr[10] = 0x404D;
	vgr[11] = 0x4059;
	vgr[12] = 0x4020;
	vgr[13] = 0x4043;
}
