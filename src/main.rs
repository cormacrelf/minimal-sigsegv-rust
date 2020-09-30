use std::sync::Arc;
use std::collections::HashMap;

mod disamb;
mod ref_ir;
mod group;
mod element;

pub mod prelude {
    pub use super::*;
}

use element::*;

fn main() {
    // use std::str::FromStr;
    // let style = r#"<style class="note" version="1.0">
    //   <macro name="a"><label variable="locator"/></macro>
    //   <citation><layout> <text macro="a"/> </layout></citation>
    // </style>"#;
    // let style = Style::from_str(style).unwrap(); dbg!(&style);
    let style = {
        use element::*;
        Style {
            macros: {
                let mut map = HashMap::default();
                map.insert("a".into(), vec![
                    Element::Label(LabelElement {
                        variable: NumberVariable::Locator,
                        form: TermForm::Long,
                        formatting: Default::default(),
                        affixes: Default::default(),
                        strip_periods: false,
                        text_case: Default::default(),
                        plural: false,
                    })
                ]);
                map
            },
            citation: Citation {
                layout: Layout {
                    elements: vec![
                        Element::Text(TextElement {
                            source: TextSource::Macro("a".into()),
                            formatting: None,
                            affixes: None,
                            quotes: Default::default(),
                            strip_periods: Default::default(),
                            text_case: TextCase::default(),
                            display: None,
                        })
                    ],
                },
            },
        }
    };
    // dbg!(&style);
    let db = MockDbForSegfault { style: Arc::new(style.clone()) };
    let ctx = RefContext {
        style: &style,
        locator_type: Some(element::LocatorType::Page),
        position: element::Position::First,
        year_suffix: false,
        names_delimiter: None,
        name_el: Arc::new(element::Name::root_default()),
        disamb_count: 0,
    };
    let ir = crate::disamb::element_ref_ir_impl(&db.style.citation.layout.elements[0], &db, &ctx);

    // let mut nfa = crate::disamb::Nfa::new();
    // let first = nfa.graph.add_node(());
    // nfa.start.insert(first);
    // let last = crate::disamb::add_to_graph(&fmt, &mut nfa, &ir.0, first);
    // nfa.accepting.insert(last);
}

#[derive(Clone)]
pub struct RefContext<'a> {
    pub style: &'a Style,
    pub locator_type: Option<element::LocatorType>,
    pub position: element::Position,
    pub year_suffix: bool,
    pub names_delimiter: Option<Delimiter>,
    pub name_el: Arc<element::Name>,
    pub disamb_count: u32,
}

pub use group::*;
pub use ref_ir::*;
pub use crate::disamb::EdgeData;

pub struct CiteId(u32);
pub struct IrGen;
struct MockDbForSegfault { style: Arc<element::Style>, }
pub trait IrDatabase { fn style(&self) -> Arc<Style>; }
impl IrDatabase for MockDbForSegfault {
    fn style(&self) -> Arc<Style> { self.style.clone() }
}



