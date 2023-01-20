use std::u8;
use std::{io::prelude::*, fs::File, path::Path/*, error::Error*/};
use std::process::Command;
use structure::{structure, structure_impl};
use std::collections::HashMap;

// Declarando los clusters como globales y mutables
static mut CLUSTERS: Vec<u8> = Vec::new();
// Tamaño del bloque y del súperbloque
const BLOCK_SIZE: usize = 1440;
const SUPER_BLOCK: usize = 54;

struct Record {
	key: String,
	val: String,
}

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
			Ok(bytes) => println!("\tTamaño total cluster[bytes]: {}", bytes),
		};
		
		let mut header: &[u8] = Default::default();

		for (i, _) in CLUSTERS.iter().enumerate().step_by(BLOCK_SIZE) {
			println!("ENCONTRADO: {}", i);
			let _header_ = cadenas(&CLUSTERS[i..(i+BLOCK_SIZE)]);
			// let _header_ver: (String, String) = cadenas2(&CLUSTERS[10..(i+BLOCK_SIZE)].to_vec());

			if _header_.contains("#+title: Proyecto 2"){
				println!("AQUI LLEGA");
				header = &CLUSTERS[i..(i+BLOCK_SIZE)];
				break;
			}
		}

		// println!("\nCONTENIDO: {}", cadenas(&header));
		let mut header_rec = HashMap::new();
		for (i, _) in header.iter().enumerate().step_by(SUPER_BLOCK) {
			let record = &header[i..(i+SUPER_BLOCK)];
			let rec_string = cadenas(record);

			match parse_rec(rec_string){
				Some(Record{key, val}) => {
					// println!("{}: {}", key, val);
					header_rec.insert(key, val);
				}
				None => {/*println!("AQUI NO HAY NADA")*/}
			}
			// let record: Record = parse_rec(rec_string);
		}

		println!("\tNOMBRE: {}", header_rec.len());
		// println!("\tVersión, Etiqueta del vol: {:?}", cadenas2(&header));
	}
}

fn parse_rec(r: String) -> Option<Record> {
	
	if r.contains("-") {
		let rec: Vec<&str> = r.splitn(2, "-").collect();
		let r = Record {key: rec[0].trim().to_string(), val: rec[1].trim().to_string()};

		return Some(r);
	}else{
		return None;
	}

}

// Funciones para transformar a ASCII
//Para mostrar nombre
fn cadenas(_data: &[u8]) -> String {
	let n = String::from_utf8_lossy(_data).into_owned();
	return n;
}

//Para mostrar versión y etiqueta del volumen
// fn cadenas2(_data: &[u8]) -> (String, String) {
// 	unsafe {
// 		let v = String::from_utf8(CLUSTERS[10..14].to_vec()).expect("ERROR AL LEER EL SÚPER BLOQUE");
// 		let e = String::from_utf8(CLUSTERS[20..35].to_vec()).expect("ERROR AL LEER EL SÚPER BLOQUE");

// 		return (v, e);
// 	}
// }

fn main() {
	// Solo para limpiar la terminal
	Command::new("clear").status().unwrap();
	println!("\tSistema de Archivos de la Facultad de Ingeniería.");
	// println!("\n\tBienvenido\n");
	sistema();
}