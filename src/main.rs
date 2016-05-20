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
    //b.add_term(&"f(g(a,b),c)".to_string());
    for st in s.lines(){
        b.add_term(&st.to_string());
    }    
    println!("{}",b.to_string());
    /*//deletion
    b.del_term(&5);
    println!("{}",b.to_string());*/
    let mut q=Index::new();
    q.make_ptree(&st.to_string());
    println!("{}",q.to_string());  
    let p=inst(&b.main_ind,&q);
    println!("inst = {:?}",p);
    for pp in &p {
        b.del_term(pp);
    }    
    println!("{}",b.to_string());
}
/*
    let v1:Vec<usize>=vec![1, 2, 3, 4]; 
    let v2:Vec<usize>=vec![3,4,5,6];
    println!("{:?}\n{:?}\n{:?}\n{:?}",v1,v2,disjunction(&v1,&v2),conjunction(&v1,&v2));
*/