/** 
 *
 */


                .text
                .align  4

/************************************************
  実行コンテキストの切替
  void _kernel_context_switch(
            ContextControlBlock *ctx_next,     rdi : 切り替え先コンテキスト
            ContextControlBlock *ctx_current)  rsi : 現在のコンテキストの保存先
 ************************************************/
                .global _kernel_context_switch
_kernel_context_switch:
                push    %r12
                push    %r13
                push    %r14
                push    %r15
                push    %rbx
                movq    %rsp, (%rsi)        /* スタックポインタ保存 */
        /* --- そのまま _asm_context_start に続ける */


/************************************************
  実行コンテキストの開始
  void _kernel_context_start(
            ContextControlBlock *ctx_next  rdi : 実行を開始するコンテキスト
 ************************************************/
                .global _kernel_context_start
_kernel_context_start:
                movq    (%rdi), %rsp        /* スタックポインタ復帰 */
                pop     %rbx
                pop     %r15
                pop     %r14
                pop     %r13
                pop     %r12
                ret

                .end


/* end of file */
