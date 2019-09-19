// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*! # Well-Known XML Namespaces

This crate contains `&'static str` constants for many well-known [XML namespaces][TR-xmlns] (i.e. used not only
within a single company and/or in public documents/APIs). You can use this crate when implementing
a namespace-aware XML processor and want to provide some namespaces with first-class support.

You may also find this useful as a structured tree/list of XML namespaces that are used in the wild.
The namespaces are sorted by the organization that registered, submitted or defined them, not
necessarily by the actual URI used as the name. If an organization defined a lot of namespaces,
they are grouped into separate modules.

Note that when using this crate, only the strings that are actually used will be included in a
generated binary. This is the default behavior of the rust compiler and not something to be
configured in this crate.

If you find any namespace missing, feel free to file an [issue][issue].

[issue]: https://github.com/xiphoseer/xmlns.rs/issues
[TR-xmlns]: https://w3.org/TR/xml-names
*/
macro_rules! DECLARE_NS_NAME {
    ($name:ident: $doc:literal: $value:literal) => (
        DECLARE_NS_NAME!($name: $doc: $value ($value));
    );
    ($name:ident: $doc:literal: $doc_value:literal ($value:expr)) => (
        #[doc = $doc]
        #[doc = "<br>`"]
        #[doc = $doc_value]
        #[doc = "`"]
        pub const $name: &'static str = $value;
    );
}

pub mod android;
pub mod apache;
pub mod daisy;
pub mod dcmi;
pub mod eclipse;
pub mod ecma;
pub mod ibm;
pub mod ietf;
pub mod idpf;
pub mod iso;
pub mod liberty;
pub mod microsoft;
pub mod mozilla;
pub mod mpeg;
pub mod oasis;
pub mod omg;
pub mod opengis;
pub mod other;
pub mod w3c;
pub mod xmpp;
pub mod iptc;
