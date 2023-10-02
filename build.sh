wasm-pack build -t nodejs
sed -i '' 's/"teo-language-server-wasm/"@teocloud\/teo-language-server-wasm/g' pkg/package.json
