#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

fn hash_object(args: &[String]) {
    if args.len() < 3 || args[2] != "-w" {
        println!("Command hash-object needs parameters (-w)")
    } else {
        let file_name = args.get(3);
        if let Some(v) = file_name {
            let content = read_to_string(v).expect(&format!("Fail to read the file {}", v));
            let byte_size = content.len();
            let new_content = format!("blob {}\0{}", byte_size, content);
            // create SHA1 hasher instance
            let mut hasher = Sha1::new();
            // Feed data into hasher
            hasher.update(new_content.as_bytes());
            // Retrive the result
            let object_hash = hasher.finalize();
            let hex = format!("{:x}", object_hash);
            // Compress the data
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder
                .write_all(new_content.as_bytes())
                .expect("Failed to compress data");
            let compressed_data = encoder.finish();
            match compressed_data {
                Ok(result) => {
                    // write the compressed data to the file
                    let dirpath = &format!(".git/objects/{}", &hex[..2]);
                    let filepath = &format!("{}/{}", dirpath, &hex[2..]);
                    fs::create_dir_all(dirpath).expect("Failed to create the directory");
                    let mut file = File::create(filepath).expect("Failed to create file");
                    file.write_all(&result[..]).expect("Failed to write")
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        } else {
            println!("Filename needed for hash-object")
        }
    }
}

fn cat_file(args: &[String]) {
    if args.len() < 3 || args[2] != "-p" {
        println!("Command cat-file needs parameters (-p)")
    } else {
        let hash_code = args.get(3);
        if let Some(v) = hash_code {
            let filepath = &format!(".git/objects/{}/{}", &v[..2], &v[2..]);
            let compress = fs::read(filepath).expect(&format!("Fail to read the file{}", filepath));
            let mut decoder = ZlibDecoder::new(&compress[..]);
            let mut decompressed = Vec::new();
            decoder
                .read_to_end(&mut decompressed)
                .expect("Failed to decompress");
            let mut iter = decompressed.split(|&c| c == b'\0');
            iter.next();
            if let Some(second) = iter.next() {
                if let Ok(s) = String::from_utf8(second.to_vec()) {
                    print!("{}", s);
                }
            }
        } else {
            println!("Hash needed for cat-file")
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Error: Command needed")
    } else if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "hash-object" {
        hash_object(&args);
    } else if args[1] == "cat-file" {
        cat_file(&args);
    } else {
        println!("unknown command: {}", args[1])
    }
}
