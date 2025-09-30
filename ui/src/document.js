console.log("FOOOO");

export const document = {
  title: "docsmith example document",
  authors: ["Manuel Woelker"],
  body: {
    "tag": "root",
    "children": [
      {
        "weight": "60",
        "title": "Tutorials",
        "no_list": "true",
        "tag": "metadata",
        "content_type": "concept",
        "main_menu": "true"
      },
      {
        "tag": "html_block"
      },
      {
        "children": [
          {
            "tag": "heading",
            "children": [
              "About Kubernetes Tutorials"
            ]
          },
          "This section of the Kubernetes documentation contains tutorials.",
          "A tutorial shows how to accomplish a goal that is larger than a single",
          {
            "tag": "link",
            "children": [
              "task"
            ]
          },
          ". Typically a tutorial has several sections,",
          "each of which has a sequence of steps.",
          "Before walking through each tutorial, you may want to bookmark the",
          {
            "tag": "link",
            "children": [
              "Standardized Glossary"
            ]
          },
          " page for later references."
        ],
        "tag": "paragraph"
      },
      {
        "tag": "html_block"
      },
      {
        "children": [
          "Basics"
        ],
        "level": "2",
        "tag": "heading"
      },
      {
        "tag": "list",
        "children": [
          {
            "tag": "item",
            "children": [
              {
                "tag": "link",
                "children": [
                  "Kubernetes Basics"
                ]
              },
              " is an in-depth interactive tutorial that helps you understand the Kubernetes system and try out some basic Kubernetes features."
            ]
          },
          {
            "children": [
              {
                "tag": "link",
                "children": [
                  "Introduction to Kubernetes (edX)"
                ]
              }
            ],
            "tag": "item"
          },
          {
            "tag": "item",
            "children": [
              {
                "tag": "link",
                "children": [
                  "Hello Minikube"
                ]
              }
            ]
          }
        ]
      },
      {
        "level": "2",
        "tag": "heading",
        "children": [
          "Configuration"
        ]
      },
      {
        "tag": "list",
        "children": [
          {
            "tag": "item",
            "children": [
              {
                "children": [
                  "Configuring Redis Using a ConfigMap"
                ],
                "tag": "link"
              }
            ]
          }
        ]
      },
      {
        "level": "2",
        "children": [
          "Authoring Pods"
        ],
        "tag": "heading"
      },
      {
        "children": [
          {
            "tag": "item",
            "children": [
              {
                "children": [
                  "Adopting Sidecar Containers"
                ],
                "tag": "link"
              }
            ]
          }
        ],
        "tag": "list"
      },
      {
        "level": "2",
        "children": [
          "Stateless Applications"
        ],
        "tag": "heading"
      },
      {
        "children": [
          {
            "tag": "item",
            "children": [
              {
                "children": [
                  "Exposing an External IP Address to Access an Application in a Cluster"
                ],
                "tag": "link"
              }
            ]
          },
          {
            "children": [
              {
                "children": [
                  "Example: Deploying PHP Guestbook application with Redis"
                ],
                "tag": "link"
              }
            ],
            "tag": "item"
          }
        ],
        "tag": "list"
      },
      {
        "level": "2",
        "children": [
          "Stateful Applications"
        ],
        "tag": "heading"
      },
      {
        "children": [
          {
            "tag": "item",
            "children": [
              {
                "tag": "link",
                "children": [
                  "StatefulSet Basics"
                ]
              }
            ]
          },
          {
            "children": [
              {
                "children": [
                  "Example: WordPress and MySQL with Persistent Volumes"
                ],
                "tag": "link"
              }
            ],
            "tag": "item"
          },
          {
            "tag": "item",
            "children": [
              {
                "children": [
                  "Example: Deploying Cassandra with Stateful Sets"
                ],
                "tag": "link"
              }
            ]
          },
          {
            "children": [
              {
                "tag": "link",
                "children": [
                  "Running ZooKeeper, A CP Distributed System"
                ]
              }
            ],
            "tag": "item"
          }
        ],
        "tag": "list"
      },
      {
        "tag": "heading",
        "level": "2",
        "children": [
          "Services"
        ]
      },
      {
        "children": [
          {
            "children": [
              {
                "children": [
                  "Connecting Applications with Services"
                ],
                "tag": "link"
              }
            ],
            "tag": "item"
          },
          {
            "children": [
              {
                "tag": "link",
                "children": [
                  "Using Source IP"
                ]
              }
            ],
            "tag": "item"
          }
        ],
        "tag": "list"
      },
      {
        "children": [
          "Security"
        ],
        "tag": "heading",
        "level": "2"
      },
      {
        "tag": "list",
        "children": [
          {
            "children": [
              {
                "children": [
                  "Apply Pod Security Standards at Cluster level"
                ],
                "tag": "link"
              }
            ],
            "tag": "item"
          },
          {
            "tag": "item",
            "children": [
              {
                "tag": "link",
                "children": [
                  "Apply Pod Security Standards at Namespace level"
                ]
              }
            ]
          },
          {
            "tag": "item",
            "children": [
              {
                "tag": "link",
                "children": [
                  "Restrict a Container's Access to Resources with AppArmor"
                ]
              }
            ]
          },
          {
            "tag": "item",
            "children": [
              {
                "tag": "link",
                "children": [
                  "Seccomp"
                ]
              }
            ]
          }
        ]
      },
      {
        "level": "2",
        "tag": "heading",
        "children": [
          "Cluster Management"
        ]
      },
      {
        "children": [
          {
            "children": [
              {
                "tag": "link",
                "children": [
                  "Running Kubelet in Standalone Mode"
                ]
              }
            ],
            "tag": "item"
          }
        ],
        "tag": "list"
      },
      {
        "tag": "heading",
        "level": "2",
        "children": [
          "{{% heading \"whatsnext\" %}}"
        ]
      },
      {
        "children": [
          "If you would like to write a tutorial, see",
          {
            "children": [
              "Content Page Types"
            ],
            "tag": "link"
          },
          "for information about the tutorial page type."
        ],
        "tag": "paragraph"
      }
    ]
  }
}