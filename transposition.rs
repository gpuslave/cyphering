// double transposition cipher keys:
// k1 = 2 4 1 3 5 (write key)
// k2 = 5 2 1 3 4 (read key)
// matrix write - left to right
// matrix read - top to bottom
// plaintext:  ШИФРОВАНИЕ_ПЕРЕСТАНОВКОЙ_

fn cypher(read_key: &str, write_key: &str, plaintext: &str) -> String {
  let mut s = String::from("");
  let size: usize = read_key.chars().count();

  for i in read_key.chars() {
    if let Some(i) = i.to_digit(10) {
      for j in write_key.chars() {
        if let Some(j) = j.to_digit(10) {
          let index = (i as usize - 1) + (j as usize - 1) * size as usize;

          // println!("i = {}, j = {}, index = {}", i, j, index);

          if let Some(char_to_push) = plaintext.chars().nth(index) {
            s.push(char_to_push);
          }
        }
        println!("{}", s);
      } 
    }
    // println!("--- {} ---", i);
  }

  return s;
}

fn decypher(read_key: &str, write_key: &str, cyphered_text: &str) -> String {
  let mut s = String::from("");
  let size: usize = read_key.chars().count();

  let mut reposition: String = String::from("");
  for i in 1..=size {
    reposition.push_str(&i.to_string());
  }

  for i in reposition.chars() {
    if let Some(i) = i.to_digit(10) {

      for j in reposition.chars() {
        if let Some(j) = j.to_digit(10) {
          let read_index = read_key.chars().position(|c| c == char::from_digit(j, 10).unwrap());
          let write_index = write_key.chars().position(|c| c == char::from_digit(i, 10).unwrap());

          if let Some(read_index) = read_index {
            if let Some(write_index) = write_index {
              // println!("read_i - {}, write_i - {}", read_index, write_index);

              let index = (write_index) + (read_index) * size as usize;

              if let Some(char_to_push) = cyphered_text.chars().nth(index) {
                s.push(char_to_push);
              }
            }
          }
        }
        println!("{}", s);
      }
    }
  }



  return s;
}

fn main() {
  let plaintext: &str = "ШИФРОВАНИЕ_ПЕРЕСТАНОВКОЙ_";
  // let read_key: &str = "35214";
  // let write_key: &str = "43251";
  let read_key: &str = "24135";
  let write_key: &str = "52134";

  let cypheredtext: String = cypher(read_key, write_key, plaintext);
  println!("Cyphered text is: {} \n", cypheredtext);

  println!("Decyphered text is: {}", decypher(read_key, write_key, &cypheredtext));
}
  // let x = 5;
  // let mut y: i32 = 100;

  // let nums: [i32; 5] = [x, y, x, y, x];

  // let nums_grow: Vec<i32> = Vec::new();
  // nums_grow.push(x);
  // nums_grow.push(y);
  // nums_grow.push(x);
  // nums_grow.push(y);
  // nums_grow.push(x);

  // println!("Hello, world!");
  // println!("x = {}, y = {}", x, y);
  // println!("nums = {:?}", nums);
  // println!("nums_grow = {:?}", nums_grow);

