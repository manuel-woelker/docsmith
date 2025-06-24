import {DocsmithElement} from "./element";

export interface DocsmithDocument {
  title: string;
  authors: string[];
  body: DocsmithElement;
}

