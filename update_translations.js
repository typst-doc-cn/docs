const fs = require('fs');
const toml = require('toml');

// Load the main translation file
const content = fs.readFileSync('locales/docs/typst-docs.toml', 'utf8');
const data = toml.parse(content);

// Translation rules based on the issue requirements
function translateTerms(text, isEnglish = false) {
  if (!text || typeof text !== 'string') return text;
  
  if (isEnglish) {
    // Skip English text - we only work on Chinese translations  
    return text;
  }
  
  // Check for collocations first (keep existing good translations)
  if (text.includes('位置参数') || text.includes('命名参数')) {
    // These are good existing translations, don't change them
    return text;
  }
  
  // Check for co-occurrence by looking at the original English text to determine context
  const hasArgumentAndParameter = /\bargument[s]?\b/i.test(text) && /\bparameter[s]?\b/i.test(text);
  
  if (hasArgumentAndParameter) {
    // When both co-occur, differentiate them
    text = text.replace(/参数(?![^「]*」)/g, match => {
      // This is a simple heuristic - in practice we need more context
      return '实际参数'; // Default to argument translation for now
    });
  } else {
    // When appearing alone, use generic "参数"
    // Most current uses are probably fine as is
  }
  
  return text;
}

function updateEntry(path, entry) {
  if (!entry.zh || !entry.en) return entry;
  
  const originalZh = entry.zh;
  const updatedZh = translateTerms(entry.zh, false);
  
  if (originalZh !== updatedZh) {
    console.log(`Updated: ${path}`);
    console.log(`  Old: ${originalZh}`);
    console.log(`  New: ${updatedZh}`);
    console.log('');
    return { ...entry, zh: updatedZh };
  }
  
  return entry;
}

function updateTranslations(obj, path = "") {
  const result = {};
  
  for (const [key, value] of Object.entries(obj)) {
    const currentPath = path ? `${path}.${key}` : key;
    
    if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
      if ('en' in value) {
        // This is a translation entry
        result[key] = updateEntry(currentPath, value);
      } else {
        // Recurse into nested objects
        result[key] = updateTranslations(value, currentPath);
      }
    } else {
      result[key] = value;
    }
  }
  
  return result;
}

// First, let's just identify what needs to be changed without making changes
console.log("Analyzing entries that need manual review...");

function analyzeEntry(path, entry) {
  if (!entry.zh || !entry.en) return;
  
  const enText = entry.en;
  const zhText = entry.zh;
  
  // Check for argument/parameter usage patterns
  const hasArgument = /\bargument[s]?\b/i.test(enText);
  const hasParameter = /\bparameter[s]?\b/i.test(enText);
  const hasParam = /\bparam[s]?\b/i.test(enText);
  
  if (!hasArgument && !hasParameter && !hasParam) return;
  
  // Check for collocations in English
  const hasPositionalArg = /positional\s+argument[s]?/i.test(enText);
  const hasNamedArg = /named\s+argument[s]?/i.test(enText);
  const hasPositionalParam = /positional\s+parameter[s]?/i.test(enText);
  const hasNamedParam = /named\s+parameter[s]?/i.test(enText);
  
  // Determine what the correct translation should be
  let recommendation = '';
  
  if (hasPositionalArg || hasNamedArg) {
    recommendation = 'Keep existing collocation (位置参数/命名参数)';
  } else if ((hasArgument && hasParameter) || (hasArgument && hasParam)) {
    recommendation = 'Co-occurring: argument→实际参数, parameter→形式参数';
  } else if (hasArgument) {
    recommendation = 'Argument alone: use 参数';
  } else if (hasParameter || hasParam) {
    recommendation = 'Parameter alone: use 参数';
  }
  
  console.log(`${path}`);
  console.log(`  EN: ${enText.substring(0, 100)}...`);
  console.log(`  ZH: ${zhText.substring(0, 100)}...`);
  console.log(`  Recommendation: ${recommendation}`);
  console.log('');
}

function analyzeTranslations(obj, path = "") {
  for (const [key, value] of Object.entries(obj)) {
    const currentPath = path ? `${path}.${key}` : key;
    
    if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
      if ('en' in value) {
        analyzeEntry(currentPath, value);
      } else {
        analyzeTranslations(value, currentPath);
      }
    }
  }
}

analyzeTranslations(data);