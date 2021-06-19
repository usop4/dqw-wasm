/*
https://qiita.com/jp_ibis/items/3205b4799cb567f8ebf5
https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html?highlight=JsValue,into#receive-it-from-javascript-with-jsvalueinto_serde
*/

pub mod monster;
pub mod job;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::monster::*;
use crate::job::*;

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen(start)]
pub fn initialize() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Option {
    pub cost: usize,
    pub job: String,
    pub param: String,
    pub remove: String,
    pub combis_size: usize
}

#[wasm_bindgen]
pub fn return_all_combis2_csv(monsters: &str, options: &JsValue) -> JsValue {

    log("return_all_combis2_csv");
    let o: Option = options.into_serde().unwrap();
    let mut m = Monsters::new();    
    let mut r = csv::ReaderBuilder::new().delimiter(b',').from_reader(monsters.as_bytes());

    let mut id :usize = 0;
    for record in r.records(){
        let record = record.unwrap();
        log(&record[0]);
        m.add_monster(
            Monster {
                id: id,
                name: record[0].to_string(),
                cost: record[1].parse().unwrap(),
                color: record[2].to_string(),
                hp: record[3].parse().unwrap(),
                mp: record[4].parse().unwrap(),
                power: record[5].parse().unwrap(),
                defense: record[6].parse().unwrap(),
                attack: record[7].parse().unwrap(),
                recover: record[8].parse().unwrap(),
                speed: record[9].parse().unwrap(),
                skill: record[10].parse().unwrap(),
                effects: record[11].to_string(),
            }
        );
        id = id + 1;
    }

    log("csv loaded");

    let mut combis = Combis::new();
    let mut combi = Combi::new();

    let colors = return_color_from_job(&o.job);
    let color: Vec<&str> = colors.split(',').collect();

    let mut remove_list : Vec<usize> = Vec::new();
    let list1 = make_num_array_from_monsters(&m);
    for i1 in &list1 {
        let mut counter = 0;
        loop {
            remove_list.push(counter);
            if counter == *i1 {
                break;
            }
            counter +=  1;
        }
        //remove_list.push(*i1);
        let mut list2 = make_num_array_from_color2(&m,color[0]);
        list2 = remove_array(list2,(*remove_list).to_vec());
        for i2 in &list2 {
            remove_list.push(*i2);
            let mut list3 = make_num_array_from_color2(&m,color[1]);
            list3 = remove_array(list3,(*remove_list).to_vec());    
            for i3 in &list3 {
                remove_list.push(*i3);
                if color.get(2) != None {
                    let mut list4 = make_num_array_from_color2(&m,color[2]);
                    list4 = remove_array(list4,(*remove_list).to_vec());
                    for i4 in &list4 {
                        combi.add_monster(m.monsters[*i1].clone());
                        combi.add_monster(m.monsters[*i2].clone());
                        combi.add_monster(m.monsters[*i3].clone());
                        combi.add_monster(m.monsters[*i4].clone());
                        if combi.cost < o.cost {
                            combis.add_combi(combi.clone());
                        }
                        combi.clear();    
                    }
                }else{
                    combi.add_monster(m.monsters[*i1].clone());
                    combi.add_monster(m.monsters[*i2].clone());
                    combi.add_monster(m.monsters[*i3].clone());    
                    if combi.cost < o.cost {
                        combis.add_combi(combi.clone());
                    }
                    combi.clear();    
                }
                combi.add_monster(m.monsters[*i1].clone());
                combi.add_monster(m.monsters[*i2].clone());
                combi.add_monster(m.monsters[*i3].clone());
                if combi.cost < o.cost {
                    combis.add_combi(combi.clone());
                }
                combi.clear();    
            }
            combi.add_monster(m.monsters[*i1].clone());
            combi.add_monster(m.monsters[*i2].clone());
            if combi.cost < o.cost {
                combis.add_combi(combi.clone());
            }
            combi.clear();    
        }
        remove_list.clear();
    }

    log("combi generated");

    let mut out = Combis::new();
    let mut max = Vec::new();
    for c in &combis.combis {
        match &*o.param {
            "cost" => max.push(c.cost),
            "hp" => max.push(c.hp),
            "mp" => max.push(c.mp),
            "power" => max.push(c.power),
            "defense" => max.push(c.defense),
            "attack" => max.push(c.attack),
            "recover" => max.push(c.recover),
            "speed" => max.push(c.speed),
            "skill" => max.push(c.skill),
            _ => ()
        }
    }

    max.sort();
    max.dedup();
    max.reverse();

    let mut count = 0;

    'outer: for i in 0..max.len() {
        for c in combis.combis.clone() {
            let cval: usize;
            match &*o.param {
                "cost" => cval = c.cost,
                "hp" => cval = c.hp,
                "mp" => cval = c.mp,
                "power" => cval = c.power,
                "defense" => cval = c.defense,
                "attack" => cval = c.attack,
                "recover" => cval = c.recover,
                "speed" => cval = c.speed,
                "skill" => cval = c.skill,
                _ => cval = 0
            }
            if cval == max[i] {
                //log(&c.name);
                if o.remove.len() > 0 {
                    if !c.name.contains(&*o.remove) {
                        out.add_combi(c);
                        count += 1;
                    }    
                }else{
                    out.add_combi(c);
                    count += 1;
                }
            }
            if count >= o.combis_size {
                break 'outer;
            }
        }
    }
    JsValue::from_serde(&out).unwrap()
}

#[wasm_bindgen]
extern {
    pub fn log(s: &str);
}

#[cfg(test)]
mod tests {
    use crate::monster::*;
    use std::collections::HashMap;

    #[test]
    fn test_hashmap() {
        let mut hash_map: HashMap<&str, i32> = HashMap::new();
        hash_map.insert("nju", 33);        
        assert_eq!(hash_map["nju"], 33);
        }


    #[test]
    fn test_two_color_separate() {
        let s = "黄赤";
        let c = s.chars().nth(1);
        assert_eq!(c,Some('赤'));
    }

    #[test]
    fn test_monster_new_and_set() {
        let mut m = Monster::new(1);
        m.set_monster("test".to_string(),100);
        assert_eq!(&m.name,"test");
    }


}
