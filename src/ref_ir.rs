// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright © 2020 Corporation for Digital Scholarship

use crate::prelude::*;
use crate::element::{Affixes, Formatting};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum RefIR {
    /// A piece of output that a cite can match in the final DFA.
    /// e.g.
    ///
    /// ```txt
    /// EdgeData::Output(r#"<span style="font-weight: bold;">"#)
    /// EdgeData::Output("Some title, <i>23/4/1969</i>")
    /// EdgeData::Locator
    /// ```
    Edge(Option<EdgeData>),

    /// A non-string EdgeData can be surrounded by a Seq with other strings to apply its
    /// formatting. This will use `OutputFormat::stack_preorder() / ::stack_postorder()`.
    ///
    /// ```txt
    /// RefIR::Seq(vec![
    ///     EdgeData::Output("<i>"),
    ///     EdgeData::Locator,
    ///     EdgeData::Output("</i>"),
    /// ])
    /// ```
    Seq(RefIrSeq),
}

impl Default for RefIR {
    fn default() -> Self {
        RefIR::Edge(None)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RefIrSeq {
    pub contents: Vec<RefIR>,
    pub formatting: Option<Formatting>,
    pub affixes: Option<Affixes>,
    pub delimiter: String,
    pub text_case: TextCase,
}

impl RefIR {
    pub fn debug(&self, db: &dyn IrDatabase) -> String {
        match self {
            RefIR::Edge(Some(e)) => format!("{:?}", e),
            RefIR::Edge(None) => "None".into(),
            RefIR::Seq(seq) => {
                let mut s = String::new();
                s.push_str("[");
                let mut seen = false;
                for x in &seq.contents {
                    if seen {
                        s.push_str(",");
                    }
                    seen = true;
                    s.push_str(&x.debug(db));
                }
                s.push_str("]");
                s
            }
        }
    }

}

