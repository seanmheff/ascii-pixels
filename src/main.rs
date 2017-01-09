extern crate image;

use std::char;
use std::vec::Vec;
use std::path::Path;
use std::collections::HashMap;

use image::GenericImage;

fn main() {
    let mut positions = Vec::new();
    let img = image::open(&Path::new("lib/image.png")).unwrap();

    // Get the width/height of the image
    let (width, height) = img.dimensions();
    println!("image width {}. height {}\n", width, height);

    // Get the position of the white pixels
    for (width, height, pixel) in img.pixels() {
        if pixel.data[0] == 255 {
            positions.push((height * 100) + width);
        } 
    }

    // Get the offset of the pixels
    for i in (1..positions.len()).rev() {
        positions[i] = positions[i] - positions[i-1];
    }

    // Convert the offsets into morse code ascii characters
    let mut morse_code = Vec::new();
    for i in 0..positions.len() {
        morse_code.push(char::from_u32(positions[i]).unwrap());
    }

    // Create a mapping of morse code to alphanumberic characters
    let mut morse = HashMap::new();
    morse.insert(".-", "a");
    morse.insert("-...", "b");
    morse.insert("-.-.", "c");
    morse.insert("-..", "d");
    morse.insert(".", "e");
    morse.insert("..-.", "f");
    morse.insert("--.", "g");
    morse.insert("....", "h");
    morse.insert("..", "i");
    morse.insert(".---", "j");
    morse.insert("-.-", "k");
    morse.insert(".-..", "l");
    morse.insert("--", "m");
    morse.insert("-.", "n");
    morse.insert("---", "o");
    morse.insert(".--.", "p");
    morse.insert("--.-", "q");
    morse.insert(".-.", "r");
    morse.insert("...", "s");
    morse.insert("-", "t");
    morse.insert("..-", "u");
    morse.insert("...-", "v");
    morse.insert(".--", "w");
    morse.insert("-..-", "x");
    morse.insert("-.--", "y");
    morse.insert("--..", "z");
    morse.insert("-----", "0");
    morse.insert(".----", "1");
    morse.insert("..---", "2");
    morse.insert("...--", "3");
    morse.insert("....-", "4");
    morse.insert(".....", "5");
    morse.insert("-....", "6");
    morse.insert("--...", "7");
    morse.insert("---..", "8");
    morse.insert("----.", "9");

    // Print out the answer
    let s: String = morse_code.iter().cloned().collect();
    for word in s.split_whitespace() {
        println!("> {}", word);
        match morse.get(word) {
            Some(x) => println!("\t{}", x),
            None => println!("Cannot find match for: {}", word)
        }
    }
}
