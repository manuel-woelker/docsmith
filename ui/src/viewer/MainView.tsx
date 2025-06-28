import {Navbar} from "./Navbar";
import {ContentView} from "./ContentView";
import {OutlineView} from "./OutlineView";

import { css } from '@emotion/css'

const navbarHeight = "52";
const outlineWidth = "200";

const containerClass = css`
  background-color: white;
  height: 100vh;
  width: 100vw;
`;
const navbarClass = css`
    margin: 0;
    position: absolute;
    width: 100%;
    height: ${navbarHeight}px;
    box-shadow: 0px 0px 15px 5px #ddd;
`;
const outlineClass = css`
    overflow-y: scroll;
    scrollbar-width: thin;
    position: absolute;
    top: ${navbarHeight}px;
    width: ${outlineWidth}px;
    height: calc(100vh - ${navbarHeight}px);
`;
const contentClass = css`
    overflow-y: scroll;
    position: absolute;
    top: ${navbarHeight}px;
    left: ${outlineWidth}px;
    width: calc(100vw - ${outlineWidth}px);
    height: calc(100vh - ${navbarHeight}px);
`;


export const MainView = () => {
  return <div class={containerClass}>
    <div class={navbarClass}>
      <Navbar />
    </div>
    <div class={outlineClass}>
      <OutlineView />
    </div>
    <div class={contentClass}>
      <ContentView />
    </div>
  </div>;


};
