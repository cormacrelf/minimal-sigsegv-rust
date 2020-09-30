#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Style {
    // pub class: StyleClass,
    pub macros: HashMap<String, Vec<Element>>,
    pub citation: Citation,
    // pub bibliography: Option<Bibliography>,
    // pub info: Info,
    // pub features: Features,
    // pub name_inheritance: Name,
    // pub names_delimiter: Option<Delimiter>,
    // pub locale_overrides: FnvHashMap<Option<Lang>, Locale>,
    // pub default_locale: Lang,
    // pub version_req: CslVersionReq,
    // pub page_range_format: Option<PageRangeFormat>,
    // pub demote_non_dropping_particle: DemoteNonDroppingParticle,
    // pub initialize_with_hyphen: bool, // default is true
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Citation {
    pub layout: Layout,
}

// TODO: Multiple layouts in CSL-M with locale="en es de" etc
#[derive(Default, Debug, Eq, Clone, PartialEq)]
pub struct Layout {
    pub elements: Vec<Element>,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Element {
    /// <cs:text>
    Text(TextElement),
    /// <cs:label>
    Label(LabelElement),
    /// <cs:number>
    Number(NumberElement),
    /// <cs:group>
    Group(Group),
    /// <cs:choose>
    /// Arc because the IR needs a reference to one, cloning deep trees is costly, and IR has
    /// to be in a Salsa db that doesn't really support lifetimes.
    Choose(Arc<()>),
    /// <cs:names>
    Names(Arc<()>),
    /// <cs:date>
    Date(Arc<()>),
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct TextElement {
    pub source: TextSource,
    pub formatting: Option<Formatting>,
    pub affixes: Option<Affixes>,
    pub quotes: Quotes,
    pub strip_periods: StripPeriods,
    pub text_case: TextCase,
    pub display: Option<DisplayMode>,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct LabelElement {
    pub variable: NumberVariable,
    pub form: TermForm,
    pub formatting: Option<Formatting>,
    pub affixes: Option<Affixes>,
    pub strip_periods: StripPeriods,
    pub text_case: TextCase,
    pub plural: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TermForm {
    Long,
    Short,
    Symbol,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct NumberElement {
    pub variable: NumberVariable,
    pub formatting: Option<Formatting>,
    pub affixes: Option<Affixes>,
    pub text_case: TextCase,
    pub display: Option<DisplayMode>,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Group {
    pub formatting: Option<Formatting>,
    pub delimiter: Delimiter,
    pub affixes: Option<Affixes>,
    pub elements: Vec<Element>,
    pub display: Option<DisplayMode>,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum TextSource {
    Macro(String),
    Value(String),
    Variable(StandardVariable, VariableForm),
    Term(TextTermSelector, TermPlural),
}

/// e.g. for <text variable="title" form="short" />
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VariableForm {
    Long,
    Short,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextTermSelector {
    Simple(SimpleTermSelector),
    Gendered,
    Role
    // You can't render ordinals using a <text> node, only using <number>
}

/// TermSelector is used
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SimpleTermSelector {
    Misc(MiscTerm, TermFormExtended),
    Category,
    Quote
}

/// Includes the extra Verb and VerbShort variants
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TermFormExtended {
    Long,
    Short,
    Symbol,
    Verb,
    VerbShort,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MiscTerm {
    Accessed,
    Ad,
    And,
    AndOthers,
    Anonymous,
    At,
    AvailableAt,
    Bc,
    By,
    Circa,
    Cited,
    // Edition,
    EtAl,
    Forthcoming,
    From,
    Ibid,
    In,
    InPress,
    Internet,
    Interview,
    Letter,
    NoDate,
    Online,
    PresentedAt,
    Reference,
    Retrieved,
    Scale,
    Version,
    PageRangeDelimiter,
    YearRangeDelimiter,
}

type TermPlural = bool;
type StripPeriods = bool;
type Quotes = bool;

#[derive(Debug, Eq, Clone, PartialEq, Default, Hash, Copy)]
pub struct Formatting;
#[derive(Debug, Eq, Clone, PartialEq, Default, Hash)]
pub struct Delimiter;
#[derive(Debug, Eq, Clone, PartialEq, Default, Hash)]
pub struct DisplayMode;
#[derive(Debug, Eq, Clone, PartialEq, Default, Hash)]
pub struct Affixes;
#[derive(Debug, Eq, Clone, PartialEq, Default, Hash)]
pub struct TextCase;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum LocatorType {
    Book,
    Chapter,
    Column,
    Figure,
    Folio,
    Issue,
    Line,
    Note,
    Opus,
    Page,
    Paragraph,
    Part,
    Section,
    SubVerbo,
    Verse,
    Volume,
    Article,
    Subparagraph,
    Rule,
    Subsection,
    Schedule,
    Title,
    Supplement,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Position {
    First,
    Ibid,
    IbidWithLocator,
    Subsequent,
    NearNote,
    IbidNear,
    IbidWithLocatorNear,
    FarNote,
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright © 2018 Corporation for Digital Scholarship

#[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
pub enum AnyVariable {
    Ordinary(Variable),
    Name(NameVariable),
    Date(DateVariable),
    Number(NumberVariable),
}

/// Contrary to the CSL-M spec's declaration that number variables in a regular `<text variable>`
/// "should fail validation", that is perfectly valid, because "number variables are a subset of the
/// standard variables":
/// [Spec](https://docs.citationstyles.org/en/stable/specification.html#number-variables)

#[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
pub enum StandardVariable {
    Ordinary(Variable),
    Number(NumberVariable),
}

impl From<&StandardVariable> for AnyVariable {
    fn from(sv: &StandardVariable) -> Self {
        match sv {
            StandardVariable::Number(n) => AnyVariable::Number(*n),
            StandardVariable::Ordinary(o) => AnyVariable::Ordinary(*o),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Variable {
    /// Not sure where this is from, but it appears sometimes.
    JournalAbbreviation,
    /// abstract of the item (e.g. the abstract of a journal article)
    Abstract,
    /// reader’s notes about the item content
    Annote,
    /// archive storing the item
    Archive,
    /// storage location within an archive (e.g. a box and folder number)
    /// technically the spec says use an underscore, but that's probably a typo.
    ArchiveLocation,
    /// geographic location of the archive,
    ArchivePlace,
    /// issuing or judicial authority (e.g. “USPTO” for a patent, “Fairfax Circuit Court” for a legal case)
    /// CSL-M only
    Authority,
    /// active={true} call number (to locate the item in a library)
    CallNumber,
    /// label identifying the item in in-text citations of label styles (e.g. “Ferr78”). May be assigned by the CSL processor based on item metadata.
    CitationLabel,
    /// title of the collection holding the item (e.g. the series title for a book)
    CollectionTitle,
    /// title of the container holding the item (e.g. the book title for a book chapter, the journal title for a journal article)
    ContainerTitle,
    /// short/abbreviated form of “container-title” (also accessible through the “short” form of the “container-title” variable)
    ContainerTitleShort,
    /// physical (e.g. size) or temporal (e.g. running time) dimensions of the item
    Dimensions,
    /// Digital Object Identifier (e.g. “10.1128/AEM.02591-07”)
    DOI,
    /// name of the related event (e.g. the conference name when citing a conference paper)
    Event,
    /// geographic location of the related event (e.g. “Amsterdam, the Netherlands”)
    EventPlace,

    /// class, type or genre of the item (e.g. “adventure” for an adventure movie, “PhD dissertation” for a PhD thesis)
    Genre,
    /// International Standard Book Number
    ISBN,
    /// International Standard Serial Number
    ISSN,
    /// geographic scope of relevance (e.g. “US” for a US patent)
    Jurisdiction,
    /// keyword(s) or tag(s) attached to the item
    Keyword,
    /// medium description (e.g. “CD”, “DVD”, etc.)
    Medium,
    /// (short) inline note giving additional item details (e.g. a concise summary or commentary)
    Note,
    /// original publisher, for items that have been republished by a different publisher
    OriginalPublisher,
    /// geographic location of the original publisher (e.g. “London, UK”)
    OriginalPublisherPlace,
    /// title of the original version (e.g. “Война и мир”, the untranslated Russian title of “War and Peace”)
    OriginalTitle,
    /// PubMed Central reference number
    PMCID,
    /// PubMed reference number
    PMID,
    /// publisher
    Publisher,
    /// geographic location of the publisher
    PublisherPlace,
    /// resources related to the procedural history of a legal case
    References,
    /// title of the item reviewed by the current item
    ReviewedTitle,
    /// scale of e.g. a map
    Scale,
    /// container section holding the item (e.g. “politics” for a newspaper article).
    /// TODO: CSL-M appears to interpret this as a number variable?
    Section,
    /// from whence the item originates (e.g. a library catalog or database)
    Source,
    /// (publication) status of the item (e.g. “forthcoming”)
    Status,
    /// primary title of the item
    Title,
    /// short/abbreviated form of “title” (also accessible through the “short” form of the “title” variable)
    TitleShort,
    ///  URL (e.g. “https://aem.asm.org/cgi/content/full/74/9/2766”)
    URL,
    /// version of the item (e.g. “2.0.9” for a software program)
    Version,
    /// disambiguating year suffix in author-date styles (e.g. “a” in “Doe, 1999a”)
    YearSuffix,

    /// CSL-M only
    // Intercept Hereinafter at CiteContext, as it isn't known at Reference-time.
    // Global-per-document config should be its own thing separate from references.
    // TODO: delete any noRef="true" and replace with serde directives not to read from
    // CSL-JSON.
    Hereinafter,
    /// CSL-M only
    Dummy,
    /// CSL-M only
    LocatorExtra,
    /// CSL-M only
    VolumeTitle,

    /// CSL-M only
    ///
    /// Not documented in the CSL-M spec.
    Committee,

    /// CSL-M only
    ///
    /// Not documented in the CSL-M spec. See [Indigo Book][ib] section 'R26. Short Form
    /// Citation for Court Documents' for its intended use case, and the Juris-M [US cheat
    /// sheet][uscs]
    ///
    /// [uscs]: https://juris-m.github.io/cheat-sheets/us.pdf
    ///
    /// [ib]: https://law.resource.org/pub/us/code/blue/IndigoBook.html
    DocumentName,

    /// CSL-M only
    ///
    /// Not documented in the CSL-M spec.
    ///
    /// TODO: I think variable="gazette-flag" may have been superseded by type="gazette",
    /// but clearly you can still tick the "Gazette Ref" checkbox in Juris-M on a statute.
    /// Ask Frank. See also https://juris-m.github.io/cheat-sheets/us.pdf
    GazetteFlag,

    // TODO: should not be accessible in condition blocks
    Language,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NumberVariable {
    ChapterNumber,
    CollectionNumber,
    Edition,
    Issue,
    Number,
    NumberOfPages,
    NumberOfVolumes,
    Volume,

    /// Locator, Page and PageFirst, FRRN, and CiteNumber: These are technically meant to be standard variables in CSL 1.0.1, but the spec
    /// requires us to treat them as numerics for `<label plural="contextual">` anyway.
    ///
    /// a cite-specific pinpointer within the item (e.g. a page number within a book, or a volume in a multi-volume work). Must be accompanied in the input data by a label indicating the locator type (see the Locators term list), which determines which term is rendered by cs:label when the “locator” variable is selected.
    Locator,

    /// range of pages the item (e.g. a journal article) covers in a container (e.g. a journal issue)
    Page,
    /// first page of the range of pages the item (e.g. a journal article) covers in a container (e.g. a journal issue)
    PageFirst,

    /// number of a preceding note containing the first reference to the item. Assigned by the CSL processor. The variable holds no value for non-note-based styles, or when the item hasn’t been cited in any preceding notes.
    FirstReferenceNoteNumber,

    /// index (starting at 1) of the cited reference in the bibliography (generated by the CSL processor)
    CitationNumber,

    /// CSL-M only
    PublicationNumber,

    /// CSL-M only
    Supplement,

    /// CSL-M only
    Authority,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum NameVariable {
    /// author
    Author,
    /// editor of the collection holding the item (e.g. the series editor for a book)
    CollectionEditor,
    /// composer (e.g. of a musical score)
    Composer,
    /// author of the container holding the item (e.g. the book author for a book chapter)
    ContainerAuthor,
    /// director (e.g. of a film)
    Director,
    /// editor
    Editor,
    /// managing editor (“Directeur de la Publication” in French)
    EditorialDirector,
    /// illustrator (e.g. of a children’s book)
    Illustrator,
    /// interviewer (e.g. of an interview)
    Interviewer,
    /// ?
    OriginalAuthor,
    /// recipient (e.g. of a letter)
    Recipient,
    /// author of the item reviewed by the current item
    ReviewedAuthor,
    /// translator
    Translator,

    EditorTranslator,

    /// CSL-M only
    Authority,

    /// CSL-M only
    ///
    /// The dummy name variable is always empty. Use it to force all name variables called through
    /// a cs:names node to render through cs:substitute, and so suppress whichever is chosen for
    /// rendering to be suppressed through the remainder of the current cite.
    Dummy,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DateVariable {
    /// date the item has been accessed
    Accessed,
    /// ?
    Container,
    /// date the related event took place
    EventDate,
    /// date the item was issued/published
    Issued,
    /// (issue) date of the original version
    OriginalDate,
    /// date the item (e.g. a manuscript) has been submitted for publication
    Submitted,
    /// CSL-M only
    LocatorDate,
    /// CSL-M only
    PublicationDate,
    /// CSL-M only
    AvailableDate,
}

#[derive(Debug, Eq, Clone, PartialEq, Hash)]
pub struct Name {
    pub and: Option<()>,
    /// Between individual names for the same variable
    pub delimiter: Option<Delimiter>,
    pub delimiter_precedes_et_al: Option<DelimiterPrecedes>,
    pub delimiter_precedes_last: Option<DelimiterPrecedes>,
    pub et_al_min: Option<u32>,
    pub et_al_use_first: Option<u32>,
    pub et_al_use_last: Option<bool>, // default is false
    pub et_al_subsequent_min: Option<u32>,
    pub et_al_subsequent_use_first: Option<u32>,
    pub form: Option<NameForm>,
    pub initialize: Option<bool>, // default is true
    pub initialize_with: Option<String>,
    pub name_as_sort_order: Option<NameAsSortOrder>,
    pub sort_separator: Option<String>,
    pub formatting: Option<Formatting>,
    pub affixes: Option<Affixes>,
    pub name_part_given: Option<NamePart>,
    pub name_part_family: Option<NamePart>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NameAsSortOrder {
    First,
    All,
}

impl Default for Name {
    fn default() -> Self {
        Name::empty()
    }
}

impl Name {
    pub fn empty() -> Self {
        Name {
            and: None,
            delimiter: None,
            delimiter_precedes_et_al: None,
            delimiter_precedes_last: None,
            et_al_min: None,
            et_al_use_first: None,
            et_al_use_last: None,
            et_al_subsequent_min: None,
            et_al_subsequent_use_first: None,
            form: None,
            initialize: None,
            initialize_with: None,
            name_as_sort_order: None,
            sort_separator: None,
            // these four aren't inherited
            formatting: None,
            affixes: Default::default(),
            name_part_given: None,
            name_part_family: None,
        }
    }

    /// All properties on a Name may be inherited from elsewhere. Therefore while the
    /// `Default::default()` implementation will give you lots of `None`s, you need to define what
    /// those Nones should default to absent a parent giving a concrete definition.
    ///
    /// This follows how [citeproc-js][defaults] sets the defaults, because this is not specified
    /// in the spec(s).
    ///
    /// [defaults]: https://github.com/Juris-M/citeproc-js/blob/30ceaf50a0ef86517a9a8cd46362e450133c7f91/src/state.js#L103-L121
    pub fn root_default() -> Self {
        Name {
            and: None,
            delimiter: Some(Delimiter),
            delimiter_precedes_et_al: Some(DelimiterPrecedes::Contextual),
            delimiter_precedes_last: Some(DelimiterPrecedes::Contextual),
            et_al_min: None,
            et_al_use_first: None,
            et_al_use_last: Some(false),
            et_al_subsequent_min: None, // must fall back to et_al_min
            et_al_subsequent_use_first: None, // must fall back to et_al_use_first
            // https://github.com/Juris-M/citeproc-js/blob/30ceaf50a0ef86517a9a8cd46362e450133c7f91/src/util_names_render.js#L710
            form: Some(NameForm::Long),
            initialize: Some(true),
            // https://github.com/Juris-M/citeproc-js/blob/30ceaf50a0ef86517a9a8cd46362e450133c7f91/src/util_names_render.js#L739
            initialize_with: None,
            name_as_sort_order: None,
            sort_separator: Some(", ".into()),
            // these four aren't inherited
            formatting: None,
            affixes: Default::default(),
            name_part_given: None,
            name_part_family: None,
        }
    }

    /// Takes an upstream Name definition, and merges it with a more local one that will
    /// override any fields set.
    ///
    /// Currently, also, it is not possible to override properties that don't accept a
    /// "none"/"default" option back to their default after setting it on a parent element.
    /// Like, once you set "name-as-sort-order", you cannot go back to Firstname Lastname.
    ///
    pub fn merge(&self, overrider: &Self) -> Self {
        Name {
            and: overrider.and.clone().or(self.and),
            delimiter: overrider
                .delimiter
                .clone()
                .or_else(|| self.delimiter.clone()),
            delimiter_precedes_et_al: overrider
                .delimiter_precedes_et_al
                .or(self.delimiter_precedes_et_al),
            delimiter_precedes_last: overrider
                .delimiter_precedes_last
                .or(self.delimiter_precedes_last),
            et_al_min: overrider.et_al_min.or(self.et_al_min),
            et_al_use_first: overrider.et_al_use_first.or(self.et_al_use_first),
            et_al_use_last: overrider.et_al_use_last.or(self.et_al_use_last),
            et_al_subsequent_min: overrider.et_al_subsequent_min.or(self.et_al_subsequent_min),
            et_al_subsequent_use_first: overrider
                .et_al_subsequent_use_first
                .or(self.et_al_subsequent_use_first),
            form: overrider.form.or(self.form),
            initialize: overrider.initialize.or(self.initialize),
            initialize_with: overrider
                .initialize_with
                .clone()
                .or_else(|| self.initialize_with.clone()),
            name_as_sort_order: overrider.name_as_sort_order.or(self.name_as_sort_order),
            sort_separator: overrider
                .sort_separator
                .clone()
                .or_else(|| self.sort_separator.clone()),

            // these four aren't inherited
            formatting: overrider.formatting,
            affixes: overrider.affixes.clone(),
            name_part_given: overrider.name_part_given.clone(),
            name_part_family: overrider.name_part_family.clone(),
        }
    }

    pub fn enable_et_al(&self) -> bool {
        self.et_al_min.is_some() && self.et_al_use_first.is_some()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DelimiterPrecedes {
    Contextual,
    AfterInvertedName,
    Always,
    Never,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NameForm {
    Long,
    Short,
    Count,
}

#[derive(Debug, Eq, Clone, PartialEq, Hash)]
pub struct NamePart {
    pub name: NamePartName,
    pub affixes: Option<Affixes>,
    pub text_case: TextCase,
    pub formatting: Option<Formatting>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NamePartName {
    Given,
    Family,
}

