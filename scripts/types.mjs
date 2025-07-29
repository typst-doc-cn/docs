/// This is a Script which investigates the types in the typst std library.

// import * as docs from "../dist/docs.json";

import fs from "fs/promises";
import { resolve } from "path";

/**
 * @type {import("./docs-types").Root}
 */
const docs = JSON.parse(
  await fs.readFile(resolve(import.meta.dirname, "../dist/docs.json"), "utf-8")
);

// console.log("Generating types...", Object.keys(docs), "types found");

const reference = docs.find((doc) => doc.title === "Reference");

console.log();

/**
 *
 * @type {{func: import("./docs-types").FunctionType, path: string[]}[]}
 */
const functions = [];

scanPage(reference, []);

/**
 *
 * @param {import("./docs-types").Children} page
 * @param {number} level
 */
function scanPage(page, path, level = 0) {
  //   console.log("  ".repeat(level) + page.title);
  //   path.push(page.title);

  for (const [key, value] of Object.entries(page.body)) {
    // console.log("  ".repeat(level + 1) + `${key}.`);

    if (key === "Type") {
      //   console.log(
      //     "  ".repeat(level + 1) + `-> ${Object.keys(value.constructor || {})}`
      //   );
      scanScope(value, path, level + 1);
      //   console.log("  ".repeat(level + 1) + `-> ${value.scope || []}`);
    }
    if (key === "Module") {
      //   console.log("  ".repeat(level + 1) + `-> ${Object.keys(value)}`);
      scanScope(value, path, level + 1);
    }
    // if (key === "Symbol") {
    //   console.log("  ".repeat(level + 1) + `-> ${Object.keys(value)}`);
    // }
    if (key === "Func") {
      scanScope(value, path, level + 1);
      //   console.log("  ".repeat(level + 1) + `-> ${Object.keys(value)}`);
    }
  }

  if (page.children) {
    for (const child of page.children) {
      scanPage(child, path, level + 1);
    }
  }

  //   path.pop();
}

/**
 *
 * @param {import("./docs-types").Scope} scope
 * @param {number} level
 */
function scanScope(scope, path, level = 0) {
  if (!scope) return;
  //   console.log("  ".repeat(level) + scope.name);
  path.push(scope.name);
  if ("params" in scope) {
    functions.push({
      func: scope,
      path: [...path],
    });
  }

  if (scope.scope) {
    for (const child of scope.scope) {
      scanScope(child, path, level + 1);
    }
  }

  path.pop();
}

/**
 * @type {Map<string, [string, import("./docs-types").FunctionType][]>}
 */
const paramByType = new Map();

for (const funcModel of functions) {
  const { func, path } = funcModel;
  const params = func.params.map((p) => p.name).join(", ");
  const returnType = func.returnType ? ` -> ${func.returnType}` : "";
  //   console.log(
  //     `export function ${func.name}(${params})${returnType}; // ${path.join(".")}`
  //   );

  for (const param of func.params) {
    for (const ty of param.types) {
      if (!paramByType.has(ty)) {
        paramByType.set(ty, []);
      }
      paramByType.get(ty).push([param, false, funcModel]);
    }
  }
  for (const ty of func.returns) {
    if (!paramByType.has(ty)) {
      paramByType.set(ty, []);
    }
    paramByType.get(ty).push([ty, true, funcModel]);
  }
}

console.log(paramByType.keys());

for (const complexType of [
  "array",
  "dictionary",
  "function",
  "type",
  "stroke",
  "any",
  "content",
]) {
  console.log(`\n=== ${complexType} ===\n`);

  for (const [loc, isRet, params] of paramByType.get(complexType)) {
    if (isRet) {
      console.log("->", params.path.join("."));
    } else {
      console.log("::", `${params.path.join(".")}(${loc.name})`);
    }
  }
}
