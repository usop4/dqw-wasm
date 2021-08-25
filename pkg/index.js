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
      }
    })
})();
