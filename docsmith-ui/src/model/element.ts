export interface DocsmithElement {
  tag: string;
  attributes?: Map<string, DocsmithValue> | undefined;
  children?: [DocsmithValue] | undefined;
}

export type DocsmithValue = string | DocsmithElement;
