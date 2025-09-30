export interface DocsmithElement {
  tag: string;
  attributes?: Map<string, DocsmithValue> | undefined;
  children?: [DocsmithValue] | undefined;
}

export type DocsmithValue = string | DocsmithElement;


export function valueAsNumber(value: DocsmithValue | undefined):  number | null {
  // check if value is string
  if(typeof value === 'string') {
    return parseFloat(value);
  }
  return null;
}
