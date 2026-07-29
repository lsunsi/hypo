#[doc(hidden)]
pub struct Element<'a, const VOID: bool>(&'a mut String, &'static str);

impl<'a, const VOID: bool> Element<'a, VOID> {
    pub fn open(s: &'a mut String, name: &'static str) -> Self {
        s.push('<');
        s.push_str(name);
        Element(s, name)
    }
}

impl<const VOID: bool> Drop for Element<'_, VOID> {
    fn drop(&mut self) {
        if !VOID {
            self.0.push_str("</");
            self.0.push_str(self.1);
            self.0.push('>');
        }
    }
}

impl<const VOID: bool> std::ops::Deref for Element<'_, VOID> {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<const VOID: bool> std::ops::DerefMut for Element<'_, VOID> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
