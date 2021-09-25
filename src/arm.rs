
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Arm {
    pub id: usize,
    pub parts: String,
    pub name: String,
    pub desc: String,
}

impl Arm{
    pub fn new(id: usize) -> Self {
        Arm {
            id: id,
            parts: String::new(),
            name: String::new(),
            desc: String::new(),
        }
    }

    pub fn weight(&mut self, seed: Vec<String>, job: String) -> usize {
        let mut weight = 0;
        let src = &self.desc;
        let mut map = HashMap::<String, usize>::new();    
        for s in src.split(" "){
            let kv: Vec<&str> = s.split("+").collect();
            let key = kv[0].to_string();
            if kv.len() != 1 {
                *map.entry(key).or_insert(0) += kv[1].replace("%","").parse().unwrap_or(0);
            }        
        }
        for t in seed{
            if map.contains_key(&t) {
                weight = weight + map[&t];
            }
            if job.len() > 0 {
                let k = &format!("{}{}",job,t);
                if map.contains_key(k) {
                    weight = weight + map[k];
                }
            }
        }
        weight
    }    

}

#[derive(Clone, Serialize, Deserialize)]
pub struct Arms {
    pub arms: Vec<Arm>
}

impl Arms {
    pub fn new() -> Self {
        Arms {
            arms: Vec::new()
        }
    }
    pub fn add_arm(&mut self, a: Arm){
        self.arms.push(a);
    }

    pub fn pickup_parts(&mut self, parts: &str) -> Arms {
        let mut out = Arms::new();
        for a in &self.arms {
            if parts == a.parts {
                out.arms.push(a.clone());
            }
        }
        out
    }

}
