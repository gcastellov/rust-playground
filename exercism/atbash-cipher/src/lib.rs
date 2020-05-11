const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const CHUNK_SIZE: usize = 5;

trait AtbashConverter {
    fn convert(&self, from: &str, to: &str) -> String;
    fn reverse(&self) -> String;
}

impl AtbashConverter for &str {
    fn convert(&self, from: &str, to: &str) -> String { 
        self
        .to_ascii_lowercase()
        .chars()
        .filter(|c|!c.is_whitespace())
        .filter_map(|c|{
            if c.is_numeric() { 
                return Some(c) 
            }
            if let Some(i) = from.find(c) {
                return to.chars().nth(i)
            }
            None
        })
        .collect::<String>()
    }

    fn reverse(&self) -> String {
        self
        .chars()
        .rev()
        .collect()
    }
}

pub fn encode(plain: &str) -> String {
    let reversed_alphabet: String = ALPHABET.reverse();
    let encoded: String = plain.convert(ALPHABET, &reversed_alphabet);
    split_by_chunks(encoded)
}

pub fn decode(cipher: &str) -> String {
    let reversed_alphabet: String = ALPHABET.reverse();
    return cipher.convert(&reversed_alphabet, ALPHABET);
}

fn split_by_chunks(encoded: String) -> String {
    (0..encoded.len())
        .filter_map(|index|{
            let rem: usize = index.rem_euclid(CHUNK_SIZE);                        
             if rem == 0 && index > 1 { 
                Some(&encoded[index-CHUNK_SIZE..index])
            }
            else if index == encoded.len() - 1 {
                Some(&encoded[index-rem..=index])
            }
            else {
                None
            }
        })
        .fold(String::from(""),|a,b| if a.is_empty() { a + &b } else { a + " " + &b })
}