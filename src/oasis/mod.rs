// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The Organization for the Advancement of Structured Information Standards (OASIS)
//!
//! Source: <http://docs.oasis-open.org>

/// The Relax-NG Schema Format
pub mod relax_ng {
    DECLARE_NS_NAME!(NS_STRUCTURE: "The Structure Namespace":
        "http://relaxng.org/ns/structure/1.0");
    DECLARE_NS_NAME!(NS_ANNOTATION: "The Annotation Namespace":
        "http://relaxng.org/ns/annotation/1.0");

    /// Compatibility Namespaces
    pub mod compatibility {
        DECLARE_NS_NAME!(NS_ANNOTATIONS: "The Annotations Namespace":
            "http://relaxng.org/ns/compatibility/annotations/1.0");
    }
}

/// The Unstructured Information Management Architecture (UIMA)
pub mod uima {
    DECLARE_NS_NAME!(NS_BASE: "The Base Namespace":
        "http://docs.oasis-open.org/uima/ns/base.ecore");
    DECLARE_NS_NAME!(NS_PE: "The Processing Element (PE) Namespace":
        "http://docs.oasis-open.org/uima/ns/pe.ecore");
    DECLARE_NS_NAME!(NS_PE_METADATA: "The PE Metadata Namespace":
        "http://docs.oasis-open.org/uima/ns/peMetadata.ecore");
    DECLARE_NS_NAME!(NS_PE_SERVICE: "The PE Service Namespace":
        "http://docs.oasis-open.org/uima/ns/peService");
}

/// The DocBook Standard
///
/// Source: <https://docbook.org/>
pub mod docbook {
    DECLARE_NS_NAME!(NS: "The DocBook Namespace": "http://docbook.org/ns/docbook");
}

/// The Darwin Information Typing Architecture (DITA)
pub mod dita {
    DECLARE_NS_NAME!(NS_ARCHITECTURE: "The Architecture Namespace":
        "http://dita.oasis-open.org/architecture/2005/");
}

/// The electronic business XML Standard (ebXML)
pub mod ebxml {
    DECLARE_NS_NAME!(NS_MESSAGE_HEADER: "The Message Header Namespace":
        "http://www.ebxml.org/namespaces/messageHeader");
}

/// The `urn:oasis:names:tc:xacml` Namespaces
pub mod xacml {
    /// The `urn:oasis:names:tc:xacml:3.0` Namespaces
    pub mod v3 {
        /// The `urn:oasis:names:tc:xacml:3.0:core` Namespaces
        pub mod core {
            /// The `urn:oasis:names:tc:xacml:3.0:core:schema` Namespaces
            pub mod schema {
                DECLARE_NS_NAME!(NS_WD_17: "The WD-17 Namespace":
                    "urn:oasis:names:tc:xacml:3.0:core:schema:wd-17");
            }
        }
    }
}

pub mod opendocument;
pub mod saml;
