use std::fs;
use std::env;
use sha1_smol::Sha1;

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
		},
		Algorithms::SHA1 => {
			let mut hasher = Sha1::new();
			hasher.update(to_hash.as_bytes());
			digest_str = hasher.digest().to_string();
		}
	}
	println!("{} Digest: {}",algorithm.to_string(), digest_str);
}

// -----------------------------------------------------------------------
enum Algorithms {
	MD5,
	SHA1
}

impl std::string::ToString for Algorithms {
	fn to_string(&self) -> String {
		match self {
			Algorithms::MD5 => "MD5".to_string(),
			Algorithms::SHA1 => "SHA1".to_string()
		}
	}
}