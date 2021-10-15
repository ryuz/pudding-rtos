use cc::Build;
//use std::{env, error::Error, fs::File, io::Write, path::PathBuf};
use std::{env, error::Error}; // , path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET").unwrap();

    /*
    {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create("env_list_.txt")?;
        for (key, value) in env::vars() {
            write!(file, "{}: {}\n", key, value)?;
        }
        file.flush()?;
    }*/

    if target.contains("armv7r") {
        // ソースファイル
        let src_files = vec![
            [
                "src/context/arm/kernel_context_create.S",
                "kernel_context_create",
            ],
            [
                "src/context/arm/kernel_context_switch.S",
                "kernel_context_switch",
            ],
            [
                "src/context/arm/kernel_exception_irq.S",
                "kernel_exception_irq",
            ],
        ];

        for name in src_files.into_iter() {
            Build::new()
                .flag("-mfpu=vfpv3-d16")
                .flag("-mthumb-interwork")
                .flag("-mfloat-abi=softfp")
                .flag("-D_KERNEL_ARM_WITH_VFP")
                .flag("-Wno-unused-parameter")
                .flag("-Wno-missing-field-initializers")
                .file(name[0])
                .compile(name[1]);
        }
    }

    if target.contains("x86_64") {
        // ソースファイル
        let src_files = vec![
            [
                "src/context/x86_64/kernel_context_create.S",
                "kernel_context_create",
            ],
            [
                "src/context/x86_64/kernel_context_switch.S",
                "kernel_context_switch",
            ],
        ];

        for name in src_files.into_iter() {
            Build::new()
                .flag("-fPIE")
                .file(name[0])
                .compile(name[1]);
        }
    }

    Ok(())
}
