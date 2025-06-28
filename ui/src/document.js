console.log("FOOOO");

export const document = {
  title: "docsmith example document",
  authors: ["Manuel Woelker"],
  body: {
    tag: "body",
    children: [
      {
        tag: "heading",
        children: [
          {
            tag: "text",
            attributes: {},
            children: ["Introduction"],
          },
            "This is an example document."
        ],
      },
    ],
  }
}