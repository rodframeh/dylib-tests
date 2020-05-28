#[no_mangle]
pub fn print_hello_world(saludo: &mut String) -> usize{
    saludo.push_str("mundo");
	saludo.len()
}