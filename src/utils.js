const fs = require('fs')

function readFileSync(path) {
    return fs.readFileSync(path, { encoding: 'utf8' })
}

function fileIsDirectory(path) {
    let stats = fs.statSync(path)
    if (stats) {
        return stats.isDirectory()
    }
    return false
}

module.exports = {
    readFileSync,
    fileIsDirectory,
}