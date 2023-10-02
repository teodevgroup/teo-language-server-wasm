const fs = require("fs")

const packageJson = JSON.parse(fs.readFileSync("pkg/package.json"))
packageJson["name"] = "@teocloud/teo-language-server-wasm"
delete packageJson["files"]
fs.writeFileSync("pkg/package.json", JSON.stringify(packageJson, null, 4))