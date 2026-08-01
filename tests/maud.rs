use hypo::*;
use maud::html;

macro_rules! assert {
    ($maud:expr, $hypo:expr) => {{
        let mut s = String::new();
        $hypo.render(&mut s);
        assert_eq!($maud.0, s);
    }};
}

#[test]
fn index() {
    assert!(
        html! {
            h1 { "Hello, world!" }
            p.intro {
                "This is an example of the "
                a href="https://github.com/lambda-fairy/maud" { "Maud" }
                " template language."
            }
        },
        (
            h1!("Hello, world!"),
            p!(
                class = "intro",
                "This is an example of the ",
                a!(href = "https://github.com/lambda-fairy/maud", "Maud"),
                " template language."
            )
        )
    );
}

#[test]
fn getting_started() {
    let name = "Lyra";
    assert!(html! { p { "Hi, " (name) "!" } }, p!("Hi, ", name, "!"));
}

#[test]
fn text_escaping_text() {
    assert!(
        html! { "Oatmeal, are you crazy?" },
        "Oatmeal, are you crazy?"
    );
}

#[test]
fn text_escaping_raw_strings() {
    assert!(
        html! {
            pre {
                r#"
                    Rocks, these are my rocks.
                    Sediments make me sedimental.
                    Smooth and round,
                    Asleep in the ground.
                    Shades of brown
                    And gray.
                "#
            }
        },
        pre!(
            r#"
                    Rocks, these are my rocks.
                    Sediments make me sedimental.
                    Smooth and round,
                    Asleep in the ground.
                    Shades of brown
                    And gray.
                "#
        )
    );
}

#[test]
fn text_escaping_prescaped() {
    assert!(
        html! {
            "<script>alert(\"XSS\")</script>" // &lt;script&gt;...
            (maud::PreEscaped("<script>alert(\"XSS\")</script>")) // <script>...
        },
        (
            "<script>alert(\"XSS\")</script>",
            (Raw("<script>alert(\"XSS\")</script>"))
        )
    );
}

#[test]
fn text_escaping_doctype() {
    assert!(html! { (maud::DOCTYPE) }, DOCTYPE);
}

#[test]
fn elements_attributes_contents() {
    assert!(
        html! {
            h1 { "Poem" }
            p {
                strong { "Rock," }
                " you are a rock."
            }
        },
        (h1!("Poem"), p!(strong!("Rock,"), " you are a rock."))
    );
}

#[test]
fn elements_attributes_void() {
    assert!(
        html! {
            link rel="stylesheet" href="poetry.css";
            p {
                "Rock, you are a rock."
                br;
                "Gray, you are gray,"
                br;
                "Like a rock, which you are."
                br;
                "Rock."
            }
        },
        (
            link!(rel = "stylesheet", href = "poetry.css"),
            p!(
                "Rock, you are a rock.",
                br!(),
                "Gray, you are gray,",
                br!(),
                "Like a rock, which you are.",
                br!(),
                "Rock."
            )
        )
    );
}

#[test]
fn elements_attributes_custom() {
    assert!(
        html! {
            article dataindex="12345" {
                h1 { "My blog" }
                tag-cloud { "pinkie pie pony cute" }
            }
        },
        article!(
            dataindex = "12345",
            h1!("My blog"),
            element!("tag-cloud", "pinkie pie pony cute")
        )
    );
}

#[test]
fn elements_attributes_nonempty() {
    assert!(
        html! {
            ul {
                li {
                    a href="about:blank" { "Apple Bloom" }
                }
                li class="lower-middle" {
                    "Sweetie Belle"
                }
                li dir="rtl" {
                    "Scootaloo "
                    small { "(also a chicken)" }
                }
            }
        },
        ul!(
            li!(a!(href = "about:blank", "Apple Bloom")),
            li!(class = "lower-middle", "Sweetie Belle"),
            li!(dir = "rtl", "Scootaloo ", small!("(also a chicken)"))
        )
    );
}

#[test]
fn elements_attributes_empty() {
    assert!(
        html! {
            form {
                input type="checkbox" name="cupcakes" checked;
                " "
                label for="cupcakes" { "Do you like cupcakes?" }
            }
        },
        form!(
            input!(type = "checkbox", name = "cupcakes", checked = true),
            " ",
            label!(for = "cupcakes", "Do you like cupcakes?")
        )
    );
}

#[test]
fn elements_attributes_classes_ids() {
    assert!(
        html! {
            input #cannon .big.scary.bright-red type="button" value="Launch Party Cannon";
        },
        input!(
            class = "big scary bright-red",
            id = "cannon",
            type = "button",
            value = "Launch Party Cannon"
        )
    );
}

#[test]
fn elements_attributes_quoted() {
    assert!(
        html! {
            div."col-sm-2" { "Bootstrap column!" }
        },
        div!(class = "col-sm-2", "Bootstrap column!")
    );
}

#[test]
fn elements_attributes_implicit() {
    assert!(
        html! {
            #main {
                "Main content!"
                .tip { "Storing food in a refrigerator can make it 20% cooler." }
            }
        },
        div!(
            id = "main",
            "Main content!",
            div!(
                class = "tip",
                "Storing food in a refrigerator can make it 20% cooler."
            )
        )
    );
}

#[test]
fn splices_toggles_splices() {
    let best_pony = "Pinkie Pie";
    let numbers = [1, 2, 3, 4];
    assert!(
        html! {
            p { "Hi, " (best_pony) "!" }
            p {
                "I have " (numbers.len()) " numbers, "
                "and the first one is " (numbers[0])
            }
        },
        (
            p!("Hi, ", best_pony, "!"),
            p!(
                ("I have ", numbers.len(), " numbers, "),
                ("and the first one is ", numbers[0])
            )
        )
    );
}

#[test]
fn splices_toggles_attributes() {
    let secret_message = "Surprise!";
    assert!(
        html! {
            p title=(secret_message) {
                "Nothing to see here, move along."
            }
        },
        p!(title = secret_message, "Nothing to see here, move along.")
    );
}

#[test]
fn splices_toggles_attributes_concat() {
    const GITHUB: &'static str = "https://github.com";
    assert!(
        html! {
            a href={ (GITHUB) "/lambda-fairy/maud" } {
                "Fork me on GitHub"
            }
        },
        a!(href = (GITHUB, "/lambda-fairy/maud"), "Fork me on GitHub")
    );
}

#[test]
fn splices_toggles_classes_ids() {
    let name = "rarity";
    let severity = "critical";
    assert!(
        html! {
            aside #(name) {
                p.{ "color-" (severity) } { "This is the worst! Possible! Thing!" }
            }
        },
        aside!(
            id = name,
            p!(
                class = ("color-", severity),
                "This is the worst! Possible! Thing!"
            )
        )
    );
}

#[test]
fn splices_toggles_spliced() {
    let post = "<p>Pre-escaped</p>";
    assert!(
        html! {
            h1 { "My super duper blog post" }
            (maud::PreEscaped(post))
        },
        (h1!("My super duper blog post"), Raw(post))
    );
}

#[test]
fn splices_toggles_toggles() {
    let allow_editing = true;
    assert!(
        html! {
            p contenteditable[allow_editing] {
                "Edit me, I "
                em { "dare" }
                " you."
            }
        },
        p!(
            contenteditable = allow_editing,
            "Edit me, I ",
            em!("dare"),
            " you."
        )
    );
}

#[test]
fn splices_toggles_classes() {
    let cuteness = 95;
    assert!(
        html! {
            p.cute[cuteness > 50] { "Squee!" }
        },
        p!(class = (cuteness > 50).then_some("cute"), "Squee!")
    );
}

#[test]
fn splices_toggles_optional() {
    assert!(
        html! {
            p title=[Some("Good password")] { "Correct horse" }

            @let value = Some(42);
            input value=[value];

            @let title: Option<&str> = None;
            p title=[title] { "Battery staple" }
        },
        (
            p!(title = Some("Good password"), "Correct horse"),
            {
                let value = Some(42);
                input!(value = value)
            },
            {
                let title: Option<&str> = None;
                p!(title = title, "Battery staple")
            }
        )
    );
}

#[test]
fn control_structures_if() {
    #[derive(PartialEq)]
    enum Princess {
        Celestia,
        Luna,
    }

    let maud = |user: Princess| {
        html! {
            @if user == Princess::Luna {
                h1 { "Super secret woona to-do list" }
                ul {
                    li { "Nuke the Crystal Empire" }
                    li { "Kick a puppy" }
                    li { "Evil laugh" }
                }
            } @else if user == Princess::Celestia {
                p { "Sister, please stop reading my diary. Its rude." }
            } @else {
                p { "Nothing to see here; move along." }
            }
        }
    };

    let hypo = |user: Princess| {
        matches!(user, Princess::Luna)
            .then_some((
                h1!("Super secret woona to-do list"),
                ul!(
                    li!("Nuke the Crystal Empire"),
                    li!("Kick a puppy"),
                    li!("Evil laugh")
                ),
            ))
            .ok_or(
                matches!(user, Princess::Celestia)
                    .then_some(p!("Sister, please stop reading my diary. Its rude."))
                    .ok_or(p!("Nothing to see here; move along.")),
            )
    };

    assert!(maud(Princess::Celestia), hypo(Princess::Celestia));
    assert!(maud(Princess::Luna), hypo(Princess::Luna));
}

#[test]
fn control_structures_iflet() {
    let user = Some("Pinkie Pie");
    assert!(
        html! {
            p {
                "Hello, "
                @if let Some(name) = user {
                    (name)
                } @else {
                    "stranger"
                }
                "!"
            }
        },
        p!(
            "Hello, ",
            if let Some(name) = user {
                name
            } else {
                "stranger"
            },
            "!"
        )
    );
}

#[test]
fn control_structures_for() {
    let names = ["Applejack", "Rarity", "Fluttershy"];
    assert!(
        html! {
            p { "My favorite ponies are:" }
            ol {
                @for name in &names {
                    li { (name) }
                }
            }
        },
        (
            p!("My favorite ponies are:"),
            ol!(names.iter().map(|name| li!(*name)).collect::<Vec<_>>())
        )
    );
}

#[test]
fn control_structures_let() {
    let names = ["Applejack", "Rarity", "Fluttershy"];
    assert!(
        html! {
            @for name in &names {
                @let first_letter = name.chars().next().unwrap();
                p {
                    "The first letter of "
                    b { (name) }
                    " is "
                    b { (first_letter) }
                    "."
                }
            }
        },
        names
            .into_iter()
            .map(|name| {
                let first_letter = name.chars().next().unwrap();
                p!(
                    "The first letter of ",
                    b!(name),
                    " is ",
                    b!(first_letter),
                    "."
                )
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn control_structures_match() {
    #[allow(dead_code, reason = "maud has it")]
    enum Princess {
        Celestia,
        Cadance,
        Luna,
    }

    let maud = |user: Princess| {
        html! {
            @match user {
                Princess::Luna => {
                    h1 { "Super secret woona to-do list" }
                    ul {
                        li { "Nuke the Crystal Empire" }
                        li { "Kick a puppy" }
                        li { "Evil laugh" }
                    }
                },
                Princess::Celestia => {
                    p { "Sister, please stop reading my diary. Its rude." }
                },
                _ => p { "Nothing to see here; move along." }
            }
        }
    };

    let hypo = |user: Princess| {
        matches!(user, Princess::Luna)
            .then_some((
                h1!("Super secret woona to-do list"),
                ul!(
                    li!("Nuke the Crystal Empire"),
                    li!("Kick a puppy"),
                    li!("Evil laugh")
                ),
            ))
            .ok_or(
                matches!(user, Princess::Celestia)
                    .then_some(p!("Sister, please stop reading my diary. Its rude."))
                    .ok_or(p!("Nothing to see here; move along.")),
            )
    };

    assert!(maud(Princess::Celestia), hypo(Princess::Celestia));
    assert!(maud(Princess::Cadance), hypo(Princess::Cadance));
    assert!(maud(Princess::Luna), hypo(Princess::Luna));
}

#[test]
fn partials() {
    mod maud {
        use maud::{DOCTYPE, Markup, html};

        fn header(page_title: &str) -> Markup {
            html! {
                (DOCTYPE)
                meta charset="utf-8";
                title { (page_title) }
            }
        }

        fn footer() -> Markup {
            html! {
                footer {
                    a href="rss.atom" { "RSS Feed" }
                }
            }
        }

        pub fn page(title: &str, greeting_box: Markup) -> Markup {
            html! {
                (header(title))
                h1 { (title) }
                (greeting_box)
                (footer())
            }
        }
    }

    mod hypo {
        use hypo::{DOCTYPE, Render, a, footer, h1, meta, title};

        fn header(page_title: &str) -> impl Render {
            (DOCTYPE, meta!(charset = "utf-8"), title!(page_title))
        }

        fn footer() -> impl Render {
            footer!(a!(href = "rss.atom", "RSS Feed"))
        }

        pub fn page(title: &str, greeting_box: impl Render) -> impl Render {
            (header(title), h1!(title), greeting_box, footer())
        }
    }

    assert!(
        maud::page("Hello", html! { div { "Greetings, Maud." } }),
        hypo::page("Hello", div!("Greetings, Maud."))
    )
}
