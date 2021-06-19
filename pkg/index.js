import * as mod from "./wasm.js";
(async () => {
    await mod.default();
    var app = new Vue({
      el: '#app',
      data: {
        monsters: null,
        combis: null,
        job: "賢者",
        cost: 380,
        param: "hp",
        remove: "",
        combis_size: 3,
        csv: null
      },
      mounted: function(){
        /*
        axios.get("./monster.json").then(
            response => ( this.monsters = response.data )
        );
        */
        axios.get("./monster.csv").then(
          response => ( this.csv = response.data )
        );
    },
      methods:{
        listCombisCsv: function(){
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
        set_cost: function(n){
          this.cost = parseInt(this.cost) + parseInt(n);
        },
        set_combis_size: function(n){
          this.combis_size = n;
        },
        csvtest: function(event){
          console.log(this.csv);
          mod.csv_test(this.csv);
        },
      }
    })
})();


