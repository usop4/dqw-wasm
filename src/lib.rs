pub mod monster;
pub mod job;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::monster::*;
use crate::job::*;

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen(start)]
pub fn initialize(){
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
pub fn return_all_combis3_csv(monsters: &str, options: &JsValue) -> JsValue {

    time("return_all_combis3_csv");

    let o: Option = options.into_serde().unwrap();

    let mut m = MonstersLite::new();
    let mut m_full = Monsters::new();
    let mut r = csv::ReaderBuilder::new().delimiter(b',').from_reader(monsters.as_bytes());

    let param_order:usize = match &*o.param {
        "cost" => 1,
        "hp" => 3,
        "mp" => 4,
        "power" => 5,
        "defense" => 6,
        "attack" => 7,
        "recover" => 8,
        "speed" => 9,
        "skill" => 10,
        _ => 0,
    };

    let mut id :usize = 0;
    for record in r.records(){
        let record = record.unwrap();
        m.add_monster(
            MonsterLite {
                id: id,
                cost: record[1].parse().unwrap(),
                color: color_to_num(&record[2]),
                val: record[param_order].parse().unwrap(),
            }
        );
        m_full.add_monster(
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

    log(&format!("monster:{:?}",id-1));

    let mut combis_lite = CombisLite::new();
    let mut combi_lite = CombiLite::new();

    let mut remove_list : Vec<usize> = Vec::new();

    let colors = return_color_from_job(&o.job);
    let color: Vec<&str> = colors.split(',').collect();

    let list1 = make_num_array_from_monsters_lite(&m);

    for i1 in &list1 {
        let mut counter = 0;
        loop {
            remove_list.push(counter);
            if counter == *i1 {
                break;
            }
            counter +=  1;
        }

        let mut list2 = make_num_array_from_color_lite(&m,color[0]);
        list2 = remove_array(list2,(*remove_list).to_vec());
        for i2 in &list2 {
            remove_list.push(*i2);
            let mut list3 = make_num_array_from_color_lite(&m,color[1]);
            list3 = remove_array(list3,(*remove_list).to_vec());    
            for i3 in &list3 {
                remove_list.push(*i3);
                if color.get(2) != None {
                    let mut list4 = make_num_array_from_color_lite(&m,color[2]);
                    list4 = remove_array(list4,(*remove_list).to_vec());
                    for i4 in &list4 {

                        combi_lite.add_monster(m.monsters[*i1]);
                        combi_lite.add_monster(m.monsters[*i2]);
                        combi_lite.add_monster(m.monsters[*i3]);
                        combi_lite.add_monster(m.monsters[*i4]);
                        if combi_lite.cost < o.cost {
                            combis_lite.add_combi(combi_lite.clone());
                        }
                        combi_lite.clear();    
                    }
                }else{
                    combi_lite.add_monster(m.monsters[*i1]);
                    combi_lite.add_monster(m.monsters[*i2]);
                    combi_lite.add_monster(m.monsters[*i3]);    
                    if combi_lite.cost < o.cost {
                        combis_lite.add_combi(combi_lite.clone());
                    }
                    combi_lite.clear();    
                }
            }
            combi_lite.add_monster(m.monsters[*i1]);
            combi_lite.add_monster(m.monsters[*i2]);
            if combi_lite.cost < o.cost {
                combis_lite.add_combi(combi_lite.clone());
            }
            combi_lite.clear();    
        }
        remove_list.clear();
    }

    let mut max = Vec::new();
    for c in &combis_lite.combis {
        max.push(c.val)
    }

    log(&format!("size:{:?}",max.len()));

    max.sort();
    max.dedup();
    max.reverse();

    let mut out = Combis::new();
    let mut combi = Combi::new();

    let mut count = 0;
    let mut remove_flag = 0;
    let removes: Vec<&str> = o.remove.split(' ').collect();


    'outer: for i in 0..max.len() {
        for c in &combis_lite.combis {
            if c.val == max[i] {
                for id in c.monsters.clone() {
                    combi.add_monster(m_full.monsters[id].clone());
                }
                if o.remove.len() > 0 {
                    remove_flag = 0;
                    for remove in &removes {     
                        if combi.name.contains(&*remove) {
                            remove_flag = 1;
                        }
                    }
                    if remove_flag == 0 {
                        combi.compress_effects();
                        out.add_combi(combi.clone());
                        count += 1;
                    }
                }else{
                    combi.compress_effects();
                    out.add_combi(combi.clone());
                    count += 1;
                }
                combi.clear();
            }
            if count >= o.combis_size {
                break 'outer;
            }
        }
    }
    
    timeEnd("return_all_combis3_csv");
    JsValue::from_serde(&out).unwrap()

}

#[wasm_bindgen]
pub fn return_all_combis2_csv(monsters: &str, options: &JsValue) -> JsValue {

    time("return_all_combis2_csv");

    let o: Option = options.into_serde().unwrap();

    let mut m = Monsters::new();    
    let mut r = csv::ReaderBuilder::new().delimiter(b',').from_reader(monsters.as_bytes());

    let mut id :usize = 0;
    for record in r.records(){
        let record = record.unwrap();
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

    log(&format!("monster:{:?}",id-1));

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

    log(&format!("size:{:?}",max.len()));

    max.sort();
    max.dedup();
    max.reverse();

    let mut count = 0;
    let mut remove_flag = 0;
    let removes: Vec<&str> = o.remove.split(' ').collect();

    'outer: for i in 0..max.len() {
        for c in combis.combis.clone() {
            let mut c_clone = c.clone();
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
                if o.remove.len() > 0 {
                    for remove in &removes {     
                        if c.name.contains(&*remove) {
                            remove_flag = 1;
                        }
                    }
                    if remove_flag == 0 {
                        c_clone.compress_effects();
                        out.add_combi(c_clone);
                        count += 1;
                    }
                    remove_flag = 0;
                }else{
                    c_clone.compress_effects();
                    out.add_combi(c_clone);
                    count += 1;
                }
            }
            if count >= o.combis_size {
                break 'outer;
            }
        }
    }
    timeEnd("return_all_combis2_csv");
    JsValue::from_serde(&out).unwrap()
}

#[wasm_bindgen]
extern {
    pub fn log(s: &str);
}

#[wasm_bindgen]
extern {
    pub fn time(s: &str);
}

#[wasm_bindgen]
extern {
    pub fn timeEnd(s: &str);
}


#[cfg(test)]
mod tests {
    use crate::monster::*;
    use crate::job::*;

    #[test]
    fn test1() {
        let m1:MonsterLite = MonsterLite{id:1,cost:1,color:1,val:1};
        let m2:MonsterLite = MonsterLite{id:2,cost:1,color:1,val:1};
        let mut c:CombiLite = CombiLite::new();
        c.add_monster(m1);
        c.add_monster(m2);
        assert_eq!(c.monsters[0],1);
    }

    #[test]
    fn test2() {
        let m1:MonsterLite = MonsterLite{id:1,cost:1,color:1,val:1};
        let m2:MonsterLite = MonsterLite{id:2,cost:1,color:1,val:1};
        let mut m:MonstersLite = MonstersLite::new();
        m.add_monster(m1);
        m.add_monster(m2);
        assert_eq!(m.monsters[0].id,1);
    }

    #[test]
    fn test3() {
        let m1:MonsterLite = MonsterLite{id:1,cost:1,color:1,val:1};
        let m2:MonsterLite = MonsterLite{id:2,cost:1,color:1,val:1};
        let mut m:MonstersLite = MonstersLite::new();
        m.add_monster(m1);
        m.add_monster(m2);
        assert_eq!(make_num_array_from_color_lite(&m,"黄")[0],1);
    }


}
