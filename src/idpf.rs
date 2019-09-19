// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The International Digital Publishing Forum (IDPF)
//!
//! The IDPF has merged with the W3C as of 2017

DECLARE_NS_NAME!(NS_OPS: "The EPUB Namespace": "http://www.idpf.org/2007/ops");
DECLARE_NS_NAME!(NS_METADATA: "The Metadata Namespace": "http://www.idpf.org/2013/metadata");

/// The Encrpytion Namespace
pub mod encryption {
    DECLARE_NS_NAME!(NS_COMPRESSION: "The Compression Namespace":
        "http://www.idpf.org/2016/encryption#compression");
}
