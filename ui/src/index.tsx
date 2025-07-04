/* @refresh reload */

import "bulma/css/bulma.min.css";

import { render } from 'solid-js/web';
import {DocsmithApp} from "./viewer/DocsmithApp";

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(() => <DocsmithApp />, root!);
