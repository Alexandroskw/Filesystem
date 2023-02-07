use std::process::Command;
use std::ptr::null;
use std::{io, u8};
use std::{fs::File, io::prelude::*, path::Path /*, error::Error*/};
use structure::{structure, structure_impl};

// struct String(u32);

// impl fmt::Display for String {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{}", self.0)
//    }
// }

// Declarando los clusters como globales y mutables
static mut CLUSTERS: Vec<u8> = Vec::new();
// Tamaño del bloque y del súperbloque
const BLOCK_SIZE: usize = 1440;
const SUPER_BLOCK: usize = 57;

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
        let s = structure!("<I");
        // Se desempaquetan los clusters del 40 al 54
        // Tamaño del cluster
        let s1 = match s.unpack(&CLUSTERS[40..44]) {
            Err(_) => panic!(),
            Ok(s1) => s1,
        };
        // Número de clusters del directorio
        let s2 = match s.unpack(&CLUSTERS[45..49]) {
            Err(_) => panic!(),
            Ok(s2) => s2,
        };
        // Número de clusters totales
        let s3 = match s.unpack(&CLUSTERS[50..54]) {
            Err(_) => panic!(),
            Ok(s3) => s3,
        };

        for (i, _) in CLUSTERS.iter().enumerate().step_by(BLOCK_SIZE) {
            let _header_name = nombre(&CLUSTERS[0..(i + BLOCK_SIZE)].to_vec());
            let _header_ver: (String, String) = _labels(&CLUSTERS[10..(i + BLOCK_SIZE)].to_vec());
        }

        println!("===============SUPER BLOQUE===============");
        println!("\tNombre: {}", nombre(&header));
        println!("\tVersión, Etiqueta del vol: {:?}", _labels(&header));
        println!("\tTamaño del Cluster: {:?}", s1);
        println!("\tNúmero de Clusters del dir: {:?}", s2);
        println!("\tNúmero de Clusters totales: {:?}", s3);
    }
}

// Función para importar dentro del sistema de archivos
fn import_file(){
    Command::new("clear").status().unwrap();
    loop {
        println!("\n\tIngresa el nombre del archivo a importar: ");
        // Se crea una cadena para leer el documento a copiar
        let mut file = String::new();
        // Se espera a que el ususario ingrese el nombre del documento
        io::stdin()
            .read_line(&mut file)
            .expect("Error al leer la línea");
        if file == "\n" {
            println!("\nIngresa un nombre válido");
        }
        else {
            println!("Has ingresado: {file}");
        }
        break;
    }
}

// Función únicamente para mostrar el menú de opciones
pub fn menu() {
    loop {
        println!("\n1. Importar (copiar) al FS.");
        println!("\nIngresa una opción>> ");
        let mut o = String::new();
        io::stdin()
            .read_line(&mut o)
            .expect("Error al leer la línea");
        // Convirtiendo a entero u32 la cadena
        let o: u32 = match o.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        if o == 1 {
            import_file();
        }
        else {
            println!("Opcion aun no implementada");
        }
        break;
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