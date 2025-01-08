
use rand::{thread_rng, Rng};



pub fn gen_simple_hash(len: i8) -> String {

  let mut rng = thread_rng();
  let charset: Vec<char> = (b'a'..=b'z')
      .chain(b'0'..=b'9')
      .map(|x| x as char)
      .collect();

  let hash: String = (0..len as usize)
      .map(|_| charset[rng.gen_range(0..charset.len())])
      .collect();


  return hash;
}