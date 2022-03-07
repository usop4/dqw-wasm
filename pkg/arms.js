import * as mod from "./dqw_wasm.js";

(async () => {
    await mod.default();
    var app = new Vue({
      el: '#app',
      data: {
        csv: null,

        job1: 'パラディン',
        job2: '賢者',
        job3: '魔法戦士',
        job4: 'レンジャ',
        job_options: [
          {value: '',text:'未選択：職業'},
          {value: 'バトマス',text:'バトルマスター'},
          {value: '賢者',text:'賢者'},
          {value: 'レンジャ',text:'レンジャー'},
          {value: '魔法戦士',text:'魔法戦士'},
          {value: 'パラディン',text:'パラディン'},
          {value: 'スパスタ',text:'スーパースター'},
          {value: '海賊',text:'海賊'},
        ],        

        z_tokugi1: 'イオ',
        z_tokugi2: '',
        z_tokugi3: '',
        z_tokugi4: 'ヒャド',
        z_tokugi_options: [
          {value: '',text:'未選択：とくぎダメージ'},
          {value: 'メラ',text:'メラとくぎダメージ'},
          {value: 'ヒャド',text:'ヒャドとくぎダメージ'},
          {value: 'ギラ',text:'ギラとくぎダメージ'},
          {value: 'バギ',text:'バギとくぎダメージ'},
          {value: 'イオ',text:'イオとくぎダメージ'},
          {value: 'ドルマ',text:'ドルマとくぎダメージ'},
          {value: 'デイン',text:'デインとくぎダメージ'},
          {value: 'ジバリア',text:'ジバリアとくぎダメージ'},
        ],
        z_jumon1: '',
        z_jumon2: 'イオ',
        z_jumon3: 'メラ',
        z_jumon4: '',
        z_jumon_options: [
          {value: '',text:'未選択：じゅもんダメージ'},
          {value: 'メラ',text:'メラじゅもんダメージ'},
          {value: 'ギラ',text:'ギラじゅもんダメージ'},
          {value: 'ヒャド',text:'ヒャドじゅもんダメージ'},
          {value: 'バギ',text:'バギじゅもんダメージ'},
          {value: 'デイン',text:'デインじゅもんダメージ'},
          {value: 'ジバリア',text:'ジバリアじゅもんダメージ'},
          {value: 'イオ',text:'イオじゅもんダメージ'},
          {value: 'ドルマ',text:'ドルマじゅもんダメージ'},
        ],
        z_taisei1: '',
        z_taisei2: '',
        z_taisei3: '',
        z_taisei4: '',
        z_taisei_options: [
          {value: '',text:'未選択：耐性'},
          {value: 'メラ',text:'メラ耐性'},
          {value: 'ギラ',text:'ギラ耐性'},
          {value: 'ヒャド',text:'ヒャド耐性'},
          {value: 'バギ',text:'バギ耐性'},
          {value: 'デイン',text:'デイン耐性'},
          {value: 'ジバリア',text:'ジバリア耐性'},
          {value: 'イオ',text:'イオ耐性'},
          {value: 'ドルマ',text:'ドルマ耐性'},
        ],
        k_taisei1: '',
        k_taisei2: '',
        k_taisei3: '',
        k_taisei4: '',
        k_taisei_options: [
          {value: '',text:'未選択：耐性'},
          {value: 'スライム',text:'スライム耐性'},
          {value: 'けもの',text:'けもの耐性'},
          {value: 'ドラゴン',text:'ドラゴン耐性'},
          {value: '虫',text:'虫耐性'},
          {value: '鳥',text:'鳥耐性'},
          {value: '植物',text:'植物耐性'},
          {value: '物質',text:'物質耐性'},
          {value: 'マシン',text:'マシン耐性'},
          {value: 'ゾンビ',text:'ゾンビ耐性'},
          {value: '悪魔',text:'悪魔耐性'},
          {value: 'エレメント',text:'エレメント耐性'},
          {value: '怪人',text:'怪人耐性'},
          {value: '水',text:'水耐性'},
        ],
        k_damage1: '',
        k_damage2: '',
        k_damage3: '',
        k_damage4: '',
        k_damage_options: [
          {value: '',text:'未選択：ダメージ'},
          {value: 'スライム',text:'スライム系ダメージ'},
          {value: 'けもの',text:'けもの系ダメージ'},
          {value: 'ドラゴン',text:'ドラゴン系ダメージ'},
          {value: '虫',text:'虫系ダメージ'},
          {value: '鳥',text:'鳥系ダメージ'},
          {value: '植物',text:'植物系ダメージ'},
          {value: '物質',text:'物質系ダメージ'},
          {value: 'マシン',text:'マシン系ダメージ'},
          {value: 'ゾンビ',text:'ゾンビ系ダメージ'},
          {value: '悪魔',text:'悪魔系ダメージ'},
          {value: 'エレメント',text:'エレメント系ダメージ'},
          {value: '怪人',text:'怪人系ダメージ'},
          {value: '水',text:'水系ダメージ'},
        ],

        a1_tate: {name:"",desc:""},
        a1_atama: {name:"",desc:""},
        a1_ue: {name:"",desc:""},
        a1_shita: {name:"",desc:""},
        a2_tate: {name:"",desc:""},
        a2_atama: {name:"",desc:""},
        a2_ue: {name:"",desc:""},
        a2_shita: {name:"",desc:""},
        a3_tate: {name:"",desc:""},
        a3_atama: {name:"",desc:""},
        a3_ue: {name:"",desc:""},
        a3_shita: {name:"",desc:""},
        a4_tate: {name:"",desc:""},
        a4_atama: {name:"",desc:""},
        a4_ue: {name:"",desc:""},
        a4_shita: {name:"",desc:""},

      },
      mounted: function(){
        axios.get("./arms.csv").then(
          response => {
            this.csv = response.data
            this.match_arm()
          }
        );
      },
      computed:{
      },
      watch: {
      },
      methods:{
        match_arm: function(){
          var param = {
            job: [this.job1,this.job2,this.job3,this.job4],
            z_tokugi: [this.z_tokugi1,this.z_tokugi2,this.z_tokugi3,this.z_tokugi4],
            z_jumon: [this.z_jumon1,this.z_jumon2,this.z_jumon3,this.z_jumon4],
            z_taisei: [this.z_taisei1,this.z_taisei2,this.z_taisei3,this.z_taisei4],
            k_taisei: [this.k_taisei1,this.k_taisei2,this.k_taisei3,this.k_taisei4],
            k_damage: [this.k_damage1,this.k_damage2,this.k_damage3,this.k_damage4],
          };
          let s = mod.return_matched_arm(
            this.csv,
            param
          );
          this.a1_tate = s.arms[0];
          this.a1_atama = s.arms[1];
          this.a1_ue = s.arms[2];
          this.a1_shita = s.arms[3];

          this.a2_tate = s.arms[4];
          this.a2_atama = s.arms[5];
          this.a2_ue = s.arms[6];
          this.a2_shita = s.arms[7];

          this.a3_tate = s.arms[8];
          this.a3_atama = s.arms[9];
          this.a3_ue = s.arms[10];
          this.a3_shita = s.arms[11];

          this.a4_tate = s.arms[12];
          this.a4_atama = s.arms[13];
          this.a4_ue = s.arms[14];
          this.a4_shita = s.arms[15];

          this.a1_tate.desc = this.a1_tate.desc.replaceAll(' ','\r\n')
          this.a1_atama.desc = this.a1_atama.desc.replaceAll(' ','\r\n')
          this.a1_ue.desc = this.a1_ue.desc.replaceAll(' ','\r\n')
          this.a1_shita.desc = this.a1_shita.desc.replaceAll(' ','\r\n')

          this.a2_tate.desc = this.a2_tate.desc.replaceAll(' ','\r\n')
          this.a2_atama.desc = this.a2_atama.desc.replaceAll(' ','\r\n')
          this.a2_ue.desc = this.a2_ue.desc.replaceAll(' ','\r\n')
          this.a2_shita.desc = this.a2_shita.desc.replaceAll(' ','\r\n')

          this.a3_tate.desc = this.a3_tate.desc.replaceAll(' ','\r\n')
          this.a3_atama.desc = this.a3_atama.desc.replaceAll(' ','\r\n')
          this.a3_ue.desc = this.a3_ue.desc.replaceAll(' ','\r\n')
          this.a3_shita.desc = this.a3_shita.desc.replaceAll(' ','\r\n')
          
          this.a4_tate.desc = this.a4_tate.desc.replaceAll(' ','\r\n')
          this.a4_atama.desc = this.a4_atama.desc.replaceAll(' ','\r\n')
          this.a4_ue.desc = this.a4_ue.desc.replaceAll(' ','\r\n')
          this.a4_shita.desc = this.a4_shita.desc.replaceAll(' ','\r\n')
        },
      } // ここまでmethods
    })
})();

