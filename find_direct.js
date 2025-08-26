const toml = require('toml');
const fs = require('fs');

const content = fs.readFileSync('./locales/docs/typst-docs.toml', 'utf8');
const data = toml.parse(content);

function findDirectTranslations(obj, path = '') {
  let directTranslations = [];
  
  for (const [key, value] of Object.entries(obj)) {
    if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
      if ('en' in value && !('zh' in value)) {
        // Check if it's a direct translation (not a file reference)
        if (!value.en.includes('{{typst-docs/')) {
          directTranslations.push({
            path: path + key,
            en: value.en.length > 100 ? value.en.substring(0, 100) + '...' : value.en
          });
        }
      } else if (!('en' in value)) {
        directTranslations = directTranslations.concat(findDirectTranslations(value, path + key + '.'));
      }
    }
  }
  
  return directTranslations;
}

const directTranslations = findDirectTranslations(data);
console.log('Found', directTranslations.length, 'entries needing direct translation (not file references)');

// Focus on foundation types first
const foundationDirect = directTranslations.filter(item => 
  item.path.startsWith('reference.foundations')
);

console.log('\nFoundation types needing direct translation (first 50):');
foundationDirect.slice(0, 50).forEach((item, i) => {
  console.log((i+1) + '. ' + item.path);
  console.log('   EN: ' + item.en);
  console.log('');
});

console.log('\nOther entries needing direct translation (first 30):');
directTranslations.filter(item => !item.path.startsWith('reference.foundations')).slice(0, 30).forEach((item, i) => {
  console.log((i+1) + '. ' + item.path);
  console.log('   EN: ' + item.en);
  console.log('');
});