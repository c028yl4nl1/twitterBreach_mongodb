use mongodb::error::Result as MongoResult;
use mongodb::options::ClientOptions;
use mongodb::{bson::doc, Client};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ptr::null;
use std::{path::PathBuf, str::FromStr};

type ResultType = Result<(), Box<dyn Error>>;

#[tokio::main]
async fn main() {
    aumentar_limite_arquivos_abertos(65535);
    read_file_chucks("a.txt").await;
}

async fn read_file_chucks<T: AsRef<str>>(file: T) -> ResultType
where
    T: ToString,
{
    let path: PathBuf = PathBuf::from_str(file.as_ref())?;
    //   open file
    let file = File::open(path)?;
    let mut Buffer_Reader = BufReader::new(file);

    let mut buffer_vec_bytes_read = vec![0; 50 * 1024 * 1024];

    let mut loop_count: isize = 0;
    loop {
        loop_count += 50;
        println!(
            "Total lindo ate o momento: {}",
            human_bytes::human_bytes((loop_count * 1024 * 1024) as f64)
        );
        let read_file_bytes = match Buffer_Reader.read(&mut buffer_vec_bytes_read) {
            Ok(number) => number,
            _ => continue,
        };

        if read_file_bytes == 0 {
            break;
        }
        if let Ok(buffer_String) =
            String::from_utf8(buffer_vec_bytes_read[..read_file_bytes].to_vec())
        {
            for line in buffer_String.lines() {
                let split: Vec<&str> = line.split_whitespace().collect();
                if split.len() > 3 {
                    let email = split[1];
                    if email.contains("@") {
                        let concat = split[2..].join(" ");
                        if let Err(error) = MongoDbSaveM(email, &concat).await {
                            println!("Error{}", error.to_string());
                        }
                    }
                }
            }
        } else {
            continue;
        }
    }

    Ok(())
}

async fn MongoDbSaveM<T>(key_email: T, buffer: T) -> ResultType
where
    T: ToString,
{
    let cliente_url = ClientOptions::parse("mongodb://localhost:9090").await?;

    let connect = Client::with_options(cliente_url)?;

    let db = connect.database("leaks");

    let collection = db.collection("twitter200milhoes");

    let document = doc! {
        "email": key_email.to_string(),
        "buffer": buffer.to_string(),
    };

    collection.insert_one(document).await?;

    Ok(())
}

extern crate libc;

use libc::{rlimit, setrlimit, RLIMIT_NOFILE};

fn aumentar_limite_arquivos_abertos(novo_limite: u64) {
    let rlim = rlimit {
        rlim_cur: novo_limite, // Soft limit
        rlim_max: novo_limite, // Hard limit
    };

    let resultado = unsafe { setrlimit(RLIMIT_NOFILE, &rlim) };

    if resultado == 0 {
        println!("Limite de arquivos abertos atualizado com sucesso");
    } else {
        println!("Falha ao atualizar o limite de arquivos abertos");
    }
}
