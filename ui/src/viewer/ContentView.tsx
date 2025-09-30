import {css} from "@emotion/css";
import {createEffect, For, Match, Switch, useContext} from "solid-js";
import {DocumentContext} from "../stores/documentStore";
import {DocsmithElement, DocsmithValue, valueAsNumber} from "../model/element";

const contentClass = css`
    padding: 30px 10px;
    scrollbar-width: thin;
`;


function ChildView(props: { element: DocsmithValue }) {
  let value = props.element;
  console.log(typeof value === "string");
  createEffect(() => {
    console.log(value);
  })
  return <Switch fallback={<p>Fallback content</p>}>
    <Match when={typeof value === "string"}>
      <>{value}</>
    </Match>
    <Match when={typeof value === "object"}>
      <><For each={(value as DocsmithElement).children}>
        {(child, index) =>
            // rendering logic for each element
            <>
              <ChildView element={child} />
            </>
        }
      </For></>
    </Match>
  </Switch>
/*  if (typeof value === "string") {
    return <>{value}</>;
  } else if (typeof value === "object") {
    let children = value.children || [];
    let childElements = <div><For each={children}>
      {(child, index) =>
          // rendering logic for each element
          <>
            <ChildView element={child} />
          </>
      }
    </For></div>;
    switch (value.tag) {
      case "heading":
        console.log(value.attributes?.get("level"));
        let level = valueAsNumber(value.attributes?.get("level")) || 1;
        return <h1 class={`title is-${level} `}>{childElements}</h1>;
      default:
        return childElements;
    }
  }
  return null;*/
}

export const ContentView = () => {
  let document = useContext(DocumentContext);
  createEffect(() => {
    console.log(document.body);
  })
  return <div class={contentClass}>
    <h1 class="title">{document.title}</h1>
    <h2 class="subtitle">Subtitle</h2>
    <ChildView element={document.body} />
    ContentView
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    <div id="anchor">
      Anchor
    </div>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
    Content<br/>
  </div>;
};
