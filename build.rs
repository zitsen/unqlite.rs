extern crate bindgen;
extern crate cc;

fn main() {
    use bindgen::builder;
    // Configure and generate bindings.
    let bindings = builder()
        .header("unqlite/unqlite.h")
        .with_codegen_config(bindgen::CodegenConfig {
            vars: false,
            ..Default::default()
        })
        .generate()
        .expect("generate unqlite bindings");
    // Write the generated bindings to an output file.
    bindings
        .write_to_file("src/ffi.rs")
        .expect("write to source file");

    cc::Build::new()
        .file("unqlite/unqlite.c")
        .warnings(false)
        .if_enable_threads()
        .if_jx9_diable_builtin_func()
        .if_jx9_enable_math_func()
        .if_jx9_disable_disk_io()
        .if_enable_jx9_hash_io()
        .compile("libunqlite.a");
}
trait ConfigExt {
    fn if_enable_threads(&mut self) -> &mut Self {
        self
    }
    fn if_jx9_diable_builtin_func(&mut self) -> &mut Self {
        self
    }
    fn if_jx9_enable_math_func(&mut self) -> &mut Self {
        self
    }
    fn if_jx9_disable_disk_io(&mut self) -> &mut Self {
        self
    }
    fn if_enable_jx9_hash_io(&mut self) -> &mut Self {
        self
    }
}

impl ConfigExt for cc::Build {
    #[cfg(feature = "enable-threads")]
    fn if_enable_threads(&mut self) -> &mut Self {
        self.define("UNQLITE_ENABLE_THREADS", None)
            .flag("-lpthread")
    }
    #[cfg(feature = "jx9-disable-builtin-func")]
    fn if_jx9_diable_builtin_func(&mut self) -> &mut Self {
        self.define("JX9_DISABLE_BUILTIN_FUNC", None)
    }
    #[cfg(feature = "jx9-enable-math-func")]
    fn if_jx9_enable_math_func(&mut self) -> &mut Self {
        self.define("JX9_ENABLE_MATH_FUNC", None)
    }
    #[cfg(feature = "jx9-disable-disk-io")]
    fn if_jx9_disable_disk_io(&mut self) -> &mut Self {
        self.define("JX9_DISABLE_DISK_IO", None)
    }
    #[cfg(feature = "enable-jx9-hash-io")]
    fn if_enable_jx9_hash_io(&mut self) -> &mut Self {
        self.define("UNQLITE_ENABLE_JX9_HASH_IO", None)
    }
}
