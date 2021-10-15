/** 
 *
 */


// #include "arm.inc"


                .text
                .align  4

/************************************************
  実行コンテキストの切替
  void _kernel_context_switch(
            ContextControlBlock *ctx_next,    x0 : 切り替え先コンテキスト
            ContextControlBlock *ctx_current) x1 : 現在のコンテキストの保存先
 ************************************************/
                .global _kernel_context_switch
_kernel_context_switch:
                stp     x29, x30, [sp, #-16]!
                stp     x27, x28, [sp, #-16]!
                stp     x25, x26, [sp, #-16]!
                stp     x23, x24, [sp, #-16]!
                stp     x21, x22, [sp, #-16]!
                stp     x19, x20, [sp, #-16]!
                stp     xzr, x18, [sp, #-16]!   /* xzr を入れてアライメントを揃える */

                mov     x19 ,sp
                str     x19, [x1]               /* スタックポインタ退避 */
        /* --- そのまま _asm_context_start に続ける */


/************************************************
  実行コンテキストの開始
  void _kernel_context_start(
            ContextControlBlock *ctx_next    x0 : 実行を開始するコンテキスト
 ************************************************/
                .global _kernel_context_start
_kernel_context_start:
                ldr     x19, [x0]
                mov     sp, x19                     /* スタックポインタ復帰 */
                ldp     x0, x18, [sp], #16
                ldp     x19, x20, [sp], #16
                ldp     x21, x22, [sp], #16
                ldp     x23, x24, [sp], #16
                ldp     x25, x26, [sp], #16
                ldp     x27, x28, [sp], #16
                ldp     x29, x30, [sp], #16
                ret

                .end


/* end of file */
