#[doc(hidden)]
#[macro_export]
macro_rules! render {
    ($name:expr, $close:literal => ($key:ident = $value:expr$(, $($tt:tt)*)?) -> ($($attrs:tt)*)) => {
        $crate::render!($name, $close => ($($($tt)*)*) -> (($crate::key!($key), $value, $($attrs)*)))
    };
    ($name:expr, $close:literal => ($($child:expr),*$(,)?) -> ($($attrs:tt)*)) => {
        (
            $crate::Raw(concat!('<', $name)),
            $crate::Attrs($($attrs)*),
            ($($child),*),
            $close.then_some($crate::Raw(concat!("</", $name, '>')))
        )
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
    ($name:expr, $void:literal, $($tt:tt)*) => {
        $crate::render!($name, $void => ($($tt)*) -> (()));
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

elements!($, // https://html.spec.whatwg.org/multipage/indices.html
    a true "Hyperlink"
    abbr true "Abbreviation"
    address true "Contact information for a page or article element"
    area false "Hyperlink or dead area on an image map"
    article true "Self-contained syndicatable or reusable composition"
    aside true "Sidebar for tangentially related content"
    audio true "Audio player"
    b true "Keywords"
    base false "Base URL and default target navigable for hyperlinks and forms"
    bdi true "Text directionality isolation"
    bdo true "Text directionality formatting"
    blockquote true "A section quoted from another source"
    body true "Document body"
    br false "Line break, e.g. in poem or postal address"
    button true "Button control"
    canvas true "Scriptable bitmap canvas"
    caption true "Table caption"
    cite true "Title of a work"
    code true "Computer code"
    col false "Table column"
    colgroup true "Group of columns in a table"
    data true "Machine-readable equivalent"
    datalist true "Container for options for combo box control"
    dd true "Content for corresponding dt element(s)"
    del true "A removal from the document"
    details true "Disclosure control for hiding details"
    dfn true "Defining instance"
    dialog true "Dialog box or window"
    div true "Generic flow container, or container for name-value groups in dl elements"
    dl true "Association list consisting of zero or more name-value groups"
    dt true "Legend for corresponding dd element(s)"
    em true "Stress emphasis"
    embed false "Plugin"
    fieldset true "Group of form controls"
    figcaption true "Caption for figure"
    figure true "Figure with optional caption"
    footer true "Footer for a page or section"
    form true "User-submittable form"
    h1 true "Heading 1"
    h2 true "Heading 2"
    h3 true "Heading 3"
    h4 true "Heading 4"
    h5 true "Heading 5"
    h6 true "Heading 6"
    head true "Container for document metadata"
    header true "Introductory or navigational aids for a page or section"
    hgroup true "Heading container"
    hr false "Thematic break"
    html true "Root element"
    i true "Alternate voice"
    iframe true "Child navigable"
    img false "Image"
    input false "Form control"
    ins true "An addition to the document"
    kbd true "User input"
    label true "Caption for a form control"
    legend true "Caption for fieldset"
    li true "List item"
    link false "Link metadata"
    main true "Container for the dominant contents of the document"
    map true "Image map"
    mark true "Highlight"
    math true "mathml root"
    menu true "Menu of commands"
    meta false "Text metadata"
    meter true "Gauge"
    nav true "Section with navigational links"
    noscript true "Fallback content for script"
    object true "Image, child navigable, or plugin"
    ol true "Ordered list"
    optgroup true "Group of options in a list box"
    option true "Option in a list box or combo box control"
    output true "Calculated output value"
    p true "Paragraph"
    picture true "Image"
    pre true "Block of preformatted text"
    progress true "Progress bar"
    q true "Quotation"
    rp true "Parenthesis for ruby annotation text"
    rt true "Ruby annotation text"
    ruby true "Ruby annotation(s)"
    s true "Inaccurate text"
    samp true "Computer output"
    script true "Embedded script"
    search true "Container for search controls"
    section true "Generic document or application section"
    select true "List box control"
    selectedcontent true "Mirrors content from an option"
    slot true "Shadow tree slot"
    small true "Side comment"
    source false "Image source for img or media source for video or audio"
    span true "Generic phrasing container"
    strong true "Importance"
    style true "Embedded styling information"
    sub true "Subscript"
    summary true "Caption for details"
    sup true "Superscript"
    svg true "svg root"
    table true "Table"
    tbody true "Group of rows in a table"
    td true "Table cell"
    template true "Template"
    textarea true "Multiline text controls"
    tfoot true "Group of footer rows in a table"
    th true "Table header cell"
    thead true "Group of heading rows in a table"
    time true "Machine-readable equivalent of date- or time-related data"
    title true "Document title"
    tr true "Table row"
    track false "Timed text track"
    u true "Unarticulated annotation"
    ul true "List"
    var true "Variable"
    video true "Video player"
    wbr false "Line breaking opportunity"
);
