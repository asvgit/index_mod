extern crate indmod;

/*use std::fs::File;
use std::io::prelude::*;*/
//use indmod::dtree::*;
//use indmod::pstring::*;
use indmod::tests_pstring;
use indmod::tests_dtree;

fn main() {
    /*let mut f = File::open("inp_qbase.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let mut f = File::open("inp_query.txt").unwrap();
    let mut st = String::new();
    f.read_to_string(&mut st).unwrap();
    s.pop();
    //let mut s="f(g(a,e(b,c,d,a)),d,a)".to_string();
    //let mut s="f(g(a),e(a,a),a)".to_string(); //term_with_error
    //s.push_str("\nf(g(a,c),c)");
    println!("Base:");
    println!("{}",s);
    println!("Question:");
    st.pop();
    println!("{}",st);
    println!("Working:");
    /*let mut ind = Index::new();
    ind.make_ptree(&s);
    println!("{}",ind.to_string());*/
    
    let mut b=Base::new();
    //b.add_term(&s.to_string());
    for st in s.lines(){
        b.add_term(&st.to_string());
    }        
    //println!("{}",b.to_string());
    //b.del_term(&1);
    println!("{}",b.to_string());  
    let mut q=Index::new();
    //let mut st="f(*,y)".to_string();
    q.make_ptree(&st.to_string());
    println!("{}",q.to_string());
    println!("{:?}",gen(&b.main_ind.next[0],&q.next[0]));*/
    /*println!("
    
    //////////////////////////////////////////////
                        PString
    //////////////////////////////////////////////
    
    
    ");
    tests_pstring::show_inst();
    tests_pstring::show_gen();*/
    println!("
    
    //////////////////////////////////////////////
                        DTree
    //////////////////////////////////////////////
    
    
    ");
    tests_dtree::show_inst();
    tests_dtree::show_gen();
    
    
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