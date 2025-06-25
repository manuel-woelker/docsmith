import {useContext} from "solid-js";
import {DocumentContext} from "../stores/documentStore";
import {css} from "@emotion/css";
const navbarClass = css`
    font-size: 20px;
    padding: 5px 20px;
`;

export const Navbar= () => {
  const document= useContext(DocumentContext);
  return <div class={navbarClass}>{document.title}</div>;
};
