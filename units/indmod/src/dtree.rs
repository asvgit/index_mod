pub struct Index {
    pub next: Vec<Index>,
    pub node_ind: usize,
    pub pointer: String,
    pub data: Vec<usize>,
    pub num_type : bool
}
impl Index {
    //----------start_Index----------\\
    pub fn new()-> Index {
        Index{
            next:vec![],
            node_ind:0,
            pointer: "&".to_string(),
            data:vec![],
            num_type : false
        }
    }
    //----------make_node_of_Index----------\\
    pub fn make(n:usize,p:String,f:bool)->Index{
        Index{
            next:vec![],
            node_ind:n,
            pointer: p,
            data:vec![],
            num_type : f
        }
    }    
    //----------add_node_to_Index----------\\
    pub fn add_new(&mut self,n:&usize,p:String,f:bool){
        self.next.push(Index::make(n.clone(),p,f));
    }    
    //----------make_Index_by_string----------\\
    pub fn make_ptree(&'static mut self,s: &String){
        let mut s=s.clone();
        making(&mut s,self,&mut 1,);
        //println!("{}",self.to_string());
    }  
    //----------chacks_for_Index----------\\
    pub fn has_sach(&self,t:&String,i:&usize)->usize{
        let mut j=1usize;
        for n in &self.next {
            j+=1;
            if (n.pointer.as_str() == t) && (n.node_ind == *i) {return j}
        }
        0
    }
    pub fn contain(&self,dat:&usize)->usize{
        let mut i=0usize;
        for d in &self.data{
            i+=1;
            if d==dat {return i}
        }
        0
    }    
    //----------make_string_for_Index---------\\
    pub fn to_string(&self)->String{  
        let mut s="root_\n".to_string();
        if self.next.len()>0{
            for n in &self.next{
                s.push_str("");
                s.push_str(n.to_string_next("".to_string()).as_str());
                //s.push_str(n.next[0].to_string_next("".to_string()).as_str());
            }
        }
        s.pop();
        s
    }
    fn to_string_next(&self,t:String)->String{  
        let mut t = t;
        t.push_str("\t");
        let mut s=t.clone();
        //if self.num_type{
            s.push_str(self.node_ind.to_string().as_str());s.push_str(" ");
        //}else{
            s.push_str(self.pointer.as_str());
        //}
        s.push_str("\n");
        if self.next.len()>0{
            for n in &self.next{
                s.push_str(n.to_string_next(t.to_string()).as_str());
            }
        }
        s
    }
    ////-------------------for_Base------------------\\\\
    //----------add_node_to_Base----------\\
    pub fn add_new_t(&mut self,n:&usize,p:String,t:usize,f:bool){
        self.next.push(Index::new_t(n.clone(),p,t,f));
    }     
    fn new_t(n:usize,s:String,title:usize,f:bool)->Index{
        Index{
            next:vec![],
            node_ind:n,
            pointer: s,
            data:vec![title],
            num_type: f
        }
    }
    //----------make_string_for_Base---------\\
    //----------for_"to_string()"
    pub fn to_string_t(&self)->String{  
        let mut s="root_\n".to_string();
        if self.next.len()>0{
            for n in &self.next{
                s.push_str("");
                s.push_str(n.to_string_next_t("".to_string()).as_str());
            }
        }
        s.pop();
        s
    }
    //----------for_"to_string_t()"
    fn to_string_next_t(&self,t:String)->String{  
        let mut t = t;
        t.push_str("\t");
        let mut s=t.clone();
        //if self.num_type{
            s.push_str(self.node_ind.to_string().as_str());
            s.push_str(" ");
        //}else{
            s.push_str(self.pointer.as_str());
        //}
        if self.next.len()==0{
            s.push_str(" {");
            for i in &self.data {
                s.push_str(&i.to_string());
                s.push_str(" ");
            }
            s.pop();
            s.push_str("}");
        }
        s.push_str("\n");
        if self.next.len()>0{
            /*s.push_str(t.as_str());
            s.push_str("consists!\n");*/
            for n in &self.next{
                s.push_str(n.to_string_next_t(t.to_string()).as_str());
            }
        }
        s
    }
    //----------end_impl_Index---------\\
}
//--------------------for_Index-------------------//
//--------------------for_"make_ptree()"
fn making(s:&mut String,ind:&'static mut Index,i:&mut usize)->Option<&'static mut Index>{
    if s.len()>0{
        if s.chars().nth(0).unwrap()==','{
            s.remove(0);            
            making(s,ind,i)
        } else if s.chars().nth(0).unwrap()=='('{
            s.remove(0);      
            let mut ii: usize = 1;    
            making(s,ind,&mut ii)        
        }else if s.chars().nth(0).unwrap()==')'{
            s.remove(0);
            Some(ind)
            /*if s.len()>0{      
                println!("it gives return!");
                //ind.node_ind=0;
                making(s,ind, i) 
            }else{None}*/
        }else if s.chars().nth(0).unwrap()!=',' && s.chars().nth(0).unwrap()!=')' && s.chars().nth(0).unwrap()!='('{
            ind.add_new(i,"&".to_string(),true);
            ind.next[0].add_new(i,s.chars().nth(0).unwrap().to_string(),false);
            s.remove(0);
            let ind = &mut ind.next[0].next[0];
            *i+=1;       
            let res = making(s,ind,i);
            if s.len()>0{      
                println!("it gives return!");
                //ind.node_ind=0;
                match res {
                    Some(ind) => {/*println!("{}",ind);*/making(s,ind,i)}
                    None => None
                }
                //making(s,ind, i) 
            }else{None} 
        } else{None}
    }else{None}
    //println!("s:{}; i:{}",&s,&i);
   /* if s.len()>0{
        if s.chars().nth(0).unwrap()==',' {
            s.remove(0); 
        }
        if s.chars().nth(0).unwrap()=='('{
            let x = *i-2; 
            //println!("ind:\n{}",ind.to_string());
            let ind  =  &mut ind.next[x].next[0];
            s.remove(0);
            let mut ii: usize=1;
            println!("a! i:{}",ii);
            making(s,ind,&mut ii);
            println!("ii:{}",ii);
            if ind.num_type{
                ind.node_ind=ii.clone();
            }
        }else if s.chars().nth(0).unwrap()==')'{  
            s.remove(0);                 
            return; 
        }else if s.chars().nth(0).unwrap()!=','{
            //let mut f = ind.next.len();
            let mut f = ind.has_sach(&"&".to_string(),i);
            if f==0 {
                ind.add_new(i,"&".to_string(),true);
                f=ind.next.len()-1;                
            }else{f-=1;}
            //f=0;
            let _ind  =  &mut ind.next[f];
            _ind.add_new(i,s.chars().nth(0).unwrap().to_string(),false);
            s.remove(0);  
            *i+=1;   
        }
            println!("a! i:{}",i);
        making(s,ind,i); 
        if ind.num_type{
            ind.node_ind=i.clone();
        }
    }*/
}
//--------------------end_for_Index-------------------//

pub struct Base {
    pub main_ind: Index,
    pub n_terms: usize
}
impl Base{
    //----------start_Base----------\\
    pub fn new()->Base{Base{main_ind: Index::new(),n_terms: 0usize}}  
    //----------add_term_to_Base----------\\
    pub fn add_term(&mut self,s:&String){
        let mut s=s.clone();
        self.n_terms+=1;
        addition(&mut s,&mut self.main_ind,&self.n_terms,&mut 1);
    //println!("{}",self.to_string());
    }
    //----------remove_term_from_Base----------\\
    pub fn del_term(&mut self,i:&usize){
        for ind in &mut self.main_ind.next{
            deletion(ind,i);
        }
    }
    //----------make_string_for_Base---------\\
    pub fn to_string(&self)->String{
        self.main_ind.next[0].to_string_t()        
    }
    //----------end_impl_Base---------\\
}
//--------------------for_Base-------------------//
//--------------------for_"add_term()"
fn addition(s:&mut String,ind:&mut Index,n:&usize,i:&mut usize){
    if s.len()>0{
        if s.chars().nth(0).unwrap()==','{
            s.remove(0); 
        }
        if s.chars().nth(0).unwrap()=='('{
            let x = *i-2; 
            let ind  =  &mut ind.next[x].next[0];
            s.remove(0);
            addition(s,ind,n,&mut 1);
        }else if s.chars().nth(0).unwrap()==')'{  
            s.remove(0);                
            return; 
        }else if s.chars().nth(0).unwrap()!=','{
            let mut f = ind.has_sach(&"&".to_string(),i);
            if f==0 {
                ind.add_new_t(i,"&".to_string(),n.clone(),true);
                f=ind.next.len()-1;
            }else{f-=2; ind.next[f].data.push(n.clone());}
            let _ind  =  &mut ind.next[f];
            if _ind.next.len()==0{
                _ind.add_new_t(i,s.chars().nth(0).unwrap().to_string(),n.clone(),false);
            }else{
                f = _ind.has_sach(&s.chars().nth(0).unwrap().to_string(),i);
                if f==0{
                    _ind.add_new_t(i,s.chars().nth(0).unwrap().to_string(),n.clone(),false);
                }else{
                    f-=2;
                    _ind.next[f].data.push(n.clone());
                }
            }
            s.remove(0);  
            *i+=1     
        } 
        addition(s,ind,n,i);
    }
}
//--------------------for_Base-------------------//
//--------------------for_"del_term()"
fn deletion(ind:&mut Index,t:&usize){
    if ind.contain(t)!=0{
        let mut i = 0usize;
        let mut for_del: Vec<usize> = vec![];
        for n in &mut ind.next{
            if n.next.len()==0{
                let mut f = n.contain(t);
                if f!=0{
                    if n.data.len()!=1{
                        f-=1;
                        n.data.remove(f);
                    }else{
                        for_del.push(i.clone());
                    }
                }
            }else{
                deletion(n,t);
            }         
            i+=1;
        }
        for i in 0..for_del.len(){
            ind.next.remove(for_del[for_del.len()-i-1]);  
        }
    }     
}
//--------------------end_for_Base------------------//
//--------------------instantiation--------------------//
pub fn inst(base:&Index,quest:&Index)->Vec<usize>{
    if quest.pointer.as_str()=="*".to_string(){
        //println!("a!");
        //println!("a! ret{:?}",base.data.clone());
        return base.data.clone()
    }else{
        let mut m:Vec<usize>=vec![];
        if quest.num_type{
            for qnext in &quest.next{
                for bnext in &base.next{                        
                    if qnext.pointer.as_str()=="*"{
                        //println!("o!");
                        m=disjunction(&m.clone(),&inst(&bnext,&qnext)).clone();
                        continue;
                    }
                    if qnext.pointer.as_str()==bnext.pointer.as_str(){
                        if bnext.next.len()==0{
                            //println!("oh!");
                            //println!("---M{:?}---B{:?}",M,&bnext.data);
                            m=disjunction(&m.clone(),&bnext.data).clone();
                            //println!("M={:?},b={},q={}",M,qnext.pointer.as_str(),bnext.pointer.as_str());
                        }else{
                            //println!("hoho!");
                            //println!("by_no_nom_no_last");
                            m=disjunction(&m.clone(),&inst(&bnext,&qnext)).clone();
                            //println!("{:?}",M);
                        }
                    }
                }
            }
        }else{
            //assert_eq!(quest.next.len(),base.next.len());
            for i in 0..quest.next.len(){
                if i==0{
                    m=inst(&base.next[i],&quest.next[i]).clone();
                    //println!("{:?}",M);
                }else{
                    m=conjunction(&m.clone(),&inst(&base.next[i],&quest.next[i])).clone();
                    //println!("{:?}",M);
                }
            }
        }
        //println!("{:?}",M);
        //println!("inst---Q:{}---res{:?}",quest.pointer.to_string(),M);
        m   
    }    
}
//--------------------disjunction
pub fn disjunction(v1:&Vec<usize>,v2:&Vec<usize>)->Vec<usize>{
    let mut vec=v1.clone();
    vec.append(&mut v2.clone());
    vec.sort();
    vec.dedup();
    //println!("disjunction---v1{:?}---v2{:?}---res{:?}",v1,v2,vec);
    vec
}
//--------------------conjunction
pub fn conjunction(v1:&Vec<usize>,v2:&Vec<usize>)->Vec<usize>{
    fn contain(d:&usize,vec:&Vec<usize>)->usize{
        let mut i=0usize;
        for v in vec{
            i+=1;
            if v==d {return i}
        }
        0
    }
    let mut vec:Vec<usize>=vec![];
    let mut f:usize;
    for v in v1{
        f = contain(v,v2);
        if f!=0{
            vec.push(v.clone());
        }        
    }
    vec.sort();
    //println!("conjunction---v1{:?}---v2{:?}---res{:?}",v1,v2,vec);
    vec
}
//--------------------end_for_instantiation--------------------//
//--------------------generslization--------------------//
pub fn gen(base:&Index,query:&Index)->Vec<usize>{
    let mut m:Vec<usize>=vec![];    
    if query.num_type{
        for i in 0..base.next.len(){
            m=disjunction(&m.clone(),&gen(&base.next[i],&query.next[0])).clone();
        }      
    }else{
        if base.pointer.as_str()=="*"{
            m=disjunction(&m.clone(),&base.data).clone();
        }else if query.pointer.as_str()==base.pointer.as_str(){
            if base.next.len()==0{
                m=disjunction(&m.clone(),&base.data).clone();
            }else{
                for i in 0..query.next.len(){
                    if i==0{
                        m=gen(&base.next[i],&query.next[i]).clone();
                    }else{
                        m=conjunction(&m.clone(),&gen(&base.next[i],&query.next[i])).clone();
                    }
                }
            }
        }
    }
    m
}
//--------------------end_for_generslization--------------------//