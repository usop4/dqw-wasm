# dqw-wasm

wasm(WebAssembly)により、ドラクエウォークのモンスターの「こころ」の組み合わせを計算するプロジェクトです。

こちらのページから試せます。

https://usop4.github.io/dqw-wasm/pkg/

# 使い方

git cloneもしくはDownload zipによりローカルに保存しmonster.csvを編集することで、自分が獲得した「こころ」の最適な組み合わせを計算することができます。

MacOSもしくはLinuxであれば展開したファイルのpkgディレクトリに移動し

`python3 -m http.server`

でHTTPサーバを起動してアクセスしてください。github pagesと同じような画面が表示されたら成功です。

筆者はWindows10のWSL2で動作するUbuntu20.04で動作確認しています。

monster.csvを編集しても反映されない場合はブラウザでキャッシュされている可能性があります。Chromeであれば右クリックで「検証」を選択して開発者ツールを起動している状態で更新ボタンを右クリックして「キャッシュをクリアしてハードリフレッシュ」を選択してみてください。

またmonster.csvで項目数が足りない場合などは画面が表示されず、コンソール上に「vue.min.js:6 RuntimeError: unreachable」などのエラーが出ます。この場合はCSVファイルのフォーマット等を確認してください。なお筆者はVisual Studio CodeのRainbow CSVというエクステンションを使って確認しています。

# 技術的なこと

WebAsemblyと呼ばれる技術を使い、ブラウザ上で「こころ」の組み合わせを計算しています。2021年7月時点で私の環境では185のこころを登録しており、戦士で6,980通り、バトルマスターで45,627通りのパターンがあります。

この全てのパターンを計算し、パラメータ順に並び変える処理をRustでコーディングし、wasm-packによりWebAssemblyを生成しています。また画面生成はVue.jsとBootstrapを組み合わせたBootstrapVueにより行っています。
