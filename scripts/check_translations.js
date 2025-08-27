const toml = require("toml");
const fs = require("fs");

const content = fs.readFileSync("locales/docs/typst-docs.toml", "utf8");
const data = toml.parse(content);

function countEntries(obj, path = "", entries = []) {
  let total = 0;
  let translated = 0;
  let missingEntries = [];

  for (const [key, value] of Object.entries(obj)) {
    if (typeof value === "object" && value !== null && !Array.isArray(value)) {
      if ("en" in value) {
        total++;
        if ("zh" in value) {
          translated++;
        } else {
          if (value.en.startsWith("{{")) {
            translated++;
          } else {
            entries.push({ key: path + key, value: value.en });
          }
        }
      } else {
        const result = countEntries(value, path + key + ".", entries);
        total += result.total;
        translated += result.translated;
      }
    }
  }

  return { total, translated, entries };
}

const stats = countEntries(data);
console.log("Total entries:", stats.total);
console.log("Translated entries:", stats.translated);
console.log(
  "Coverage:",
  ((stats.translated / stats.total) * 100).toFixed(1) + "%"
);
console.log("\nNext missing translations:");
stats.entries
  // .forEach(({ key, value }) => console.log(`- ${key}\n  ${value}`));
  .forEach(({ key, value }) => console.log(`- ${key}`));
