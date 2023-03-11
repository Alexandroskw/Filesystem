use std::process::Command;
use std::{io, u8, fs};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use structure::{structure, structure_impl};

// Declarando los clusters como globales y mutables
static mut CLUSTERS: Vec<u8> = Vec::new();
// Tamaño del bloque y del súperbloque
const BLOCK_SIZE: usize = 1440;
// const SUPER_BLOCK: usize = 64;

// Funcion para leer el sistema de archivos
fn sistema() {
    // Se crea la ruta principal al sistema de archivos
    let path = Path::new("src/fiunamfs.img");
    // Haciendo comparación con match para montar el sistema de archivos
    let mut file = match File::open(path) {
        // Si no se ha podido montar, se muestra el aviso y el tipo de error
        Err(e) => panic!(
            "\n\tNo se ha podido montar {}: {}",
            path.display(),
            e.to_string()
        ),
        Ok(file) => file,
    };
    // Para poder usar las variables estáticas mutables -y globales- se tiene que usar 'unsafe'
    unsafe {
        match file.read_to_end(&mut CLUSTERS) {
            Err(e) => panic!(
                "\n\tNo se ha podido leer {}: {}",
                path.display(),
                e.to_string()
            ),
            Ok(bytes) => println!("\tExito al leer {}[bytes] de {}\n\n", bytes, path.display()),
        };

        let header: &[u8] = Default::default();

        /* Para usar 'pack' y ''unpack', se tiene que importar la bliblioteca 'structure'.
        Una vez importada, podemos hacer uso de ambas funciones. */
        // Se desempaquetan los clusters del 40 al 54
        let s = structure!("<I");
        
        for (i, _) in CLUSTERS.iter().enumerate().step_by(BLOCK_SIZE) {
            let _header_name = nombre(&CLUSTERS[0..(i + BLOCK_SIZE)].to_vec());
            let _header_ver: (String, String) = _labels(&CLUSTERS[10..(i + BLOCK_SIZE)].to_vec());
        }

        println!("===============SUPER BLOQUE===============");
        println!("Nombre: {}", nombre(&header));
        println!("Versión, Etiqueta del vol: {:?}", _labels(&header));
        // Tamaño del cluster
        let _s1 = match s.unpack(&CLUSTERS[40..44]) {
            Err(_) => panic!("ERROR"),
            Ok(s1) => println!("Tamaño del Cluster: {:?}", s1),
        };
        // Número de clusters del directorio
        let _s2 = match s.unpack(&CLUSTERS[45..49]) {
            Err(_) => panic!("ERROR"),
            Ok(s2) => println!("Número de Clusters del dir: {:?}", s2),
        };
        // Número de clusters totales
        let _s3 = match s.unpack(&CLUSTERS[50..54]) {
            Err(_) => panic!("ERROR"),
            Ok(s3) => println!("Número de Clusters totales: {:?}", s3),
        };
        println!("==========================================");
    }
}

//Función para buscar el archivo en el directorio
fn search_file(file_user: &str) -> bool {
    let path: &Path = Path::new(&file_user);
    // Cambiando de tipos para extraer únicamente el nombre del archivo dado por el usuario
    let user_filename: &std::ffi::OsStr = path.file_name().unwrap();

    if !user_filename.is_empty() {
        println!("Existe el archivo: '{}'", path.display());

        return true;
    }
    else {
        println!("\nRevisa que esté bien escrito o exista el archivo: '{}'", path.display());
        // Si el archivo no existe, se entra al ciclo para revisar que exista el archivo dentro de la carpeta
        for i in fs::read_dir("./src").unwrap() {
            println!("{}", i.unwrap().path().display());
        }

        return false;
    }
}

// Función para "importar" de la computadora al sistema de archivos
fn import_file() {
    Command::new("clear").status().unwrap();
    
    loop {
        println!("\nIngresa el nombre del archivo a importar: ");
        // Se crea una cadena para leer el documento a copiar
        let mut file = String::new();
        // Se espera a que el ususario ingrese el nombre del documento
        io::stdin()
            .read_line(&mut file)
            .expect("Error al leer la línea");
        // Borrando '\n' con 'trim()' para que encuentre correctamente el archivo dado por el usuario
        let file: &str = file.trim();

        let file_import: bool = search_file(file);

        if file_import == true {
            println!("SIGO EXISTIENDO");
        }
        else {
            println!("SIGO SIN EXISTIR");
        }
    }
}

// Función únicamente para mostrar el menú de opciones
pub fn menu() {
    loop {
        println!("\n1. Importar (copiar) al FS.");
        println!("5. Salir.");
        println!("\nIngresa una opción>> ");
        let mut o = String::new();
        io::stdin()
            .read_line(&mut o)
            .expect("Error al leer la línea");
        // Convirtiendo a entero u32 la cadena y cortando "\n" del final
        let o: u32 = match o.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        if o == 1 {
            import_file();
        }
        else if o == 5 {
            std::process::exit(1);
        }
        else {
            println!("Opcion aun no implementada");
        }
        // break;
    }
}

/*==========Funciones para transformar a ASCII==========*/
//Para mostrar nombre
fn nombre(_data: &[u8]) -> String {
    unsafe {
        let n = String::from_utf8(CLUSTERS[0..8].to_vec()).expect("ERROR AL LEER EL SÚPER BLOQUE");
        return n;
    }
}

//Para mostrar versión y etiqueta del volumen
fn _labels(_data: &[u8]) -> (String, String) {
    unsafe {
        let v = String::from_utf8(CLUSTERS[10..14].to_vec()).expect("ERROR AL LEER EL SÚPER BLOQUE");
        let e = String::from_utf8(CLUSTERS[20..35].to_vec()).expect("ERROR AL LEER EL SÚPER BLOQUE");

        return (v, e);
    }
}

fn main() {
    // Solo para limpiar la terminal
    Command::new("clear").status().unwrap();
    println!("\tSistema de Archivos de la Facultad de Ingeniería.");
    println!("\n\tBienvenido\n");
    sistema();
    menu();
}