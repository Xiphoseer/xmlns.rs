// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The Open Geospatial Consortium
//!
//! Source <https://schemas.opengis.net/>

DECLARE_NS_NAME!(NS_KML: "The KML Namespace":
    "http://www.opengis.net/kml/2.2");
DECLARE_NS_NAME!(NS_GML: "The GML Namespace":
    "http://www.opengis.net/gml/3.2" (gml::v3_2::NS));
DECLARE_NS_NAME!(NS_CITYGML: "The CityGML Namespace":
    "http://www.opengis.net/citygml/2.0");

/// Geography Markup Language
pub mod gml {
    DECLARE_NS_NAME!(NS: "The GML Namespace":
        "http://www.opengis.net/gml");

    /// Version 3.3.x
    pub mod v3_3 {
        DECLARE_NS_NAME!(NS_XBT: "Extended Base Types":
            "http://www.opengis.net/gml/3.3/xbt");
        DECLARE_NS_NAME!(NS_EXR: "Extended Encoding Rule":
            "http://www.opengis.net/gml/3.3/exr");
        DECLARE_NS_NAME!(NS_CE: "Compact Geometry Namespace":
            "http://www.opengis.net/gml/3.3/ce");
        DECLARE_NS_NAME!(NS_LR: "Linear Ref Namespace":
            "http://www.opengis.net/gml/3.3/lr");
        DECLARE_NS_NAME!(NS_LRO: "Linear Ref Offset Namespace":
            "http://www.opengis.net/gml/3.3/lro");
        DECLARE_NS_NAME!(NS_LROV: "Linear Ref Offset Vector Namespace":
            "http://www.opengis.net/gml/3.3/lrov");
        DECLARE_NS_NAME!(NS_LRTR: "Linear Ref Towards Referent Namespace":
            "http://www.opengis.net/gml/3.3/lrtr");
        DECLARE_NS_NAME!(NS_RGRID: "Referenceable Grid Namespace":
            "http://www.opengis.net/gml/3.3/rgrid");
        DECLARE_NS_NAME!(NS_TIN: "Tin Namespace":
            "http://www.opengis.net/gml/3.3/tin");
    }

    /// Version 3.2.x
    pub mod v3_2 {
        DECLARE_NS_NAME!(NS: "The GML Namespace":
            "http://www.opengis.net/gml/3.2");
    }
}
