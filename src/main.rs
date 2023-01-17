use std::{io::prelude::*, fs::File, path::Path/*, error::Error*/};
//use packs::{Pack, Unpack};
use std::process::Command;

//Declarando los clusters como globales y mutables
static mut CLUSTERS: Vec<u8> = Vec::new();
// Tamaño del bloque y del súperbloque
const BLOCK_SIZE: usize = 1440;
const SUPER_BLOCK: usize = 54;

// Funcion para escribir dentro del sistema de archivos
fn sistema() {
	// Se crea la ruta principal al sistema de archivos
	let path = Path::new("fiunamfs.img");
	// Haciendo comparación con match para montar el sistema de archivos
	let mut file = match File::open(path) {
		// Si no se ha podido montar, se muestra el aviso y el tipo de error
		Err(e) => panic!("\n\tNo se ha podido montar {}: {}", path.display(), e.to_string()),
		Ok(file) => file,
	};
	// Para poder usar las variables estáticas mutables -y globales- se tiene que usar 'unsafe'
	unsafe { 
		match file.read_to_end(&mut CLUSTERS) {
			Err(e) => panic!("\n\tNo se ha podido leer {}: {}", path.display(), e.to_string()),
			Ok(bytes) => println!("\n\tExito al leer los bytes {} de {}\n\n", bytes, path.display()),
		};
		
		for (i, _) in CLUSTERS.iter().enumerate().step_by(BLOCK_SIZE) {
			println!("\tBLOQUE ENCONTRADO: {}", i);
			let header = &CLUSTERS[i..(i+BLOCK_SIZE)];
			println!("Cabezal del superbloque: {}", cadenas(&header));
		}
	}

}

// Función para transformar a ASCII
fn cadenas(data: &[u8]) -> String {
	unsafe {
		let s = String::from_utf8(CLUSTERS[0..8].to_vec()).expect("INVALIDO");
		return s;
	}
}

fn main() {
	// Solo para limpiar la terminal
	Command::new("clear").status().unwrap();
	println!("\tSistema de Archivos de la Facultad de Ingenieria.");
	println!("\n\tBienvenido\n");
	sistema();
}