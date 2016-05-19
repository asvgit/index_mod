pub struct Index {
    pub next: Vec<Index>,
    pub node_ind: usize,
    pub pointer: String,
    pub data: Vec<usize>
}
impl Index {
    pub fn start()-> Index {
        Index{
            next:vec![],
            node_ind:0,
            pointer: "&".to_string(),
            data:vec![]
        }
    }
    pub fn new(n:usize,p:String)->Index{
        Index{
            next:vec![],
            node_ind:n,
            pointer: p,
            data:vec![]
        }
    }    
    pub fn has_sach(&self,t:&String,i:&usize)->usize{
        for n in &self.next {
            if (n.pointer.as_str() == t) && (n.node_ind == *i) {return n.node_ind.clone()}
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
    pub fn to_string(&self)->String{  
        let mut s=self.node_ind.to_string();
        s.push_str(" ");
        s.push_str(self.pointer.as_str());
        s.push_str("\n");
        if self.next.len()>0{
            s.push_str("root_\n");
            for n in &self.next{
                s.push_str("");
                s.push_str(n.to_string_next("".to_string()).as_str());
            }
        }
        s.pop();
        s
    }
    fn to_string_next(&self,t:String)->String{  
        let mut t = t;
        t.push_str("\t");
        let mut s=t.clone();
        s.push_str(self.node_ind.to_string().as_str());
        s.push_str(" ");
        s.push_str(self.pointer.as_str());
        s.push_str("\n");
        if self.next.len()>0{
            /*s.push_str(t.as_str());
            s.push_str("consists!\n");*/
            for n in &self.next{
                s.push_str(n.to_string_next(t.to_string()).as_str());
            }
        }
        s
    }
    pub fn to_string_t(&self)->String{  
        let mut s=self.node_ind.to_string();
        s.push_str(" ");
        s.push_str(self.pointer.as_str());        
        for i in &self.data {
            s.push_str(" ");
            s.push_str(&i.to_string());
        }
        s.push_str("\n");
        if self.next.len()>0{
            s.push_str("root_\n");
            for n in &self.next{
                s.push_str("");
                s.push_str(n.to_string_next_t("".to_string()).as_str());
            }
        }
        s.pop();
        s
    }
    fn to_string_next_t(&self,t:String)->String{  
        let mut t = t;
        t.push_str("\t");
        let mut s=t.clone();
        s.push_str(self.node_ind.to_string().as_str());
        s.push_str(" ");
        s.push_str(self.pointer.as_str());
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
    pub fn add_new(&mut self,p:String){
        let n = self.next.len()+1;
        self.next.push(Index::new(n,p));
    }  
    pub fn add_new_t(&mut self,p:String,t:usize,n:&usize){
        self.next.push(Index::new_t(n.clone(),p,t));
    }     
    fn new_t(n:usize,s:String,title:usize)->Index{
        Index{
            next:vec![],
            node_ind:n,
            pointer: s,
            data:vec![title]
        }
    } 
    pub fn make_ptree(&mut self,s: &String){
        let mut s=s.clone();
        making(&mut s,self);
        //println!("{}",self.to_string());
    }
}

fn making(s:&mut String,ind:&mut Index){
    if s.len()>0{
        if s.chars().nth(0).unwrap()==',' {
            s.remove(0); 
        }
        if s.chars().nth(0).unwrap()=='('{
            let x = ind.next.len()-1; 
            let ind  =  &mut ind.next[x];
            s.remove(0);
            making(s,ind);
        }else if s.chars().nth(0).unwrap()==')'{  
            s.remove(0);                 
            return; 
        }else if s.chars().nth(0).unwrap()!=','{
            ind.add_new(s.chars().nth(0).unwrap().to_string());
            s.remove(0);       
        } 
        making(s,ind);
    }
}

pub struct Base {
    main_ind: Index,
    n_terms: usize
}

impl Base{
    pub fn new()->Base{
        Base{
            main_ind: Index::start(),
            n_terms: 0usize
        }
    }  
    pub fn add_term(&mut self,s:&String){
        let mut s=s.clone();
        self.n_terms+=1;
        addition(&mut s,&mut self.main_ind,&self.n_terms,&mut 1);
        //println!("{}",self.main_ind.to_string_t());
    }
    pub fn to_string(&self)->String{
        self.main_ind.to_string_t()        
    }
    pub fn del_term(&mut self,i:&usize){
        for ind in &mut self.main_ind.next{
            deletion(ind,i);
        }
    }
    pub fn inst(&self,quest:&Index)->Vec<usize>{
        institution(&self.main_ind,quest)
    }
}
fn addition(s:&mut String,ind:&mut Index,n:&usize,i:&mut usize){
    if s.len()>0{
        if s.chars().nth(0).unwrap()==','{
            s.remove(0); 
        }
        if s.chars().nth(0).unwrap()=='('{
            let x = *i-2; 
            let ind  =  &mut ind.next[x];
            s.remove(0);
            addition(s,ind,n,&mut 1);
        }else if s.chars().nth(0).unwrap()==')'{  
            s.remove(0);                
            return; 
        }else if s.chars().nth(0).unwrap()!=','{
            let f = ind.has_sach(&s.chars().nth(0).unwrap().to_string(),i);
            if f==0{
                ind.add_new_t(s.chars().nth(0).unwrap().to_string(),n.clone(),i);
            }else{
                let f=f-1;
                ind.next[f].data.push(n.clone());
            }
            s.remove(0);  
            *i+=1     
        } 
        addition(s,ind,n,i);
    }
}
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
fn institution(base:&Index,quest:&Index)->Vec<usize>{
        let mut m:Vec<usize>=vec![];
        if quest.pointer=="*"{
            m=base.data.clone();
        }else{
            /*for b in base.next{
                if b.next.len()==0{
                    
                }
            }*/
        }
        m
}

pub fn hello()-> String {
    "Hello".to_string()
}