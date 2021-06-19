use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Attr {
    pub tokugi: usize,
    pub jumon: usize,
    pub taisei: usize, 
}

impl Attr{
    pub fn new() -> Self {
        Attr{
            tokugi: 0,
            jumon: 0,
            taisei: 0,    
        }
    }
    pub fn clear(&mut self){
        self.tokugi = 0;
        self.jumon = 0;
        self.taisei = 0;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Group {
    pub damage: usize,
    pub taisei: usize, 
}

impl Group{
    pub fn new() -> Self {
        Group{
            damage: 0,
            taisei: 0,    
        }
    }
    pub fn clear(&mut self){
        self.damage = 0;
        self.taisei = 0;
    }
}

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

    pub fn add_effects(&mut self, effects: &str){
        if self.effects.len() == 0 {
            self.effects = effects.replace(" ","\r\n").to_string();
        }else{
            self.effects = format!("{}\r\n{}",&self.effects,&effects.replace(" ","\r\n").to_string());
        }
    }

    pub fn add_monster(&mut self, m: Monster){
        
        if self.name.len() == 0 {
            //self.name = m.name;
            self.name = format!("{}({})",&m.name,&m.color);
        }else{
            //self.name = format!("{}\r\n{}",&self.name,&m.name);
            self.name = format!("{}\r\n{}({})",&self.name,&m.name,&m.color);
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

        self.add_effects(&m.effects);
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
