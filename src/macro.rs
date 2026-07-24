#[macro_export]
macro_rules! render {
    ($el:ident => ($key:ident = $value:expr$(, $($tt:tt)*)?) -> ($($attrs:tt)*)) => {
        $crate::render!($el => ($($($tt)*)*) -> ((stringify!($key), $value, $($attrs)*)))
    };
    ($el:ident => ($($child:expr),*) -> ($($attrs:tt)*)) => {
        $crate::Attributes($($attrs)*).render(&mut $el);
        $($crate::Render::render(&$child, &mut $el);)*
    };
}

#[macro_export]
macro_rules! element {
    ($name:expr, $void:literal, $($tt:tt)*) => {
        $crate::Fn(move |s: &mut String| {
            let mut el = $crate::Element::<$void>::open(s, $name);
            $crate::render!(el => ($($tt)*) -> (()));
        })
    };
}

macro_rules! elements {
    ($dol:tt, $($name:ident $void:literal $doc:literal)+) => {
        $(
            #[doc = $doc]
            #[macro_export]
            macro_rules! $name {
                ($dol($doltt:tt)*) => {
                    $crate::element!(stringify!($name), $void, $dol($doltt)*)
                };
            }
        )*
    };
}

elements!($, // https://www.w3.org/TR/2012/WD-html-markup-20121025/elements.html
    a false "hyperlink"
    abbr false "abbreviation"
    address false "contact information"
    area true "image-map hyperlink"
    article false "article"
    aside false "tangential content"
    audio false "audio stream"
    b false "offset text conventionally styled in bold"
    base true "base URL"
    bdi false "BiDi isolate"
    bdo false "BiDi override"
    blockquote false "block quotation"
    body false "document body"
    br true "line break"
    button false "button"
    canvas false "canvas for dynamic graphics"
    caption false "table title"
    cite false "cited title of a work"
    code false "code fragment"
    col true "table column"
    colgroup false "table column group"
    command true "command"
    datalist false "predefined options for other controls"
    dd false "description or value"
    del false "deleted text"
    details false "control for additional on-demand information"
    dfn false "defining instance"
    div false "generic flow container"
    dl false "description list"
    dt false "term or name"
    em false "emphatic stress"
    embed true "integration point for plugins"
    fieldset false "set of related form controls"
    figcaption false "figure caption"
    figure false "figure with optional caption"
    footer false "footer"
    form false "user-submittable form"
    h1 false "heading"
    h2 false "heading"
    h3 false "heading"
    h4 false "heading"
    h5 false "heading"
    h6 false "heading"
    head false "document metadata container"
    header false "header"
    hgroup false "heading group"
    hr true "thematic break"
    html false "root element"
    i false "offset text conventionally styled in italic"
    iframe false "nested browsing context (inline frame)"
    img true "image"
    input true "input control"
    ins false "inserted text"
    kbd false "user input"
    keygen true "key-pair generator/input control"
    label false "caption for a form control"
    legend false "title or explanatory caption"
    li false "list item"
    link true "inter-document relationship metadata"
    map false "image-map definition"
    mark false "marked (highlighted) text"
    menu false "list of commands"
    meta true "metadata"
    meter false "scalar gauge"
    nav false "group of navigational links"
    noscript false "fallback content for script"
    object false "generic external content"
    ol false "ordered list"
    optgroup false "group of options"
    option false "option"
    output false "result of a calculation in a form"
    p false "paragraph"
    param true "initialization parameters for plugins"
    pre false "preformatted text"
    progress false "progress indicator"
    q false "quoted text"
    rp false "ruby parenthesis"
    rt false "ruby text"
    ruby false "ruby annotation"
    s false "struck text"
    samp false "(sample) output"
    script false "embedded script"
    section false "section"
    select false "option-selection form control"
    small false "small print"
    source true "media source"
    span false "generic span"
    strong false "strong importance"
    style false "style (presentation) information"
    sub false "subscript"
    summary false "summary, caption, or legend for a details control"
    sup false "superscript"
    table false "table"
    tbody false "table row group"
    td false "table cell"
    textarea false "text input area"
    tfoot false "table footer row group"
    th false "table header cell"
    thead false "table heading group"
    time false "date and/or time"
    title false "document title"
    tr false "table row"
    track true "supplementary media track"
    u false "offset text conventionally styled with an underline"
    ul false "unordered list"
    var false "variable or placeholder text"
    video false "video"
    wbr true "line-break opportunity "
);
