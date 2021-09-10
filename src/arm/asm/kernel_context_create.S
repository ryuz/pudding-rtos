/** 
 *  
 */



#include "arm.inc"


                .text
                .align  2

/************************************************
  実行コンテキストの作成
  void _kernel_context_create(
            ContexControlBlock *ctxcb,             r0   :作成するコンテキスト
            void               *isp,               r1   :初期スタックポインタ
            void               (*entry)(intptr_t), r2   :実行アドレス
            intptr_t           exinf               r3   :実行時パラメータ
 ************************************************/
                .global _kernel_context_create
                .code   16
                .thumb_func
                .type   _kernel_context_create, %function
_kernel_context_create:
                sub     r1, #36                     /* スタックを確保 */
                str     r2, [r1, #0]                /* r4 部分にentryを設定 */
                str     r3, [r1, #4]                /* r5 部分にpar1を設定 */
                adr     r2, context_entry           /* エントリーアドレス設定 */
                add     r2, #1                      /* Thumbモードアドレスとして設定 */
                str     r2, [r1, #32]               /* lr 部分にエントリーアドレスを設定 */
                str     r1, [r0]                    /* ctxcb に新 sp を保存 */
                bx      lr                          /* リターン */


/************************************************
  実行コンテキストエントリーアドレス
 ************************************************/
context_entry:
                mov     r0, r5                      /* extを設定 */
/*              mov     r1, r6      */              /* par2を設定 */
                bx      r4                          /* entry を実行 */

                .size   _kernel_context_create, .-_kernel_context_create


                .end


/* end of file */
