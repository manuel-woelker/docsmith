import {Context, createContext} from "solid-js";
import {DocsmithDocument} from "../model/document";

export const defaultDocument: DocsmithDocument = {
  title: "Document",
  authors: [],
  body: {
    tag: "body",
  },
};

export const DocumentContext: Context<DocsmithDocument> = createContext(defaultDocument);
