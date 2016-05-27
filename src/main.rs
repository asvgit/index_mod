extern crate indmod;

use std::fs::File;
use std::io::prelude::*;
use indmod::dtree::*;
//use indmod::tests;

fn main() {
    let mut f = File::open("inp_term.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s.pop();
    println!("{}",s);
    let mut ind = Index::new();
    ind.make_ptree(&s);
    println!("{}",ind.to_string());
}

//show psrting
/*tests::show_inst();
tests::show_gen();*/ 