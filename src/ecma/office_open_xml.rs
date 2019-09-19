// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The ECMA-376 Open Office XML
//!
//! See <http://www.ecma-international.org/publications/standards/Ecma-376.htm>
pub mod package {
    DECLARE_NS_NAME!(NS_CONTENT_TYPES: "The ECMA-376 ContentTypes Namespace":
        "http://schemas.openxmlformats.org/package/2006/content-types");
    DECLARE_NS_NAME!(NS_RELATIONSHIPS: "The ECMA-376 Relationships Namespace":
        "http://schemas.openxmlformats.org/package/2006/relationships");
    DECLARE_NS_NAME!(NS_DIGITAL_SIGNATURE: "The ECMA-376 Digital Signature Namespace":
        "http://schemas.openxmlformats.org/package/2006/digital-signature");
    DECLARE_NS_NAME!(NS_CORE_PROPERTIES: "The ECMA-376 Core Properties Namespace":
        "http://schemas.openxmlformats.org/package/2006/metadata/core-properties");
}

pub mod office_document {
    DECLARE_NS_NAME!(NS_EXTENDED_PROPERTIES: "The ECMA-376 Extended Properties Namespace":
        "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties");
    DECLARE_NS_NAME!(NS_ADDITIONAL_CHARACTERISTICS: "The ECMA-376 Additional Characteristics Namespace":
        "http://schemas.openxmlformats.org/officeDocument/2006/additionalCharacteristics");
    DECLARE_NS_NAME!(NS_BIBLIOGRAPHY: "The Bibliography Namespace":
        "http://schemas.openxmlformats.org/officeDocument/2006/bibliography");
    DECLARE_NS_NAME!(NS_CUSTOM_XML_DATA_PROPS: "The customXmlDataProps Namespace":
        "http://schemas.openxmlformats.org/officeDocument/2006/customXmlDataProps");
    DECLARE_NS_NAME!(NS_CUSTOM_PROPERTIES: "The Custom Properties Namespace":
        "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties");
}

pub mod wordprocessing_ml {
    DECLARE_NS_NAME!(NS_MAIN: "The ECMA-376 WordProcessingML Namespace":
        "http://schemas.openxmlformats.org/wordprocessingml/2006/main");
}

pub mod spreadsheet_ml {
    DECLARE_NS_NAME!(NS_MAIN: "The ECMA-376 SpreadsheetML Namespace":
        "http://schemas.openxmlformats.org/spreadsheetml/2006/main");
    DECLARE_NS_NAME!(NS_STYLE: "The ECMA-376 SpreadsheetML Style Namespace":
        "http://schemas.openxmlformats.org/spreadsheetml/2006/mains");
}

pub mod drawing_ml {
    DECLARE_NS_NAME!(NS_MAIN: "The ECMA-376 DrawingML Namespace":
        "http://schemas.openxmlformats.org/drawingml/2006/main");
    DECLARE_NS_NAME!(NS_CHART: "The ECMA-376 DrawingML Namespace":
        "http://schemas.openxmlformats.org/drawingml/2006/chart");
    DECLARE_NS_NAME!(NS_DIAGRAM: "The ECMA-376 DrawingML Namespace":
        "http://schemas.openxmlformats.org/drawingml/2006/diagram");
    DECLARE_NS_NAME!(NS_SPREADSHEET_DRAWING: "The ECMA-376 DrawingML SpreadsheetDrawing":
        "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing");
}

pub mod presentation_ml {
    DECLARE_NS_NAME!(NS_MAIN: "The ECMA-376 PresentationML Namespace":
        "http://schemas.openxmlformats.org/presentationml/2006/main");
}
