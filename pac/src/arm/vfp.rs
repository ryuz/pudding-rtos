#![allow(dead_code)]
#![cfg(target_arch = "arm")]
#![cfg(target_feature = "vfp2")]


/// VFP有効化
pub unsafe fn enable_vfp() {
    asm!(
        r#"
                mrc     p15, 0, r0, c1, c0, 2   /* CP アクセスレジスタを読み込む */
                orr     r0, r0, #0x00f00000     /* NEON/VFP（コプロセッサ 10 および 11）へのフルアクセス権を有効にする */
                mcr     p15, 0, r0, c1, c0, 2   /* CP アクセスレジスタを書き込む */
                isb
                mov     r0, #0x40000000         /* VFP および NEON ハードウェアをオンにする */
                vmsr    fpexc, r0               /* FPEXC の EN ビットを設定する */
        "#
    );
}


pub fn vabs_f32(v: f32) ->f32 {
    unsafe {
        let mut q: f32;
        asm!(
            r#"
                    vabs.f32    {0}, {1}
            "#,
            out(sreg) q,
            in(sreg) v,
        );
        q
    }
}

pub fn vabs_f64(v: f64) ->f64 {
    unsafe {
        let mut q: f64;
        asm!(
            r#"
                    vabs.f64    {0}, {1}
            "#,
            out(dreg) q,
            in(dreg) v,
        );
        q
    }
}


pub fn vsqrt_f32(v: f32) ->f32 {
    unsafe {
        let mut q: f32;
        asm!(
            r#"
                    vsqrt.f32   {0}, {1}
            "#,
            out(sreg) q,
            in(sreg) v,
        );
        q
    }
}

pub fn vsqrt_f64(v: f64) ->f64 {
    unsafe {
        let mut q: f64;
        asm!(
            r#"
                    vsqrt.f64   {0}, {1}
            "#,
            out(dreg) q,
            in(dreg) v,
        );
        q
    }
}

