import "./styles.css";

import { registerDragonSupport } from "@lexical/dragon";
import { createEmptyHistoryState, registerHistory } from "@lexical/history";
import { HeadingNode, QuoteNode, registerRichText } from "@lexical/rich-text";
import { ListItemNode, ListNode, registerList } from "@lexical/list";
import { mergeRegister } from "@lexical/utils";
import {
  $createParagraphNode,
  createEditor,
  HISTORY_MERGE_TAG,
  LexicalEditor,
} from "lexical";
import {
  convertTranslated as convertTranslated,
  convertTranslating as convertTranslating,
} from "./convert";
import { TranslationNode } from "./TranslationNode";
import { $generateNodesFromSerializedNodes } from "@lexical/clipboard";
import {
  $convertFromMarkdownString,
  $convertToMarkdownString,
  registerMarkdownShortcuts,
  TRANSFORMERS,
} from "@lexical/markdown";
import { CodeNode } from "@lexical/code";
import { LinkNode } from "@lexical/link";

const fileSelect = document.getElementById("files") as HTMLInputElement;
const translating = document.getElementById(
  "translating-editor"
) as HTMLDivElement;
const fileContent = document.getElementById("file-editor") as HTMLDivElement;
const saveButton = document.getElementById("save-button") as HTMLButtonElement;

function createTranslateEditor(attachTo: HTMLElement) {
  const initialConfig = {
    namespace: "Translate Editor",
    // Register nodes specific for @lexical/rich-text
    nodes: [
      // HorizontalRuleNode,
      CodeNode,
      HeadingNode,
      LinkNode,
      ListNode,
      ListItemNode,
      QuoteNode,
      TranslationNode,
    ],
    onError: (error: Error) => {
      throw error;
    },
    theme: {
      // Adding styling to Quote node, see styles.css
      quote: "PlaygroundEditorTheme__quote",
    },
  };
  const editor = createEditor(initialConfig);
  editor.setRootElement(attachTo);

  // Registering Plugins
  mergeRegister(
    registerRichText(editor),
    registerDragonSupport(editor),
    registerList(editor),
    registerMarkdownShortcuts(editor),
    registerHistory(editor, createEmptyHistoryState(), 300)
  );

  return editor;
}

initEditor();

function initEditor() {
  const translatingEditor = createTranslateEditor(translating);
  const translatedEditor = createTranslateEditor(fileContent);
  translatingEditor.setEditable(false);

  const translatesRef = { current: {} };

  registerSave(translatedEditor, translatesRef);
  fetchData(translatesRef).then(() => {
    const files = Object.keys(translatesRef.current).sort((a, b) =>
      a.localeCompare(b)
    );

    for (const file of files) {
      const option = document.createElement("option");
      option.value = file;
      option.textContent = file;
      fileSelect.appendChild(option);
    }

    const handleChange = (fileName: string) => {
      translatingEditor.update(
        () => convertTranslating(translatesRef.current[fileName] || ""),
        {
          tag: HISTORY_MERGE_TAG,
        }
      );
      translatedEditor.update(
        () => convertTranslated(translatesRef.current[fileName] || ""),
        {
          tag: HISTORY_MERGE_TAG,
        }
      );
    };

    handleChange("typst-docs.toml");
    fileSelect.addEventListener("change", (event) => {
      const target = event.target as HTMLInputElement;
      handleChange(target.value);
    });
  });
}

function registerSave(editor: LexicalEditor, translatesRef) {
  let saved = true;
  /// Notifies editor changes
  editor.registerUpdateListener((results) => {
    if (results.tags.has(HISTORY_MERGE_TAG)) {
      console.log("History merge completed");
      return;
    }

    console.log("dirtyLeaves", results.dirtyLeaves);
    if (results.dirtyLeaves.size > 0) {
      saveButton.disabled = false;
      saved = false;
      fetchData(translatesRef);
    } else if (saved) {
      saveButton.disabled = true;
    }
  });
  /// Save when clicking save button
  saveButton.addEventListener("click", () => {
    const nodes = editor.getEditorState().toJSON().root;
    const translations = [];
    editor.update(() => walkNode(nodes, translations));
    console.log("translations:", translations);
    if (translations.length === 0) {
      alert("No translations found to save.");
      return;
    }
    saveTranslation(translations);
  });
  /// Walk and get changed translations
  function walkNode(node: any, translations: any[]) {
    if (node.type === "translation") {
      const path = node.path;

      const translated = node.children[1];
      if (
        translated.children?.length === 1 &&
        translated.children[0].text === "NotTranslatedYet"
      ) {
        return;
      }
      const el = $createParagraphNode();
      el.splice(
        0,
        0,
        $generateNodesFromSerializedNodes(node.children.slice(1))
      );
      const content = $convertToMarkdownString(TRANSFORMERS, el);

      translations.push({ path, content });
    } else if (node.children) {
      for (const child of node.children) {
        walkNode(child, translations);
      }
    }
  }
  function saveTranslation(translates: any[]) {
    fetch("/api/translates/", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        file: fileSelect.value,
        translates,
      }),
    })
      .then((response) => response.json())
      .then(async ({ success }) => {
        if (success) {
          saved = true;
          saveButton.disabled = true;
          await fetchData(translatesRef);
        } else {
          alert("Failed to save translations.");
        }
      })
      .catch((error) => {
        alert("Error saving translations:" + error);
        alert("An error occurred while saving translations.");
      });
  }
}

function fetchData(translatesRef) {
  return fetch("/api/translates/")
    .then((response) => response.json())
    .then(({ translates }) => {
      translatesRef.current = translates;
    });
}
