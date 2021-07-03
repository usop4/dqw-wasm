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
        auto_update_checkbox: "accepted",
        options: [null,null,null,null,null]
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
          if(this.options.slice(0,5).toString() == [this.cost,this.job,this.param,this.remove,this.combis_size].toString()){
            return this.combis;
          }else{
            let s = mod.return_all_combis2_csv(
              this.csv,
              param
            );
            this.combis = s.combis;
            this.options[0] = this.cost;
            this.options[1] = this.job;  
            this.options[2] = this.param;
            this.options[3] = this.remove;  
            this.options[4] = this.combis_size;
            return s.combis;
          }
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
        csvtest: function(event){
          console.log(this.csv);
          mod.csv_test(this.csv);
        },
      }
    })
})();


