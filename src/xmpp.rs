// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The XMPP Standards Foundation

/// The Jabber Protocol
pub mod protocol {
    DECLARE_NS_NAME!(NS_ACTIVITY: "The Activity Namespace":
        "http://jabber.org/protocol/activity");
    DECLARE_NS_NAME!(NS_ADDRESS: "The Address Namespace":
        "http://jabber.org/protocol/address");
    DECLARE_NS_NAME!(NS_AMP: "The Advanced Message Processing Namespace":
        "http://jabber.org/protocol/amp" (amp::NS));
    DECLARE_NS_NAME!(NS_AMP_ERRORS: "The AMP Errors Namespace":
        "http://jabber.org/protocol/amp#errors");
    DECLARE_NS_NAME!(NS_BYTESTREAMS: "The Bytestreams Namespace":
        "http://jabber.org/protocol/bytestreams");
    DECLARE_NS_NAME!(NS_CAPS: "The Capabilities Namespace":
        "http://jabber.org/protocol/caps");
    DECLARE_NS_NAME!(NS_CHATSTATE: "The Chat States Namespace":
        "http://jabber.org/protocol/chatstates");
    DECLARE_NS_NAME!(NS_COMMANDS: "The Commands Namespace":
        "http://jabber.org/protocol/commands");
    DECLARE_NS_NAME!(NS_COMPRESS: "The Compression Namespace":
        "http://jabber.org/protocol/compress");
    /// The Discovery Namespaces
    pub mod disco {
        DECLARE_NS_NAME!(NS_INFO: "The Discovery Info Namespace":
            "http://jabber.org/protocol/disco#info");
        DECLARE_NS_NAME!(NS_ITEMS: "The Discovery Items Namespace":
            "http://jabber.org/protocol/disco#items");
    }
    DECLARE_NS_NAME!(NS_FEATURE_NEG: "The Feature Negotiation Namespace":
        "http://jabber.org/protocol/feature-neg");
    DECLARE_NS_NAME!(NS_GEOLOC: "The Geolocation Namespace":
        "http://jabber.org/protocol/geoloc");
    DECLARE_NS_NAME!(NS_HTTP_AUTH: "The HTTP Auth Namespace":
        "http://jabber.org/protocol/http-auth");
    DECLARE_NS_NAME!(NS_HTTPBIND: "The HTTP Bind Namespace":
        "http://jabber.org/protocol/httpbind");
    DECLARE_NS_NAME!(NS_IBB: "The In-Band Bytestreams Namespace":
        "http://jabber.org/protocol/ibb");
    DECLARE_NS_NAME!(NS_MOOD: "The User Mood Namespace":
        "http://jabber.org/protocol/mood");
    DECLARE_NS_NAME!(NS_MUC: "The Multi-User-Chat Namespace":
        "http://jabber.org/protocol/muc");
    DECLARE_NS_NAME!(NS_MUC_ADMIN: "The MUC Admin Namespace":
        "http://jabber.org/protocol/muc#admin");
    DECLARE_NS_NAME!(NS_MUC_OWNER: "The MUC Owner Namespace":
        "http://jabber.org/protocol/muc#owner");
    DECLARE_NS_NAME!(NS_MUC_USER: "The MUC User Namespace":
        "http://jabber.org/protocol/muc#user");
    DECLARE_NS_NAME!(NS_NICK: "The User Nickname Namespace":
        "http://jabber.org/protocol/nick");
    DECLARE_NS_NAME!(NS_OFFLINE: "The Offline Message Retrieval Namespace":
        "http://jabber.org/protocol/offline");
    DECLARE_NS_NAME!(NS_PHYSLOC: "The Physical Location Namespace":
        "http://jabber.org/protocol/physloc");
    DECLARE_NS_NAME!(NS_PUBSUB: "The Publish-Subscribe Namespace":
        "http://jabber.org/protocol/pubsub" (pubsub::NS));

    /// The Publish-Subscribe Namespaces
    pub mod pubsub {
        DECLARE_NS_NAME!(NS: "The Publish-Subscribe Namespace":
            "http://jabber.org/protocol/pubsub");
        DECLARE_NS_NAME!(NS_ERRORS: "The Errors Namespace":
            "http://jabber.org/protocol/pubsub#errors");
        DECLARE_NS_NAME!(NS_EVENT: "The Event Namespace":
            "http://jabber.org/protocol/pubsub#event");
        DECLARE_NS_NAME!(NS_OWNER: "The Owner Namespace":
            "http://jabber.org/protocol/pubsub#owner");
    }

    DECLARE_NS_NAME!(NS_RC: "The Remote Controlling Namespace":
        "http://jabber.org/protocol/rc");
    DECLARE_NS_NAME!(NS_ROSTERX: "The Roster Item Exchange Namespace":
        "http://jabber.org/protocol/rosterx");
    DECLARE_NS_NAME!(NS_SIPUB: "The SI Request Publishing Namespace":
        "http://jabber.org/protocol/sipub");
    DECLARE_NS_NAME!(NS_SOAP: "The SOAP over XMPP Namespace":
        "http://jabber.org/protocol/soap" (soap::NS));

    /// The SOAP over XMPP Namespaces
    pub mod soap {
        DECLARE_NS_NAME!(NS: "The SOAP over XMPP Namespace":
            "http://jabber.org/protocol/soap");
        DECLARE_NS_NAME!(NS_FAULT: "The SOAP over XMPP Fault Namespace":
            "http://jabber.org/protocol/soap#fault");
    }
    DECLARE_NS_NAME!(NS_WAITINGLIST: "The Waiting List Namespace":
        "http://jabber.org/protocol/waitinglist");
    DECLARE_NS_NAME!(NS_XHTML_IM: "The XHTML-IM Namespace":
        "http://jabber.org/protocol/xhtml-im");
    DECLARE_NS_NAME!(NS_XDATA_LAYOUT: "The Data Forms Layout Namespace":
        "http://jabber.org/protocol/xdata-layout");
    DECLARE_NS_NAME!(NS_XDATA_VALIDATE: "The Data Forms Validation":
        "http://jabber.org/protocol/xdata-validate");

    /// The AMP Sub-Namespaces
    pub mod amp {
        DECLARE_NS_NAME!(NS: "The AMP Namespace":
            "http://jabber.org/protocol/amp");

        /// The AMP Actions
        pub mod actions {
            DECLARE_NS_NAME!(NS_DROP: "The Drop Action Namespace":
                "http://jabber.org/protocol/amp?action=drop");
            DECLARE_NS_NAME!(NS_ERROR: "The Error Action Namespace":
                "http://jabber.org/protocol/amp?action=error");
            DECLARE_NS_NAME!(NS_NOTIFY: "The Notify Action Namespace":
                "http://jabber.org/protocol/amp?action=notify");
        }

        /// The AMP Conditions
        pub mod conditions {
            DECLARE_NS_NAME!(NS_DELIVER: "The Deliver Condition Namespace":
                "http://jabber.org/protocol/amp?condition=deliver");
            DECLARE_NS_NAME!(NS_EXPIRE_AT: "The Expire At Condition Namespace":
                "http://jabber.org/protocol/amp?condition=expire-at");
            DECLARE_NS_NAME!(NS_MATCH_RESOURCE: "The Match Resource Condition Namespace":
                "http://jabber.org/protocol/amp?condition=match-resource");
        }
    }
}

/// The `jabber` Namespaces
pub mod jabber {
    DECLARE_NS_NAME!(NS_CLIENT: "The Client Namespace":
        "jabber:client");
    DECLARE_NS_NAME!(NS_SERVER: "The Server Namespace":
        "jabber:server" (server::NS));

    /// The `jabber:server` Namespaces
    pub mod server {
        DECLARE_NS_NAME!(NS: "The Server Namespace":
            "jabber:server");
        DECLARE_NS_NAME!(NS_DIALBACK: "The Server-Dialback Namespace":
            "jabber:server:dialback");
    }

    /// The `jabber:component` Namespaces
    pub mod component {
        DECLARE_NS_NAME!(NS_ACCEPT: "The Accept Namespace":
            "jabber:component:accept");
        DECLARE_NS_NAME!(NS_CONNECT: "The Connect Namespace":
            "jabber:component:connect");
    }

    /// The `jabber:iq` (Query) Namespaces
    pub mod iq {
        DECLARE_NS_NAME!(NS_AUTH: "The Auth Namespace":
            "jabber:iq:auth");
        DECLARE_NS_NAME!(NS_GATEWAY: "The Gateway Namespace":
            "jabber:iq:gateway");
        DECLARE_NS_NAME!(NS_LAST: "The Last Activity Namespace":
            "jabber:iq:last");
        DECLARE_NS_NAME!(NS_OOB: "The Out of Band Namespace":
            "jabber:iq:oob");
        DECLARE_NS_NAME!(NS_PRIVACY: "The Privacy Namespace":
            "jabber:iq:privacy");
        DECLARE_NS_NAME!(NS_PRIVATE: "The Private XML Storage Namespace":
            "jabber:iq:private");
        DECLARE_NS_NAME!(NS_REGISTER: "The In-Band Registration Namespace":
            "jabber:iq:register");
        DECLARE_NS_NAME!(NS_ROSTER: "The Roster Namespace":
            "jabber:iq:roster");
        DECLARE_NS_NAME!(NS_RPC: "The Remote Procedure Call Namespace":
            "jabber:iq:rpc");
        DECLARE_NS_NAME!(NS_SEARCH: "The Search Namespace":
            "jabber:iq:search");
        DECLARE_NS_NAME!(NS_VERSION: "The Version Namespace":
            "jabber:iq:version");
    }

    /// The `jabber:x` Namespaces
    pub mod x {
        DECLARE_NS_NAME!(NS_CONFERENCE: "The Conference Namespace":
            "jabber:x:conference");
        DECLARE_NS_NAME!(NS_DATA: "The Data Forms Namespace":
            "jabber:x:data");
        DECLARE_NS_NAME!(NS_ENCRYPTED: "The Encrpytion Namespace":
            "jabber:x:encrypted");
        DECLARE_NS_NAME!(NS_OOB: "The Out of Band Data Namespace":
            "jabber:x:oob");
        DECLARE_NS_NAME!(NS_SIGNED: "The Signed Namespace":
            "jabber:x:signed");
    }
}

/// The `roster:` Namespaces
pub mod roster {
    DECLARE_NS_NAME!(NS_DELIMITER: "The Delimiter Namespace": "roster:delimiter");
}

/// The `urn:ietf:params:xml:ns:xmpp-` Namespaces
pub use super::ietf::xmpp as ietf;

/// The `urn:xmpp:` Namespaces
pub mod urn {
    DECLARE_NS_NAME!(NS_ARCHIVE: "The Archive Namespace":
        "urn:xmpp:archive");
    DECLARE_NS_NAME!(NS_ATTENTION: "The Attention 0 Namespace":
        "urn:xmpp:attention:0");
    DECLARE_NS_NAME!(NS_AVATAR_DATA: "The Avatar Data Namespace":
        "urn:xmpp:avatar:data");
    DECLARE_NS_NAME!(NS_AVATAR_METADATA: "The Avatar Metadata Namespace":
        "urn:xmpp:avatar:metadata");
    DECLARE_NS_NAME!(NS_BIDI: "The Bidirectional Namespace":
        "urn:xmpp:bidi");
    DECLARE_NS_NAME!(NS_BOB: "The Bits of Binary Namespace":
        "urn:xmpp:bob");
    DECLARE_NS_NAME!(NS_CAPTCHA: "The CAPTCHA Forms Namespace":
        "urn:xmpp:captcha");
    DECLARE_NS_NAME!(NS_DELAY: "The Delivery Delay Namespace":
        "urn:xmpp:delay");
    DECLARE_NS_NAME!(NS_ERRORS: "The Errors Namespace":
        "urn:xmpp:errors");
    DECLARE_NS_NAME!(NS_FORWARD: "The Stanza Forwarding Namespace":
        "urn:xmpp:forward:0");
    DECLARE_NS_NAME!(NS_JINGLE: "The Jingle Namespace":
        "urn:xmpp:jingle:1" (jingle::NS));

    /// The `urn:xmpp:jingle` Namespaces
    pub mod jingle {
        DECLARE_NS_NAME!(NS: "The Jingle Namespace":
            "urn:xmpp:jingle:1");

        DECLARE_NS_NAME!(NS_ERRORS: "The Jingle Errors Namespace":
            "urn:xmpp:jingle:errors:1");

        /// The `urn:xmpp:jingle:apps` Namespaces
        pub mod apps {
            DECLARE_NS_NAME!(NS_RTP: "The RTP Namespace":
                "urn:xmpp:jingle:apps:rtp:1" (rtp::NS));

            /// The `urn:xmpp:jingle:apps:rtp` Namespaces
            pub mod rtp {
                DECLARE_NS_NAME!(NS: "The RTP Namespace":
                    "urn:xmpp:jingle:apps:rtp:1");
                DECLARE_NS_NAME!(NS_ERRORS: "The Errors Namespace":
                    "urn:xmpp:jingle:apps:rtp:errors:1");
                DECLARE_NS_NAME!(NS_INFO: "The Info Namespace":
                    "urn:xmpp:jingle:apps:rtp:info:1");
                DECLARE_NS_NAME!(NS_RTCP_FB: "The Feedback Negotiation Namespace":
                    "urn:xmpp:jingle:apps:rtp:rtcp-fb:0");
                DECLARE_NS_NAME!(NS_RTP_HDREXT: "The Header Extension Negotiation Namespace":
                    "urn:xmpp:jingle:apps:rtp:rtp-hdrext:0");
                DECLARE_NS_NAME!(NS_ZRTP: "the ZRTP Namespace":
                    "urn:xmpp:jingle:apps:rtp:zrtp:1");
            }
        }

        /// The `urn:xmpp:jingle:transports` Namespaces
        pub mod transports {
            DECLARE_NS_NAME!(NS_IBB: "The In Byte Bytestreams Namespace":
                "urn:xmpp:jingle:transports:ibb:1");
            DECLARE_NS_NAME!(NS_ICE_UDP: "The ICE-UDP Namespace":
                "urn:xmpp:jingle:transports:ice-udp:1");
            DECLARE_NS_NAME!(NS_RAW_UDP: "The Raw UDP Namespace":
                "urn:xmpp:jingle:transports:raw-udp:1");
            DECLARE_NS_NAME!(NS_S5B: "The SOCKS5 Bytstream Namespace":
                "urn:xmpp:jingle:transports:s5b:1");
        }
    }

    DECLARE_NS_NAME!(NS_LANGTRANS: "The Language Translation Namespace":
        "urn:xmpp:langtrans" (langtrans::NS));

    /// The `urn:xmpp:langtrans` Namespaces
    pub mod langtrans {
        DECLARE_NS_NAME!(NS: "The Language Translation Namespace":
            "urn:xmpp:langtrans");
        DECLARE_NS_NAME!(NS_ITEMS: "The Items Namespace":
            "urn:xmpp:langtrans:items");
    }

    DECLARE_NS_NAME!(NS_MEDIA_ELEMENT: "The Media Element Namespace":
        "urn:xmpp:media-element");
    DECLARE_NS_NAME!(NS_MESSAGE_CORRECT: "The Message Correct Namespacce":
        "urn:xmpp:message-correct:0");
    DECLARE_NS_NAME!(NS_PIE: "The Portable Import-Export Namespace":
        "urn:xmpp:pie");
    DECLARE_NS_NAME!(NS_PING: "The Ping Namespace":
        "urn:xmpp:ping");
    DECLARE_NS_NAME!(NS_REACH: "The Reachability Namespace":
        "urn:xmpp:reach:0");
    DECLARE_NS_NAME!(NS_RECIEPTS: "The Reciepts Namespace":
        "urn:xmpp:receipts");
    DECLARE_NS_NAME!(NS_RTT: "The Real Time Text Namespace":
        "urn:xmpp:rtt:0");
    DECLARE_NS_NAME!(NS_SEC_LABEL: "The Security Label Namespace":
        "urn:xmpp:sec-label:0" (sec_label::NS));

    /// The `urn:xmpp:sec-label` Namespaces
    pub mod sec_label {
        DECLARE_NS_NAME!(NS: "The Security Label Namespace":
            "urn:xmpp:sec-label:0");
        DECLARE_NS_NAME!(NS_CATALOG: "The Catalog Namespace":
            "urn:xmpp:sec-label:catalog:2");
        DECLARE_NS_NAME!(NS_ESS: "The ESS Namespace":
            "urn:xmpp:sec-label:ess:0");
    }

    DECLARE_NS_NAME!(NS_SM: "The Stream Management Namespace":
        "urn:xmpp:sm:3");
    DECLARE_NS_NAME!(NS_SSN: "The Stanza Session Negotiation Namespace":
        "urn:xmpp:ssn");
    DECLARE_NS_NAME!(NS_TIME: "The Entity Time Namespace":
        "urn:xmpp:time");
    DECLARE_NS_NAME!(NS_XBOSH: "The XMPP over BOSH Namespace":
        "urn:xmpp:xbosh");
}

/// The `vcard-temp` Namespaces
pub mod vcard_temp {
    DECLARE_NS_NAME!(NS: "The VCard Temp Namespace": "vcard-temp");
    DECLARE_NS_NAME!(NS_X_UPDATE: "The Update Namespace": "vcard-temp:x:update");
}

DECLARE_NS_NAME!(NS_JABBER_STREAMS: "The Streams Namespace": "http://etherx.jabber.org/streams");
DECLARE_NS_NAME!(NS_EXTENSIONS: "The Extensions Namespace": "http://www.xmpp.org/extensions");
