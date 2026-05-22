type BaseNode = {
  type: string;
  named: boolean;
};

type ChildNode = {
  multiple: boolean;
  required: boolean;
  types: BaseNode[];
};

type NodeInfo =
  | (BaseNode & {
      subtypes: BaseNode[];
    })
  | (BaseNode & {
      fields: { [name: string]: ChildNode };
      children: ChildNode[];
    });

type Language = {
  /**
   * Type guard to check if a value is a tree-sitter Language object
   */
  readonly __brand: unique symbol;
};

/**
 * Get the tree-sitter Language for PRQL
 */
export declare const language: Language;

/**
 * Node type information for PRQL
 */
export declare const nodeTypeInfo: NodeInfo[];
