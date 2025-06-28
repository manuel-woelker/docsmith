import {useContext} from "solid-js";
import {DocumentContext} from "../stores/documentStore";
import {css} from "@emotion/css";
const navbarClass = css`
    font-size: 20px;
    padding: 0 20px;
`;

export const Navbar= () => {
  const document= useContext(DocumentContext);
  return <div class={navbarClass}>
    <nav class="navbar" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
        <a class="navbar-item" href="https://github.com/manuel-woelker/docsmith">
          {document.title}
        </a>

        <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false"
           data-target="navbarBasicExample">
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>

      <div class="navbar-menu">
        <div class="navbar-start">
          <a class="navbar-item">
            Home
          </a>

          <a class="navbar-item">
            Documentation
          </a>

      </div>
      </div>
    </nav>

  </div>;
};
