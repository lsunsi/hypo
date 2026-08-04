/// what can be rendered to html
pub trait Render {
    fn render(self, to: &mut String);
}

impl Render for char {
    fn render(self, to: &mut String) {
        crate::escape::char(self, to);
    }
}

impl Render for &str {
    fn render(self, to: &mut String) {
        crate::escape::str(self, to);
    }
}

impl Render for String {
    fn render(self, to: &mut String) {
        crate::escape::str(&self, to);
    }
}

macro_rules! impl_for_numbers {
    ($($number:ty)+) => {$(
        impl Render for $number {
            fn render(self, to: &mut String) {
                #[cfg(feature = "perf")]
                itoap::write_to_string(to, self);
                #[cfg(not(feature = "perf"))]
                to.push_str(&self.to_string());
            }
        }
    )*};
}

impl_for_numbers!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize);

impl<R: Render> Render for Option<R> {
    fn render(self, to: &mut String) {
        if let Some(r) = self {
            r.render(to);
        }
    }
}

impl<R1: Render, R2: Render> Render for Result<R1, R2> {
    fn render(self, to: &mut String) {
        match self {
            Ok(r) => r.render(to),
            Err(r) => r.render(to),
        }
    }
}

impl<const N: usize, R: Render> Render for [R; N] {
    fn render(self, to: &mut String) {
        for r in self {
            r.render(to);
        }
    }
}

impl<R: Render> Render for Vec<R> {
    fn render(self, to: &mut String) {
        for r in self {
            r.render(to);
        }
    }
}

impl<R: Render, I: Iterator, F: FnMut(I::Item) -> R> Render for std::iter::Map<I, F> {
    fn render(self, to: &mut String) {
        for r in self {
            r.render(to);
        }
    }
}

macro_rules! impl_for_tuples {
    ($head:ident) => {
        impl Render for () {
            fn render(self, _: &mut String) {}
        }
    };
    ($head:ident $($tail:ident)+) => {
        impl <$($tail: Render),*> Render for ($($tail),*,) {
            fn render(self, to: &mut String) {
                #[allow(non_snake_case, reason = "macro")]
                let ($($tail),*,) = self;
                ($($tail.render(to)),*);
            }
        }

        impl_for_tuples!($($tail)*);
    };
}

impl_for_tuples!(Q P O N M L K J I H G F E D C B A);

/// renders directly without any escaping
pub struct Raw<T>(pub T);
impl<T: AsRef<str>> Render for Raw<T> {
    fn render(self, to: &mut String) {
        to.push_str(self.0.as_ref());
    }
}

/// render template into a raw string
pub fn render(r: impl Render) -> Raw<String> {
    let mut s = String::new();
    r.render(&mut s);
    Raw(s)
}
