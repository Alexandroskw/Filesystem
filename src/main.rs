use std::io::Read;
use std::fs::File;
use std::path::{Path, self};

// Estructura para los elementos del superbloque
#[derive(Debug)]
struct Superblock{          
    fs_name: String,
    ver: String,
    vol_label: String,
    cluster_size: u32,
    dir_cluster: u32,
    total_cluster: u32,
}

// Estructura para listar todos los archivos dentro del FS
#[derive(Debug)]
struct dirEntry{
    file_type: char,
    name: String,
    size: u32,
    initial_cluster: u32,
    create_date: String,
    modif_date: String,
    unused: String,
}

// Función para leer el superbloque
fn read_superblock(path: &str) -> Result<Superblock, String>{
    let mut file = File::open(path).map_err(|e| format!("No se pudo montar: {}", path))?;           // Se da la ruta del sistema de archivos y si no existe, lanza el error
    let mut buffer = [0u8; 54];         // Se crea el búfer del tamaño del superbloque

    file.read_exact(&mut buffer).map_err(|e| format!("No se pudo montar: {}", path))?;              // Si no existe el superbloque lanza el aviso de que no se pudo montar

    let fs_name = String::from_utf8_lossy(&buffer[0..8]).trim().to_string();                // Se leen las primeras nueve posiciones del arreglo del superbloque con el búfer y haciendo borrowing del mismo
    let ver = String::from_utf8_lossy(&buffer[10..14]).trim().to_string();
    let vol_label = String::from_utf8_lossy(&buffer[20..35]).trim().to_string();            
    let cluster_size = u32::from_le_bytes(buffer[40..44].try_into().unwrap());                 // Para hacer uso del formato little-endian, se usa la función from_le_bytes para hacer la correcta interpretación de este formato
    let dir_cluster = u32::from_le_bytes(buffer[45..49].try_into().unwrap());                  // try_into() es una especie de  "try-catch", con el adicional de que si no puede transformar el tipo de dato, manda un error
    let total_cluster = u32::from_le_bytes(buffer[50..54].try_into().unwrap());

    Ok(Superblock{
        fs_name,
        ver,
        vol_label,
        cluster_size,
        dir_cluster,
        total_cluster,
    })
}

fn main(){
    let path = "src/fiunamfs.img";

    match read_superblock(path) {
        Ok(superblock) => {
            println!("Superbloque: {:?}", superblock);
        }
        Err(_err) => {
            eprint!("Error!!");
        }
    }
}