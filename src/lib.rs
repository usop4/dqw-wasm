pub mod monster;
pub mod job;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::monster::*;
use crate::job::*;

extern crate console_error_panic_hook;
use std::panic;

use kmeans::*;
extern crate rand;

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

    log("return_all_combis");

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
                        out.add_combi(c);
                        count += 1;
                    }
                    remove_flag = 0;
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
    log("finish");
    JsValue::from_serde(&out).unwrap()
}

#[wasm_bindgen]
pub fn kmean_test(data: &str) {

    log("kmean_test");

    let k = 16;
    let max_iter = 10;

    let sample_dims:usize = 8;
    let mut sample_cnt:usize = 0;

    let mut m = Monsters::new();
    let mut r = csv::ReaderBuilder::new().delimiter(b',')
        .has_headers(true)
        .from_reader(data.as_bytes());

    let mut samples: Vec<f64> = Vec::new();

    for record in r.records(){
        let record = record.unwrap();
        m.add_monster(
            Monster {
                id: sample_cnt,
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
        log(&record[0].to_string());
        for i in 3..11 {
            samples.push(record[i].parse().unwrap());
        }
        sample_cnt = sample_cnt + 1;
    }

    log(&format!("sample_cnt: {:?}", sample_cnt));
    log(&format!("sample_dims: {:?}", sample_dims));
    log(&format!("samples.len(): {:?}", samples.len()));

    let kmean = KMeans::new(samples, sample_cnt, sample_dims);
    log("kmeans new");

    //let result = kmean.kmeans_lloyd(k, max_iter, KMeans::init_random_sample, &KMeansConfig::default());
    let result = kmean.kmeans_lloyd(k, max_iter, KMeans::init_kmeanplusplus, &KMeansConfig::default());
    log("kmeans lloyd");

    for i in 0..sample_cnt {
        log(&format!("{},{:?}",m.ret_monster(i),result.assignments[i]));
    }

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
