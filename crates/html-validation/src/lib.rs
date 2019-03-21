//! The html-validation crate provides method that can be used when validating html elements
//! and attributes.
//!
//! The original goal of this crate was to be used as a dependency in procedural macros that
//! validate html at compile time, but it is general purpose and can be used in other problem
//! spaces.
//!
//! ## Optimistic Validation
//!
//! The html-validation crate is optimistic by nature.
//!
//! This means that as we develop the create we'll blacklist more and more things - but in general
//! we default to not saying that something is invalid until we've specifically encoded that it is
//! not allowed.
//!
//! This means that you'll see methods with names like `is_definitely_invalid_child` - hinting
//! that we're telling you that we're certain that the relationship is not allowed.
//!
//! Over time we'll cover more and more cases and this should become a non issue, but at the
//! beginning it will mean that our validation is less strict than it should really be.
//!
//! The reason behind this strategy is that it lets people get up and running from day one without
//! needing to wait until our validation is perfect.
//! A downside is that as we become more and more strict there might be situations where you have
//! to go back and tweak your html if you had something that we are now calling invalid.

#![deny(missing_docs)]

pub use self_closing::is_self_closing;
pub use svg_namespace::is_svg_namespace;

mod self_closing;
mod svg_namespace;

