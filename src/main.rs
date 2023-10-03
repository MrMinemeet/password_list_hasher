use sha1_smol::Sha1;

fn main() {
	hash_string("HELLO", Algorithms::MD5);
	hash_string("HELLO", Algorithms::SHA1);
}

fn hash_string(to_hash: &str, algorithm: Algorithms) {
	println!("Hashing {} with {}", to_hash, algorithm.to_string());
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
	println!("Digest: {}", digest_str);
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