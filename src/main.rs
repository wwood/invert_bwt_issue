extern crate bio;

use bio::data_structures::suffix_array::suffix_array;
use bio::data_structures::bwt::bwt;
use bio::io::fasta;

use std::fs::File;
use std::str;

fn main(){
    let reader = fasta::Reader::new(File::open("test.fasta").unwrap());
    let mut text = vec![];
    for record in reader.records() {
        let record = record.unwrap();
        text.extend(record.seq().iter().cloned());
        text.extend_from_slice(b"$");
    }

    println!("huh1: {:?}", str::from_utf8(&text[0..60]).unwrap());

    let sa = suffix_array(&text);
    let bwt1 = bwt(&text, &sa);

    let text2 = bio::data_structures::bwt::invert_bwt(&bwt1);
    println!("huh2: {:?}", str::from_utf8(&text2[0..60]).unwrap());

    println!();
    println!("{}",str::from_utf8(&text).unwrap());
    println!("{}",str::from_utf8(&text2).unwrap());
}
