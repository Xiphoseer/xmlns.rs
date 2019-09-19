// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The SAML Version 2.0

DECLARE_NS_NAME!(NS_ASSERTION: "The SAML Assertion Namespace":
    "urn:oasis:names:tc:SAML:2.0:assertion");
DECLARE_NS_NAME!(NS_PROTOCOL: "The SAML Protocol Namespace":
    "urn:oasis:names:tc:SAML:2.0:protocol" (protocol::NS));
DECLARE_NS_NAME!(NS_METADATA: "The SAML Metadata Namespace":
    "urn:oasis:names:tc:SAML:2.0:metadata" (metadata::NS));
DECLARE_NS_NAME!(NS_AC: "The Authentication Context Namespace":
    "urn:oasis:names:tc:SAML:2.0:ac" (ac::NS));

/// The Confirmation Methods Namespaces <br> `urn:oasis:names:tc:SAML:2.0:cm`
pub mod cm {
    DECLARE_NS_NAME!(NS_KERBEROS: "The Kerberos Namespace":
        "urn:oasis:names:tc:SAML:2.0:cm:kerberos");
}

/// The `urn:oasis:names:tc:SAML:2.0:conditions` Namespaces
pub mod conditions {
    DECLARE_NS_NAME!(NS_DELEGATION: "The Delegation Namespace":
        "urn:oasis:names:tc:SAML:2.0:conditions:delegation");
}

/// The `urn:oasis:names:tc:SAML:2.0:metadata` Namespaces
pub mod metadata {
    DECLARE_NS_NAME!(NS: "The Metadata Namespace":
        "urn:oasis:names:tc:SAML:2.0:metadata");
}

/// The `urn:oasis:names:tc:SAML:2.0:protocol` Namespaces
pub mod protocol {
    DECLARE_NS_NAME!(NS: "The SAML Protocol Namespace":
        "urn:oasis:names:tc:SAML:2.0:protocol");

    /// The `urn:oasis:names:tc:SAML:2.0:protocol:ext` Namespaces
    pub mod ext {
        DECLARE_NS_NAME!(NS_ASYNC_SLO: "The Async SLO Namespace":
            "urn:oasis:names:tc:SAML:2.0:protocol:ext:async-slo");
        DECLARE_NS_NAME!(NS_CHANNEL_BINDING: "The Channel Binding Namespace":
            "urn:oasis:names:tc:SAML:protocol:ext:channel-binding");
    }
}

/// The `urn:oasis:names:tc:SAML:2.0:profiles` Namespaces
pub mod profiles {
    DECLARE_NS_NAME!(NS_SSO_ECP: "The ECP Profile Namespace":
        "urn:oasis:names:tc:SAML:2.0:profiles:SSO:ecp");
    DECLARE_NS_NAME!(NS_KERBEROS_SSO_BROWSER: "The Kerberos SSO Browser Namespace":
        "urn:oasis:names:tc:SAML:2.0:profiles:kerberos:SSO:browser");
    DECLARE_NS_NAME!(NS_HOLDER_OF_KEY: "The Holder-of-Key Namespace":
        "urn:oasis:names:tc:SAML:2.0:profiles:holder-of-key" (holder_of_key::NS));
    DECLARE_NS_NAME!(NS_SESSION_METADATA: "The Session Metadata Namespace":
        "urn:oasis:names:tc:SAML:2.0:profiles:session:metadata");
    DECLARE_NS_NAME!(NS_ATTRIBUTE_PREDICATES: "The Attribute Predicates Namespace":
        "http://www.zurich.ibm.com/csc/security/SAMLAttributePredicatesProfile");

    /// The Holder-of-Key Namespaces <br> `urn:oasis:names:tc:SAML:2.0:profiles:holder-of-key`
    pub mod holder_of_key {
        DECLARE_NS_NAME!(NS: "The Holder-of-Key Namespace":
            "urn:oasis:names:tc:SAML:2.0:profiles:holder-of-key");
        DECLARE_NS_NAME!(NS_SSO_BROWSER: "The SSO Browser Namespace":
            "urn:oasis:names:tc:SAML:2.0:profiles:holder-of-key:SSO:browser");
    }

    /// The Attribute Namespaces <br> `urn:oasis:names:tc:SAML:2.0:profiles:attribute`
    pub mod attribute {
        DECLARE_NS_NAME!(NS_DCE: "The DCE PAC Attribute Profile Namespace":
            "urn:oasis:names:tc:SAML:2.0:profiles:attribute:DCE");
        DECLARE_NS_NAME!(NS_X500: "The X500 Attribute Profile Namespace":
            "urn:oasis:names:tc:SAML:2.0:profiles:attribute:X500");
        DECLARE_NS_NAME!(NS_XACML: "The XACML Attribute Profile Namespace":
            "urn:oasis:names:tc:SAML:2.0:profiles:attribute:XACML");
        DECLARE_NS_NAME!(NS_LDAP: "The LDAP Attribute Profile Namespace":
            "urn:oasis:names:tc:SAML:2.0:profiles:attribute:LDAP");
        DECLARE_NS_NAME!(NS_KERBEROS: "The Kerberos Attribute Profile Namespace":
            "urn:oasis:names:tc:SAML:2.0:profiles:attribute:kerberos");
    }
}

/// The Authentication Context Namespaces <br> `urn:oasis:names:tc:SAML:2.0:ac`
pub mod ac {
    DECLARE_NS_NAME!(NS: "The Namespace":
        "urn:oasis:names:tc:SAML:2.0:ac");

    /// The `urn:oasis:names:tc:SAML:2.0:ac:classes` Namespaces
    pub mod classes {
        DECLARE_NS_NAME!(NS_XMLDSIG: "The XML DSig Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:XMLDSig");
        DECLARE_NS_NAME!(NS_SPKI: "The SPKI Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:SPKI");
        DECLARE_NS_NAME!(NS_SMARTCARD: "The Smartcard Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:Smartcard");
        DECLARE_NS_NAME!(NS_SMARTCARD_PKI: "The SmartcardPKI Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:SmartcardPKI");
        DECLARE_NS_NAME!(NS_SOFTWARE_PKI: "The SoftwarePKI Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:SoftwarePKI");
        DECLARE_NS_NAME!(NS_TELEPHONY: "The Telephony Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:Telephony");
        DECLARE_NS_NAME!(NS_NOMAD_TELEPHONY: "The Nomad Telephony Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:NomadTelephony");
        DECLARE_NS_NAME!(NS_PERSONALIZED_TELEPHONY: "The Personalized Telephony Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:PersonalizedTelephony");
        DECLARE_NS_NAME!(NS_AUTHENTICATED_TELEPHONY: "The Authenticated Telephony Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:AuthenticatedTelephony");
        DECLARE_NS_NAME!(NS_SECURE_REMOTE_PASSWORD: "The Secure Remote Password Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:SecureRemotePassword");
        DECLARE_NS_NAME!(NS_TLS_CLIENT: "The TLS Client Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:TLSClient");
        DECLARE_NS_NAME!(NS_TIME_SYNC_TOKEN: "The Time Sync Token Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:TimeSyncToken");
        DECLARE_NS_NAME!(NS_UNSPECIFIED: "The Unspecified Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:unspecified");
        DECLARE_NS_NAME!(NS_INTERNET_PROTOCOL: "The Internet Protocol Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:InternetProtocol");
        DECLARE_NS_NAME!(NS_INTERNET_PROTOCOL_PASSWORD: "The Internet Protocol Password Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:InternetProtocolPassword");
        DECLARE_NS_NAME!(NS_KERBEROS: "The Kerberos Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:Kerberos");
        DECLARE_NS_NAME!(NS_MOBILE_ONE_FACTOR_CONTRACT: "The Mobile One Factor Contract Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:MobileOneFactorContract");
        DECLARE_NS_NAME!(NS_MOBILE_ONE_FACTOR_UNREGISTERED: "The Mobile One Factor Unregistered Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:MobileOneFactorUnregistered");
        DECLARE_NS_NAME!(NS_MOBILE_TWO_FACTOR_CONTRACT: "The Mobile Two Factor Contract Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:MobileTwoFactorContract");
        DECLARE_NS_NAME!(NS_MOBILE_TWO_FACTOR_UNREGISTERED: "The Mobile Two Factor Unregistered Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:MobileTwoFactorUnregistered");
        DECLARE_NS_NAME!(NS_PGP: "The PGP Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:PGP");
        DECLARE_NS_NAME!(NS_PASSWORD_PROTECTED_TRANSPORT: "The Password Protected Transport Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:PasswordProtectedTransport");
        DECLARE_NS_NAME!(NS_PASSWORD: "The Password Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:Password");
        DECLARE_NS_NAME!(NS_PREVIOUS_SESSION: "The Previous Session Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:PreviousSession");
        DECLARE_NS_NAME!(NS_X509: "The X509 Namespace":
            "urn:oasis:names:tc:SAML:2.0:ac:classes:X509");
    }
}
