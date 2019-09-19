// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The International Standards Organization (ISO)

/// The Document Schema Definition Language (ISO/IEC 19757)

pub mod dsdl {
    /// The Namespace-based Validation Dispatching Language (ISO/IEC 19757, Part 4)
    pub mod nvdl {
        DECLARE_NS_NAME!(NS_STRUCTURE: "The Structure Namespace":
            "http://purl.oclc.org/dsdl/nvdl/ns/structure/1.0");
        DECLARE_NS_NAME!(NS_PREDEFINED_SCHEMA: "The Predefined Schema Namespace":
            "http://purl.oclc.org/dsdl/nvdl/ns/predefinedSchema/1.0");
        DECLARE_NS_NAME!(NS_INSTANCE: "The Instance Namespace":
            "http://purl.oclc.org/dsdl/nvdl/ns/instance/1.0");
    }

    DECLARE_NS_NAME!(NS_SCHEMATRON: "The Schematron Namespace":
        "http://purl.oclc.org/dsdl/schematron");
}
