// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright © 2019 Corporation for Digital Scholarship

use crate::prelude::*;

// I'm just keeping this around because add_to_graph below is where I originally found the segfault
#[cfg(feature = "petgraph")]
mod finite_automata;
#[cfg(feature = "petgraph")]
pub use finite_automata::{Dfa, Nfa, NfaEdge};

use crate::element::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EdgeData {
    Output(String),

    // The rest are synchronised with fields on CiteContext and IR.
    Locator,
    NotUsed,
    LocatorLabel,

    /// TODO: add a parameter to Dfa::accepts_data to supply the actual year suffix for the particular reference.
    YearSuffix,

    /// Not for DFA matching, must be turned into YearSuffix via `RefIR::keep_first_ysh` before DFA construction
    YearSuffixExplicit,
    /// Not for DFA matching, must be turned into YearSuffix via `RefIR::keep_first_ysh` before DFA construction
    YearSuffixPlain,

    CitationNumber,
    CitationNumberLabel,

    // TODO: treat this specially? Does it help you disambiguate back-referencing cites?
    Frnn,
    FrnnLabel,

    /// The accessed date, which should not help disambiguate cites.
    Accessed,
}

pub fn element_ref_ir_impl(el: &Element, db: &dyn IrDatabase, ctx: &RefContext) -> (RefIR, GroupVars) {
    match el {
        Element::Text(text) => match text.source {
            TextSource::Macro(ref name) => {
                let macro_elements = ctx.style.macros.get(name).unwrap();
                ref_sequence(db, ctx, &macro_elements)
            }
            _ => {
                (RefIR::Edge(None), GroupVars::new())
            }
        },
        Element::Label(label) => {
            let var = label.variable;
            let custom = match var {
                NumberVariable::Locator if ctx.locator_type.is_some() => {
                    eprintln!("added LocatorLabel");
                    Some(EdgeData::LocatorLabel)
                }
                _ => None,
            };
            if let Some(edge_data) = custom {
                let edge = edge_data;
                return (RefIR::Edge(Some(edge)), GroupVars::Important);
            }
            (RefIR::Edge(None), GroupVars::Plain)
        }
        _ => {
            (RefIR::Edge(None), GroupVars::Plain)
        }
    }
}
fn ref_sequence<'c>(
    db: &dyn IrDatabase,
    ctx: &RefContext<'c>,
    els: &[Element],
) -> (RefIR, GroupVars) {

    let mut contents = Vec::with_capacity(els.len());
    let mut overall_gv = GroupVars::new();

    // let mut dropped_gv = GroupVars::new();
    // let els_ptr = els.as_ptr();
    // eprintln!("els_ptr {:x}", els_ptr as usize);

    for el in els {
        let (got_ir, gv) =
            crate::disamb::element_ref_ir_impl(el, db, ctx);
            eprintln!("{:?}", got_ir);
        match got_ir {
            RefIR::Edge(None) => {
                // dropped_gv = dropped_gv.neighbour(gv);
                overall_gv = overall_gv.neighbour(gv);
            }
            _ => {
                contents.push(got_ir);
                overall_gv = overall_gv.neighbour(gv)
            }
        }
    }

    if !contents.iter().any(|x| !matches!(x,  RefIR::Edge(None))) {
        (RefIR::Edge(None), overall_gv)
    } else {
        (
            RefIR::Seq(RefIrSeq {
                contents,
                formatting: Default::default(),
                affixes: Default::default(),
                delimiter: Default::default(),
                text_case: Default::default(),
            }),
            overall_gv,
        )
    }
}

#[cfg(feature = "petgraph")]
use petgraph::{visit::EdgeRef, graph::NodeIndex};
#[cfg(feature = "petgraph")]
pub fn graph_with_stack(
    fmt: &Markup,
    nfa: &mut Nfa,
    formatting: Option<Formatting>,
    affixes: Option<&Affixes>,
    mut spot: NodeIndex,
    f: impl FnOnce(&mut Nfa, NodeIndex) -> NodeIndex,
) -> NodeIndex {
    let stack = fmt.tag_stack(formatting.unwrap_or_else(Default::default), None);
    let mut open_tags = String::new();
    let mut close_tags = String::new();
    fmt.stack_preorder(&mut open_tags, &stack);
    fmt.stack_postorder(&mut close_tags, &stack);
    let mkedge = |s: String| {
        RefIR::Edge(if !s.is_empty() {
            Some(EdgeData::Output(s))
        } else {
            None
        })
    };
    let mkedge_esc = |s: &str| {
        RefIR::Edge(if !s.is_empty() {
            Some(EdgeData::Output(
                // TODO: fmt.ingest
                fmt.output_in_context(fmt.plain(s), Default::default(), None),
            ))
        } else {
            None
        })
    };
    let open_tags = &mkedge(open_tags);
    let close_tags = &mkedge(close_tags);
    if let Some(pre) = affixes.as_ref().map(|a| mkedge_esc(&*a.prefix)) {
        spot = add_to_graph(fmt, nfa, &pre, spot);
    }
    spot = add_to_graph(fmt, nfa, open_tags, spot);
    spot = f(nfa, spot);
    spot = add_to_graph(fmt, nfa, close_tags, spot);
    if let Some(suf) = affixes.as_ref().map(|a| mkedge_esc(&*a.suffix)) {
        spot = add_to_graph(fmt, nfa, &suf, spot);
    }
    spot
}

#[cfg(feature = "petgraph")]
pub fn add_to_graph(
    fmt: &Markup,
    nfa: &mut Nfa,
    ir: &RefIR,
    spot: NodeIndex,
) -> NodeIndex {
    match ir {
        RefIR::Edge(None) => spot,
        RefIR::Edge(Some(e)) => {
            let to = nfa.graph.add_node(());
            nfa.graph.add_edge(spot, to, NfaEdge::Token(e.clone()));
            to
        }
        RefIR::Seq(ref seq) => {
            let RefIrSeq {
                formatting,
                ref contents,
                ref affixes,
                ref delimiter,
                // TODO: use these
            } = *seq;
            let affixes = affixes.as_ref();
            let mkedge = |s: &str| {
                RefIR::Edge(if !s.is_empty() {
                    Some(EdgeData::Output(fmt.output_in_context(
                        fmt.plain(s),
                        Default::default(),
                        None,
                    )))
                } else {
                    None
                })
            };
            let delim = &mkedge(&*delimiter);
            graph_with_stack(fmt, nfa, formatting, affixes, spot, |nfa, mut spot| {
                let mut seen = false;
                for x in contents {
                    if !matches!(x, RefIR::Edge(None)) {
                        if seen {
                            spot = add_to_graph(fmt, nfa, delim, spot);
                        }
                        seen = true;
                    }
                    spot = add_to_graph(fmt, nfa, x, spot);
                }
                spot
            })
        }
    }
}

