mod my_module {
        pub mod my_sub_module {
                pub fn module_function() {
                        println!("I am submodule function");   
                }
        }
}
fn main() {
   my_module::my_sub_module::module_function();
}
