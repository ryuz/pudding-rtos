/** 
 *  
 */



// #include "arm.inc"


                .text
                .align  2

/************************************************
  実行コンテキストの作成
  void _kernel_context_create(
            ContexControlBlock *ctxcb,             x0   :作成するコンテキスト
            void               *isp,               x1   :初期スタックポインタ
            void               (*entry)(intptr_t), x2   :実行アドレス
            intptr_t           exinf               x3   :実行時パラメータ
 ************************************************/
                .global _kernel_context_create
                .type   _kernel_context_create, %function
_kernel_context_create:
                sub     x1, x1, #112                /* スタックを確保 */
                str     x3, [x1, #0]                /* x0 部分にpar1を設定 */
                str     x2, [x1, #8]                /* x18 部分にentryを設定 */
                adr     x2, context_entry           /* エントリーアドレス設定 */
                str     x2, [x1, #104]              /* lr 部分にエントリーアドレスを設定 */
                str     x1, [x0]                    /* ctxcb に新 sp を保存 */
                ret                                 /* リターン */


/************************************************
  実行コンテキストエントリーアドレス
 ************************************************/
context_entry:
      /*        mov     x0, x0 */                   /* exinf を設定 */
                br      x18                         /* entry を実行 */

                .size   _kernel_context_create, .-_kernel_context_create


                .end


/* end of file */
