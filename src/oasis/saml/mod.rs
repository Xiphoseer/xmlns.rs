// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The Security Assertion Markup Language (SAML)

/// The SAML Version 1.0
pub mod v1_0 {
    /// The `urn:oasis:names:tc:SAML:1.0:action` Namespaces
    pub mod action {
        DECLARE_NS_NAME!(NS_RWEDC_NEGOTIATION: "The RWEDC Negotiation":
            "urn:oasis:names:tc:SAML:1.0:action:rwedc-negation");
    }
}

/// The SAML Version 1.1
pub mod v1_1 {
    /// The `urn:oasis:names:tc:SAML:1.1:profiles` Namespace
    pub mod profiles {
        /// The `urn:oasis:names:tc:SAML:1.1:profiles:assertion` Namespace
        pub mod assertion {
            DECLARE_NS_NAME!(NS_SUBJECT: "The Subject Namespace":
                "urn:oasis:names:tc:SAML:1.1:profiles:assertion:subject");
        }
    }
}

pub mod v2_0;

/// The `urn:oasis:names:tc:SAML:metadata` Namespaces
pub mod metadata {
    DECLARE_NS_NAME!(NS_RPI: "The Registration and Publication Namespace":
        "urn:oasis:names:tc:SAML:metadata:rpi");
    DECLARE_NS_NAME!(NS_ATTRIBUTE: "The Attribute Namespace":
        "urn:oasis:names:tc:SAML:metadata:attribute");
    DECLARE_NS_NAME!(NS_ALGSUPPORT: "The ALG-Support Namespace":
        "urn:oasis:names:tc:SAML:metadata:algsupport");
    DECLARE_NS_NAME!(NS_UI: "The UI Namespace":
        "urn:oasis:names:tc:SAML:metadata:ui");

    /// The X509 Namespaces <br> `urn:oasis:names:tc:SAML:metadata:X509`
    pub mod x509 {
        DECLARE_NS_NAME!(NS_QUERY: "The Query Namespace":
            "urn:oasis:names:tc:SAML:metadata:X509:query");
    }

    /// The Extension Namespaces <br> `urn:oasis:names:tc:SAML:metadata:ext`
    pub mod ext {
        DECLARE_NS_NAME!(NS_QUERY: "The Query Namespace":
            "urn:oasis:names:tc:SAML:metadata:ext:query");
    }
}

/// The `urn:oasis:names:tc:SAML:profiles` Namespaces
pub mod profiles {
    /// The `urn:oasis:names:tc:SAML:profiles:SSO` Namespaces
    pub mod sso {
        DECLARE_NS_NAME!(NS_REQUEST_INIT: "The Request Init Namespace":
            "urn:oasis:names:tc:SAML:profiles:SSO:request-init");
        DECLARE_NS_NAME!(NS_IDP_DISCOVERY_PROTOCOL: "The IDP Discovery Protocol":
            "urn:oasis:names:tc:SAML:profiles:SSO:idp-discovery-protocol");
    }

    DECLARE_NS_NAME!(NS_V1_METADATA: "The V1 Metadata Namespace":
        "urn:oasis:names:tc:SAML:profiles:v1metadata");
}

/// The `urn:oasis:names:tc:SAML:attributes` Namespaces
pub mod attributes {
    DECLARE_NS_NAME!(NS_EXT: "The EXT Namespace":
        "urn:oasis:names:tc:SAML:attributes:ext");
}
