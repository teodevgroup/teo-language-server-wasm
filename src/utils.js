const fs = require('fs')

function readFileSync(path) {
    return fs.readFileSync(path, { encoding: 'utf8' })
}

module.exports = {
    readFileSync
}