const fs = require('fs');
const toml = require('toml');

// Load the main translation file
const content = fs.readFileSync('locales/docs/typst-docs.toml', 'utf8');
const data = toml.parse(content);

function findTermEntries(obj, path = "") {
  let entries = [];
  
  for (const [key, value] of Object.entries(obj)) {
    const currentPath = path ? `${path}.${key}` : key;
    
    if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
      if ('en' in value) {
        // This is a translation entry
        const enText = value.en;
        const zhText = value.zh || '';
        
        // Check if contains argument or parameter (case insensitive)
        const hasArgument = /\bargument[s]?\b/i.test(enText);
        const hasParameter = /\bparameter[s]?\b/i.test(enText);
        const hasParam = /\bparam[s]?\b/i.test(enText);
        
        if (hasArgument || hasParameter || hasParam) {
          entries.push({
            path: currentPath,
            en: enText,
            zh: zhText,
            hasArgument,
            hasParameter: hasParameter || hasParam,
            coOccur: (hasArgument && (hasParameter || hasParam))
          });
        }
      } else {
        // Recurse into nested objects
        entries.push(...findTermEntries(value, currentPath));
      }
    }
  }
  
  return entries;
}

const entries = findTermEntries(data);

console.log(`Found ${entries.length} entries containing argument/parameter/param:`);
console.log('');

// Group by co-occurrence
const coOccurEntries = entries.filter(e => e.coOccur);
const argumentOnlyEntries = entries.filter(e => e.hasArgument && !e.hasParameter);
const parameterOnlyEntries = entries.filter(e => e.hasParameter && !e.hasArgument);

console.log(`Co-occurring entries (${coOccurEntries.length}):`);
coOccurEntries.forEach(entry => {
  console.log(`- ${entry.path}`);
  console.log(`  EN: ${entry.en.substring(0, 100)}...`);
  console.log(`  ZH: ${entry.zh.substring(0, 100)}...`);
  console.log('');
});

console.log(`\nArgument-only entries (${argumentOnlyEntries.length}):`);
argumentOnlyEntries.forEach(entry => {
  console.log(`- ${entry.path}`);
  console.log(`  EN: ${entry.en.substring(0, 100)}...`);
  console.log(`  ZH: ${entry.zh.substring(0, 100)}...`);
  console.log('');
});

console.log(`\nParameter-only entries (${parameterOnlyEntries.length}):`);
parameterOnlyEntries.forEach(entry => {
  console.log(`- ${entry.path}`);
  console.log(`  EN: ${entry.en.substring(0, 100)}...`);
  console.log(`  ZH: ${entry.zh.substring(0, 100)}...`);
  console.log('');
});