# https://qiita.com/ne_no_usa/items/6d0a23769776aa61401f
wasm-pack build --target web

cd pkg
python3 -m http.server

cd ..