/*!
The xpct user docs.

This is the xpct user documentation. If you're new here, check out the
[tutorial][crate::docs::tutorial].
*/

#![cfg(docsrs)]
#![cfg_attr(docsrs, doc(cfg(feature = "docs.rs")))]

pub mod cargo_features;
pub mod list_of_matchers;
pub mod tutorial;
pub mod writing_formatters;
pub mod writing_matchers;
