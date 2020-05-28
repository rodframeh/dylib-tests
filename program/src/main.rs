#[link (name = "C:/Users/Jimena/Desktop/dylib-tests/lib-hello-world/target/debug/lib_hello_world.dll", kind = "dylib")]
extern "C" {
    fn print_hello_world();
}
fn main () {
    unsafe {print_hello_world();}
}