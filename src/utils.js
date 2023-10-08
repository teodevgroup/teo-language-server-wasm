const fs = require('fs')

function readFileSync(path) {
    return fs.readFileSync(path, { encoding: 'utf8' })
}

function fileIsDirectory(path) {
    let stats = fs.statSync(path)
    if (stats) {
        return fs.isDirectory(path)
    }
    return false
}

module.exports = {
    readFileSync,
    fileIsDirectory,
}