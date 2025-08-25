// validate the toml file in `locales/` and `docs/terms.toml`

const fs = require('fs');
const toml = require('toml');
const path = require('path');

function validateTomlFile(path) {
    const file = fs.readFileSync(path, 'utf8');
    // const _data = toml.parse(file);
    try {
        toml.parse(file);
    } catch (e) {
        if (e.line) {
            console.error(`Error parsing ${path}:${e.line}:${e.column}: ${e.message}`);
        } else {
            console.error(`Error parsing ${path}: ${e.message}`);
        } 
    }
}

function validateTomlInDir(dir) {
    const files = fs.readdirSync(dir, { withFileTypes: true });
    for (const file of files) {
        if (file.isDirectory()) {
            validateTomlInDir(path.join(dir, file.name));
        } else if (file.name.endsWith('.toml')) {
            validateTomlFile(path.join(dir, file.name));
        }
    }
}

validateTomlInDir('locales/');
validateTomlFile('docs/terms.toml');
