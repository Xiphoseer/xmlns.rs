// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The Internet Engineering Task Force (IETF)

/// The XMPP Namespaces
pub mod xmpp {
    DECLARE_NS_NAME!(NS_BIND: "The Bind Namespace": "urn:ietf:params:xml:ns:xmpp-bind");
    DECLARE_NS_NAME!(NS_E2E: "The End to End Namespace": "urn:ietf:params:xml:ns:xmpp-e2e");
    DECLARE_NS_NAME!(NS_SASL: "The SASL Namespace": "urn:ietf:params:xml:ns:xmpp-sasl");
    DECLARE_NS_NAME!(NS_SESSION: "The Session Namespace": "urn:ietf:params:xml:ns:xmpp-session");
    DECLARE_NS_NAME!(NS_STANZAS: "The Stanzas Namespace": "urn:ietf:params:xml:ns:xmpp-stanzas");
    DECLARE_NS_NAME!(NS_STREAMS: "The Streams Namespace": "urn:ietf:params:xml:ns:xmpp-streams");
    DECLARE_NS_NAME!(NS_TLS: "The TLS Namespace": "urn:ietf:params:xml:ns:xmpp-tls");
}
