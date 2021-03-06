extern crate libloading;

use libloading::{Library};

pub struct Persona {
    nombre: String,
	edad: u8,
	saludo: String,
}

#[no_mangle]
pub fn print_hello_world(persona: &mut Persona) -> usize{
	
	let lib = Library::new("c:/Users/Jimena/Desktop/dylib-tests/lib-suma-numeros/target/debug/lib_suma_numeros.dll").unwrap();
	unsafe {
		match lib.get::<fn(i32,i32)->i32>(b"suma_numeros\0"){
			Err(_error) => {println!("Error");}
            Ok(_funcion) => {
				println!("valor de la suma: {}",_funcion(6,6));
				}
		}
	}
	
	persona.saludo.push_str(&persona.nombre);
	persona.saludo.len()
}