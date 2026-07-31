#[doc(hidden)]
#[macro_export]
macro_rules! render {
    ($open:expr, $($close:expr)? => ($key:ident = $value:expr$(, $($tt:tt)*)?) -> ($($attrs:tt)*)) => {
        $crate::render!($open, $($close)* => ($($($tt)*)*) -> (($crate::key!($key), $value, $($attrs)*)))
    };
    ($open:expr, $close:expr => ($($child:expr),*$(,)?) -> ($($attrs:tt)*)) => {
        ($open, $crate::Attrs($($attrs)*), ($($child),*), $close)
    };
    ($open:expr, => () -> ($($attrs:tt)*)) => {
        ($open, $crate::Attrs($($attrs)*))
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "kebab"))]
macro_rules! key {
    ($key:ident) => {
        stringify!($key)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "kebab")]
macro_rules! key {
    ($key:ident) => {
        $crate::const_str::replace!(
            $crate::const_str::replace!(stringify!($key), "r#", ""),
            "_",
            "-"
        )
    };
}

#[macro_export]
/// renders arbitrary element
macro_rules! element {
    ($name:expr, $($tt:tt)*) => {
        $crate::render!($crate::Raw(concat!('<', $name)), $crate::Raw(concat!("</", $name, '>')) => ($($tt)*) -> (()));
    };
    ($name:expr => void, $($tt:tt)*) => {
        $crate::render!($crate::Raw(concat!('<', $name)), => ($($tt)*) -> (()));
    };
}

macro_rules! elements {
    ($dol:tt, $($name:ident $(@$tag:tt)? $doc:literal,)+) => {
        $(
            #[doc = $doc]
            #[macro_export]
            macro_rules! $name {
                ($dol($doltt:tt)*) => {
                    $crate::element!(stringify!($name)$(=> $tag)*, $dol($doltt)*)
                };
            }
        )*
    };
}

elements!($, // https://html.spec.whatwg.org/multipage/indices.html
    a "Hyperlink",
    abbr "Abbreviation",
    address "Contact information for a page or article element",
    area @void "Hyperlink or dead area on an image map",
    article "Self-contained syndicatable or reusable composition",
    aside "Sidebar for tangentially related content",
    audio "Audio player",
    b "Keywords",
    base @void "Base URL and default target navigable for hyperlinks and forms",
    bdi "Text directionality isolation",
    bdo "Text directionality formatting",
    blockquote "A section quoted from another source",
    body "Document body",
    br @void "Line break, e.g. in poem or postal address",
    button "Button control",
    canvas "Scriptable bitmap canvas",
    caption "Table caption",
    cite "Title of a work",
    code "Computer code",
    col "Table column",
    colgroup "Group of columns in a table",
    data "Machine-readable equivalent",
    datalist "Container for options for combo box control",
    dd "Content for corresponding dt element(s)",
    del "A removal from the document",
    details "Disclosure control for hiding details",
    dfn "Defining instance",
    dialog "Dialog box or window",
    div "Generic flow container, or container for name-value groups in dl elements",
    dl "Association list consisting of zero or more name-value groups",
    dt "Legend for corresponding dd element(s)",
    em "Stress emphasis",
    embed @void "Plugin",
    fieldset "Group of form controls",
    figcaption "Caption for figure",
    figure "Figure with optional caption",
    footer "Footer for a page or section",
    form "User-submittable form",
    h1 "Heading 1",
    h2 "Heading 2",
    h3 "Heading 3",
    h4 "Heading 4",
    h5 "Heading 5",
    h6 "Heading 6",
    head "Container for document metadata",
    header "Introductory or navigational aids for a page or section",
    hgroup "Heading container",
    hr @void "Thematic break",
    html "Root element",
    i "Alternate voice",
    iframe "Child navigable",
    img @void "Image",
    input @void "Form control",
    ins "An addition to the document",
    kbd "User input",
    label "Caption for a form control",
    legend "Caption for fieldset",
    li "List item",
    link @void "Link metadata",
    main "Container for the dominant contents of the document",
    map "Image map",
    mark "Highlight",
    math "mathml root",
    menu "Menu of commands",
    meta @void "Text metadata",
    meter "Gauge",
    nav "Section with navigational links",
    noscript "Fallback content for script",
    object "Image, child navigable, or plugin",
    ol "Ordered list",
    optgroup "Group of options in a list box",
    option "Option in a list box or combo box control",
    output "Calculated output value",
    p "Paragraph",
    picture "Image",
    pre "Block of preformatted text",
    progress "Progress bar",
    q "Quotation",
    rp "Parenthesis for ruby annotation text",
    rt "Ruby annotation text",
    ruby "Ruby annotation(s)",
    s "Inaccurate text",
    samp "Computer output",
    script "Embedded script",
    search "Container for search controls",
    section "Generic document or application section",
    select "List box control",
    selectedcontent "Mirrors content from an option",
    slot "Shadow tree slot",
    small "Side comment",
    source @void "Image source for img or media source for video or audio",
    span "Generic phrasing container",
    strong "Importance",
    style "Embedded styling information",
    sub "Subscript",
    summary "Caption for details",
    sup "Superscript",
    svg "svg root",
    table "Table",
    tbody "Group of rows in a table",
    td "Table cell",
    template "Template",
    textarea "Multiline text controls",
    tfoot "Group of footer rows in a table",
    th "Table header cell",
    thead "Group of heading rows in a table",
    time "Machine-readable equivalent of date- or time-related data",
    title "Document title",
    tr "Table row",
    track @void "Timed text track",
    u "Unarticulated annotation",
    ul "List",
    var "Variable",
    video "Video player",
    wbr @void "Line breaking opportunity",
);
