// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The world-wide-web consortium (W3C)

/// The XML Namespaces
pub mod xml {
    DECLARE_NS_NAME!(NS: "The XML Namespace":
    "http://www.w3.org/XML/1998/namespace");
    DECLARE_NS_NAME!(NS_SCHEMA: "The XML Schema Namespace":
    "http://www.w3.org/2001/XMLSchema");
    DECLARE_NS_NAME!(NS_SCHEMA_INSTANCE: "The XML Schema Instance Namespace":
    "http://www.w3.org/2001/XMLSchema-instance");
    DECLARE_NS_NAME!(NS_XMLNS: "The XML Namespaces Namespace":
    "http://www.w3.org/2000/xmlns/");
    DECLARE_NS_NAME!(NS_DSIG: "The XML Signatures Namespace":
    "http://www.w3.org/2000/09/xmldsig#");
    DECLARE_NS_NAME!(NS_ENC: "The XML Encryption Namespace":
    "http://www.w3.org/2001/04/xmlenc#");
    DECLARE_NS_NAME!(NS_EVENTS: "The XML Events Namespace":
    "http://www.w3.org/2001/xml-events");
    DECLARE_NS_NAME!(NS_XLINK: "The XLink Namespace":
    "http://www.w3.org/1999/xlink");
    DECLARE_NS_NAME!(NS_XFORMS: "The XForms Namespace":
    "http://www.w3.org/2002/xforms");
    DECLARE_NS_NAME!(NS_EXC_C14N: "The XML EXC C14N Namespace":
    "http://www.w3.org/2001/10/xml-exc-c14n#");
}

/// The XSL Namespaces
pub mod xsl {
    DECLARE_NS_NAME!(NS_TRANSFORM: "The XSLT Namespace":
        "http://www.w3.org/1999/XSL/Transform");
    DECLARE_NS_NAME!(NS_FORMAT: "The XSL:FO Namespace":
        "http://www.w3.org/1999/XSL/Format");
    DECLARE_NS_NAME!(NS_T_XQUERY_SERIALIZATION: "The XSLT XQuery Serialization Namespace":
        "http://www.w3.org/2010/xslt-xquery-serialization");
}

/// Other Namespaces
pub mod other {
    DECLARE_NS_NAME!(NS_ATOM: "The Atom Namespace":
        "http://www.w3.org/2005/Atom");
    DECLARE_NS_NAME!(NS_G_DATA_VIEWS: "The GRDDL Data Views Namespace":
        "http://www.w3.org/2003/g/data-view#");
    DECLARE_NS_NAME!(NS_MATHML: "The MathML Namespace":
        "http://www.w3.org/1998/Math/MathML");
    DECLARE_NS_NAME!(NS_OWL: "The OWL Ontology Namespace":
        "http://www.w3.org/2002/07/owl#");
    DECLARE_NS_NAME!(NS_P3P: "The P3P Namespace":
        "http://www.w3.org/2000/P3Pv");
    DECLARE_NS_NAME!(NS_RDF_SCHEMA: "The RDF Schema Namespace":
        "http://www.w3.org/2000/01/rdf-schema#");
    DECLARE_NS_NAME!(NS_RDF_SYNTAX: "The RDF Namespace":
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#");
    DECLARE_NS_NAME!(NS_SMIL: "The SMIL Namespace":
        "https://www.w3.org/ns/SMIL");
    DECLARE_NS_NAME!(NS_SOAP_ENVELOPE: "The SOAP Envelope Namespace":
        "http://www.w3.org/2003/05/soap-envelope");
    DECLARE_NS_NAME!(NS_SVG: "The SVG Namespace":
        "http://www.w3.org/2000/svg");
    DECLARE_NS_NAME!(NS_SYNTHESIS: "The SSML (Synthesis) Namespace":
        "https://www.w3.org/2001/10/synthesis");
    DECLARE_NS_NAME!(NS_WD_PROFILE_VOCAB: "The WD Profile Vocabulary Namrspace":
        "http://www.w3.org/TR/WD-profile-vocabulary#");
    DECLARE_NS_NAME!(NS_XHTML: "The XHTML Namespace":
        "http://www.w3.org/1999/xhtml");
    DECLARE_NS_NAME!(NS_XQT_ERRORS: "The XQT Errors Namespace":
        "http://www.w3.org/2005/xqt-errors");
    DECLARE_NS_NAME!(NS_XQUERY_LOCAL_FUNCTIONS: "The XQuery LocalFunctions Namespace":
        "http://www.w3.org/2005/xquery-local-functions");
}

DECLARE_NS_NAME!(NS_XPATH_FUNCTIONS: "The XPath Functions Namespace":
    "http://www.w3.org/2005/xpath-functions" (xpath_functions::NS));

/// The XPath Functions Namespaces
pub mod xpath_functions {
    DECLARE_NS_NAME!(NS: "The XPath Functions Namespace":
        "http://www.w3.org/2005/xpath-functions");
    DECLARE_NS_NAME!(NS_MAP: "The Map Namespace":
        "http://www.w3.org/2005/xpath-functions/map");
    DECLARE_NS_NAME!(NS_ARRAY: "The Array Namespace":
        "http://www.w3.org/2005/xpath-functions/array");
    DECLARE_NS_NAME!(NS_MATH: "The Math Namespace":
        "http://www.w3.org/2005/xpath-functions/math");
}
