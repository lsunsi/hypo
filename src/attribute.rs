pub struct Attributes<T>(pub T);

impl<T: Attribute> Attributes<T> {
    pub fn render(self, to: &mut String) {
        self.0.render(to);
        to.push('>');
    }
}

pub trait Attribute {
    fn render(self, to: &mut String) -> &'static str;
}

impl Attribute for () {
    fn render(self, _: &mut String) -> &'static str {
        ""
    }
}

impl<T: Attribute> Attribute for (&'static str, bool, T) {
    fn render(self, to: &mut String) -> &'static str {
        let (name, boolean, next) = self;
        let next = next.render(to);
        if boolean {
            if name != next {
                to.push(' ');
                to.push_str(name);
            }
            name
        } else {
            next
        }
    }
}

impl<T: Attribute, V: crate::Render> Attribute for (&'static str, V, T) {
    fn render(self, to: &mut String) -> &'static str {
        let (name, value, next) = self;

        let next = next.render(to);
        if name == next {
            to.pop();
            to.push(' ');

            let l1 = to.len();
            value.render(to);
            if l1 == to.len() {
                to.pop();
            }

            to.push('"');
            name
        } else {
            let l0 = to.len();
            to.push(' ');
            to.push_str(name);
            to.push_str("=\"");

            let l1 = to.len();
            value.render(to);
            if l1 == to.len() {
                to.truncate(l0);
                next
            } else {
                to.push('"');
                name
            }
        }
    }
}
