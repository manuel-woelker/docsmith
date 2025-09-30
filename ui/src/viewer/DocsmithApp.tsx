import {MainView} from "./MainView";
import {defaultDocument, DocumentContext} from "../stores/documentStore";
import {createStore} from "solid-js/store";
import {createEffect, onMount} from "solid-js";

export const DocsmithApp = () => {
  // Initialize store
  const [documentStore, setDocumentStore] = createStore(defaultDocument);
  onMount(async () => {
    let path = "../document.js";
    let module = await import(/* @vite-ignore */ path);
    let document = module.document;
    window.document.title = document.title;
    console.log(module.document);
    createEffect(() => {
      setDocumentStore(s => module.document);
    });
  });
  return <DocumentContext.Provider value={documentStore}>
    <MainView />
  </DocumentContext.Provider>;
};
