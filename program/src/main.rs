extern crate libloading;

use libloading::{Library};

struct Persona {
    nombre: String,
	edad: u8,
	saludo: String,
}

fn main(){
	let lib = Library::new("c:/Users/Jimena/Desktop/dylib-tests/lib-hello-world/target/debug/lib_hello_world.dll").unwrap();
	unsafe {
		match lib.get::<fn(&mut Persona)->usize>(b"print_hello_world\0"){
			Err(_error) => {println!("Error");}
            Ok(_funcion) => {
				let mut persona = Persona {
					nombre: String::from("Julisa"),
					edad: 29,
					saludo: String::from("Hola "),
				};
				
				println!("tamaño: {}, texto: {}",_funcion(&mut persona), persona.saludo);
				}
		}
	}
}