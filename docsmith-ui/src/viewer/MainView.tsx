import {Navbar} from "./Navbar";
import {ContentView} from "./ContentView";
import {OutlineView} from "./OutlineView";

import { css } from '@emotion/css'

const navbarHeight = "30";
const outlineWidth = "200";

const containerClass = css`
  background-color: red;
  height: 100vh;
  width: 100vw;
`;
const navbarClass = css`
    background-color: lavender;
    position: absolute;
    width: 100%;
    height: ${navbarHeight}px;
`;
const outlineClass = css`
    background-color: aquamarine;
    overflow-y: scroll;
    position: absolute;
    top: ${navbarHeight}px;
    width: ${outlineWidth}px;
    height: calc(100vh - ${navbarHeight}px);
`;
const contentClass = css`
    background-color: blanchedalmond;
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
