pub mod my_lib_package {
    pub mod my_lib_nested_package {
        #[warn(dead_code)]
        pub fn test() {
            println!("I am  function");   
        }
    }
}
