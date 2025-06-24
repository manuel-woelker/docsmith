/* @refresh reload */
import { render } from 'solid-js/web';
import {DocsmithApp} from "./viewer/DocsmithApp";

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(() => <DocsmithApp />, root!);

let path = "./document.js";
import(/* @vite-ignore */ path).then((module) => {
  let document = module.document;
  window.document.title = document.title;

});