use serde::{Serialize, Deserialize};

use std::collections::HashMap;

/*
高速計算用の軽量版structと組み合わせ計算
*/

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct MonsterLite {
    pub id: usize,
    pub cost: usize,
    pub color: usize,
    pub val: usize,
}

impl MonsterLite{
    pub fn new(id: usize) -> Self {
        MonsterLite {
            id: id,
            cost: 0,
            color: 0,
            val: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MonstersLite {
    pub monsters: Vec<MonsterLite>
}

impl MonstersLite {
    pub fn new() -> Self {
        MonstersLite {
            monsters: Vec::new()
        }
    }
    pub fn add_monster(&mut self, m: MonsterLite){
        self.monsters.push(m);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CombiLite {
    pub monsters: Vec<usize>,
    pub cost: usize,
    pub val: usize,
}

impl CombiLite{
    pub fn new() -> Self {
        CombiLite {
            monsters:   Vec::new(),
            cost:       0,
            val:        0,
        }
    }
    pub fn clear(&mut self) {
        self.monsters = Vec::new();
        self.cost = 0;
        self.val = 0;
    }

    pub fn add_monster(&mut self, m: MonsterLite){
        self.monsters.push(m.id);        
        self.cost = self.cost + m.cost;
        self.val = self.val + m.val;
    }
    
}

#[derive(Serialize, Deserialize)]
pub struct CombisLite {
    pub combis: Vec<CombiLite>,
    pub num: usize
}

impl CombisLite {
    pub fn new() -> Self {
        CombisLite {
            combis: Vec::new(),
            num : 0
        }
    }
    pub fn add_combi(&mut self, c: CombiLite){
        self.combis.push(c);
        self.num = self.combis.len();
    }
}

pub fn make_num_array_from_monsters_lite(m: &MonstersLite) -> Vec<usize>{
    let mut out: Vec<usize> = Vec::new();
    for monster in &m.monsters[..]{
        out.push(monster.id);
    }
    out
}

// usage make_num_array_from_color_lite(monstes,"黄赤")
pub fn make_num_array_from_color_lite(m: &MonstersLite, color: &str) -> Vec<usize>{
    let mut color_list = MonstersLite::new();
    for monster in &m.monsters[..] {
        if monster.color == 1 && color.contains("黄"){
            color_list.add_monster(monster.clone());
        }
        if monster.color == 2 && color.contains("赤"){
            color_list.add_monster(monster.clone());
        }
        if monster.color == 4 && color.contains("青"){
            color_list.add_monster(monster.clone());
        }
        if monster.color == 8 && color.contains("紫"){
            color_list.add_monster(monster.clone());
        }
        if monster.color == 16 && color.contains("緑"){
            color_list.add_monster(monster.clone());
        }
    }
    make_num_array_from_monsters_lite(&color_list)
}

/*
最終出力用のstruct
*/

#[derive(Clone, Serialize, Deserialize)]
pub struct Monster {
    pub id: usize,
    pub name: String,
    pub cost: usize,
    pub color: String,
    pub hp: usize,
    pub mp: usize,
    pub power: usize,
    pub defense: usize,
    pub attack: usize,
    pub recover: usize,
    pub speed: usize,
    pub skill: usize,
    pub effects: String,
}

impl Monster{    
    pub fn new(id: usize) -> Self {
        Monster {
            id: id,
            name: String::new(),
            cost: 0,
            color: String::new(),
            hp: 0,
            mp: 0,
            power: 0,
            defense: 0,
            attack: 0,
            recover: 0,
            speed: 0,
            skill: 0,
            effects: String::new(),
        }
    }
    pub fn set_monster(&mut self, name: String, cost: usize,){
        self.name = name;
        self.cost = cost;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Monsters {
    pub monsters: Vec<Monster>
}

impl Monsters {
    pub fn new() -> Self {
        Monsters {
            monsters: Vec::new()
        }
    }
    pub fn add_monster(&mut self, m: Monster){
        self.monsters.push(m);
    }

    pub fn ret_monster(&mut self,i: usize) -> &mut String {
        &mut self.monsters[i].name
    }

}

#[derive(Clone, Serialize, Deserialize)]
pub struct Combi {
    pub name: String,
    pub cost: usize,
    pub hp: usize,
    pub mp: usize,
    pub power: usize,
    pub defense: usize,
    pub attack: usize,
    pub recover: usize,
    pub speed: usize,
    pub skill: usize,
    pub effects: String,
}

impl Combi{
    pub fn new() -> Self {
        Combi {
            name:   String::new(),
            cost:    0,
            hp:      0,
            mp:      0,
            power:   0,
            defense: 0,
            attack:  0,
            recover: 0,
            speed:   0,
            skill:   0,
            effects: String::new(),    
        }
    }
    pub fn clear(&mut self) {
        self.name = String::new();
        self.cost = 0;
        self.hp = 0;
        self.mp = 0;
        self.power = 0;
        self.defense = 0;
        self.attack = 0;
        self.recover = 0;
        self.speed = 0;
        self.skill = 0;
        self.effects = String::new();
    }

    pub fn add_monster(&mut self, m: Monster){
        
        if self.name.len() == 0 {
            self.name = format!("{}({}{})",&m.name,&m.color,&m.cost);
        }else{
            self.name = format!("{}\r\n{}({}{})",&self.name,&m.name,&m.color,&m.cost);
        }
        self.cost = self.cost + m.cost;
        self.hp = self.hp + m.hp;
        self.mp = self.mp + m.mp;
        self.power = self.power + m.power;
        self.defense = self.defense + m.defense;
        self.attack = self.attack + m.attack;
        self.recover = self.recover + m.recover;
        self.speed = self.speed + m.speed;
        self.skill = self.skill + m.skill;

        if m.effects.len() > 1 {
            if self.effects.len() == 0 {
                self.effects = m.effects;
            }else{
                self.effects = format!("{} {}",&self.effects,&m.effects);
            }
        }
    }

    pub fn compress_effects(&mut self){
        let src = &self.effects;
        let mut map = HashMap::<String, usize>::new();    
        let mut out = "".to_string();
        for s in src.split(" "){
            let kv: Vec<&str> = s.split("+").collect();
            let key = kv[0].to_string();
            if kv.len() == 1 {                
                if key.len() > 0 {
                    if out.len() == 0 {
                        out = key;
                    }else{
                        out = format!("{}\r\n{}",&out,&key);
                    }
                }
            }else{
                *map.entry(key).or_insert(0) += kv[1].replace("%","").parse().unwrap_or(0);
            }        
        }
        for(k,v) in &map {
            if out.len() == 0 {
                out = format!("{}+{:?}",k,v);
            }else{
                out = format!("{}\r\n{}",&out,&format!("{}+{:?}",k,v));
            }
        }
        self.effects = out;
    }
    
}

#[derive(Serialize, Deserialize)]
pub struct Combis {
    pub combis: Vec<Combi>,
    pub num: usize
}

impl Combis {
    pub fn new() -> Self {
        Combis {
            combis: Vec::new(),
            num : 0
        }
    }
    pub fn add_combi(&mut self, c: Combi){
        self.combis.push(c);
        self.num = self.combis.len();
    }
}

pub fn make_num_array_from_monsters(m: &Monsters) -> Vec<usize>{
    let mut out: Vec<usize> = Vec::new();
    for monster in &m.monsters[..]{
        out.push(monster.id);
    }
    out
}

// usage make_num_array_from_color(monstes,"黄")
pub fn make_num_array_from_color(m: &Monsters, color: &str) -> Vec<usize>{
    let mut color_list = Monsters::new();
    for monster in &m.monsters[..] {
        if monster.color == color {
            color_list.add_monster(monster.clone());
        }
    }
    make_num_array_from_monsters(&color_list)
}

// usage make_num_array_from_color(monstes,"黄赤")
pub fn make_num_array_from_color2(m: &Monsters, color: &str) -> Vec<usize>{
    let mut color_list = Monsters::new();
    for monster in &m.monsters[..] {
        if monster.color == color {
            color_list.add_monster(monster.clone());
        }
        match color.chars().nth(1) {
            Some(result) => {
                if monster.color == result.to_string() {
                    color_list.add_monster(monster.clone());
                }
            },
            None => {
            }
        }
    }
    make_num_array_from_monsters(&color_list)
}

pub fn remove_array(array: Vec<usize>, remove_list: Vec<usize>) -> Vec<usize>{
    let mut out = Vec::new();
    let mut flag: bool;
    for i in &array {
        flag = true;
        for j in &remove_list {
            if i == j {
                flag = false;
            }
        }
        if flag {
            out.push(*i);
        }
    }
    out
}

