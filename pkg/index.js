import * as mod from "./dqw_wasm.js";

(async () => {
    await mod.default();
    var app = new Vue({
      el: '#app',
      data: {
        monsters: null,
        combis: null,
        job: "賢者",
        cost: 400,
        param: "hp",
        remove: "",
        combis_size: 5,
        csv: null,
        remove_monster_0: null,
        remove_monster_1: null,
        remove_monster_2: null,
        remove_monster_3: null,
        auto_update_checkbox: "accepted"
      },
      mounted: function(){
        axios.get("./monster.csv").then(
          response => ( this.csv = response.data )
        );
      },
      computed:{
      },
      methods:{
        list_combis_auto: function(){
          if(this.auto_update_checkbox){
            return this.list_combis();
          }
        },
        list_combis: function(){
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
          return s.combis;
        },
         add_cost: function(n){
          this.cost = parseInt(this.cost) + parseInt(n);
        },
        set_cost: function(n){
          this.cost = parseInt(n);
        },
        set_combis_size: function(n){
          this.combis_size = n;
        },
        show_combis: function(n){
          let remove_monsters = this.combis[n].name.split('\r\n');
          this.remove_monster_0 = remove_monsters[0];
          this.remove_monster_1 = remove_monsters[1];
          this.remove_monster_2 = remove_monsters[2];
          this.remove_monster_3 = remove_monsters[3];
          this.$bvModal.show("modal-1");
        },
        remove_monster: function(n){
          if(n==0){this.remove = this.remove_monster_0.split('(')[0];}
          if(n==1){this.remove = this.remove_monster_1.split('(')[0];}
          if(n==2){this.remove = this.remove_monster_2.split('(')[0];}
          if(n==3){this.remove = this.remove_monster_3.split('(')[0];}
          this.$bvModal.hide("modal-1");
        },
        csvtest: function(event){
          console.log(this.csv);
          mod.csv_test(this.csv);
        },
      }
    })
})();


