//! Module for built-in filter functions
//!
//! Contains all the built-in filter functions for use in templates.
//! You can define your own filters, as well.
//!
//! ## Note
//!
//! All **result types of any filter function** in this module is **subject to change** at any
//! point, and is **not indicated by as semver breaking** version bump.
//! The traits [`AutoEscape`] and [`WriteWritable`] are used by [`rinja_derive`]'s generated code
//! to work with all compatible types.

#[cfg(feature = "alloc")]
mod alloc;
mod builtin;
mod escape;
mod humansize;
#[cfg(feature = "serde_json")]
mod json;
#[cfg(feature = "urlencode")]
mod urlencode;

#[cfg(feature = "alloc")]
pub use self::alloc::{
    capitalize, fmt, format, indent, linebreaks, linebreaksbr, lower, lowercase, paragraphbreaks,
    title, trim, upper, uppercase, wordcount,
};
pub use self::builtin::{PluralizeCount, center, join, pluralize, truncate};
pub use self::escape::{
    AutoEscape, AutoEscaper, Escaper, FastWritable, Html, HtmlSafe, HtmlSafeOutput, MaybeSafe,
    Safe, Text, Unsafe, Writable, WriteWritable, e, escape, safe,
};
pub use self::humansize::filesizeformat;
#[cfg(feature = "serde_json")]
pub use self::json::{AsIndent, json, json_pretty};
#[cfg(feature = "urlencode")]
pub use self::urlencode::{urlencode, urlencode_strict};

// MAX_LEN is maximum allowed length for filters.
const MAX_LEN: usize = 10_000;
