import {
  EditorConfig,
  NodeKey,
  ParagraphNode,
  SerializedParagraphNode,
} from "lexical";

export function $createTranslationNode(path: string): TranslationNode {
  return new TranslationNode(path);
}

export class TranslationNode extends ParagraphNode {
  __path: string;

  static getType(): string {
    return "translation";
  }

  static clone(node: TranslationNode): TranslationNode {
    return new TranslationNode(node.__path, node.__key);
  }

  constructor(path: string, key?: NodeKey) {
    super(key);

    this.__path = path;
  }

  /**
   * DOM that will be rendered by browser within contenteditable
   * This is what Lexical renders
   */
  createDOM(cfg: EditorConfig): HTMLElement {
    const createdDom = super.createDOM(cfg);
    return createdDom;
  }

  static importJSON(
    serializedNode: SerializedTranslationNode
  ): TranslationNode {
    return $createTranslationNode(serializedNode.path).updateFromJSON(
      serializedNode
    );
  }

  exportJSON(): SerializedTranslationNode {
    return {
      ...super.exportJSON(),
      path: this.__path,
    };
  }
}

interface SerializedTranslationNode extends SerializedParagraphNode {
  path: string;
}
