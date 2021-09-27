pub use self::rename_demo::foo;

pub mod rename_demo {
    pub fn foo() {
        println!("cargo rename demo");
    }
}