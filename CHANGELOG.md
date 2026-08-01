# Changelog

## [Unreleased]
- **Added**: Tuples of up to 16 elements implement Render
- **Added**: Trailing commas for children and attributes
- **Added**: Render Fn takes FnMut now (because it's broader)
- **Changed**: Void elements now won't compile with children
- **Changed**: Element macro now defaults to non-void
- **Changed**: Render trait takes ownership instead of borrow
- **Removed**: Render trait no longer implemented for slice

## [0.1.1]
- **Changed**: Replaced element list (maybe some got removed, or some got added)
- **Changed**: Macros now yield tuples instead of fns
- **Fixed**: Bug that prevented nesting interpolation deeply

## [0.1.0](https://github.com/lsunsi/hypotext/releases/tag/v0.1.0)
- **Added**: Render, Element, Attributes, macro. In short, hypo.

<!--**Added** for new features.-->
<!--**Changed** for changes in existing functionality.-->
<!--**Deprecated** for soon-to-be removed features.-->
<!--**Removed** for now removed features.-->
<!--**Fixed** for any bug fixes.-->
<!--**Security** in case of vulnerabilities. -->
