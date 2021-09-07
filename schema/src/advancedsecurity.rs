use crate::transport;
use crate::validate::Validate;
use std::str::FromStr;
use xsd_macro_utils::*;
use xsd_types::types as xs;

// Unique identifier for keys in the keystore.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct KeyID(pub String);

impl Validate for KeyID {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Unique identifier for certificates in the keystore.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct CertificateID(pub String);

impl Validate for CertificateID {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Unique identifier for certification paths in the keystore.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct CertificationPathID(pub String);

impl Validate for CertificationPathID {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Unique identifier for passphrases in the keystore.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct PassphraseID(pub String);

impl Validate for PassphraseID {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Unique identifier for 802.1X configurations in the keystore.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Dot1XID(pub String);

impl Validate for Dot1XID {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// The status of a key in the keystore.
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum KeyStatus {
    // Key is ready for use
    #[yaserde(rename = "ok")]
    Ok,
    // Key is being generated
    #[yaserde(rename = "generating")]
    Generating,
    // Key has not been successfully generated and cannot be used.
    #[yaserde(rename = "corrupt")]
    Corrupt,
    __Unknown__(String),
}

impl Default for KeyStatus {
    fn default() -> KeyStatus {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for KeyStatus {}

// An object identifier (OID) in dot-decimal form as specified in RFC4512.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DotDecimalOID(pub String);

impl Validate for DotDecimalOID {}
// The distinguished name attribute type encoded as specified in RFC 4514.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DnattributeType(pub String);

impl Validate for DnattributeType {}
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DnattributeValue(pub String);

impl Validate for DnattributeValue {}
// The attributes of a key in the keystore.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct KeyAttribute {
    // The ID of the key.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,

    // The client-defined alias of the key.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    // Absent if the key is not a key pair. True if and only if the key is a key
    // pair and contains a private key. False if and only if the key is a key
    // pair and does not contain a private key.
    #[yaserde(prefix = "tas", rename = "hasPrivateKey")]
    pub has_private_key: Option<bool>,

    // The status of the key. The value should be one of the values in the
    // tas:KeyStatus enumeration.
    #[yaserde(prefix = "tas", rename = "KeyStatus")]
    pub key_status: String,

    // True if and only if the key was generated outside the device.
    #[yaserde(prefix = "tas", rename = "externallyGenerated")]
    pub externally_generated: Option<bool>,

    // True if and only if the key is stored in a specially protected hardware
    // component inside the device.
    #[yaserde(prefix = "tas", rename = "securelyStored")]
    pub securely_stored: Option<bool>,

    #[yaserde(prefix = "tas", rename = "Extension")]
    pub extension: Option<key_attribute::ExtensionType>,
}

impl Validate for KeyAttribute {}

pub mod key_attribute {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct ExtensionType {}

    impl Validate for ExtensionType {}
}

// A distinguished name attribute type and value pair.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DnattributeTypeAndValue {
    // The attribute type.
    #[yaserde(prefix = "tas", rename = "Type")]
    pub _type: DnattributeType,

    // The value of the attribute.
    #[yaserde(prefix = "tas", rename = "Value")]
    pub value: DnattributeValue,
}

impl Validate for DnattributeTypeAndValue {}

// A multi-valued RDN
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct MultiValuedRDN {
    // A list of types and values defining a multi-valued RDN
    #[yaserde(prefix = "tas", rename = "Attribute")]
    pub attribute: Vec<DnattributeTypeAndValue>,
}

impl Validate for MultiValuedRDN {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DistinguishedName {
    // A country name as specified in
    // X.500.
    #[yaserde(prefix = "tas", rename = "Country")]
    pub country: Vec<DnattributeValue>,

    // An organization name as specified in
    // X.500.
    #[yaserde(prefix = "tas", rename = "Organization")]
    pub organization: Vec<DnattributeValue>,

    // An organizational unit name as specified in
    // X.500.
    #[yaserde(prefix = "tas", rename = "OrganizationalUnit")]
    pub organizational_unit: Vec<DnattributeValue>,

    // A distinguished name qualifier as specified in
    // X.500.
    #[yaserde(prefix = "tas", rename = "DistinguishedNameQualifier")]
    pub distinguished_name_qualifier: Vec<DnattributeValue>,

    // A state or province name as specified in
    // X.500.
    #[yaserde(prefix = "tas", rename = "StateOrProvinceName")]
    pub state_or_province_name: Vec<DnattributeValue>,

    // A common name as specified in
    // X.500.
    #[yaserde(prefix = "tas", rename = "CommonName")]
    pub common_name: Vec<DnattributeValue>,

    // A serial number as specified in
    // X.500.
    #[yaserde(prefix = "tas", rename = "SerialNumber")]
    pub serial_number: Vec<DnattributeValue>,

    // A locality as specified in X.500.
    #[yaserde(prefix = "tas", rename = "Locality")]
    pub locality: Vec<DnattributeValue>,

    // A title as specified in X.500.
    #[yaserde(prefix = "tas", rename = "Title")]
    pub title: Vec<DnattributeValue>,

    // A surname as specified in X.500.
    #[yaserde(prefix = "tas", rename = "Surname")]
    pub surname: Vec<DnattributeValue>,

    // A given name as specified in X.500.
    #[yaserde(prefix = "tas", rename = "GivenName")]
    pub given_name: Vec<DnattributeValue>,

    // Initials as specified in X.500.
    #[yaserde(prefix = "tas", rename = "Initials")]
    pub initials: Vec<DnattributeValue>,

    // A pseudonym as specified in X.500.
    #[yaserde(prefix = "tas", rename = "Pseudonym")]
    pub pseudonym: Vec<DnattributeValue>,

    // A generation qualifier as specified in
    // X.500.
    #[yaserde(prefix = "tas", rename = "GenerationQualifier")]
    pub generation_qualifier: Vec<DnattributeValue>,

    // A generic type-value pair
    // attribute.
    #[yaserde(prefix = "tas", rename = "GenericAttribute")]
    pub generic_attribute: Vec<DnattributeTypeAndValue>,

    // A multi-valued RDN
    #[yaserde(prefix = "tas", rename = "MultiValuedRDN")]
    pub multi_valued_rdn: Vec<MultiValuedRDN>,

    // Required extension point. It is recommended to not use this element, and
    // instead use GenericAttribute and the numeric Distinguished Name Attribute
    // Type.
    #[yaserde(prefix = "tas", rename = "anyAttribute")]
    pub any_attribute: Option<distinguished_name::AnyAttributeType>,
}

impl Validate for DistinguishedName {}

pub mod distinguished_name {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct AnyAttributeType {
        // Domain Component as specified in RFC3739
        #[yaserde(prefix = "tas", rename = "DomainComponent")]
        pub domain_component: Vec<DnattributeValue>,
    }

    impl Validate for AnyAttributeType {}
}

// An identifier of an algorithm.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct AlgorithmIdentifier {
    // The OID of the algorithm in dot-decimal form.
    #[yaserde(prefix = "tas", rename = "algorithm")]
    pub algorithm: DotDecimalOID,

    // Optional parameters of the algorithm (depending on the algorithm).
    #[yaserde(prefix = "tas", rename = "parameters")]
    pub parameters: Option<Base64DERencodedASN1Value>,

    #[yaserde(prefix = "tas", rename = "anyParameters")]
    pub any_parameters: Option<algorithm_identifier::AnyParametersType>,
}

impl Validate for AlgorithmIdentifier {}

pub mod algorithm_identifier {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct AnyParametersType {}

    impl Validate for AnyParametersType {}
}

// A CSR attribute as specified in RFC 2986.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct BasicRequestAttribute {
    // The OID of the attribute.
    #[yaserde(prefix = "tas", rename = "OID")]
    pub oid: DotDecimalOID,

    // The value of the attribute as a base64-encoded DER representation of an
    // ASN.1 value.
    #[yaserde(prefix = "tas", rename = "value")]
    pub value: Base64DERencodedASN1Value,
}

impl Validate for BasicRequestAttribute {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum CsrattributeChoice {
    // An X.509v3 extension field.
    #[yaserde(rename = "X509v3Extension")]
    X509V3Extension(X509V3Extension),
    // A basic CSR attribute.
    BasicRequestAttribute(BasicRequestAttribute),
    #[yaserde(rename = "anyAttribute")]
    AnyAttribute,
    __Unknown__(String),
}

impl Default for CsrattributeChoice {
    fn default() -> CsrattributeChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for CsrattributeChoice {}

// A CSR attribute as specified in PKCS#10.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct Csrattribute {
    #[yaserde(flatten)]
    pub csr_attribute_choice: CsrattributeChoice,
}

impl Validate for Csrattribute {}

// A base64-encoded ASN.1 value.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Base64DERencodedASN1Value(pub String);

impl Validate for Base64DERencodedASN1Value {}
// An X.509v3 extension field as specified in RFC 5280
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct X509V3Extension {
    // The OID of the extension field.
    #[yaserde(prefix = "tas", rename = "extnOID")]
    pub extn_oid: DotDecimalOID,

    // True if and only if the extension is critical.
    #[yaserde(prefix = "tas", rename = "critical")]
    pub critical: bool,

    // The value of the extension field as a base64-encoded DER representation
    // of an ASN.1 value.
    #[yaserde(prefix = "tas", rename = "extnValue")]
    pub extn_value: Base64DERencodedASN1Value,
}

impl Validate for X509V3Extension {}

// An X.509 cerficiate as specified in RFC 5280.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct X509Certificate {
    // The ID of the certificate.
    #[yaserde(prefix = "tas", rename = "CertificateID")]
    pub certificate_id: CertificateID,

    // The ID of the key that this certificate associates to the certificate
    // subject.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,

    // The client-defined alias of the certificate.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    // The base64-encoded DER representation of the X.509 certificate.
    #[yaserde(prefix = "tas", rename = "CertificateContent")]
    pub certificate_content: Base64DERencodedASN1Value,
}

impl Validate for X509Certificate {}

// A sequence of certificate IDs.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CertificateIDs {
    // A certificate ID.
    #[yaserde(prefix = "tas", rename = "CertificateID")]
    pub certificate_id: Vec<CertificateID>,
}

impl Validate for CertificateIDs {}

// An X.509 certification path as defined in RFC 5280.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CertificationPath {
    // A certificate in the certification path.
    #[yaserde(prefix = "tas", rename = "CertificateID")]
    pub certificate_id: Vec<CertificateID>,

    // The client-defined alias of the certification path.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    #[yaserde(prefix = "tas", rename = "anyElement")]
    pub any_element: Option<certification_path::AnyElementType>,
}

impl Validate for CertificationPath {}

pub mod certification_path {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct AnyElementType {}

    impl Validate for AnyElementType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct PassphraseAttribute {
    // The ID of the passphrase.
    #[yaserde(prefix = "tas", rename = "PassphraseID")]
    pub passphrase_id: PassphraseID,

    // The alias of the passphrase.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,
}

impl Validate for PassphraseAttribute {}

// A list of supported 802.1X authentication methods, such as
// "EAP-PEAP/MSCHAPv2" and "EAP-MD5". The '/' character is used as a separator
// between the outer and inner methods.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Dot1XMethods(pub Vec<String>);

impl Validate for Dot1XMethods {}
// The capabilities of the 802.1X implementation on a device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct Dot1XCapabilities {
    // The maximum number of 802.1X configurations that may be defined
    // simultaneously.
    #[yaserde(attribute, rename = "MaximumNumberOfDot1XConfigurations")]
    pub maximum_number_of_dot_1x_configurations: Option<xs::Integer>,

    // The authentication methods supported by the 802.1X implementation.
    #[yaserde(attribute, rename = "Dot1XMethods")]
    pub dot_1x_methods: Option<Dot1XMethods>,
}

impl Validate for Dot1XCapabilities {}

// The configuration parameters required for a particular authentication method.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct Dot1XStage {
    // The identity used in this authentication method, if required.
    #[yaserde(prefix = "tas", rename = "Identity")]
    pub identity: Option<String>,

    // The unique identifier of the certification path used in this
    // authentication method, if required.
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: Option<CertificationPathID>,

    // The identifier for the password used in this authentication method, if
    // required. If Identity is used as an anonymous identity for this
    // authentication method, PassphraseID is ignored.
    #[yaserde(prefix = "tas", rename = "PassphraseID")]
    pub passphrase_id: Option<PassphraseID>,

    // The configuration of the next stage of authentication, if required.
    #[yaserde(prefix = "tas", rename = "Inner")]
    pub inner: Vec<Dot1XStage>,

    #[yaserde(prefix = "tas", rename = "Extension")]
    pub extension: Option<Dot1XStageExtension>,

    // The authentication method for this stage (e.g., "EAP-PEAP").
    #[yaserde(attribute, rename = "Method")]
    pub method: String,
}

impl Validate for Dot1XStage {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct Dot1XStageExtension {}

impl Validate for Dot1XStageExtension {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct Dot1XConfiguration {
    // The unique identifier of the IEEE 802.1X configuration.
    #[yaserde(prefix = "tas", rename = "Dot1XID")]
    pub dot_1xid: Option<Dot1XID>,

    // The client-defined alias of the 802.1X configuration.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    // The outer level authentication method used in this 802.1X configuration.
    #[yaserde(prefix = "tas", rename = "Outer")]
    pub outer: Dot1XStage,
}

impl Validate for Dot1XConfiguration {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Crlid(pub String);

impl Validate for Crlid {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct CertPathValidationPolicyID(pub String);

impl Validate for CertPathValidationPolicyID {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > "64".parse().unwrap() {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 64 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct Crl {
    #[yaserde(prefix = "tas", rename = "CRLID")]
    pub crlid: Crlid,

    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: String,

    #[yaserde(prefix = "tas", rename = "CRLContent")]
    pub crl_content: Base64DERencodedASN1Value,
}

impl Validate for Crl {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CertPathValidationParameters {
    // True if and only if the TLS server shall not authenticate client
    // certificates that do not contain the TLS WWW client authentication key
    // usage extension as specified in RFC 5280, Sect. 4.2.1.12.
    #[yaserde(prefix = "tas", rename = "RequireTLSWWWClientAuthExtendedKeyUsage")]
    pub require_tlswww_client_auth_extended_key_usage: Option<bool>,

    // True if and only if delta CRLs, if available, shall be applied to CRLs.
    #[yaserde(prefix = "tas", rename = "UseDeltaCRLs")]
    pub use_delta_cr_ls: Option<bool>,

    #[yaserde(prefix = "tas", rename = "anyParameters")]
    pub any_parameters: Option<cert_path_validation_parameters::AnyParametersType>,
}

impl Validate for CertPathValidationParameters {}

pub mod cert_path_validation_parameters {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct AnyParametersType {}

    impl Validate for AnyParametersType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct TrustAnchor {
    // The certificate ID of the certificate to be used as trust anchor.
    #[yaserde(prefix = "tas", rename = "CertificateID")]
    pub certificate_id: CertificateID,
}

impl Validate for TrustAnchor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CertPathValidationPolicy {
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicyID")]
    pub cert_path_validation_policy_id: CertPathValidationPolicyID,

    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    #[yaserde(prefix = "tas", rename = "Parameters")]
    pub parameters: CertPathValidationParameters,

    #[yaserde(prefix = "tas", rename = "TrustAnchor")]
    pub trust_anchor: Vec<TrustAnchor>,

    #[yaserde(prefix = "tas", rename = "anyParameters")]
    pub any_parameters: Option<cert_path_validation_policy::AnyParametersType>,
}

impl Validate for CertPathValidationPolicy {}

pub mod cert_path_validation_policy {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct AnyParametersType {}

    impl Validate for AnyParametersType {}
}

// A list of RSA key lenghts in bits.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct RsakeyLengths(pub Vec<xs::Integer>);

impl Validate for RsakeyLengths {}
// A list of X.509 versions.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct X509Versions(pub Vec<i32>);

impl Validate for X509Versions {}
// A list of TLS versions.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Tlsversions(pub Vec<String>);

impl Validate for Tlsversions {}
// A list of password based encryption algorithms.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct PasswordBasedEncryptionAlgorithms(pub Vec<String>);

impl Validate for PasswordBasedEncryptionAlgorithms {}
// A list of password based MAC algorithms.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct PasswordBasedMACAlgorithms(pub Vec<String>);

impl Validate for PasswordBasedMACAlgorithms {}
// The capabilities of a keystore implementation on a device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct KeystoreCapabilities {
    // The signature algorithms supported by the keystore implementation.
    #[yaserde(prefix = "tas", rename = "SignatureAlgorithms")]
    pub signature_algorithms: Vec<AlgorithmIdentifier>,

    #[yaserde(prefix = "tas", rename = "anyElement")]
    pub any_element: Option<keystore_capabilities::AnyElementType>,

    // Indicates the maximum number of keys that the device can store
    // simultaneously.
    #[yaserde(attribute, rename = "MaximumNumberOfKeys")]
    pub maximum_number_of_keys: Option<xs::Integer>,

    // Indicates the maximum number of certificates that the device can store
    // simultaneously.
    #[yaserde(attribute, rename = "MaximumNumberOfCertificates")]
    pub maximum_number_of_certificates: Option<xs::Integer>,

    // Indicates the maximum number of certification paths that the device can
    // store simultaneously.
    #[yaserde(attribute, rename = "MaximumNumberOfCertificationPaths")]
    pub maximum_number_of_certification_paths: Option<xs::Integer>,

    // Indication that the device supports on-board RSA key pair generation.
    #[yaserde(attribute, rename = "RSAKeyPairGeneration")]
    pub rsa_key_pair_generation: Option<bool>,

    // Indicates which RSA key lengths are supported by the device.
    #[yaserde(attribute, rename = "RSAKeyLengths")]
    pub rsa_key_lengths: Option<RsakeyLengths>,

    // Indicates support for creating PKCS#10 requests for RSA keys and
    // uploading the certificate obtained from a CA..
    #[yaserde(attribute, rename = "PKCS10ExternalCertificationWithRSA")]
    pub pkcs10_external_certification_with_rsa: Option<bool>,

    // Indicates support for creating self-signed certificates for RSA keys.
    #[yaserde(attribute, rename = "SelfSignedCertificateCreationWithRSA")]
    pub self_signed_certificate_creation_with_rsa: Option<bool>,

    // Indicates which X.509 versions are supported by the device.
    #[yaserde(attribute, rename = "X509Versions")]
    pub x509_versions: Option<X509Versions>,

    // Indicates the maximum number of passphrases that the device is able to
    // store simultaneously.
    #[yaserde(attribute, rename = "MaximumNumberOfPassphrases")]
    pub maximum_number_of_passphrases: Option<xs::Integer>,

    // Indicates support for uploading an RSA key pair in a PKCS#8 data
    // structure.
    #[yaserde(attribute, rename = "PKCS8RSAKeyPairUpload")]
    pub pkcs8rsa_key_pair_upload: Option<bool>,

    // Indicates support for uploading a certificate along with an RSA private
    // key in a PKCS#12 data structure.
    #[yaserde(attribute, rename = "PKCS12CertificateWithRSAPrivateKeyUpload")]
    pub pkcs12_certificate_with_rsa_private_key_upload: Option<bool>,

    // Indicates which password-based encryption algorithms are supported by the
    // device.
    #[yaserde(attribute, rename = "PasswordBasedEncryptionAlgorithms")]
    pub password_based_encryption_algorithms: Option<PasswordBasedEncryptionAlgorithms>,

    // Indicates which password-based MAC algorithms are supported by the
    // device.
    #[yaserde(attribute, rename = "PasswordBasedMACAlgorithms")]
    pub password_based_mac_algorithms: Option<PasswordBasedMACAlgorithms>,

    // Indicates the maximum number of CRLs that the device is able to store
    // simultaneously.
    #[yaserde(attribute, rename = "MaximumNumberOfCRLs")]
    pub maximum_number_of_cr_ls: Option<xs::Integer>,

    // Indicates the maximum number of certification path validation policies
    // that the device is able to store simultaneously.
    #[yaserde(
        attribute,
        rename = "MaximumNumberOfCertificationPathValidationPolicies"
    )]
    pub maximum_number_of_certification_path_validation_policies: Option<xs::Integer>,

    // Indicates whether a device supports checking for the TLS WWW client auth
    // extended key usage extension while validating certification paths.
    #[yaserde(attribute, rename = "EnforceTLSWebClientAuthExtKeyUsage")]
    pub enforce_tls_web_client_auth_ext_key_usage: Option<bool>,

    // Indicates the device requires that each certificate with private key has
    // its own unique key.
    #[yaserde(attribute, rename = "NoPrivateKeySharing")]
    pub no_private_key_sharing: Option<bool>,
}

impl Validate for KeystoreCapabilities {}

pub mod keystore_capabilities {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct AnyElementType {}

    impl Validate for AnyElementType {}
}

// The capabilities of a TLS server implementation on a device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct TlsserverCapabilities {
    // Indicates which TLS versions are supported by the device.
    #[yaserde(attribute, rename = "TLSServerSupported")]
    pub tls_server_supported: Option<Tlsversions>,

    // Indicates whether the device supports enabling and disabling specific TLS
    // versions.
    #[yaserde(attribute, rename = "EnabledVersionsSupported")]
    pub enabled_versions_supported: Option<bool>,

    // Indicates the maximum number of certification paths that may be assigned
    // to the TLS server simultaneously.
    #[yaserde(attribute, rename = "MaximumNumberOfTLSCertificationPaths")]
    pub maximum_number_of_tls_certification_paths: Option<xs::Integer>,

    // Indicates whether the device supports TLS client authentication.
    #[yaserde(attribute, rename = "TLSClientAuthSupported")]
    pub tls_client_auth_supported: Option<bool>,

    // Indicates the maximum number of certification path validation policies
    // that may be assigned to the TLS server simultaneously.
    #[yaserde(
        attribute,
        rename = "MaximumNumberOfTLSCertificationPathValidationPolicies"
    )]
    pub maximum_number_of_tls_certification_path_validation_policies: Option<xs::Integer>,
}

impl Validate for TlsserverCapabilities {}

// The capabilities of a Security Configuration Service implementation on a
// device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct Capabilities {
    // The capabilities of the keystore implementation.
    #[yaserde(prefix = "tas", rename = "KeystoreCapabilities")]
    pub keystore_capabilities: Vec<KeystoreCapabilities>,

    // The capabilities of the TLS server implementation.
    #[yaserde(prefix = "tas", rename = "TLSServerCapabilities")]
    pub tls_server_capabilities: Vec<TlsserverCapabilities>,

    // The capabilities of the 802.1X implementation.
    #[yaserde(prefix = "tas", rename = "Dot1XCapabilities")]
    pub dot_1x_capabilities: Vec<Dot1XCapabilities>,
}

impl Validate for Capabilities {}

// pub type Capabilities = Capabilities;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetServiceCapabilities {}

impl Validate for GetServiceCapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetServiceCapabilitiesResponse {
    // The capabilities for the security configuration service is returned in
    // the Capabilities element.
    #[yaserde(prefix = "tas", rename = "Capabilities")]
    pub capabilities: Capabilities,
}

impl Validate for GetServiceCapabilitiesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreateRSAKeyPair {
    // The length of the key to be created.
    #[yaserde(prefix = "tas", rename = "KeyLength")]
    pub key_length: xs::Integer,

    // The client-defined alias of the key.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,
}

impl Validate for CreateRSAKeyPair {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreateRSAKeyPairResponse {
    // The key ID of the key pair being generated.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,

    // Best-effort estimate of how long the key generation will take.
    #[yaserde(prefix = "tas", rename = "EstimatedCreationTime")]
    pub estimated_creation_time: xs::Duration,
}

impl Validate for CreateRSAKeyPairResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadKeyPairInPKCS8 {
    // The key pair to be uploaded in a PKCS#8 data structure.
    #[yaserde(prefix = "tas", rename = "KeyPair")]
    pub key_pair: Base64DERencodedASN1Value,

    // The client-defined alias of the key pair.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    // The ID of the passphrase to use for decrypting the uploaded key pair.
    #[yaserde(prefix = "tas", rename = "EncryptionPassphraseID")]
    pub encryption_passphrase_id: Option<PassphraseID>,

    // The passphrase to use for decrypting the uploaded key pair.
    #[yaserde(prefix = "tas", rename = "EncryptionPassphrase")]
    pub encryption_passphrase: Option<String>,
}

impl Validate for UploadKeyPairInPKCS8 {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadKeyPairInPKCS8Response {
    // The key ID of the uploaded key pair.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,
}

impl Validate for UploadKeyPairInPKCS8Response {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadCertificateWithPrivateKeyInPKCS12 {
    // The certificates and key pair to be uploaded in a PKCS#12 data structure.
    #[yaserde(prefix = "tas", rename = "CertWithPrivateKey")]
    pub cert_with_private_key: Base64DERencodedASN1Value,

    // The client-defined alias of the certification path.
    #[yaserde(prefix = "tas", rename = "CertificationPathAlias")]
    pub certification_path_alias: Option<String>,

    // The client-defined alias of the key pair.
    #[yaserde(prefix = "tas", rename = "KeyAlias")]
    pub key_alias: Option<String>,

    // True if and only if the device shall behave as if the client had only
    // supplied the first certificate in the sequence of certificates.
    #[yaserde(prefix = "tas", rename = "IgnoreAdditionalCertificates")]
    pub ignore_additional_certificates: Option<bool>,

    // The ID of the passphrase to use for integrity checking of the uploaded
    // PKCS#12 data structure.
    #[yaserde(prefix = "tas", rename = "IntegrityPassphraseID")]
    pub integrity_passphrase_id: Option<PassphraseID>,

    // The ID of the passphrase to use for decrypting the uploaded PKCS#12 data
    // structure.
    #[yaserde(prefix = "tas", rename = "EncryptionPassphraseID")]
    pub encryption_passphrase_id: Option<PassphraseID>,

    // The passphrase to use for integrity checking and decrypting the uploaded
    // PKCS#12 data structure.
    #[yaserde(prefix = "tas", rename = "Passphrase")]
    pub passphrase: Option<String>,
}

impl Validate for UploadCertificateWithPrivateKeyInPKCS12 {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadCertificateWithPrivateKeyInPKCS12Response {
    // The certification path ID of the uploaded certification path.
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: CertificationPathID,

    // The key ID of the uploaded key pair.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,
}

impl Validate for UploadCertificateWithPrivateKeyInPKCS12Response {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetKeyStatus {
    // The ID of the key for which to return the status.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,
}

impl Validate for GetKeyStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetKeyStatusResponse {
    // Status of the requested key. The value should be one of the values in the
    // tas:KeyStatus enumeration.
    #[yaserde(prefix = "tas", rename = "KeyStatus")]
    pub key_status: String,
}

impl Validate for GetKeyStatusResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetPrivateKeyStatus {
    // The ID of the key pair for which to return whether it contains a private
    // key.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,
}

impl Validate for GetPrivateKeyStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetPrivateKeyStatusResponse {
    // True if and only if the key pair contains a private key.
    #[yaserde(prefix = "tas", rename = "hasPrivateKey")]
    pub has_private_key: bool,
}

impl Validate for GetPrivateKeyStatusResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllKeys {}

impl Validate for GetAllKeys {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllKeysResponse {
    // Information about a key in the keystore.
    #[yaserde(prefix = "tas", rename = "KeyAttribute")]
    pub key_attribute: Vec<KeyAttribute>,
}

impl Validate for GetAllKeysResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteKey {
    // The ID of the key that is to be deleted from the keystore.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,
}

impl Validate for DeleteKey {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteKeyResponse {}

impl Validate for DeleteKeyResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreatePKCS10CSR {
    // The subject to be included in the CSR.
    #[yaserde(prefix = "tas", rename = "Subject")]
    pub subject: DistinguishedName,

    // The ID of the key for which the CSR shall be created.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,

    // An attribute to be included in the CSR.
    #[yaserde(prefix = "tas", rename = "CSRAttribute")]
    pub csr_attribute: Vec<Csrattribute>,

    // The signature algorithm to be used to sign the CSR.
    #[yaserde(prefix = "tas", rename = "SignatureAlgorithm")]
    pub signature_algorithm: AlgorithmIdentifier,
}

impl Validate for CreatePKCS10CSR {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreatePKCS10CSRResponse {
    // The DER encoded PKCS#10 certification request.
    #[yaserde(prefix = "tas", rename = "PKCS10CSR")]
    pub pkcs10csr: Base64DERencodedASN1Value,
}

impl Validate for CreatePKCS10CSRResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreateSelfSignedCertificate {
    // The X.509 version that the generated certificate shall comply to.
    #[yaserde(prefix = "tas", rename = "X509Version")]
    pub x509_version: Option<xs::Integer>,

    // Distinguished name of the entity that the certificate shall belong to.
    #[yaserde(prefix = "tas", rename = "Subject")]
    pub subject: DistinguishedName,

    // The ID of the key for which the certificate shall be created.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,

    // The client-defined alias of the certificate to be created.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    // The X.509 not valid before information to be included in the certificate.
    // Defaults to the device's current time or a time before the device's
    // current time.
    #[yaserde(prefix = "tas", rename = "notValidBefore")]
    pub not_valid_before: Option<xs::DateTime>,

    // The X.509 not valid after information to be included in the certificate.
    // Defaults to the time 99991231235959Z as specified in RFC 5280.
    #[yaserde(prefix = "tas", rename = "notValidAfter")]
    pub not_valid_after: Option<xs::DateTime>,

    // The signature algorithm to be used for signing the certificate.
    #[yaserde(prefix = "tas", rename = "SignatureAlgorithm")]
    pub signature_algorithm: AlgorithmIdentifier,

    // An X.509v3 extension to be included in the certificate.
    #[yaserde(prefix = "tas", rename = "Extension")]
    pub extension: Vec<X509V3Extension>,
}

impl Validate for CreateSelfSignedCertificate {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreateSelfSignedCertificateResponse {
    // The ID of the generated certificate.
    #[yaserde(prefix = "tas", rename = "CertificateID")]
    pub certificate_id: CertificateID,
}

impl Validate for CreateSelfSignedCertificateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadCertificate {
    // The base64-encoded DER representation of the X.509 certificate to be
    // uploaded.
    #[yaserde(prefix = "tas", rename = "Certificate")]
    pub certificate: Base64DERencodedASN1Value,

    // The client-defined alias of the certificate.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    // The client-defined alias of the key pair.
    #[yaserde(prefix = "tas", rename = "KeyAlias")]
    pub key_alias: Option<String>,

    // Indicates if the device shall verify that a matching key pair with a
    // private key exists in the keystore.
    #[yaserde(prefix = "tas", rename = "PrivateKeyRequired")]
    pub private_key_required: Option<bool>,
}

impl Validate for UploadCertificate {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadCertificateResponse {
    // The ID of the uploaded certificate.
    #[yaserde(prefix = "tas", rename = "CertificateID")]
    pub certificate_id: CertificateID,

    // The ID of the key that the uploaded certificate certifies.
    #[yaserde(prefix = "tas", rename = "KeyID")]
    pub key_id: KeyID,
}

impl Validate for UploadCertificateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetCertificate {
    // The ID of the certificate to retrieve.
    #[yaserde(prefix = "tas", rename = "CertificateID")]
    pub certificate_id: CertificateID,
}

impl Validate for GetCertificate {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetCertificateResponse {
    // The DER representation of the certificate.
    #[yaserde(prefix = "tas", rename = "Certificate")]
    pub certificate: X509Certificate,
}

impl Validate for GetCertificateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllCertificates {}

impl Validate for GetAllCertificates {}

// A list with all certificates stored in the keystore.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllCertificatesResponse {
    // A certificate stored in the keystore.
    #[yaserde(prefix = "tas", rename = "Certificate")]
    pub certificate: Vec<X509Certificate>,
}

impl Validate for GetAllCertificatesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteCertificate {
    // The ID of the certificate to delete.
    #[yaserde(prefix = "tas", rename = "CertificateID")]
    pub certificate_id: CertificateID,
}

impl Validate for DeleteCertificate {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteCertificateResponse {}

impl Validate for DeleteCertificateResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreateCertificationPath {
    // The IDs of the certificates to include in the certification path, where
    // each certificate signature except for the last one in the path must be
    // verifiable with the public key certified by the next certificate in the
    // path.
    #[yaserde(prefix = "tas", rename = "CertificateIDs")]
    pub certificate_i_ds: CertificateIDs,

    // The client-defined alias of the certification path.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,
}

impl Validate for CreateCertificationPath {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreateCertificationPathResponse {
    // The ID of the generated certification path.
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: CertificationPathID,
}

impl Validate for CreateCertificationPathResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetCertificationPath {
    // The ID of the certification path to retrieve.
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: CertificationPathID,
}

impl Validate for GetCertificationPath {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetCertificationPathResponse {
    // The certification path that is stored under the given ID in the keystore.
    #[yaserde(prefix = "tas", rename = "CertificationPath")]
    pub certification_path: CertificationPath,
}

impl Validate for GetCertificationPathResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllCertificationPaths {}

impl Validate for GetAllCertificationPaths {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllCertificationPathsResponse {
    // An ID of a certification path in the keystore.
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: Vec<CertificationPathID>,
}

impl Validate for GetAllCertificationPathsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteCertificationPath {
    // The ID of the certification path to delete.
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: CertificationPathID,
}

impl Validate for DeleteCertificationPath {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteCertificationPathResponse {}

impl Validate for DeleteCertificationPathResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadPassphrase {
    // The passphrase to upload.
    #[yaserde(prefix = "tas", rename = "Passphrase")]
    pub passphrase: String,

    // The alias for the passphrase to upload.
    #[yaserde(prefix = "tas", rename = "PassphraseAlias")]
    pub passphrase_alias: Option<String>,
}

impl Validate for UploadPassphrase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadPassphraseResponse {
    // The PassphraseID of the uploaded passphrase.
    #[yaserde(prefix = "tas", rename = "PassphraseID")]
    pub passphrase_id: PassphraseID,
}

impl Validate for UploadPassphraseResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllPassphrases {}

impl Validate for GetAllPassphrases {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllPassphrasesResponse {
    // Information about a passphrase in the keystore.
    #[yaserde(prefix = "tas", rename = "PassphraseAttribute")]
    pub passphrase_attribute: Vec<PassphraseAttribute>,
}

impl Validate for GetAllPassphrasesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeletePassphrase {
    // The ID of the passphrase that is to be deleted from the keystore.
    #[yaserde(prefix = "tas", rename = "PassphraseID")]
    pub passphrase_id: PassphraseID,
}

impl Validate for DeletePassphrase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeletePassphraseResponse {}

impl Validate for DeletePassphraseResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct AddServerCertificateAssignment {
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: CertificationPathID,
}

impl Validate for AddServerCertificateAssignment {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct AddServerCertificateAssignmentResponse {}

impl Validate for AddServerCertificateAssignmentResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct RemoveServerCertificateAssignment {
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: CertificationPathID,
}

impl Validate for RemoveServerCertificateAssignment {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct RemoveServerCertificateAssignmentResponse {}

impl Validate for RemoveServerCertificateAssignmentResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct ReplaceServerCertificateAssignment {
    #[yaserde(prefix = "tas", rename = "OldCertificationPathID")]
    pub old_certification_path_id: CertificationPathID,

    #[yaserde(prefix = "tas", rename = "NewCertificationPathID")]
    pub new_certification_path_id: CertificationPathID,
}

impl Validate for ReplaceServerCertificateAssignment {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct ReplaceServerCertificateAssignmentResponse {}

impl Validate for ReplaceServerCertificateAssignmentResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAssignedServerCertificates {}

impl Validate for GetAssignedServerCertificates {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAssignedServerCertificatesResponse {
    // The IDs of all certification paths that are assigned to the TLS server on
    // the device.
    #[yaserde(prefix = "tas", rename = "CertificationPathID")]
    pub certification_path_id: Vec<CertificationPathID>,
}

impl Validate for GetAssignedServerCertificatesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct SetEnabledTLSVersions {
    // List of TLS versions to allow.
    #[yaserde(prefix = "tas", rename = "Versions")]
    pub versions: Tlsversions,
}

impl Validate for SetEnabledTLSVersions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct SetEnabledTLSVersionsResponse {}

impl Validate for SetEnabledTLSVersionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetEnabledTLSVersions {}

impl Validate for GetEnabledTLSVersions {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetEnabledTLSVersionsResponse {
    // List of allowed TLS versions.
    #[yaserde(prefix = "tas", rename = "Versions")]
    pub versions: Tlsversions,
}

impl Validate for GetEnabledTLSVersionsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadCRL {
    // The CRL to be uploaded to the device.
    #[yaserde(prefix = "tas", rename = "Crl")]
    pub crl: Base64DERencodedASN1Value,

    // The alias to assign to the uploaded CRL.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    #[yaserde(prefix = "tas", rename = "anyParameters")]
    pub any_parameters: Option<upload_crl::AnyParametersType>,
}

impl Validate for UploadCRL {}

pub mod upload_crl {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct AnyParametersType {}

    impl Validate for AnyParametersType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct UploadCRLResponse {
    // The ID of the uploaded CRL.
    #[yaserde(prefix = "tas", rename = "CrlID")]
    pub crl_id: Crlid,
}

impl Validate for UploadCRLResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetCRL {
    // The ID of the CRL to be returned.
    #[yaserde(prefix = "tas", rename = "CrlID")]
    pub crl_id: Crlid,
}

impl Validate for GetCRL {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetCRLResponse {
    // The CRL with the requested ID.
    #[yaserde(prefix = "tas", rename = "Crl")]
    pub crl: Crl,
}

impl Validate for GetCRLResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllCRLs {}

impl Validate for GetAllCRLs {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllCRLsResponse {
    // A list of all CRLs that are stored in the keystore on the device.
    #[yaserde(prefix = "tas", rename = "Crl")]
    pub crl: Vec<Crl>,
}

impl Validate for GetAllCRLsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteCRL {
    // The ID of the CRL to be deleted.
    #[yaserde(prefix = "tas", rename = "CrlID")]
    pub crl_id: Crlid,
}

impl Validate for DeleteCRL {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteCRLResponse {}

impl Validate for DeleteCRLResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreateCertPathValidationPolicy {
    // The alias to assign to the created certification path validation policy.
    #[yaserde(prefix = "tas", rename = "Alias")]
    pub alias: Option<String>,

    // The parameters of the certification path validation policy to be created.
    #[yaserde(prefix = "tas", rename = "Parameters")]
    pub parameters: CertPathValidationParameters,

    // The trust anchors of the certification path validation policy to be
    // created.
    #[yaserde(prefix = "tas", rename = "TrustAnchor")]
    pub trust_anchor: Vec<TrustAnchor>,

    #[yaserde(prefix = "tas", rename = "anyParameters")]
    pub any_parameters: Option<create_cert_path_validation_policy::AnyParametersType>,
}

impl Validate for CreateCertPathValidationPolicy {}

pub mod create_cert_path_validation_policy {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "tas",
        namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
    )]
    pub struct AnyParametersType {}

    impl Validate for AnyParametersType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct CreateCertPathValidationPolicyResponse {
    // The ID of the created certification path validation policy.
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicyID")]
    pub cert_path_validation_policy_id: CertPathValidationPolicyID,
}

impl Validate for CreateCertPathValidationPolicyResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetCertPathValidationPolicy {
    // The ID of the certification path validation policy to be created.
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicyID")]
    pub cert_path_validation_policy_id: CertPathValidationPolicyID,
}

impl Validate for GetCertPathValidationPolicy {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetCertPathValidationPolicyResponse {
    // The certification path validation policy that is stored under the
    // requested ID.
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicy")]
    pub cert_path_validation_policy: CertPathValidationPolicy,
}

impl Validate for GetCertPathValidationPolicyResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllCertPathValidationPolicies {}

impl Validate for GetAllCertPathValidationPolicies {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllCertPathValidationPoliciesResponse {
    // A list of all certification path validation policies that are stored in
    // the keystore on the device.
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicy")]
    pub cert_path_validation_policy: Vec<CertPathValidationPolicy>,
}

impl Validate for GetAllCertPathValidationPoliciesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteCertPathValidationPolicy {
    // The ID of the certification path validation policy to be deleted.
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicyID")]
    pub cert_path_validation_policy_id: CertPathValidationPolicyID,
}

impl Validate for DeleteCertPathValidationPolicy {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteCertPathValidationPolicyResponse {}

impl Validate for DeleteCertPathValidationPolicyResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct SetClientAuthenticationRequired {
    #[yaserde(prefix = "tas", rename = "clientAuthenticationRequired")]
    pub client_authentication_required: bool,
}

impl Validate for SetClientAuthenticationRequired {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct SetClientAuthenticationRequiredResponse {}

impl Validate for SetClientAuthenticationRequiredResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetClientAuthenticationRequired {}

impl Validate for GetClientAuthenticationRequired {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetClientAuthenticationRequiredResponse {
    #[yaserde(prefix = "tas", rename = "clientAuthenticationRequired")]
    pub client_authentication_required: bool,
}

impl Validate for GetClientAuthenticationRequiredResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct AddCertPathValidationPolicyAssignment {
    // The ID of the certification path validation policy to assign to the TLS
    // server.
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicyID")]
    pub cert_path_validation_policy_id: CertPathValidationPolicyID,
}

impl Validate for AddCertPathValidationPolicyAssignment {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct AddCertPathValidationPolicyAssignmentResponse {}

impl Validate for AddCertPathValidationPolicyAssignmentResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct RemoveCertPathValidationPolicyAssignment {
    // The ID of the certification path validation policy to de-assign from the
    // TLS server.
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicyID")]
    pub cert_path_validation_policy_id: CertPathValidationPolicyID,
}

impl Validate for RemoveCertPathValidationPolicyAssignment {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct RemoveCertPathValidationPolicyAssignmentResponse {}

impl Validate for RemoveCertPathValidationPolicyAssignmentResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct ReplaceCertPathValidationPolicyAssignment {
    // The ID of the certification path validation policy to be de-assigned from
    // the TLS server.
    #[yaserde(prefix = "tas", rename = "OldCertPathValidationPolicyID")]
    pub old_cert_path_validation_policy_id: CertPathValidationPolicyID,

    // The ID of the certification path validation policy to assign to the TLS
    // server.
    #[yaserde(prefix = "tas", rename = "NewCertPathValidationPolicyID")]
    pub new_cert_path_validation_policy_id: CertPathValidationPolicyID,
}

impl Validate for ReplaceCertPathValidationPolicyAssignment {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct ReplaceCertPathValidationPolicyAssignmentResponse {}

impl Validate for ReplaceCertPathValidationPolicyAssignmentResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAssignedCertPathValidationPolicies {}

impl Validate for GetAssignedCertPathValidationPolicies {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAssignedCertPathValidationPoliciesResponse {
    // A list of IDs of the certification path validation policies that are
    // assigned to the TLS server.
    #[yaserde(prefix = "tas", rename = "CertPathValidationPolicyID")]
    pub cert_path_validation_policy_id: Vec<CertPathValidationPolicyID>,
}

impl Validate for GetAssignedCertPathValidationPoliciesResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct AddDot1XConfiguration {
    // The desired 802.1X configuration.
    #[yaserde(prefix = "tas", rename = "Dot1XConfiguration")]
    pub dot_1x_configuration: Dot1XConfiguration,
}

impl Validate for AddDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct AddDot1XConfigurationResponse {
    // The unique identifier of the created 802.1X configuration.
    #[yaserde(prefix = "tas", rename = "Dot1XID")]
    pub dot_1xid: Dot1XID,
}

impl Validate for AddDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllDot1XConfigurations {}

impl Validate for GetAllDot1XConfigurations {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetAllDot1XConfigurationsResponse {
    // The list of unique identifiers of 802.1X configurations on the device.
    #[yaserde(prefix = "tas", rename = "Configuration")]
    pub configuration: Vec<Dot1XConfiguration>,
}

impl Validate for GetAllDot1XConfigurationsResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetDot1XConfiguration {
    // The unique identifier of the desired 802.1X configuration.
    #[yaserde(prefix = "tas", rename = "Dot1XID")]
    pub dot_1xid: Dot1XID,
}

impl Validate for GetDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetDot1XConfigurationResponse {
    // The 802.1X configuration, without password information.
    #[yaserde(prefix = "tas", rename = "Dot1XConfiguration")]
    pub dot_1x_configuration: Dot1XConfiguration,
}

impl Validate for GetDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteDot1XConfiguration {
    // The unique identifier of the 802.1X configuration to be deleted.
    #[yaserde(prefix = "tas", rename = "Dot1XID")]
    pub dot_1xid: Dot1XID,
}

impl Validate for DeleteDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteDot1XConfigurationResponse {}

impl Validate for DeleteDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct SetNetworkInterfaceDot1XConfiguration {
    // The unique identifier of the Network Interface on which the 802.1X
    // configuration is to be set. (NOTE: the network interface token is defined
    // in devicemgmt.wsdl as tt:ReferenceToken, which is a derived type of
    // xs:string. To avoid importing all of common.xsd for this single type, the
    // base type is used here.)
    #[yaserde(prefix = "tas", rename = "token")]
    pub token: String,

    // The unique identifier of the 802.1X configuration to be set.
    #[yaserde(prefix = "tas", rename = "Dot1XID")]
    pub dot_1xid: Dot1XID,
}

impl Validate for SetNetworkInterfaceDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct SetNetworkInterfaceDot1XConfigurationResponse {
    // Indicates whether or not a reboot is required after configuration
    // updates.
    #[yaserde(prefix = "tas", rename = "RebootNeeded")]
    pub reboot_needed: bool,
}

impl Validate for SetNetworkInterfaceDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetNetworkInterfaceDot1XConfiguration {
    // The unique identifier of the Network Interface for which the 802.1X
    // configuration is to be retrieved. (NOTE: the network interface token is
    // defined in devicemgmt.wsdl as tt:ReferenceToken, which is a derived type
    // of xs:string. To avoid importing all of common.xsd for this single type,
    // the base type is used here.)
    #[yaserde(prefix = "tas", rename = "token")]
    pub token: String,
}

impl Validate for GetNetworkInterfaceDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct GetNetworkInterfaceDot1XConfigurationResponse {
    // The unique identifier of 802.1X configuration assigned to the Network
    // Interface.
    #[yaserde(prefix = "tas", rename = "Dot1XID")]
    pub dot_1xid: Option<Dot1XID>,
}

impl Validate for GetNetworkInterfaceDot1XConfigurationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteNetworkInterfaceDot1XConfiguration {
    // The unique identifier of the Network Interface for which the 802.1X
    // configuration is to be deleted. (NOTE: the network interface token is
    // defined in devicemgmt.wsdl as tt:ReferenceToken, which is a derived type
    // of xs:string. To avoid importing all of common.xsd for this single type,
    // the base type is used here.)
    #[yaserde(prefix = "tas", rename = "token")]
    pub token: String,
}

impl Validate for DeleteNetworkInterfaceDot1XConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tas",
    namespace = "tas: http://www.onvif.org/ver10/advancedsecurity/wsdl"
)]
pub struct DeleteNetworkInterfaceDot1XConfigurationResponse {
    // Indicates whether or not a reboot is required after configuration
    // updates.
    #[yaserde(prefix = "tas", rename = "RebootNeeded")]
    pub reboot_needed: bool,
}

impl Validate for DeleteNetworkInterfaceDot1XConfigurationResponse {}

// (to be written)
pub async fn add_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &AddDot1XConfiguration,
) -> Result<AddDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// (to be written)
pub async fn get_all_dot_1x_configurations<T: transport::Transport>(
    transport: &T,
    request: &GetAllDot1XConfigurations,
) -> Result<GetAllDot1XConfigurationsResponse, transport::Error> {
    transport::request(transport, request).await
}

// (to be written)
pub async fn get_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetDot1XConfiguration,
) -> Result<GetDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// (to be written)
pub async fn delete_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &DeleteDot1XConfiguration,
) -> Result<DeleteDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// (to be written)
pub async fn set_network_interface_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &SetNetworkInterfaceDot1XConfiguration,
) -> Result<SetNetworkInterfaceDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// (to be written)
pub async fn get_network_interface_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &GetNetworkInterfaceDot1XConfiguration,
) -> Result<GetNetworkInterfaceDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// (to be written)
pub async fn delete_network_interface_dot_1x_configuration<T: transport::Transport>(
    transport: &T,
    request: &DeleteNetworkInterfaceDot1XConfiguration,
) -> Result<DeleteNetworkInterfaceDot1XConfigurationResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation triggers the asynchronous generation of an RSA key pair of a
// particular key length (specified as the number of bits) as specified in [RFC
// 3447], with a suitable key generation mechanism on the device.
// Keys, especially RSA key pairs, are uniquely identified using key IDs.
pub async fn create_rsa_key_pair<T: transport::Transport>(
    transport: &T,
    request: &CreateRSAKeyPair,
) -> Result<CreateRSAKeyPairResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation uploads a key pair in a PKCS#8 data structure as specified in
// [RFC 5958, RFC 5959].
pub async fn upload_key_pair_in_pkcs8<T: transport::Transport>(
    transport: &T,
    request: &UploadKeyPairInPKCS8,
) -> Result<UploadKeyPairInPKCS8Response, transport::Error> {
    transport::request(transport, request).await
}

// This operation uploads a certification path consisting of X.509 certificates
// as specified by [RFC 5280] in DER encoding along with a private key to a
// device’s keystore.
// Certificates and private key are supplied in the form of a PKCS#12 file as
// specified in [PKCS#12].
pub async fn upload_certificate_with_private_key_in_pkcs12<T: transport::Transport>(
    transport: &T,
    request: &UploadCertificateWithPrivateKeyInPKCS12,
) -> Result<UploadCertificateWithPrivateKeyInPKCS12Response, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the status of a key.
pub async fn get_key_status<T: transport::Transport>(
    transport: &T,
    request: &GetKeyStatus,
) -> Result<GetKeyStatusResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns whether a key pair contains a private key.
pub async fn get_private_key_status<T: transport::Transport>(
    transport: &T,
    request: &GetPrivateKeyStatus,
) -> Result<GetPrivateKeyStatusResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns information about all keys that are stored in the
// device’s keystore.
pub async fn get_all_keys<T: transport::Transport>(
    transport: &T,
    request: &GetAllKeys,
) -> Result<GetAllKeysResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a key from the device’s keystore.
pub async fn delete_key<T: transport::Transport>(
    transport: &T,
    request: &DeleteKey,
) -> Result<DeleteKeyResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation generates a DER-encoded PKCS#10 v1.7 certification request
// (sometimes also called certificate signing request or CSR) as specified in
// RFC 2986
// for a public key on the device.
pub async fn create_pkcs10csr<T: transport::Transport>(
    transport: &T,
    request: &CreatePKCS10CSR,
) -> Result<CreatePKCS10CSRResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation generates for a public key on the device a self-signed X.509
// certificate that complies to RFC 5280.
pub async fn create_self_signed_certificate<T: transport::Transport>(
    transport: &T,
    request: &CreateSelfSignedCertificate,
) -> Result<CreateSelfSignedCertificateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation uploads an X.509 certificate as specified by [RFC 5280] in DER
// encoding and the public key in the certificate to a device’s keystore.
pub async fn upload_certificate<T: transport::Transport>(
    transport: &T,
    request: &UploadCertificate,
) -> Result<UploadCertificateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns a specific certificate from the device’s keystore.
pub async fn get_certificate<T: transport::Transport>(
    transport: &T,
    request: &GetCertificate,
) -> Result<GetCertificateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the IDs of all certificates that are stored in the
// device’s keystore.
pub async fn get_all_certificates<T: transport::Transport>(
    transport: &T,
    request: &GetAllCertificates,
) -> Result<GetAllCertificatesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a certificate from the device’s keystore.
pub async fn delete_certificate<T: transport::Transport>(
    transport: &T,
    request: &DeleteCertificate,
) -> Result<DeleteCertificateResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates a sequence of certificates that may be used, e.g., for
// certification path validation or for TLS server authentication.
pub async fn create_certification_path<T: transport::Transport>(
    transport: &T,
    request: &CreateCertificationPath,
) -> Result<CreateCertificationPathResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns a specific certification path from the device’s
// keystore.
pub async fn get_certification_path<T: transport::Transport>(
    transport: &T,
    request: &GetCertificationPath,
) -> Result<GetCertificationPathResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the IDs of all certification paths that are stored in
// the device’s keystore.
pub async fn get_all_certification_paths<T: transport::Transport>(
    transport: &T,
    request: &GetAllCertificationPaths,
) -> Result<GetAllCertificationPathsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a certification path from the device’s keystore.
pub async fn delete_certification_path<T: transport::Transport>(
    transport: &T,
    request: &DeleteCertificationPath,
) -> Result<DeleteCertificationPathResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation uploads a passphrase to the keystore of the device.
pub async fn upload_passphrase<T: transport::Transport>(
    transport: &T,
    request: &UploadPassphrase,
) -> Result<UploadPassphraseResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns information about all passphrases that are stored in
// the keystore of the device.
// This operation may be used, e.g., if a client lost track of which passphrases
// are present on the device.
// If no passphrase is stored on the device, the device shall return an empty
// list.
pub async fn get_all_passphrases<T: transport::Transport>(
    transport: &T,
    request: &GetAllPassphrases,
) -> Result<GetAllPassphrasesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a passphrase from the keystore of the device.
pub async fn delete_passphrase<T: transport::Transport>(
    transport: &T,
    request: &DeletePassphrase,
) -> Result<DeletePassphraseResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation uploads a certificate revocation list (CRL) as specified in
// [RFC 5280] to the keystore on the device.
// If the device does not have enough storage space to store the CRL to be
// uploaded, the device shall produce a MaximumNumberOfCRLsReached fault and
// shall not store the supplied CRL.
// If the device is not able to process the supplied CRL, the device shall
// produce a BadCRL fault and shall not store the supplied CRL.
// If the device does not support the signature algorithm that was used to sign
// the supplied CRL, the device shall produce an UnsupportedSignatureAlgorithm
// fault and shall not store the supplied CRL.
pub async fn upload_crl<T: transport::Transport>(
    transport: &T,
    request: &UploadCRL,
) -> Result<UploadCRLResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns a specific certificate revocation list (CRL) from the
// keystore on the device.
// Certification revocation lists are uniquely identified using CRLIDs. If no
// CRL is stored under the requested CRLID, the device shall produce a CRLID
// fault.
pub async fn get_crl<T: transport::Transport>(
    transport: &T,
    request: &GetCRL,
) -> Result<GetCRLResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns all certificate revocation lists (CRLs) that are
// stored in the keystore on the device.
// If no certificate revocation list is stored in the device’s keystore, an
// empty list is returned.
pub async fn get_all_cr_ls<T: transport::Transport>(
    transport: &T,
    request: &GetAllCRLs,
) -> Result<GetAllCRLsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a certificate revocation list (CRL) from the keystore
// on the device.
// Certification revocation lists are uniquely identified using CRLIDs. If no
// CRL is stored under the requested CRLID, the device shall produce a CRLID
// fault.
// If a reference exists for the specified CRL, the device shall produce a
// ReferenceExists fault and shall not delete the CRL.
// After a CRL has been successfully deleted, a device may assign its former ID
// to other CRLs.
pub async fn delete_crl<T: transport::Transport>(
    transport: &T,
    request: &DeleteCRL,
) -> Result<DeleteCRLResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation creates a certification path validation policy.
// Certification path validation policies are uniquely identified using
// certification path validation policy IDs. The device shall generate a new
// certification path validation policy ID for the created certification path
// validation policy.
// For the certification path validation parameters that are not represented in
// the certPathValidationParameters data type, the device shall use the default
// values specified in Sect. 3.
// If the device does not have enough storage capacity for storing the
// certification path validation policy to be created, the device shall produce
// a maximum number of certification path validation policies reached fault and
// shall not create a certification path validation policy.
// If there is at least one trust anchor certificate ID in the request for which
// there exists no certificate in the device’s keystore, the device shall
// produce a CertificateID fault and shall not create a certification path
// validation policy.
// If the device cannot process the supplied certification path validation
// parameters, the device shall produce a CertPathValidationParameters fault and
// shall not create a certification path validation policy.
pub async fn create_cert_path_validation_policy<T: transport::Transport>(
    transport: &T,
    request: &CreateCertPathValidationPolicy,
) -> Result<CreateCertPathValidationPolicyResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns a certification path validation policy from the
// keystore on the device.
// Certification path validation policies are uniquely identified using
// certification path validation policy IDs. If no certification path validation
// policy is stored under the requested certification path validation policy ID,
// the device shall produce a CertPathValidationPolicyID fault.
pub async fn get_cert_path_validation_policy<T: transport::Transport>(
    transport: &T,
    request: &GetCertPathValidationPolicy,
) -> Result<GetCertPathValidationPolicyResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns all certification path validation policies that are
// stored in the keystore on the device.
// If no certification path validation policy is stored in the device’s
// keystore, an empty list is returned.
pub async fn get_all_cert_path_validation_policies<T: transport::Transport>(
    transport: &T,
    request: &GetAllCertPathValidationPolicies,
) -> Result<GetAllCertPathValidationPoliciesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation deletes a certification path validation policy from the
// keystore on the device.
// Certification path validation policies are uniquely identified using
// certification path validation policy IDs. If no certification path validation
// policy is stored under the requested certification path validation policy ID,
// the device shall produce an InvalidCertPathValidationPolicyID fault.
// If a reference exists for the requested certification path validation policy,
// the device shall produce a ReferenceExists fault and shall not delete the
// certification path validation policy.
// After the certification path validation policy has been deleted, the device
// may assign its former ID to other certification path validation policies.
pub async fn delete_cert_path_validation_policy<T: transport::Transport>(
    transport: &T,
    request: &DeleteCertPathValidationPolicy,
) -> Result<DeleteCertPathValidationPolicyResponse, transport::Error> {
    transport::request(transport, request).await
}

// Returns the capabilities of the security configuraiton service. The result is
// returned in a typed answer.
pub async fn get_service_capabilities<T: transport::Transport>(
    transport: &T,
    request: &GetServiceCapabilities,
) -> Result<GetServiceCapabilitiesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation assigns a key pair and certificate along with a certification
// path (certificate chain) to the TLS server on the device.
// The TLS server shall use this information for key exchange during the TLS
// handshake, particularly for constructing server certificate messages as
// specified in RFC 4346 and RFC 2246.
pub async fn add_server_certificate_assignment<T: transport::Transport>(
    transport: &T,
    request: &AddServerCertificateAssignment,
) -> Result<AddServerCertificateAssignmentResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes a key pair and certificate assignment (including
// certification path) to the TLS server on the device.
pub async fn remove_server_certificate_assignment<T: transport::Transport>(
    transport: &T,
    request: &RemoveServerCertificateAssignment,
) -> Result<RemoveServerCertificateAssignmentResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation replaces an existing key pair and certificate assignment to
// the TLS server on the device by a new key pair and certificate assignment
// (including certification paths).
pub async fn replace_server_certificate_assignment<T: transport::Transport>(
    transport: &T,
    request: &ReplaceServerCertificateAssignment,
) -> Result<ReplaceServerCertificateAssignmentResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation sets the version(s) of TLS which the device shall use. Valid
// values are taken from the TLSServerSupported capability.
pub async fn set_enabled_tls_versions<T: transport::Transport>(
    transport: &T,
    request: &SetEnabledTLSVersions,
) -> Result<SetEnabledTLSVersionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation retrieves the version(s) of TLS which are currently enabled on
// the device.
pub async fn get_enabled_tls_versions<T: transport::Transport>(
    transport: &T,
    request: &GetEnabledTLSVersions,
) -> Result<GetEnabledTLSVersionsResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the IDs of all key pairs and certificates (including
// certification paths) that are assigned to the TLS server on the device.
pub async fn get_assigned_server_certificates<T: transport::Transport>(
    transport: &T,
    request: &GetAssignedServerCertificates,
) -> Result<GetAssignedServerCertificatesResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation activates or deactivates TLS client authentication for the TLS
// server on the device.
// The TLS server on the device shall require client authentication if and only
// if clientAuthenticationRequired is set to true.
// If TLS client authentication is requested to be enabled and no certification
// path validation policy is assigned to the TLS server, the device shall return
// an EnablingTLSClientAuthenticationFailed fault and shall not enable TLS
// client authentication.
// The device shall execute this command regardless of the TLS enabled/disabled
// state configured in the ONVIF Device Management Service.
pub async fn set_client_authentication_required<T: transport::Transport>(
    transport: &T,
    request: &SetClientAuthenticationRequired,
) -> Result<SetClientAuthenticationRequiredResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns whether TLS client authentication is active.
pub async fn get_client_authentication_required<T: transport::Transport>(
    transport: &T,
    request: &GetClientAuthenticationRequired,
) -> Result<GetClientAuthenticationRequiredResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation assigns a certification path validation policy to the TLS
// server on the device. The TLS server shall enforce the policy when
// authenticating TLS clients and consider a client authentic if and only if the
// algorithm returns valid.
// If no certification path validation policy is stored under the requested
// CertPathValidationPolicyID, the device shall produce a
// CertPathValidationPolicyID fault.
// A TLS server may use different certification path validation policies to
// authenticate clients. Therefore more than one certification path validation
// policy may be assigned to the TLS server. If the maximum number of
// certification path validation policies that may be assigned to the TLS server
// simultaneously is reached, the device shall produce a
// MaximumNumberOfTLSCertPathValidationPoliciesReached fault and shall not
// assign the requested certification path validation policy to the TLS server.
pub async fn add_cert_path_validation_policy_assignment<T: transport::Transport>(
    transport: &T,
    request: &AddCertPathValidationPolicyAssignment,
) -> Result<AddCertPathValidationPolicyAssignmentResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation removes a certification path validation policy assignment from
// the TLS server on the device.
// If the certification path validation policy identified by the requested
// CertPathValidationPolicyID is not associated to the TLS server, the device
// shall produce a CertPathValidationPolicy fault.
pub async fn remove_cert_path_validation_policy_assignment<T: transport::Transport>(
    transport: &T,
    request: &RemoveCertPathValidationPolicyAssignment,
) -> Result<RemoveCertPathValidationPolicyAssignmentResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation replaces a certification path validation policy assignment to
// the TLS server on the device with another certification path validation
// policy assignment.
// If the certification path validation policy identified by the requested
// OldCertPathValidationPolicyID is not associated to the TLS server, the device
// shall produce an OldCertPathValidationPolicyID fault and shall not associate
// the certification path validation policy identified by the
// NewCertPathValidationPolicyID to the TLS server.
// If no certification path validation policy exists under the requested
// NewCertPathValidationPolicyID in the device’s keystore, the device shall
// produce a NewCertPathValidationPolicyID fault and shall not remove the
// association of the old certification path validation policy to the TLS
// server.
pub async fn replace_cert_path_validation_policy_assignment<T: transport::Transport>(
    transport: &T,
    request: &ReplaceCertPathValidationPolicyAssignment,
) -> Result<ReplaceCertPathValidationPolicyAssignmentResponse, transport::Error> {
    transport::request(transport, request).await
}

// This operation returns the IDs of all certification path validation policies
// that are assigned to the TLS server on the device.
pub async fn get_assigned_cert_path_validation_policies<T: transport::Transport>(
    transport: &T,
    request: &GetAssignedCertPathValidationPolicies,
) -> Result<GetAssignedCertPathValidationPoliciesResponse, transport::Error> {
    transport::request(transport, request).await
}
