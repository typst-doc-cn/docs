/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 */
import { $createHeadingNode, $createQuoteNode } from "@lexical/rich-text";
import { $createListItemNode, $createListNode } from "@lexical/list";
import { $createParagraphNode, $createTextNode, $getRoot } from "lexical";
import { load } from "js-toml";
import { $createTranslationNode } from "./TranslationNode";

export function convertChange(toml: string) {
  const data = load(toml) as TranslateData;
  const root = $getRoot();
  root.clear();

  console.log(data);

  createTranslateArea(root, data, []);

  // const quote = $createQuoteNode();
  // quote.append($createTextNode(toml));
  // //   root.append(quote);
  // const paragraph = $createParagraphNode();
  // paragraph.append(
  //   $createTextNode(toml)
  //   // $createTextNode("lexical").toggleFormat("code"),
  //   // $createTextNode("."),
  //   // $createTextNode(" Try typing in "),
  //   // $createTextNode("some text").toggleFormat("bold"),
  //   // $createTextNode(" with "),
  //   // $createTextNode("different").toggleFormat("italic"),
  //   // $createTextNode(" formats.")
  // );
  // root.append(paragraph);
}

interface TranslationMap {
  en: string;
  zh?: string;
}

type TranslateData =
  | TranslationMap
  | {
      [key: string]: TranslationMap | TranslateData;
    };

function isTranslationMap(data: any): data is TranslationMap {
  return "en" in data;
}

function createTranslateArea(root: any, data: TranslateData, path: string[]) {
  if (isTranslationMap(data)) {
    const par = $createTranslationNode(path.join("."));
    const heading = $createHeadingNode("h2");
    heading.append($createTextNode(path.join(" » ")));
    root.append(heading);
    const quote = $createQuoteNode();
    quote.append($createTextNode(data.en).toggleFormat("code"));
    par.append(quote);
    const par2 = $createParagraphNode();
    par2.append($createTextNode(data.zh || "NotTranslatedYet"));
    par.append(par2);
    root.append(par);

    return;
  } else {
    for (const key of Object.keys(data)) {
      path.push(key);
      createTranslateArea(root, data[key], path);
      path.pop();
    }
  }
}
