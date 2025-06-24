import {useContext} from "solid-js";
import {DocumentContext} from "../stores/documentStore";

export const Navbar= () => {
  const document= useContext(DocumentContext);
  console.log(document);
  return <div>Navbar {document.title}</div>;
};
