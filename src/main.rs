use std::{io::prelude::*, fs::File, path::Path/*, error::Error*/};
use std::process::Command;

// Declarando los clusters como globales y mutables
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
		
		let header: &[u8] = Default::default();

		for (i, _) in CLUSTERS.iter().enumerate().step_by(BLOCK_SIZE) {
			let header_name = cadenas(&CLUSTERS[0..(i+BLOCK_SIZE)].to_vec());
			let header_ver = cadenas2(&CLUSTERS[10..(i+BLOCK_SIZE)].to_vec());
		}
		println!("\tNombre: {}", cadenas(&header));
		println!("\tVersion: {}", cadenas2(&header));
	}
}

// Función para transformar a ASCII
fn cadenas(_data: &[u8]) -> String {
	unsafe {
		let n = String::from_utf8(CLUSTERS[0..8].to_vec()).expect("ERROR AL LEER EL SÚPER BLOQUE");
		return n;
	}
}

fn cadenas2(_data: &[u8]) -> String {
	unsafe {
		let v = String::from_utf8(CLUSTERS[10..14].to_vec()).expect("ERROR AL LEER EL SÚPER BLOQUE");
		return v;
	}
}

fn main() {
	// Solo para limpiar la terminal
	Command::new("clear").status().unwrap();
	println!("\tSistema de Archivos de la Facultad de Ingenieria.");
	println!("\n\tBienvenido\n");
	sistema();
}