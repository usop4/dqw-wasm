clear
wasm-pack build --target web

cd pkg
python3 -m http.server

cd ..
