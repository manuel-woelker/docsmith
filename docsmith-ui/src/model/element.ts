export interface Element {
  tag: string;
  attributes: Map<string, Value> | undefined;
  children: [Value] | undefined;
}

export type Value = string | Element;
