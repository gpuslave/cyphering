fn gcd(mut a: usize, mut b: usize) -> usize {
  while a != 0 && b !=0 {
    if a > b {
      a = a % b;
    } else {
      b = b % a;
    }
  }

  return a + b
}

fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
  if a == 0 {
    return (b, 0, 1);
  } else {
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    return (gcd, x, y);
  }
}

fn inverse_mod(a: isize, m: isize) -> isize {
  let (g, x, _) = extended_gcd(a, m);
  if g != 1 {
    panic!("Inverse doesn't exist");
  }
  return (x % m + m) % m;
}

fn rsa_generate(mut p: usize, mut q: usize) -> ((usize, usize), (usize, usize)) {
  let n: usize = p*q;

  let phi: usize = (p - 1) * (q - 1);

  let mut possible_coprimes: Vec<usize> = Vec::new();

  for num in 2..phi {
    let gcd_r: usize = gcd(phi, num);
    if gcd_r == 1 {
      possible_coprimes.push(num);
    }
  }
  println!("possible_coprimes are: {:?}", possible_coprimes);

  let e: usize = match possible_coprimes.iter().min() {
    Some(min) => {
      println!("min is: {}", min);
      *min
    },
    None => {
      println!("No coprimes found");
      0
    },
  };

  // let d: usize = e.powi(-1) as usize % phi;
  let d: usize = inverse_mod(e as isize, phi as isize) as usize;

  return ((e, n), (d, n));
}

fn rsa_cypher(m: &str, key: (usize, usize)) -> Vec<usize> {
  let (e, n) = key;
  m.bytes().map(|byte| {
    let m_val: usize = byte as usize;
    // let c_val: usize = m_val.pow(e as u32) % n;
    let c_val: usize = mod_pow(m_val, e, n);
    return c_val;
  }).collect()
}

fn rsa_decypher(m: Vec<usize>, key: (usize, usize)) -> String {
  let (d, n) = key;
  let bytes: Vec<u8> = m.iter().map(|&c_val| {
    // let m_val: usize = c_val.pow(d as u32) % n;
    let m_val: usize = mod_pow(c_val, d, n);
    return m_val as u8;
  }).collect();
  return String::from_utf8(bytes).expect("Invalid UTF-8");
}

fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
  if modulus == 1 { return 0; }
  let mut result = 1;
  base = base % modulus;
  while exp > 0 {
      if exp % 2 == 1 { // If exp is odd, multiply base with
          result = (result * base) % modulus;
      }
      exp = exp >> 1; // exp = exp / 2
      base = (base * base) % modulus; // Change base to base^2
    }
  return result;
}


fn main() {
  // println!("Hello, world!");
  // let a: usize = 30;
  // let b: usize = 18;
  // let gcd_r: usize = gcd(a, b);
  // println!("gcd is: {gcd_r}");

  let p: usize = 7;
  let q: usize = 17;
  let public_private: ((usize, usize), (usize, usize)) = rsa_generate(p, q);

  let public_key: (usize, usize) = public_private.0;
  let private_key: (usize, usize) = public_private.1;

  println!("public key is: {:?}", public_key);
  println!("private key is: {:?}", private_key);

  let plaintext: &str = "AMEBA";
  let cyphered_text: Vec<usize> = rsa_cypher(plaintext, public_key);
  println!("cyphered text is: {:?}", cyphered_text);
  println!("decyphered text is: {}", rsa_decypher(cyphered_text, private_key));

}