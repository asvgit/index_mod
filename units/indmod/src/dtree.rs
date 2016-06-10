pub struct Index<T>{
    pub next: Vec<Index<T>>,
    pub node_ind: usize,
    pub pointer: T,
    pub data: Vec<usize>,
    pub num_type : bool
}
impl Index<String>{
    //----------start_Index----------\\
    pub fn new()-> Index<String>{
        Index{
            next:vec![],
            node_ind:0,
            pointer: "&".to_string(),
            data:vec![],
            num_type : false
        }
    }
    //----------make_node_of_Index----------\\
    pub fn make(n:usize,p:String,f:bool)->Index<String>{
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
    pub fn make_ptree(&mut self,s: &String){
        let mut s=s.clone();
        let mut v=vec![];
        v.push(0);
        making(&mut s,self,&mut 1,&mut v);
        //println!("{}",self.to_string());
    }  
    //----------chacks_for_Index----------\\
    pub fn has_sach(&self,t:&String)->usize{
        let mut j=0usize;
        for n in &self.next {
            j+=1;
            if n.pointer.as_str() == t {return j}
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
                //s.push_str(n.to_string_next("".to_string()).as_str());
                s.push_str(n.next[0].to_string_next("".to_string()).as_str());
            }
        }
        s.pop();
        s
    }
    fn to_string_next(&self,t:String)->String{  
        let mut t = t;
        t.push_str("\t");
        let mut s=t.clone();
        /*//if self.num_type{
            s.push_str(self.node_ind.to_string().as_str());s.push_str(" ");
        //}else{
            s.push_str(self.pointer.as_str());
        //}
        s.push_str("\n");*/
        s.push_str(self.pointer.as_str());s.push_str(" ");
        if self.node_ind>0{
            s.push_str(self.node_ind.to_string().as_str());
        }
        s.push_str("\n");
        if self.next.len()>0{
            for n in &self.next{
                //s.push_str(n.to_string_next(t.to_string()).as_str());
                s.push_str(n.next[0].to_string_next(t.to_string()).as_str());
            }
        }
        s
    }
    ////-------------------for_Base------------------\\\\
    //----------add_node_to_Base----------\\
    pub fn add_new_t(&mut self,n:&usize,p:String,t:usize,f:bool){
        self.next.push(Index::new_t(n.clone(),p,t,f));
    }     
    fn new_t<T>(n:usize,s:T,title:usize,f:bool)->Index<T>{
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
        /*//if self.num_type{
            s.push_str(self.node_ind.to_string().as_str());
            s.push_str(" ");
        //}else{
            s.push_str(self.pointer.as_str());
        //}
        */
        s.push_str(self.pointer.as_str());
        if self.node_ind>0{
            s.push_str(" [");
            s.push_str(self.node_ind.to_string().as_str());
            s.push_str("]");
        }
        //s.push_str("\n");
        //if self.next.len()==0{
            s.push_str(" {");
            for i in &self.data {
                s.push_str(&i.to_string());
                s.push_str(" ");
            }
            s.pop();
            s.push_str("}");
        //}
        s.push_str("\n");
        if self.next.len()>0{
            /*s.push_str(t.as_str());
            s.push_str("consists!\n");*/
            for n in &self.next{
                //s.push_str(n.to_string_next_t(t.to_string()).as_str());
                s.push_str(n.to_string_next_t(t.to_string()).as_str());
            }
        }
        s
    }
    //----------end_impl_Index---------\\
}
//--------------------for_Index-------------------//
//--------------------for_"make_ptree()"
fn making(s:&mut String,ind:&mut Index<String>,i:&mut usize,ticket:&mut Vec<usize>){
    //println!("ticket: {:?}; string:{}",ticket,s.to_string());
    if s.len()>0{
        if s.chars().nth(0).unwrap()==','{
            s.remove(0);            
            making(s,ind,i,ticket);
        } else if s.chars().nth(0).unwrap()=='('{
            s.remove(0);      
            let mut ii: usize = 1;  
            ticket.push(0);  
            making(s,ind,&mut ii,ticket);   
            //println!("head: {}", ind.pointer.to_string());  
            //println!("ticket(last): {}, head: {}",ticket[ticket.len()-1], ind.pointer.to_string()); 
            ind.node_ind=ticket[ticket.len()-1].clone();
            ticket.pop();
        }else if s.chars().nth(0).unwrap()==')'{
            s.remove(0);
            let p=ticket[ticket.len()-1].clone();
            ticket.pop();
            ticket.insert(0,p.clone());
            if s.len()>0{      
                //println!("it gives return!");
                ind.node_ind=0;
                making(s,ind, i,ticket); 
            }else{/*println!("it gives return (end)!");*/ticket.pop();}
            //println!("pop: {}, head: {}",p, ind.pointer.to_string());
        }else if s.chars().nth(0).unwrap()!=',' && s.chars().nth(0).unwrap()!=')' && s.chars().nth(0).unwrap()!='('{
            let n = ticket.len()-1;
            ticket[n]+=1;
            ind.add_new(&ticket[n],"&".to_string(),true);
            ind.next[0].add_new(&0,s.chars().nth(0).unwrap().to_string(),false);
            s.remove(0);
            let _ind = &mut ind.next[0].next[0];
            *i+=1;       
            making(s,_ind,i,ticket);
        }
    }
}
//--------------------end_for_Index-------------------//

pub struct Base<T>{
    pub main_ind: Index<T>,
    pub n_terms: usize,
    pub lin: Vec<Vec<usize>>
}
impl Base<String>{
    //----------start_Base----------\\
    pub fn new()->Base<String>{Base{main_ind: Index::new(),n_terms: 0usize,lin: vec![]}}  
    //----------add_term_to_Base----------\\
    pub fn add_term(&mut self,s:&String){
        let mut s=s.clone();
        self.n_terms+=1;
        let mut v=vec![];
        v.push(0);
        addition(&mut s,&mut self.main_ind,&self.n_terms,&mut 1,&mut v);
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
    //----------back_racking_for_Base---------\\    
    pub fn back_quacking(&mut self,l:Vec<String>){
       let mut set: Vec<usize>=vec![];
        for s in l{            
            self.add_term(&s.to_string());
            set.push(self.n_terms.clone());
        }    
        self.lin.push(set);
    }
    pub fn back_racking(&mut self){
        let i = self.lin.len()-1; 
        let v = self.lin[i].clone();
        for t in &v{
            self.del_term(t);     
        }   
        let n = self.lin[i].len();
        self.n_terms-=n;
        self.lin.pop();
    }  
    //----------end_impl_Base---------\\
}
//--------------------for_Base-------------------//
//--------------------for_"add_term()"
fn addition(s:&mut String,ind:&mut Index<String>,n:&usize,i:&mut usize,ticket:&mut Vec<usize>){
    if s.len()>0{
        if s.chars().nth(0).unwrap()==','{
            s.remove(0);            
            addition(s,ind,n,i,ticket);
        } else if s.chars().nth(0).unwrap()=='('{
            s.remove(0);      
            let mut ii: usize = 1;  
            ticket.push(0);  
            addition(s,ind,n,&mut ii,ticket);   
            //println!("head: {}", ind.pointer.to_string());  
            //println!("ticket(last): {}, head: {}",ticket[ticket.len()-1], ind.pointer.to_string()); 
            if ind.node_ind==0{
                ind.node_ind=ticket[ticket.len()-1].clone();
                ticket.pop();
            }else{
                if ind.node_ind!=ticket[ticket.len()-1]{
                panic!("
//////////////////////////////////////////
\nmy_messege_about_error:\n
    Terms' structs aren't same!\n
//////////////////////////////////////////
");
                }
            }
        }else if s.chars().nth(0).unwrap()==')'{
            s.remove(0);
            let p=ticket[ticket.len()-1].clone();
            ticket.pop();
            ticket.insert(0,p.clone());
            if s.len()>0{      
                //println!("it gives return!");
                //ind.node_ind=0;
                addition(s,ind,n,i,ticket); 
            }else{ticket.pop();}
            //println!("pop: {}, head: {}",p, ind.pointer.to_string());
        }else if s.chars().nth(0).unwrap()!=',' && s.chars().nth(0).unwrap()!=')' && s.chars().nth(0).unwrap()!='('{
            if ind.next.len()==0{               
                let k = ticket.len()-1;
                ticket[k]+=1;
                ind.add_new(&ticket[k],"&".to_string(),true);
                ind.next[0].add_new(&0,s.chars().nth(0).unwrap().to_string(),false);
                ind.next[0].data.push(n.clone());                
                //println!("{:?} - {}",ind.data,ind.next[0].next[0].pointer.to_string());
                s.remove(0);
                if s.chars().nth(0).unwrap()==')'{
                    ind.next[0].pointer="$".to_string();
                }
                let _ind = &mut ind.next[0].next[0];
                _ind.data.push(n.clone());
                *i+=1;       
                addition(s,_ind,n,i,ticket);
            }else{                
                let mut f = ind.next[0].has_sach(&s.chars().nth(0).unwrap().to_string().to_string());
                //println!("I have some! f={} pointer: {},next_p: {}",f,ind.pointer.to_string(),ind.next[0].pointer.to_string());
                //println!("next/next_p:{}",ind.next[0].next[0].pointer.to_string());
                if f==0{ 
                    //println!("I haven't any!");
                    let k = ticket.len()-1;
                    ticket[k]+=1;
                    ind.next[0].add_new(&0,s.chars().nth(0).unwrap().to_string(),false);
                    ind.next[0].data.push(n.clone());
                    //println!("{:?} - {}",ind.data,ind.next[0].next[0].pointer.to_string());
                    s.remove(0);
                    let k = ind.next[0].next.len()-1;
                    let _ind = &mut ind.next[0].next[k];
                    _ind.data.push(n.clone());
                    *i+=1;       
                    addition(s,_ind,n,i,ticket);
                }else{
                    //println!("I have sach!");
                    f-=1;
                    let k = ticket.len()-1;
                    ticket[k]+=1;
                    //ind.next[0].add_new(&0,s.chars().nth(0).unwrap().to_string(),false);
                    //println!("{:?} - {}",ind.data,ind.next[0].next[0].pointer.to_string());
                    ind.next[0].data.push(n.clone());
                    s.remove(0);
                    let _ind = &mut ind.next[0].next[f];
                    _ind.data.push(n.clone());
                    *i+=1;       
                    addition(s,_ind,n,i,ticket);
                }
            }
        }
    }
}
//--------------------for_Base-------------------//
//--------------------for_"del_term()"
fn deletion(ind:&mut Index<String>,t:&usize){
    if ind.contain(t)!=0{
        //println!("I have some!{}",ind.pointer.to_string());
        let mut i = 0usize;
        let mut for_del: Vec<usize> = vec![];
        for n in &mut ind.next{
            let mut f = n.contain(t);
            if n.next.len()==0{
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
                if n.data.len()==0{
                        for_del.push(i.clone());
                }
            }         
            i+=1;
        }
        //println!("I have for del{:?}!",for_del);
        for i in 0..for_del.len(){
            ind.next.remove(for_del[for_del.len()-i-1]);  
        }
        let r=ind.contain(t)-1;
        ind.data.remove(r);
    }
}
//--------------------end_for_Base------------------//
//--------------------instantiation--------------------//
pub fn inst(base:&Index<String>,quest:&Index<String>)->Vec<usize>{
    let mut m:Vec<usize>=vec![];
    if quest.num_type{        
        for b in &base.next{
            if quest.next[0].pointer.as_str()=="*"{
                if b.next.len()==0{
                    m=disjunction(&m.clone(),&b.data).clone();
                }else{
                    if b.node_ind==0{
                        m=disjunction(&m.clone(),&inst(&b.next[0],&quest.next[0].next[0])).clone();
                    }else{
                        m=disjunction(&m.clone(),&get_after(b,&quest.next[0],&b.node_ind)).clone();                
                    }
                }
            }else if quest.next[0].pointer.as_str()==b.pointer.as_str(){
                if b.next.len()==0{
                    m=disjunction(&m.clone(),&b.data).clone();
                }else{
                    m=disjunction(&m.clone(),&inst(&b.next[0],&quest.next[0].next[0])).clone();
                }
            }
        }
        
    }
    return m;
}
fn get_after(b:&Index<String>,q:&Index<String>,n:&usize)->Vec<usize>{
    if *n>0{
        let mut v:Vec<usize>=vec![]; 
        for i in &b.next{
            if i.num_type{
                v.append(&mut get_after(&i,q,n));
            }else{
                v.append(&mut get_after(&i,q,&(n-1)));
            } 
        }
        v
    }else{
            inst(&b.next[0],&q.next[0])
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
pub fn gen(base:&Index<String>,query:&Index<String>)->Vec<usize>{
    let mut m:Vec<usize>=vec![];    
    if query.num_type{
        for b in &base.next{
            if b.pointer.as_str()=="*"{
                if query.next[0].node_ind>0{
                    m.append(&mut gen(&b.next[0],get_a(&query.next[0].next[0],&query.next[0].node_ind)).clone());
                }else{
                    if b.next.len()==0{
                        m.append(&mut b.data.clone());
                    }else{
                        m.append(&mut gen(&b.next[0],&query.next[0].next[0]).clone());
                    }                    
                }
                continue;
            }
            if b.pointer.as_str()==query.next[0].pointer.as_str(){
                if b.next.len()==0{
                    m.append(&mut b.data.clone());
                }else{
                    m.append(&mut gen(&b.next[0],&query.next[0].next[0]).clone());
                }
            }
            //m=disjunction(&m.clone(),&gen(&base.next[i],&query.next[0])).clone();
        }      
    }else{
        panic!("Wrong step! {}",query.pointer.to_string());
    }
    m
}
fn get_a<'a>(ind:&'a Index<String>,n:&usize)->&'a Index<String>{
    if *n>0{
        if ind.num_type{
            get_a(&ind.next[0],n)
        }else{
            get_a(&ind.next[0],&(n-1))
        }
    }else{
        &ind
    }
}
//--------------------end_for_generslization--------------------//
