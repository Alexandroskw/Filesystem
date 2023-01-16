use std::{io::Cursor, fs::File, path::Path, os};
use packs::{Pack, Unpack};
use std::io::prelude::*;

//Declarando los clusters como globales y mutables
// static mut CLUSTERS: Vec<u32> = Vec::new();
// static mut CLUSTER: [u32; 0] = [];

// Funcion para escribir dentro del sistema de archivos
fn sistema() {
	// Se crea la ruta principal al sistema de archivos
	let path = Path::new("fiunamfs.img");
	let _display = path.display();

	// let mut new_path = path.join("a").join("b");
	// new_path.push("c");
	// new_path.push("archivo.txt");
	// new_path.set_file_name("file.txt");

	match path.to_str() {
		None => panic!("No es una secuencia UTF-8"),
		Some(s) => println!("La ruta es: {}", s),
	}
}



fn main() {
	sistema();
}