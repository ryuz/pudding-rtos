

#define Mode_USR        0x10        /* USR モード */
#define Mode_FIQ        0x11        /* FIQ モード */
#define Mode_IRQ        0x12        /* IRQ モード */
#define Mode_SVC        0x13        /* SVC モード */
#define Mode_ABT        0x17        /* ABT モード */
#define Mode_UND        0x1b        /* UND モード */
#define Mode_SYS        0x1f        /* SYS モード */
#define Mode_MASK       0x1f        /* モードビットマスク */

#define T_Bit           0x20        /* T ビット */
#define F_Bit           0x40        /* F ビット */
#define I_Bit           0x80        /* I ビット */

/* CpuControlBlock構造体オフセット */
#define CPUCB_IMSK		0
#define CPUCB_INTCNT	1
#define CPUCB_ISP		4

#define INTNO_IRQ		0
#define INTNO_FIQ		1

/* end of file */
