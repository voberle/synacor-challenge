#![allow(dead_code)]

// MD5 hashes of the 8 codes produced by the challenge.
// echo -n "<Code Here>" | md5sum
const CODES: [&str; 8] = [
    "76ec2408e8fe3f1753c25db51efd8eb3",
    "0e6aa7be1f68d930926d72b3741a145c",
    "7997a3b2941eab92c1c0345d5747b420",
    "186f842951c0dcfe8838af1e7222b7d4",
    "2bf84e54b95ce97aefd9fc920451fc45",
    "e09640936b3ef532b7b8e83ce8f125f4",
    "4873cf6b76f62ac7d5a53605b2535a0c",
    "d0c54d4ed7f943280ce3e19532dbb1a6",
];

pub fn verify_code(code_nb: usize, code: &str) -> bool {
    format!("{:x}", md5::compute(code.as_bytes())) == CODES[code_nb]
}

pub fn check_code(code_nb: usize, code: &str) {
    if verify_code(code_nb, code) {
        println!("Code {} is correct", code_nb);
    } else {
        panic!("Wrong code {}", code_nb)
    }
}
