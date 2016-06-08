extern crate indmod;

/*use std::fs::File;
use std::io::prelude::*;*/
//use indmod::dtree::*;
//use indmod::pstring::*;
use indmod::tests_pstring;
use indmod::tests_dtree;

fn main() {
    println!("
    
    //////////////////////////////////////////////
                        PString
    //////////////////////////////////////////////
    
    
    ");
    tests_pstring::show_inst();
    tests_pstring::show_gen();
    /*println!("
    
    //////////////////////////////////////////////
                        DTree
    //////////////////////////////////////////////
    
    
    ");
    tests_dtree::show_inst();
    tests_dtree::show_gen();*/
    
    
}



//show psrting
/*tests::show_inst();
tests::show_gen();*/ 

//////////////////////////////////////////
///////////////it_works!//////////////////
    /*let s = "f(g(a,c),c)";
    let mut quary = Index::new();
    quary.make_ptree(&s.to_string());
    println!("{}",quary.to_string());
    {
        let mut ind =  &quary;
        for i in 0..2{
            ind =& ind.next[0].next[0];       
        }
        println!("{}",ind.to_string());
    }*/
//////////////////////////////////////////