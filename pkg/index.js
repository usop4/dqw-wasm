import * as mod from "./dqw_wasm.js";

(async () => {
    await mod.default();
    var app = new Vue({
      el: '#app',
      data: {
        monsters: null,
        combis: null,
        job: "戦士",
        cost: 400,
        param: "hp",
        remove: "",
        combis_size: 5,
        csv: null,
        monster_list: [],
        show: false,
      },
      mounted: function(){
        axios.get("./monster.csv").then(
          response => ( this.csv = response.data )
        );
      },
      computed:{
      },
      watch: {
        show: function(){
          console.log(this.show);
        }
      },
      methods:{
        list_combis_auto: function(){
          return this.combis;
        },
        list_combis2: function(){
          var param = {
            cost: parseInt(this.cost),
            job: this.job,
            param: this.param,
            remove: this.remove,
            combis_size: parseInt(this.combis_size)
          };
          let s = mod.return_all_combis2_csv(
            this.csv,
            param
          );
          this.combis = s.combis;
        },
        list_combis3: function(){
          var param = {
            cost: parseInt(this.cost),
            job: this.job,
            param: this.param,
            remove: this.remove,
            combis_size: parseInt(this.combis_size)
          };
          let s = mod.return_all_combis3_csv(
            this.csv,
            param
          );
          this.combis = s.combis;
        },
        show_combis: function(n){
          this.monster_list = this.combis[n].name.split('\r\n').map(value => value);
          this.$bvModal.show("modal-1");
        },
        remove_monster: function(n){
          if( this.remove.length ){
            this.remove = this.remove + ' ' + this.monster_list[n].split('(')[0];
          }else{
            this.remove = this.monster_list[n].split('(')[0];
          }
          this.$bvModal.hide("modal-1");
        },
        // ここから性能比較用JavaScript実装
        list_combis_js: function(){
          console.time("list_combis_js");
          var param = {
            cost: parseInt(this.cost),
            job: this.job,
            param: this.param,
            remove: this.remove,
            combis_size: parseInt(this.combis_size)
          };
          var monsters = [];
          var combis = [];

          var lines = this.csv.split("\n");
          var id = 0;
          for(var i=1;i<lines.length;i++){
            var temp = lines[i].split(",");
            monsters.push({
              id:       id,
              name:     temp[0],
              cost:     parseInt(temp[1]),
              color:    temp[2],
              hp:       parseInt(temp[3]),
              mp:       parseInt(temp[4]),
              power:    parseInt(temp[5]),
              defense:  parseInt(temp[6]),
              attack:   parseInt(temp[7]),
              recover:  parseInt(temp[8]),
              speed:    parseInt(temp[9]),
              skill:    parseInt(temp[10]),
              special:  temp[11],
            });
            id++;
          }
          console.log("monster:"+String(id));

          var list1 = make_num_array(monsters,null);
          list1.forEach(function(l1){
            var remove_list = [];
            for(var i=0;i<l1;i++){
              remove_list.push(l1);
            }
            var list2 = make_num_array(monsters,job_color(param.job,1));
            list2 = list2.filter(a=>remove_list.indexOf(a)==-1);
            list2.forEach(function(l2){
              remove_list.push(l2);
              var list3 = make_num_array(monsters,job_color(param.job,2));
              list3 = list3.filter(a=>remove_list.indexOf(a)==-1);
              list3.forEach(function(l3){
                remove_list.push(l3);
                if( job_color(param.job,3) ){
                  var list4 = make_num_array(monsters,job_color(param.job,3));
                  list4 = list4.filter(a=>remove_list.indexOf(a)==-1);
                  list4.forEach(function(l4){
                    if( param.cost > monsters[l1].cost + monsters[l2].cost + monsters[l3].cost + monsters[l4].cost ){
                      combis.push({
                        name: monsters[l1].name + "\r\n"
                          + monsters[l2].name + "\r\n"
                          + monsters[l3].name + "\r\n"
                          + monsters[l4].name,
                        cost: monsters[l1].cost + monsters[l2].cost + monsters[l3].cost + monsters[l4].cost,
                        hp: monsters[l1].hp + monsters[l2].hp + monsters[l3].hp + monsters[l4].hp,
                        mp: monsters[l1].mp + monsters[l2].mp + monsters[l3].mp + monsters[l4].mp,
                        power: monsters[l1].power + monsters[l2].power + monsters[l3].power + monsters[l4].power,
                        defense: monsters[l1].defense + monsters[l2].defense + monsters[l3].defense + monsters[l4].defense,
                        attack: monsters[l1].attack + monsters[l2].attack + monsters[l3].attack + monsters[l4].attack,
                        recover: monsters[l1].recover + monsters[l2].recover + monsters[l3].recover + monsters[l4].recover,
                        speed: monsters[l1].speed + monsters[l2].speed + monsters[l3].speed + monsters[l4].speed,
                        skill: monsters[l1].skill + monsters[l2].skill + monsters[l3].skill + monsters[l4].skill,
                        special: monsters[l1].special + " "
                          + monsters[l2].special + " "
                          + monsters[l3].special + " "
                          + monsters[l4].special,
                      });
                    }
                  });
                }else{
                  if( param.cost > monsters[l1].cost + monsters[l2].cost + monsters[l3].cost ){
                    combis.push({
                      name: monsters[l1].name + "\r\n" 
                      + monsters[l2].name + "\r\n" 
                      + monsters[l3].name,
                      cost: monsters[l1].cost + monsters[l2].cost + monsters[l3].cost,
                      hp: monsters[l1].hp + monsters[l2].hp + monsters[l3].hp,
                      mp: monsters[l1].mp + monsters[l2].mp + monsters[l3].mp,
                      power: monsters[l1].power + monsters[l2].power + monsters[l3].power,
                      defense: monsters[l1].defense + monsters[l2].defense + monsters[l3].defense,
                      attack: monsters[l1].attack + monsters[l2].attack + monsters[l3].attack,
                      recover: monsters[l1].recover + monsters[l2].recover + monsters[l3].recover,
                      speed: monsters[l1].speed + monsters[l2].speed + monsters[l3].speed,
                      skill: monsters[l1].skill + monsters[l2].skill + monsters[l3].skill,
                      special: monsters[l1].special + " " 
                      + monsters[l2].special + " " 
                      + monsters[l3].special,
                    });  
                  }
                }  
              });
              if( param.cost > monsters[l1].cost + monsters[l2].cost ){
                combis.push({
                  name: monsters[l1].name + " " + monsters[l2].name,
                  cost: monsters[l1].cost + monsters[l2].cost,
                  hp: monsters[l1].hp + monsters[l2].hp,
                });
              }
            });
          });
          console.log(combis.length);

          var max = [];
          combis.forEach(function(c){
            if(param.param=="hp"){max.push(c.hp);}
            if(param.param=="mp"){max.push(c.mp);}
            if(param.param=="power"){max.push(c.power);}
            if(param.param=="defense"){max.push(c.defense);}
            if(param.param=="attack"){max.push(c.attack);}
            if(param.param=="recover"){max.push(c.recover);}
            if(param.param=="speed"){max.push(c.speed);}
            if(param.param=="skill"){max.push(c.skill);}
          });

          max.sort(function(a, b){return b-a;});

          var out = [];
          max.forEach(function(t){
            combis.forEach(function(c){
              if( out.length < param.combis_size ){
                if(t==c[param.param]){
                  out.push(c);
                }  
              }  
            });
          });
          this.combis = out;
          console.timeEnd("list_combis_js");
        },
      } // ここまでmethods
    })
})();

function make_num_array(monsters,color=null){
  var array = [];
  monsters.forEach(function(m){
    if( color == null ){
      array.push(m.id);
    }else{
      if ( color.includes(m.color) ){
        array.push(m.id);
      }
    }
  });
  return array
}

function job_color(job,num){
  var colors = {
    "戦士":["黄","黄"],
    "魔法使い":["紫","紫"],
    "僧侶":["緑","緑"],
    "武闘家":["赤","赤"],
    "盗賊":["青","青"],
    "遊び人":["青","紫"],
    "踊り子":["青","緑"],
    "レンジャー":["青","青","青"],
    "賢者":["紫緑","紫緑","紫緑"],
    "バトルマスター":["黄赤","赤","赤"],
    "魔法戦士":["黄紫","黄紫","黄紫"],
    "パラディン":["黄緑","黄","黄"],
    "スパスタ":["青緑","青","緑"],
    "海賊":["黄青","黄","青"],
  }
  return colors[job][num-1];
}

