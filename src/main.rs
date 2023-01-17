use std::{io::Cursor, fs::File, path::Path, os, error::Error};
use packs::{Pack, Unpack};
use std::io::prelude::*;

//Declarando los clusters como globales y mutables
static mut CLUSTERS: Vec<u8> = Vec::new();
// static mut CLUSTER: [u32; 0] = [];

// Funcion para escribir dentro del sistema de archivos
fn sistema() {
	// Se crea la ruta principal al sistema de archivos
	let path = Path::new("fiunamfs.img");
	// Haciendo comparación con match para montar el sistema de archivos
	let mut file = match File::open(path) {
		// Si no se ha podido montar, se muestra el aviso y el tipo de error
		Err(e) => panic!("No se ha podido montar {}: {}", path.display(), e.description()),
		Ok(file) => file,
	};
	// Para poder usar las variables estáticas mutables -y globales- se tiene que usar 'unsafe'
	unsafe { 
		file.read_to_end(&mut CLUSTERS);
	}

	// let mut new_path = path.join("a").join("b");
	// new_path.push("c");
	// new_path.push("archivo.txt");

	// new_path.set_file_name("file.txt");
	// match path.to_str() {
	// 	None => panic!("No es una secuencia UTF-8"),
	// 	Some(s) => println!("\n\tSe ha montado: {}", s),
	// }
}

fn main() {
	println!("\n\n\tSistema de Archivos de la Facultad de Ingenieria.");
	println!("\n\tBienvenido\n");
	sistema();
}