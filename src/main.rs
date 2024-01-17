use std::io::Read;
use std::fs::File;
use std::path::{Path, self};

#[derive(Debug)]
struct Superblock{
    fs_name: String,
    ver: String,
    vol_label: String,
    cluster_size: u32,
    dir_cluster: u32,
    total_cluster: u32,
}

fn read_superblock(path: &str) -> Result<Superblock, String>{
    let mut file = File::open(path).map_err(|e| format!("No se pudo montar: {}", path))?;
    let mut buffer = [0u8; 54];

    file.read_exact(&mut buffer).map_err(|e| format!("No se pudo montar: {}", path))?;

    let fs_name = String::from_utf8_lossy(&buffer[0..8]).trim().to_string();
    let ver = String::from_utf8_lossy(&buffer[10..14]).trim().to_string();
    let vol_label = String::from_utf8_lossy(&buffer[20..35]).trim().to_string();
    let cluster_size = u32::from_le_bytes(buffer[40..44].try_into().unwrap());
    let dir_cluster = u32::from_le_bytes(buffer[45..49].try_into().unwrap());
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