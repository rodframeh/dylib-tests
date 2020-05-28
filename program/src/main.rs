extern crate libloading;

use libloading::{Library};

fn main(){
	let lib = Library::new("c:/Users/Jimena/Desktop/dylib-tests/lib-hello-world/target/debug/lib_hello_world.dll").unwrap();
	unsafe {
		match lib.get::<fn(&mut String)->usize>(b"print_hello_world\0"){
			Err(_error) => {println!("Error");}
            Ok(_funcion) => {
				let mut saludo = String::from("Hola");
				println!("tamaño: {}, texto: {}",_funcion(&mut saludo), saludo);
				}
		}
	}
}