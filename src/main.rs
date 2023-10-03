use sha1_smol::Sha1;
use sha2::{Digest, Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};
use std::env;
use std::fs;

fn main() {
    let path = retrieve_path();
    let content = read_from_file(path.as_str());

    if content.is_err() {
        println!("Could not read from file: {}", content.unwrap_err());
        std::process::exit(1)
    }

    for line in content.unwrap().split("\n") {
        let to_hash = line.trim();
        if to_hash.len() > 0 {
            println!("Hashing {}", to_hash);
            hash_string(line, Algorithms::MD5);
            hash_string(line, Algorithms::SHA1);
            hash_string(line, Algorithms::SHA2_256);
            hash_string(line, Algorithms::SHA2_512);
            hash_string(line, Algorithms::SHA3_256);
            hash_string(line, Algorithms::SHA3_512);
        }
    }
}

/**
 * Allows passing of a file as an argument. Otherwise it will default to "passwords.txt"
 */
fn retrieve_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Defaulting to 'passwords.txt'");
        return String::from("passwords.txt");
    }
    let file = args[1].clone();
    println!("Using {file} as the input");
    return file;
}

fn read_from_file(file_path: &str) -> Result<String, std::io::Error> {
    return fs::read_to_string(file_path);
}

fn hash_string(to_hash: &str, algorithm: Algorithms) {
    let digest_str: String;
    match algorithm {
        Algorithms::MD5 => {
            let digest = md5::compute(to_hash);
            digest_str = format!("{:x}", digest);
        }
        Algorithms::SHA1 => {
            let mut hasher = Sha1::new();
            hasher.update(to_hash.as_bytes());
            digest_str = hasher.digest().to_string();
        }
        Algorithms::SHA2_256 => {
            let mut hasher = Sha256::new();
            hasher.update(to_hash.as_bytes());
            let digest = hasher.finalize();
            digest_str = format!("{:x}", digest);
        }
        Algorithms::SHA2_512 => {
            let mut hasher = Sha512::new();
            hasher.update(to_hash.as_bytes());
            let digest = hasher.finalize();
            digest_str = format!("{:x}", digest);
        }
        Algorithms::SHA3_256 => {
            let mut hasher = Sha3_256::new();
            hasher.update(to_hash.as_bytes());
            let digest = hasher.finalize();
            digest_str = format!("{:x}", digest);
        }
        Algorithms::SHA3_512 => {
            let mut hasher = Sha3_512::new();
            hasher.update(to_hash.as_bytes());
            let digest = hasher.finalize();
            digest_str = format!("{:x}", digest);
        }
    }
    println!("{} Digest: {}", algorithm.to_string(), digest_str);
}

// -----------------------------------------------------------------------
enum Algorithms {
    MD5,
    SHA1,
    SHA2_256,
    SHA2_512,
    SHA3_256,
    SHA3_512,
}

impl std::string::ToString for Algorithms {
    fn to_string(&self) -> String {
        match self {
            Algorithms::MD5 => "MD5".to_string(),
            Algorithms::SHA1 => "SHA1".to_string(),
            Algorithms::SHA2_256 => "SHA2-256".to_string(),
            Algorithms::SHA2_512 => "SHA2-512".to_string(),
            Algorithms::SHA3_256 => "SHA3-256".to_string(),
            Algorithms::SHA3_512 => "SHA-512".to_string(),
        }
    }
}
