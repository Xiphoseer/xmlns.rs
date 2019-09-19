// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The Microsoft Corporation

/// Microsoft Office

pub mod office {
    DECLARE_NS_NAME!(NS_OFFICE: "The MS Office Namespace":
        "urn:schemas-microsoft-com:office:office");
    DECLARE_NS_NAME!(NS_EXCEL: "The MS Excel Namespace":
        "urn:schemas-microsoft-com:office:excel");
    DECLARE_NS_NAME!(NS_SPREADSHEET: "The MS Office Spreadsheet":
        "urn:schemas-microsoft-com:office:spreadsheet");
    DECLARE_NS_NAME!(NS_SPREADSHEET_COMPONENT: "The MS Office Spreadsheet Component":
        "urn:schemas-microsoft-com:office:component:spreadsheet");
    DECLARE_NS_NAME!(NS_XML_SCHEMA: "The MS Office XML Schema":
        "uuid:BDC6E3F0-6DA3-11d1-A2A3-00AA00C14882");
    DECLARE_NS_NAME!(NS_XML_DATA_TYPE: "The MS Office XML Data Type":
        "uuid:C2F41010-65B3-11d1-A29F-00AA00C14882");
    DECLARE_NS_NAME!(NS_ROWSET: "The MS Rowset Schema":
        "#RowsetSchema");
}

/// The BizTalk Protocol

pub mod biztalk {
    DECLARE_NS_NAME!(NS_DELIVERY: "The Delivery Namespace":
        "http://schemas.biztalk.org/btf-2-0/delivery");
    DECLARE_NS_NAME!(NS_PROPERTIES: "The Properties Namespace":
        "http://schemas.biztalk.org/btf-2-0/properties");
}


pub mod other {
    DECLARE_NS_NAME!(NS_PERSIST_ROWSET: "The MS-Persist Rowset":
        "urn:schemas-microsoft-com:rowset");
}


pub mod xaml {
    DECLARE_NS_NAME!(NS: "The XAML Namespace":
        "http://schemas.microsoft.com/winfx/2006/xaml");
    DECLARE_NS_NAME!(NS_PRESENTATION: "The XAML Presentation Namespace":
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation");
}
