extern crate indmod;

use std::fs::File;
use std::io::prelude::*;
use indmod::indmod::*;

fn main() {
    //take_base
    let mut f = File::open("inp_base.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    //take_guestion
    let mut f = File::open("inp_quest.txt").unwrap();
    let mut st = String::new();
    f.read_to_string(&mut st).unwrap();
    //show_inputs
    println!("Base:");
    s.pop();
    println!("{}",s);
    println!("Question:");
    st.pop();
    println!("{}",st);
    //do_our_best!
    println!("Working:");
    let mut b=Base::new();
    for st in s.lines(){
        b.add_term(&st.to_string());
    }    
    println!("{}",b.to_string());
    b.del_term(&3);
    println!("{}",b.to_string());
    let mut q=Index::start();
    q.make_ptree(&st.to_string());
    println!("{}",q.to_string());    
}