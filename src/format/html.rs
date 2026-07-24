use crate::format::pretty::{Fillable, fill};

use super::pretty::{Doc, Pretty};

struct Attr(String, String);

impl Pretty for Attr {
    fn to_doc(&self) -> Doc {
        match self {
            Attr(name, value) => Doc::Text(format!("{name}=\"{value}\"")),
        }
    }
}

enum XML {
    Elt(String, Vec<Attr>, Vec<XML>),
    Txt(String),
}

fn opening_tag(tag: &str, attrs: &Vec<Attr>, self_closing: bool) -> Doc {
    let mut closer = if self_closing {
        Doc::text("/>")
    } else {
        Doc::text(">")
    };

    if attrs.len() != 0 {
        closer =
            (fill(&attrs.iter().map(|a| a.to_doc()).collect::<Vec<_>>())).bracket("", "") + closer
    };

    Doc::text("<") + Doc::text(tag) + closer
}

fn closing_tag(tag: &str) -> Doc {
    Doc::Text(format!("</{tag}>"))
}

impl XML {
    fn to_docs(&self) -> Vec<Doc> {
        match self {
            XML::Elt(tag, attrs, xmls) => {
                if xmls.len() == 0 {
                    vec![opening_tag(tag, attrs, true)]
                } else {
                    vec![
                        (opening_tag(tag, attrs, false)
                            + ((fill(&xmls.iter().flat_map(|x| x.to_docs()).collect::<Vec<_>>()))
                                .bracket("", ""))
                            + closing_tag(tag)),
                    ]
                }
            }
            XML::Txt(data) => data.trim().split(" ").map(Doc::text).collect(),
        }
    }
}

impl Pretty for XML {
    fn to_doc(&self) -> Doc {
        self.to_docs().iter().fold(Doc::Nil, |acc, val| match acc {
            Doc::Nil => val.clone(),
            acc => acc + val.clone(),
        })
    }
}

fn get_xml() -> XML {
    XML::Elt(
        "p".to_owned(),
        vec![
            Attr("color".into(), "red".into()),
            Attr("font".into(), "Times".into()),
            Attr("size".into(), "10".into()),
        ],
        vec![
            XML::Txt("Here is some".into()),
            XML::Elt("em".into(), vec![], vec![XML::Txt("emphasized".into())]),
            XML::Txt("text.".into()),
            XML::Txt("Here is a".into()),
            XML::Elt(
                "a".into(),
                vec![Attr("href".into(), "http://www.eg.com/".into())],
                vec![XML::Txt("link".into())],
            ),
            XML::Txt("elsewhere.".into()),
        ],
    )
}

pub fn test() -> String {
    get_xml().pretty(30)
}
