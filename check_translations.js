const toml = require('toml');
const fs = require('fs');

const content = fs.readFileSync('/home/runner/work/docs/docs/locales/docs/typst-docs.toml', 'utf8');
const data = toml.parse(content);

function countEntries(obj, path = '') {
  let total = 0;
  let translated = 0;
  let missingEntries = [];
  
  for (const [key, value] of Object.entries(obj)) {
    if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
      if ('en' in value) {
        total++;
        if ('zh' in value) {
          translated++;
        } else {
          missingEntries.push(path + key);
        }
      } else {
        const result = countEntries(value, path + key + '.');
        total += result.total;
        translated += result.translated;
        missingEntries = missingEntries.concat(result.missingEntries);
      }
    }
  }
  
  return { total, translated, missingEntries };
}

const stats = countEntries(data);
console.log('Total entries:', stats.total);
console.log('Translated entries:', stats.translated);
console.log('Coverage:', ((stats.translated / stats.total) * 100).toFixed(1) + '%');
console.log('\nNext 20 missing translations:');
stats.missingEntries.slice(0, 20).forEach(entry => console.log('- ' + entry));