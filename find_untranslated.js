const toml = require('toml');
const fs = require('fs');

const content = fs.readFileSync('/home/runner/work/docs/docs/locales/docs/typst-docs.toml', 'utf8');
const data = toml.parse(content);

function findUntranslated(obj, path = '') {
  let untranslated = [];
  
  for (const [key, value] of Object.entries(obj)) {
    if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
      if ('en' in value && !('zh' in value)) {
        untranslated.push({
          path: path + key,
          en: value.en.substring(0, 80) + (value.en.length > 80 ? '...' : '')
        });
      } else if (!('en' in value)) {
        untranslated = untranslated.concat(findUntranslated(value, path + key + '.'));
      }
    }
  }
  
  return untranslated;
}

const untranslated = findUntranslated(data);
console.log('Found', untranslated.length, 'untranslated entries');

// Focus on foundation types first
const foundationUntranslated = untranslated.filter(item => 
  item.path.startsWith('reference.foundations') && 
  !item.path.includes('{{typst-docs/')
);

console.log('\nFoundation types needing translation (first 20):');
foundationUntranslated.slice(0, 20).forEach((item, i) => {
  console.log((i+1) + '. ' + item.path);
  console.log('   EN: ' + item.en);
  console.log('');
});