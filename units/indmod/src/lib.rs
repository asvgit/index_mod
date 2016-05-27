
pub mod pstring;
pub mod dtree;
pub mod tests {
//--------------------shower_for_instantiation--------------------//
use std::fs::File;
use std::io::prelude::*;
pub use pstring::*;
//pub use dtree::*; //так нельзя!!!
    pub fn show_inst(){
        println!("
        /////////////////////////////////////////////
        ");
        println!("\t\tDoing instantiation!");
        println!("
        /////////////////////////////////////////////
        ");
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
        println!("Instantiation (inst) : {:?}",p);    
    }
//--------------------end_for_instantiation--------------------//
//--------------------shower_for_generslization--------------------//
    pub fn show_gen(){
        println!("
        /////////////////////////////////////////////
        ");
        println!("\t\tDoing generslization!");
        println!("
        /////////////////////////////////////////////
        ");
        //take_base
        let mut f = File::open("inp_qbase.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        //take_guestion
        let mut f = File::open("inp_query.txt").unwrap();
        let mut st = String::new();
        f.read_to_string(&mut st).unwrap();
        //show_inputs
        s.pop();
        println!("Questions:\n{}",s);
        st.pop();
        println!("Query:\n{}",st);
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
        let p=gen(&b.main_ind,&q);
        println!("generslization (gen) : {:?}",p);    
    }
//--------------------end_for_generslization--------------------//
}