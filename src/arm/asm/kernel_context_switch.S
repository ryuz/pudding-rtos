/** 
 *
 */


#include "arm.inc"


                .text
                .align  2

/************************************************
  実行コンテキストの切替
  void _kernel_context_switch(
            ContextControlBlock *ctx_next,    r0 : 切り替え先コンテキスト
            ContextControlBlock *ctx_current) r1 : 現在のコンテキストの保存先
 ************************************************/
                .global _kernel_context_switch
_kernel_context_switch:
                stmfd   sp!, {r4-r11, lr}           /* レジスタ保存 */
                str     sp, [r1]                    /* スタックポインタ保存 */
        /* --- そのまま _asm_context_start に続ける */


/************************************************
  実行コンテキストの開始
  void _kernel_context_start(
            ContextControlBlock *ctx_next    r0 : 実行を開始するコンテキスト
 ************************************************/
                .global _kernel_context_start
_kernel_context_start:
                ldr     sp, [r0]                    /* スタックポインタ復帰 */
                ldmfd   sp!, {r4-r11, lr}           /* レジスタ復帰 */
                bx      lr

                .end


/* end of file */
