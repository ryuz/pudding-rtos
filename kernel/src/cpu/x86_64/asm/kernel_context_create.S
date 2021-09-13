/** 
 *  
 */



//                .text
//                .align  4

/************************************************
  実行コンテキストの作成
  void _kernel_context_create(
            ContexControlBlock *ctxcb,             rdi  :作成するコンテキスト
            void               *isp,               rsi  :初期スタックポインタ
            void               (*entry)(intptr_t), rdx  :実行アドレス
            intptr_t           exinf               rcx  :実行時パラメータ
 ************************************************/
                .global _kernel_context_create
                .type   _kernel_context_create, %function
_kernel_context_create:
                subq    $48, %rsi                   /* スタックを確保 */
                movq    %rdx, (%rsi)                /* rbx 部分にentryを設定 */
                movq    %rcx, 8(%rsi)               /* r15 部分にexinfを設定 */
                leaq    context_entry(%rip), %rdx
                movq    %rdx, 40(%rsi)              /* リターンアドレス部分にエントリーアドレスを設定 */
                movq    %rsi, (%rdi)                /* ctxcb に新 sp を保存 */
                retq                                /* リターン */


/************************************************
  実行コンテキストエントリーアドレス
 ************************************************/
context_entry:
                  movq    %r15, %rdi                 /* exinfを設定 */
                  jmp     *%rbx                       /* entry を実行 */

                  .size   _kernel_context_create, .-_kernel_context_create


                  .end


/* end of file */
