<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="../../../ver20/util/onvif-wsdl-viewer.xsl"?>
<!--
	Copyright (c) 2013 - 2018 by ONVIF: Open Network Video Interface Forum. All rights reserved.
	
	Recipients of this document may copy, distribute, publish, or display this document so long as this copyright notice, license and disclaimer are retained with all copies of the document. No license is granted to modify this document.
	
	THIS DOCUMENT IS PROVIDED "AS IS," AND THE CORPORATION AND ITS MEMBERS AND THEIR AFFILIATES, MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, OR TITLE; THAT THE CONTENTS OF THIS DOCUMENT ARE SUITABLE FOR ANY PURPOSE; OR THAT THE IMPLEMENTATION OF SUCH CONTENTS WILL NOT INFRINGE ANY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
	IN NO EVENT WILL THE CORPORATION OR ITS MEMBERS OR THEIR AFFILIATES BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE OR CONSEQUENTIAL DAMAGES, ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT, WHETHER OR NOT (1) THE CORPORATION, MEMBERS OR THEIR AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES, OR (2) SUCH DAMAGES WERE REASONABLY FORESEEABLE, AND ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT.  THE FOREGOING DISCLAIMER AND LIMITATION ON LIABILITY DO NOT APPLY TO, INVALIDATE, OR LIMIT REPRESENTATIONS AND WARRANTIES MADE BY THE MEMBERS AND THEIR RESPECTIVE AFFILIATES TO THE CORPORATION AND OTHER MEMBERS IN CERTAIN WRITTEN POLICIES OF THE CORPORATION.
-->
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" xmlns:tas="http://www.onvif.org/ver10/advancedsecurity/wsdl" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/" targetNamespace="http://www.onvif.org/ver10/advancedsecurity/wsdl">
	<wsdl:types>
		<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified" targetNamespace="http://www.onvif.org/ver10/advancedsecurity/wsdl" version="18.12">
			<!--===================================================-->
			<!-- Data types used by the security configuration features -->
			<!--===================================================-->
			<xs:simpleType name="KeyID">
				<xs:annotation>
					<xs:documentation>Unique identifier for keys in the keystore.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:NCName">
					<xs:maxLength value="64"/>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:simpleType name="CertificateID">
				<xs:annotation>
					<xs:documentation>Unique identifier for certificates in the keystore.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:NCName">
					<xs:maxLength value="64"/>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:simpleType name="CertificationPathID">
				<xs:annotation>
					<xs:documentation>Unique identifier for certification paths in the keystore.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:NCName">
					<xs:maxLength value="64"/>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:simpleType name="PassphraseID">
				<xs:annotation>
					<xs:documentation>Unique identifier for passphrases in the keystore.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:NCName">
					<xs:maxLength value="64"/>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:simpleType name="Dot1XID">
				<xs:annotation>
					<xs:documentation>Unique identifier for 802.1X configurations in the keystore.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:NCName">
					<xs:maxLength value="64"/>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:simpleType name="KeyStatus">
				<xs:annotation>
					<xs:documentation>The status of a key in the keystore.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:string">
					<xs:enumeration value="ok">
						<xs:annotation>
							<xs:documentation>Key is ready for use</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
					<xs:enumeration value="generating">
						<xs:annotation>
							<xs:documentation>Key is being generated</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
					<xs:enumeration value="corrupt">
						<xs:annotation>
							<xs:documentation>Key has not been successfully generated and cannot be used.</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:simpleType name="DotDecimalOID">
				<xs:annotation>
					<xs:documentation>An object identifier (OID) in dot-decimal form as specified in RFC4512.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:string">
					<xs:pattern value="[0-9]+(.[0-9]+)*"/>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:simpleType name="DNAttributeType">
				<xs:annotation>
					<xs:documentation>The distinguished name attribute type encoded as specified in RFC 4514.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:string"/>
			</xs:simpleType>
			<!--===============================-->
			<xs:simpleType name="DNAttributeValue">
				<xs:restriction base="xs:string">
					<xs:annotation>
						<xs:documentation>The distinguished name attribute values are encoded in UTF-8 or in hexadecimal form as specified in RFC 4514.</xs:documentation>
					</xs:annotation>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="KeyAttribute">
				<xs:annotation>
					<xs:documentation>The attributes of a key in the keystore.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="KeyID" type="tas:KeyID">
						<xs:annotation>
							<xs:documentation>The ID of the key.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Alias" type="xs:string" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The client-defined alias of the key.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="hasPrivateKey" type="xs:boolean" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Absent if the key is not a key pair. True if and only if the key is a key pair and contains a private key. False if and only if the key is a key pair and does not contain a private key.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="KeyStatus" type="xs:string">
						<xs:annotation>
							<xs:documentation>The status of the key. The value should be one of the values in the tas:KeyStatus enumeration.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##other" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>
					<xs:element name="externallyGenerated" type="xs:boolean" minOccurs="0">
						<xs:annotation>
							<xs:documentation>True if and only if the key was generated outside the device.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="securelyStored" type="xs:boolean" minOccurs="0">
						<xs:annotation>
							<xs:documentation>True if and only if the key is stored in a specially protected hardware component inside the device.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Extension" minOccurs="0">
						<xs:complexType>
							<xs:sequence>
								<xs:any namespace="##targetNamespace" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="DNAttributeTypeAndValue">
				<xs:annotation>
					<xs:documentation>A distinguished name attribute type and value pair.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="Type" type="tas:DNAttributeType">
						<xs:annotation>
							<xs:documentation>The attribute type.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Value" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>The value of the attribute.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="MultiValuedRDN">
				<xs:annotation>
					<xs:documentation>A multi-valued RDN</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="Attribute" type="tas:DNAttributeTypeAndValue">
						<xs:annotation>
							<xs:documentation>A list of types and values defining a multi-valued RDN</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="DistinguishedName">
				<xs:sequence>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="Country" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A country name as specified in
								X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="Organization" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>An organization name as specified in
								X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="OrganizationalUnit" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>An organizational unit name as specified in
								X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="DistinguishedNameQualifier" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A distinguished name qualifier as specified in
								X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="StateOrProvinceName" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A state or province name as specified in
								X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="CommonName" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A common name as specified in
								X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="SerialNumber" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A serial number as specified in
								X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="Locality" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A locality as specified in X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="Title" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A title as specified in X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="Surname" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A surname as specified in X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="GivenName" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A given name as specified in X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="Initials" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>Initials as specified in X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="Pseudonym" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A pseudonym as specified in X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="GenerationQualifier" type="tas:DNAttributeValue">
						<xs:annotation>
							<xs:documentation>A generation qualifier as specified in
								X.500.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="GenericAttribute" type="tas:DNAttributeTypeAndValue">
						<xs:annotation>
							<xs:documentation>A generic type-value pair
								attribute.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="MultiValuedRDN" type="tas:MultiValuedRDN">
						<xs:annotation>
							<xs:documentation>A multi-valued RDN</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" name="anyAttribute">
						<xs:annotation>
							<xs:documentation>
                Required extension point. It is recommended to not use this element, and instead use GenericAttribute and the numeric Distinguished Name Attribute Type.
              </xs:documentation>
						</xs:annotation>
						<xs:complexType>
							<xs:sequence>
								<xs:element minOccurs="0" maxOccurs="unbounded" name="DomainComponent" type="tas:DNAttributeValue">
									<xs:annotation>
										<xs:documentation>Domain Component as specified in RFC3739</xs:documentation>
									</xs:annotation>
								</xs:element>
								<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="AlgorithmIdentifier">
				<xs:annotation>
					<xs:documentation>An identifier of an algorithm.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="algorithm" type="tas:DotDecimalOID">
						<xs:annotation>
							<xs:documentation>The OID of the algorithm in dot-decimal form.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" name="parameters" type="tas:Base64DERencodedASN1Value">
						<xs:annotation>
							<xs:documentation>Optional parameters of the algorithm (depending on the algorithm).</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" name="anyParameters">
						<xs:complexType>
							<xs:sequence>
								<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="BasicRequestAttribute">
				<xs:annotation>
					<xs:documentation>A CSR attribute as specified in RFC 2986.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="OID" type="tas:DotDecimalOID">
						<xs:annotation>
							<xs:documentation>The OID of the attribute.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="value" type="tas:Base64DERencodedASN1Value">
						<xs:annotation>
							<xs:documentation>The value of the attribute as a base64-encoded DER representation of an ASN.1 value.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="CSRAttribute">
				<xs:annotation>
					<xs:documentation>A CSR attribute as specified in PKCS#10.</xs:documentation>
				</xs:annotation>
				<xs:choice>
					<xs:element name="X509v3Extension" type="tas:X509v3Extension">
						<xs:annotation>
							<xs:documentation>An X.509v3 extension field.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="BasicRequestAttribute" type="tas:BasicRequestAttribute">
						<xs:annotation>
							<xs:documentation>A basic CSR attribute.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" name="anyAttribute">
						<xs:complexType>
							<xs:sequence>
								<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:choice>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:simpleType name="Base64DERencodedASN1Value">
				<xs:annotation>
					<xs:documentation>A base64-encoded ASN.1 value.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:base64Binary"/>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="X509v3Extension">
				<xs:annotation>
					<xs:documentation>An X.509v3 extension field as specified in RFC 5280</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="extnOID" type="tas:DotDecimalOID">
						<xs:annotation>
							<xs:documentation>The OID of the extension field.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element default="false" name="critical" type="xs:boolean">
						<xs:annotation>
							<xs:documentation>True if and only if the extension is critical.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="extnValue" type="tas:Base64DERencodedASN1Value">
						<xs:annotation>
							<xs:documentation>The value of the extension field as a base64-encoded DER representation of an ASN.1 value.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="X509Certificate">
				<xs:annotation>
					<xs:documentation>An X.509 cerficiate as specified in RFC 5280.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="CertificateID" type="tas:CertificateID">
						<xs:annotation>
							<xs:documentation>The ID of the certificate.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="KeyID" type="tas:KeyID">
						<xs:annotation>
							<xs:documentation>The ID of the key that this certificate associates to the certificate subject.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Alias" type="xs:string" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The client-defined alias of the certificate.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="CertificateContent" type="tas:Base64DERencodedASN1Value">
						<xs:annotation>
							<xs:documentation>The base64-encoded DER representation of the X.509 certificate.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="CertificateIDs">
				<xs:annotation>
					<xs:documentation>A sequence of certificate IDs.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element maxOccurs="unbounded" name="CertificateID" type="tas:CertificateID">
						<xs:annotation>
							<xs:documentation>A certificate ID.</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="CertificationPath">
				<xs:annotation>
					<xs:documentation>An X.509 certification path as defined in RFC 5280.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element maxOccurs="unbounded" name="CertificateID" type="tas:CertificateID">
						<xs:annotation>
							<xs:documentation>A certificate in the certification path.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Alias" type="xs:string" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The client-defined alias of the certification path.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" name="anyElement">
						<xs:complexType>
							<xs:sequence>
								<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="PassphraseAttribute">
				<xs:sequence>
					<xs:element name="PassphraseID" type="tas:PassphraseID">
						<xs:annotation>
							<xs:documentation>The ID of the passphrase.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Alias" type="xs:string" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The alias of the passphrase.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:simpleType name="Dot1XMethods">
				<xs:annotation>
					<xs:documentation>A list of supported 802.1X authentication methods, such as "EAP-PEAP/MSCHAPv2" and "EAP-MD5".  The '/' character is used as a separator between the outer and inner methods.</xs:documentation>
				</xs:annotation>
				<xs:list itemType="xs:string"/>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="Dot1XCapabilities">
				<xs:annotation>
					<xs:documentation>The capabilities of the 802.1X implementation on a device.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:attribute name="MaximumNumberOfDot1XConfigurations" type="xs:positiveInteger">
					<xs:annotation>
						<xs:documentation>The maximum number of 802.1X configurations that may be defined simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Dot1XMethods" type="tas:Dot1XMethods">
					<xs:annotation>
						<xs:documentation>The authentication methods supported by the 802.1X implementation.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="Dot1XStage">
				<xs:annotation>
					<xs:documentation>The configuration parameters required for a particular authentication method.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="Identity" type="xs:string" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The identity used in this authentication method, if required.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="CertificationPathID" type="tas:CertificationPathID" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The unique identifier of the certification path used in this authentication method, if required.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="PassphraseID" type="tas:PassphraseID" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The identifier for the password used in this authentication method, if required.  If Identity is used as an anonymous identity for this authentication method, PassphraseID is ignored.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Inner" type="tas:Dot1XStage" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The configuration of the next stage of authentication, if required.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Extension" type="tas:Dot1XStageExtension" minOccurs="0"/>
				</xs:sequence>
				<xs:attribute name="Method" type="xs:string" use="required">
					<xs:annotation>
						<xs:documentation>The authentication method for this stage (e.g., "EAP-PEAP").</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:complexType name="Dot1XStageExtension">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="Dot1XConfiguration">
				<xs:sequence>
					<xs:element name="Dot1XID" type="tas:Dot1XID" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The unique identifier of the IEEE 802.1X configuration.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Alias" type="xs:string" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The client-defined alias of the 802.1X configuration.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Outer" type="tas:Dot1XStage">
						<xs:annotation>
							<xs:documentation>The outer level authentication method used in this 802.1X configuration.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--==========================================-->
			<!-- Begin Cert-based client auth data types  -->
			<!--==========================================-->
			<xs:simpleType name="CRLID">
				<xs:restriction base="xs:NCName">
					<xs:maxLength value="64"/>
				</xs:restriction>
			</xs:simpleType>
			<xs:simpleType name="CertPathValidationPolicyID">
				<xs:restriction base="xs:NCName">
					<xs:maxLength value="64"/>
				</xs:restriction>
			</xs:simpleType>
			<xs:complexType name="CRL">
				<xs:sequence>
					<xs:element name="CRLID" type="tas:CRLID"/>
					<xs:element name="Alias" type="xs:string"/>
					<xs:element name="CRLContent" type="tas:Base64DERencodedASN1Value"/>
					<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="CertPathValidationParameters">
				<xs:sequence>
					<xs:element name="RequireTLSWWWClientAuthExtendedKeyUsage" type="xs:boolean" minOccurs="0" default="false">
						<xs:annotation>
							<xs:documentation>True if and only if the TLS server shall not authenticate client certificates that do not contain the TLS WWW client authentication key usage extension as specified in RFC 5280, Sect. 4.2.1.12.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="UseDeltaCRLs" type="xs:boolean" minOccurs="0" default="false">
						<xs:annotation>
							<xs:documentation>True if and only if delta CRLs, if available, shall be applied to CRLs.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" name="anyParameters">
						<xs:complexType>
							<xs:sequence>
								<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="TrustAnchor">
				<xs:sequence>
					<xs:element name="CertificateID" type="tas:CertificateID">
						<xs:annotation>
							<xs:documentation>The certificate ID of the certificate to be used as trust anchor.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
			</xs:complexType>
			<xs:complexType name="CertPathValidationPolicy">
				<xs:sequence>
					<xs:element name="CertPathValidationPolicyID" type="tas:CertPathValidationPolicyID"/>
					<xs:element name="Alias" type="xs:string" minOccurs="0"/>
					<xs:element name="Parameters" type="tas:CertPathValidationParameters"/>
					<xs:element name="TrustAnchor" type="tas:TrustAnchor" maxOccurs="unbounded"/>
					<xs:element minOccurs="0" name="anyParameters">
						<xs:complexType>
							<xs:sequence>
								<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			<!--========================================-->
			<!-- End Cert-based client auth data types-->
			<!--========================================-->
			<!--===============================-->
			<xs:simpleType name="RSAKeyLengths">
				<xs:annotation>
					<xs:documentation>A list of RSA key lenghts in bits.</xs:documentation>
				</xs:annotation>
				<xs:list itemType="xs:nonNegativeInteger"/>
			</xs:simpleType>
			<xs:simpleType name="X509Versions">
				<xs:annotation>
					<xs:documentation>A list of X.509 versions.</xs:documentation>
				</xs:annotation>
				<xs:list itemType="xs:int"/>
			</xs:simpleType>
			<xs:simpleType name="TLSVersions">
				<xs:annotation>
					<xs:documentation>A list of TLS versions.</xs:documentation>
				</xs:annotation>
				<xs:list itemType="xs:string"/>
			</xs:simpleType>
			<xs:simpleType name="PasswordBasedEncryptionAlgorithms">
				<xs:annotation>
					<xs:documentation>A list of password based encryption algorithms.</xs:documentation>
				</xs:annotation>
				<xs:list itemType="xs:string"/>
			</xs:simpleType>
			<xs:simpleType name="PasswordBasedMACAlgorithms">
				<xs:annotation>
					<xs:documentation>A list of password based MAC algorithms.</xs:documentation>
				</xs:annotation>
				<xs:list itemType="xs:string"/>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="KeystoreCapabilities">
				<xs:annotation>
					<xs:documentation>The capabilities of a keystore implementation on a device.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element minOccurs="0" maxOccurs="unbounded" name="SignatureAlgorithms" type="tas:AlgorithmIdentifier">
						<xs:annotation>
							<xs:documentation>The signature algorithms supported by the keystore implementation.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element minOccurs="0" name="anyElement">
						<xs:complexType>
							<xs:sequence>
								<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
				<xs:attribute name="MaximumNumberOfKeys" type="xs:positiveInteger">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of keys that the device can store simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaximumNumberOfCertificates" type="xs:positiveInteger">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of certificates that the device can store simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaximumNumberOfCertificationPaths" type="xs:positiveInteger">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of certification paths that the device can store simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RSAKeyPairGeneration" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indication that the device supports on-board RSA key pair generation.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RSAKeyLengths" type="tas:RSAKeyLengths">
					<xs:annotation>
						<xs:documentation>Indicates which RSA key lengths are supported by the device.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="PKCS10ExternalCertificationWithRSA" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for creating PKCS#10 requests for RSA keys and uploading the certificate obtained from a CA..</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SelfSignedCertificateCreationWithRSA" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for creating self-signed certificates for RSA keys.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="X509Versions" type="tas:X509Versions">
					<xs:annotation>
						<xs:documentation>Indicates which X.509 versions are supported by the device.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaximumNumberOfPassphrases" type="xs:nonNegativeInteger">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of passphrases that the device is able to store simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="PKCS8RSAKeyPairUpload" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for uploading an RSA key pair in a PKCS#8 data structure.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="PKCS12CertificateWithRSAPrivateKeyUpload" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for uploading a certificate along with an RSA private key in a PKCS#12 data structure.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="PasswordBasedEncryptionAlgorithms" type="tas:PasswordBasedEncryptionAlgorithms">
					<xs:annotation>
						<xs:documentation>Indicates which password-based encryption algorithms are supported by the device.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="PasswordBasedMACAlgorithms" type="tas:PasswordBasedMACAlgorithms">
					<xs:annotation>
						<xs:documentation>Indicates which password-based MAC algorithms are supported by the device.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<!-- ========================================= -->
				<!-- Begin cert-based client auth capabilities -->
				<!-- ========================================= -->
				<xs:attribute name="MaximumNumberOfCRLs" type="xs:nonNegativeInteger">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of CRLs that the device is able to store simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaximumNumberOfCertificationPathValidationPolicies" type="xs:nonNegativeInteger">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of certification path validation policies that the device is able to store simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="EnforceTLSWebClientAuthExtKeyUsage" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates whether a device supports checking for the TLS WWW client auth extended key usage extension while validating certification paths.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<!-- ========================================= -->
				<!-- End cert-based client auth capabilities   -->
				<!-- ========================================= -->
				<xs:attribute name="NoPrivateKeySharing" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates the device requires that each certificate with private key has its own unique key.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="TLSServerCapabilities">
				<xs:annotation>
					<xs:documentation>The capabilities of a TLS server implementation on a device.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:attribute name="TLSServerSupported" type="tas:TLSVersions">
					<xs:annotation>
						<xs:documentation>Indicates which TLS versions are supported by the device.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="EnabledVersionsSupported" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates whether the device supports enabling and disabling specific TLS versions.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaximumNumberOfTLSCertificationPaths" type="xs:positiveInteger">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of certification paths that may be assigned to the TLS server simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<!-- ========================================= -->
				<!-- Begin cert-based client auth capabilities -->
				<!-- ========================================= -->
				<xs:attribute name="TLSClientAuthSupported" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates whether the device supports TLS client authentication.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaximumNumberOfTLSCertificationPathValidationPolicies" type="xs:nonNegativeInteger">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of certification path validation policies that may be assigned to the TLS server simultaneously.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<!-- ========================================= -->
				<!-- End cert-based client auth capabilities   -->
				<!-- ========================================= -->
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="Capabilities">
				<xs:annotation>
					<xs:documentation>The capabilities of a Security Configuration Service implementation on a device.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="KeystoreCapabilities" type="tas:KeystoreCapabilities">
						<xs:annotation>
							<xs:documentation>The capabilities of the keystore implementation.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="TLSServerCapabilities" type="tas:TLSServerCapabilities">
						<xs:annotation>
							<xs:documentation>The capabilities of the TLS server implementation.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Dot1XCapabilities" type="tas:Dot1XCapabilities" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The capabilities of the 802.1X implementation.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:element name="Capabilities" type="tas:Capabilities"/>
			<!--=========================================-->
			<!-- Request/response elements               -->
			<!--=========================================-->
			<xs:element name="GetServiceCapabilities">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetServiceCapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="tas:Capabilities">
							<xs:annotation>
								<xs:documentation>The capabilities for the security configuration service is returned in the Capabilities element.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateRSAKeyPair">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyLength" type="xs:nonNegativeInteger">
							<xs:annotation>
								<xs:documentation>The length of the key to be created.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Alias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The client-defined alias of the key.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateRSAKeyPairResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The key ID of the key pair being generated.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="EstimatedCreationTime" type="xs:duration">
							<xs:annotation>
								<xs:documentation>Best-effort estimate of how long the key generation will take.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="UploadKeyPairInPKCS8">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyPair" type="tas:Base64DERencodedASN1Value">
							<xs:annotation>
								<xs:documentation>The key pair to be uploaded in a PKCS#8 data structure.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Alias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The client-defined alias of the key pair.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="EncryptionPassphraseID" type="tas:PassphraseID" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The ID of the passphrase to use for decrypting the uploaded key pair.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="EncryptionPassphrase" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The passphrase to use for decrypting the uploaded key pair.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="UploadKeyPairInPKCS8Response">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The key ID of the uploaded key pair.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="UploadCertificateWithPrivateKeyInPKCS12">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertWithPrivateKey" type="tas:Base64DERencodedASN1Value">
							<xs:annotation>
								<xs:documentation>The certificates and key pair to be uploaded in a PKCS#12 data structure.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="CertificationPathAlias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The client-defined alias of the certification path.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeyAlias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The client-defined alias of the key pair.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="IgnoreAdditionalCertificates" type="xs:boolean" minOccurs="0" default="false">
							<xs:annotation>
								<xs:documentation>True if and only if the device shall behave as if the client had only supplied the first certificate in the sequence of certificates.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="IntegrityPassphraseID" type="tas:PassphraseID" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The ID of the passphrase to use for integrity checking of the uploaded PKCS#12 data structure.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="EncryptionPassphraseID" type="tas:PassphraseID" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The ID of the passphrase to use for decrypting the uploaded PKCS#12 data structure.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Passphrase" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The passphrase to use for integrity checking and decrypting the uploaded PKCS#12 data structure.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="UploadCertificateWithPrivateKeyInPKCS12Response">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPathID" type="tas:CertificationPathID">
							<xs:annotation>
								<xs:documentation>The certification path ID of the uploaded certification path.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The key ID of the uploaded key pair.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetKeyStatus">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The ID of the key for which to return the status.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetKeyStatusResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyStatus" type="xs:string">
							<xs:annotation>
								<xs:documentation>Status of the requested key. The value should be one of the values in the tas:KeyStatus enumeration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetPrivateKeyStatus">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The ID of the key pair for which to return whether it contains a private key.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetPrivateKeyStatusResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="hasPrivateKey" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>True if and only if the key pair contains a private key.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAllKeys">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAllKeysResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyAttribute" type="tas:KeyAttribute" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Information about a key in the keystore.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteKey">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The ID of the key that is to be deleted from the keystore.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteKeyResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreatePKCS10CSR">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Subject" type="tas:DistinguishedName">
							<xs:annotation>
								<xs:documentation>The subject to be included in the CSR.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The ID of the key for which the CSR shall be created.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="CSRAttribute" minOccurs="0" maxOccurs="unbounded" type="tas:CSRAttribute">
							<xs:annotation>
								<xs:documentation>An attribute to be included in the CSR.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="SignatureAlgorithm" type="tas:AlgorithmIdentifier">
							<xs:annotation>
								<xs:documentation>The signature algorithm to be used to sign the CSR.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreatePKCS10CSRResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PKCS10CSR" type="tas:Base64DERencodedASN1Value">
							<xs:annotation>
								<xs:documentation>The DER encoded PKCS#10 certification request.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateSelfSignedCertificate">
				<xs:complexType>
					<xs:sequence>
						<xs:element minOccurs="0" name="X509Version" type="xs:positiveInteger">
							<xs:annotation>
								<xs:documentation>The X.509 version that the generated certificate shall comply to.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Subject" type="tas:DistinguishedName">
							<xs:annotation>
								<xs:documentation>Distinguished name of the entity that the certificate shall belong to.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The ID of the key for which the certificate shall be created.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Alias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The client-defined alias of the certificate to be created.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element minOccurs="0" name="notValidBefore" type="xs:dateTime">
							<xs:annotation>
								<xs:documentation>The X.509 not valid before information to be included in the certificate. Defaults to the device's current time or a time before the device's current time.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element minOccurs="0" name="notValidAfter" type="xs:dateTime">
							<xs:annotation>
								<xs:documentation>The X.509 not valid after information to be included in the certificate. Defaults to the time 99991231235959Z as specified in RFC 5280.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="SignatureAlgorithm" type="tas:AlgorithmIdentifier">
							<xs:annotation>
								<xs:documentation>The signature algorithm to be used for signing the certificate.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element minOccurs="0" maxOccurs="unbounded" name="Extension" type="tas:X509v3Extension">
							<xs:annotation>
								<xs:documentation>An X.509v3 extension to be included in the certificate.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateSelfSignedCertificateResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateID" type="tas:CertificateID">
							<xs:annotation>
								<xs:documentation>The ID of the generated certificate.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="UploadCertificate">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Certificate" type="tas:Base64DERencodedASN1Value">
							<xs:annotation>
								<xs:documentation>The base64-encoded DER representation of the X.509 certificate to be uploaded.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Alias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The client-defined alias of the certificate.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeyAlias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The client-defined alias of the key pair.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="PrivateKeyRequired" type="xs:boolean" minOccurs="0" default="false">
							<xs:annotation>
								<xs:documentation>Indicates if the device shall verify that a matching key pair with a private key exists in the keystore.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="UploadCertificateResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateID" type="tas:CertificateID">
							<xs:annotation>
								<xs:documentation>The ID of the uploaded certificate.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeyID" type="tas:KeyID">
							<xs:annotation>
								<xs:documentation>The ID of the key that the uploaded certificate certifies.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCertificate">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateID" type="tas:CertificateID">
							<xs:annotation>
								<xs:documentation>The ID of the certificate to retrieve.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCertificateResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Certificate" type="tas:X509Certificate">
							<xs:annotation>
								<xs:documentation>The DER representation of the certificate.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAllCertificates">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAllCertificatesResponse">
				<xs:complexType>
					<xs:annotation>
						<xs:documentation>A list with all certificates stored in the keystore.</xs:documentation>
					</xs:annotation>
					<xs:sequence>
						<xs:element minOccurs="0" maxOccurs="unbounded" name="Certificate" type="tas:X509Certificate">
							<xs:annotation>
								<xs:documentation>A certificate stored in the keystore.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteCertificate">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateID" type="tas:CertificateID">
							<xs:annotation>
								<xs:documentation>The ID of the certificate to delete.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteCertificateResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateCertificationPath">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateIDs" type="tas:CertificateIDs">
							<xs:annotation>
								<xs:documentation>The IDs of the certificates to include in the certification path, where each certificate signature except for the last one in the path must be verifiable with the public key certified by the next certificate in the path.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Alias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The client-defined alias of the certification path.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateCertificationPathResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPathID" type="tas:CertificationPathID">
							<xs:annotation>
								<xs:documentation>The ID of the generated certification path.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCertificationPath">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPathID" type="tas:CertificationPathID">
							<xs:annotation>
								<xs:documentation>The ID of the certification path to retrieve.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCertificationPathResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPath" type="tas:CertificationPath">
							<xs:annotation>
								<xs:documentation>The certification path that is stored under the given ID in the keystore.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAllCertificationPaths">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAllCertificationPathsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPathID" type="tas:CertificationPathID" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>An ID of a certification path in the keystore.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteCertificationPath">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPathID" type="tas:CertificationPathID">
							<xs:annotation>
								<xs:documentation>The ID of the certification path to delete.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteCertificationPathResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="UploadPassphrase">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Passphrase" type="xs:string">
							<xs:annotation>
								<xs:documentation>The passphrase to upload.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="PassphraseAlias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The alias for the passphrase to upload.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="UploadPassphraseResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PassphraseID" type="tas:PassphraseID">
							<xs:annotation>
								<xs:documentation>The PassphraseID of the uploaded passphrase.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAllPassphrases">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAllPassphrasesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:annotation>
							<xs:documentation>A list with information about all passphrases in the keystore.</xs:documentation>
						</xs:annotation>
						<xs:element name="PassphraseAttribute" type="tas:PassphraseAttribute" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Information about a passphrase in the keystore.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeletePassphrase">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PassphraseID" type="tas:PassphraseID">
							<xs:annotation>
								<xs:documentation>The ID of the passphrase that is to be deleted from the keystore.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeletePassphraseResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddServerCertificateAssignment">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPathID" type="tas:CertificationPathID"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddServerCertificateAssignmentResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveServerCertificateAssignment">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPathID" type="tas:CertificationPathID"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveServerCertificateAssignmentResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="ReplaceServerCertificateAssignment">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OldCertificationPathID" type="tas:CertificationPathID"/>
						<xs:element name="NewCertificationPathID" type="tas:CertificationPathID"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="ReplaceServerCertificateAssignmentResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAssignedServerCertificates">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAssignedServerCertificatesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificationPathID" type="tas:CertificationPathID" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>The IDs of all certification paths that are assigned to the TLS server on the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetEnabledTLSVersions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Versions" type="tas:TLSVersions">
							<xs:annotation>
								<xs:documentation>List of TLS versions to allow.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetEnabledTLSVersionsResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetEnabledTLSVersions">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetEnabledTLSVersionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Versions" type="tas:TLSVersions">
							<xs:annotation>
								<xs:documentation>List of allowed TLS versions.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================================-->
			<!-- Begin Cert-based client auth message types    -->
			<!--===============================================-->
			<xs:element name="UploadCRL">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Crl" type="tas:Base64DERencodedASN1Value">
							<xs:annotation>
								<xs:documentation>
                  The CRL to be uploaded to the device.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Alias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>
                  The alias to assign to the uploaded CRL.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element minOccurs="0" name="anyParameters">
							<xs:complexType>
								<xs:sequence>
									<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>
								</xs:sequence>
							</xs:complexType>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="UploadCRLResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CrlID" type="tas:CRLID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the uploaded CRL.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCRL">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CrlID" type="tas:CRLID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the CRL to be returned.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCRLResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Crl" type="tas:CRL">
							<xs:annotation>
								<xs:documentation>
                  The CRL with the requested ID.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAllCRLs">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAllCRLsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Crl" type="tas:CRL" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
                  A list of all CRLs that are stored in the keystore on the device.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteCRL">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CrlID" type="tas:CRLID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the CRL to be deleted.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteCRLResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateCertPathValidationPolicy">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Alias" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>
                  The alias to assign to the created certification path validation policy.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Parameters" type="tas:CertPathValidationParameters">
							<xs:annotation>
								<xs:documentation>
                  The parameters of the certification path validation policy to be created.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TrustAnchor" type="tas:TrustAnchor" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
                  The trust anchors of the certification path validation policy to be created.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element minOccurs="0" name="anyParameters">
							<xs:complexType>
								<xs:sequence>
									<xs:any minOccurs="0" maxOccurs="unbounded" namespace="##any" processContents="lax"/>
								</xs:sequence>
							</xs:complexType>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateCertPathValidationPolicyResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertPathValidationPolicyID" type="tas:CertPathValidationPolicyID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the created certification path validation policy.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCertPathValidationPolicy">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertPathValidationPolicyID" type="tas:CertPathValidationPolicyID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the certification path validation policy to be created.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCertPathValidationPolicyResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertPathValidationPolicy" type="tas:CertPathValidationPolicy">
							<xs:annotation>
								<xs:documentation>
                  The certification path validation policy that is stored under the requested ID.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAllCertPathValidationPolicies">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAllCertPathValidationPoliciesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertPathValidationPolicy" type="tas:CertPathValidationPolicy" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
                  A list of all certification path validation policies that are stored in the keystore on the device.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteCertPathValidationPolicy">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertPathValidationPolicyID" type="tas:CertPathValidationPolicyID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the certification path validation policy to be deleted.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteCertPathValidationPolicyResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetClientAuthenticationRequired">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="clientAuthenticationRequired" type="xs:boolean">
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetClientAuthenticationRequiredResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetClientAuthenticationRequired">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetClientAuthenticationRequiredResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="clientAuthenticationRequired" type="xs:boolean">
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddCertPathValidationPolicyAssignment">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertPathValidationPolicyID" type="tas:CertPathValidationPolicyID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the certification path validation policy to assign to the TLS server.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddCertPathValidationPolicyAssignmentResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveCertPathValidationPolicyAssignment">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertPathValidationPolicyID" type="tas:CertPathValidationPolicyID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the certification path validation policy to de-assign from the TLS server.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveCertPathValidationPolicyAssignmentResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="ReplaceCertPathValidationPolicyAssignment">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OldCertPathValidationPolicyID" type="tas:CertPathValidationPolicyID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the certification path validation policy to be de-assigned from the TLS server.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="NewCertPathValidationPolicyID" type="tas:CertPathValidationPolicyID">
							<xs:annotation>
								<xs:documentation>
                  The ID of the certification path validation policy to assign to the TLS server.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="ReplaceCertPathValidationPolicyAssignmentResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAssignedCertPathValidationPolicies">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAssignedCertPathValidationPoliciesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertPathValidationPolicyID" type="tas:CertPathValidationPolicyID" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
                  A list of IDs of the certification path validation policies that are assigned to the TLS server.
                </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--=============================================-->
			<!-- Begin Cert-based client auth message types   -->
			<!--=============================================-->
			<!--===============================-->
			<xs:element name="AddDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XConfiguration" type="tas:Dot1XConfiguration">
							<xs:annotation>
								<xs:documentation>The desired 802.1X configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XID" type="tas:Dot1XID">
							<xs:annotation>
								<xs:documentation>The unique identifier of the created 802.1X configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAllDot1XConfigurations">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAllDot1XConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tas:Dot1XConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>The list of unique identifiers of 802.1X configurations on the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XID" type="tas:Dot1XID">
							<xs:annotation>
								<xs:documentation>The unique identifier of the desired 802.1X configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XConfiguration" type="tas:Dot1XConfiguration">
							<xs:annotation>
								<xs:documentation>The 802.1X configuration, without password information.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XID" type="tas:Dot1XID">
							<xs:annotation>
								<xs:documentation>The unique identifier of the 802.1X configuration to be deleted.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetNetworkInterfaceDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="token" type="xs:string">
							<xs:annotation>
								<xs:documentation>The unique identifier of the Network Interface on which the 802.1X configuration is to be set. (NOTE: the network interface token is defined in devicemgmt.wsdl as tt:ReferenceToken, which is a derived type of xs:string.  To avoid importing all of common.xsd for this single type, the base type is used here.)</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Dot1XID" type="tas:Dot1XID">
							<xs:annotation>
								<xs:documentation>The unique identifier of the 802.1X configuration to be set.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetNetworkInterfaceDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RebootNeeded" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>Indicates whether or not a reboot is required after configuration updates.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetNetworkInterfaceDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="token" type="xs:string">
							<xs:annotation>
								<xs:documentation>The unique identifier of the Network Interface for which the 802.1X configuration is to be retrieved. (NOTE: the network interface token is defined in devicemgmt.wsdl as tt:ReferenceToken, which is a derived type of xs:string.  To avoid importing all of common.xsd for this single type, the base type is used here.)</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetNetworkInterfaceDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XID" type="tas:Dot1XID" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The unique identifier of 802.1X configuration assigned to the Network Interface.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteNetworkInterfaceDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="token" type="xs:string">
							<xs:annotation>
								<xs:documentation>The unique identifier of the Network Interface for which the 802.1X configuration is to be deleted. (NOTE: the network interface token is defined in devicemgmt.wsdl as tt:ReferenceToken, which is a derived type of xs:string.  To avoid importing all of common.xsd for this single type, the base type is used here.)</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteNetworkInterfaceDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RebootNeeded" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>Indicates whether or not a reboot is required after configuration updates.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
		</xs:schema>
	</wsdl:types>
	<wsdl:message name="GetServiceCapabilitiesRequest">
		<wsdl:part name="parameters" element="tas:GetServiceCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesResponse">
		<wsdl:part name="parameters" element="tas:GetServiceCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateRSAKeyPairRequest">
		<wsdl:part name="parameters" element="tas:CreateRSAKeyPair"/>
	</wsdl:message>
	<wsdl:message name="CreateRSAKeyPairResponse">
		<wsdl:part name="parameters" element="tas:CreateRSAKeyPairResponse"/>
	</wsdl:message>
	<wsdl:message name="UploadKeyPairInPKCS8Request">
		<wsdl:part name="parameters" element="tas:UploadKeyPairInPKCS8"/>
	</wsdl:message>
	<wsdl:message name="UploadKeyPairInPKCS8Response">
		<wsdl:part name="parameters" element="tas:UploadKeyPairInPKCS8Response"/>
	</wsdl:message>
	<wsdl:message name="UploadCertificateWithPrivateKeyInPKCS12Request">
		<wsdl:part name="parameters" element="tas:UploadCertificateWithPrivateKeyInPKCS12"/>
	</wsdl:message>
	<wsdl:message name="UploadCertificateWithPrivateKeyInPKCS12Response">
		<wsdl:part name="parameters" element="tas:UploadCertificateWithPrivateKeyInPKCS12Response"/>
	</wsdl:message>
	<wsdl:message name="GetKeyStatusRequest">
		<wsdl:part name="parameters" element="tas:GetKeyStatus"/>
	</wsdl:message>
	<wsdl:message name="GetKeyStatusResponse">
		<wsdl:part name="parameters" element="tas:GetKeyStatusResponse"/>
	</wsdl:message>
	<wsdl:message name="GetPrivateKeyStatusRequest">
		<wsdl:part name="parameters" element="tas:GetPrivateKeyStatus"/>
	</wsdl:message>
	<wsdl:message name="GetPrivateKeyStatusResponse">
		<wsdl:part name="parameters" element="tas:GetPrivateKeyStatusResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAllKeysRequest">
		<wsdl:part name="parameters" element="tas:GetAllKeys"/>
	</wsdl:message>
	<wsdl:message name="GetAllKeysResponse">
		<wsdl:part name="parameters" element="tas:GetAllKeysResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteKeyRequest">
		<wsdl:part name="parameters" element="tas:DeleteKey"/>
	</wsdl:message>
	<wsdl:message name="DeleteKeyResponse">
		<wsdl:part name="parameters" element="tas:DeleteKeyResponse"/>
	</wsdl:message>
	<wsdl:message name="CreatePKCS10CSRRequest">
		<wsdl:part name="parameters" element="tas:CreatePKCS10CSR"/>
	</wsdl:message>
	<wsdl:message name="CreatePKCS10CSRResponse">
		<wsdl:part name="parameters" element="tas:CreatePKCS10CSRResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateSelfSignedCertificateRequest">
		<wsdl:part name="parameters" element="tas:CreateSelfSignedCertificate"/>
	</wsdl:message>
	<wsdl:message name="CreateSelfSignedCertificateResponse">
		<wsdl:part name="parameters" element="tas:CreateSelfSignedCertificateResponse"/>
	</wsdl:message>
	<wsdl:message name="UploadCertificateRequest">
		<wsdl:part name="parameters" element="tas:UploadCertificate"/>
	</wsdl:message>
	<wsdl:message name="UploadCertificateResponse">
		<wsdl:part name="parameters" element="tas:UploadCertificateResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCertificateRequest">
		<wsdl:part name="parameters" element="tas:GetCertificate"/>
	</wsdl:message>
	<wsdl:message name="GetCertificateResponse">
		<wsdl:part name="parameters" element="tas:GetCertificateResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAllCertificatesRequest">
		<wsdl:part name="parameters" element="tas:GetAllCertificates"/>
	</wsdl:message>
	<wsdl:message name="GetAllCertificatesResponse">
		<wsdl:part name="parameters" element="tas:GetAllCertificatesResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteCertificateRequest">
		<wsdl:part name="parameters" element="tas:DeleteCertificate"/>
	</wsdl:message>
	<wsdl:message name="DeleteCertificateResponse">
		<wsdl:part name="parameters" element="tas:DeleteCertificateResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateCertificationPathRequest">
		<wsdl:part name="parameters" element="tas:CreateCertificationPath"/>
	</wsdl:message>
	<wsdl:message name="CreateCertificationPathResponse">
		<wsdl:part name="parameters" element="tas:CreateCertificationPathResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCertificationPathRequest">
		<wsdl:part name="parameters" element="tas:GetCertificationPath"/>
	</wsdl:message>
	<wsdl:message name="GetCertificationPathResponse">
		<wsdl:part name="parameters" element="tas:GetCertificationPathResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAllCertificationPathsRequest">
		<wsdl:part name="parameters" element="tas:GetAllCertificationPaths"/>
	</wsdl:message>
	<wsdl:message name="GetAllCertificationPathsResponse">
		<wsdl:part name="parameters" element="tas:GetAllCertificationPathsResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteCertificationPathRequest">
		<wsdl:part name="parameters" element="tas:DeleteCertificationPath"/>
	</wsdl:message>
	<wsdl:message name="DeleteCertificationPathResponse">
		<wsdl:part name="parameters" element="tas:DeleteCertificationPathResponse"/>
	</wsdl:message>
	<wsdl:message name="UploadPassphraseRequest">
		<wsdl:part name="parameters" element="tas:UploadPassphrase"/>
	</wsdl:message>
	<wsdl:message name="UploadPassphraseResponse">
		<wsdl:part name="parameters" element="tas:UploadPassphraseResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAllPassphrasesRequest">
		<wsdl:part name="parameters" element="tas:GetAllPassphrases"/>
	</wsdl:message>
	<wsdl:message name="GetAllPassphrasesResponse">
		<wsdl:part name="parameters" element="tas:GetAllPassphrasesResponse"/>
	</wsdl:message>
	<wsdl:message name="DeletePassphraseRequest">
		<wsdl:part name="parameters" element="tas:DeletePassphrase"/>
	</wsdl:message>
	<wsdl:message name="DeletePassphraseResponse">
		<wsdl:part name="parameters" element="tas:DeletePassphraseResponse"/>
	</wsdl:message>
	<wsdl:message name="AddServerCertificateAssignmentRequest">
		<wsdl:part name="parameters" element="tas:AddServerCertificateAssignment"/>
	</wsdl:message>
	<wsdl:message name="AddServerCertificateAssignmentResponse">
		<wsdl:part name="parameters" element="tas:AddServerCertificateAssignmentResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveServerCertificateAssignmentRequest">
		<wsdl:part name="parameters" element="tas:RemoveServerCertificateAssignment"/>
	</wsdl:message>
	<wsdl:message name="RemoveServerCertificateAssignmentResponse">
		<wsdl:part name="parameters" element="tas:RemoveServerCertificateAssignmentResponse"/>
	</wsdl:message>
	<wsdl:message name="ReplaceServerCertificateAssignmentRequest">
		<wsdl:part name="parameters" element="tas:ReplaceServerCertificateAssignment"/>
	</wsdl:message>
	<wsdl:message name="ReplaceServerCertificateAssignmentResponse">
		<wsdl:part name="parameters" element="tas:ReplaceServerCertificateAssignmentResponse"/>
	</wsdl:message>
	<wsdl:message name="SetEnabledTLSVersionsRequest">
		<wsdl:part name="parameters" element="tas:SetEnabledTLSVersions"/>
	</wsdl:message>
	<wsdl:message name="SetEnabledTLSVersionsResponse">
		<wsdl:part name="parameters" element="tas:SetEnabledTLSVersionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetEnabledTLSVersionsRequest">
		<wsdl:part name="parameters" element="tas:GetEnabledTLSVersions"/>
	</wsdl:message>
	<wsdl:message name="GetEnabledTLSVersionsResponse">
		<wsdl:part name="parameters" element="tas:GetEnabledTLSVersionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAssignedServerCertificatesRequest">
		<wsdl:part name="parameters" element="tas:GetAssignedServerCertificates"/>
	</wsdl:message>
	<wsdl:message name="GetAssignedServerCertificatesResponse">
		<wsdl:part name="parameters" element="tas:GetAssignedServerCertificatesResponse"/>
	</wsdl:message>
	<wsdl:message name="UploadCRLRequest">
		<wsdl:part name="parameters" element="tas:UploadCRL"/>
	</wsdl:message>
	<wsdl:message name="UploadCRLResponse">
		<wsdl:part name="parameters" element="tas:UploadCRLResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCRLRequest">
		<wsdl:part name="parameters" element="tas:GetCRL"/>
	</wsdl:message>
	<wsdl:message name="GetCRLResponse">
		<wsdl:part name="parameters" element="tas:GetCRLResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAllCRLsRequest">
		<wsdl:part name="parameters" element="tas:GetAllCRLs"/>
	</wsdl:message>
	<wsdl:message name="GetAllCRLsResponse">
		<wsdl:part name="parameters" element="tas:GetAllCRLsResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteCRLRequest">
		<wsdl:part name="parameters" element="tas:DeleteCRL"/>
	</wsdl:message>
	<wsdl:message name="DeleteCRLResponse">
		<wsdl:part name="parameters" element="tas:DeleteCRLResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateCertPathValidationPolicyRequest">
		<wsdl:part name="parameters" element="tas:CreateCertPathValidationPolicy"/>
	</wsdl:message>
	<wsdl:message name="CreateCertPathValidationPolicyResponse">
		<wsdl:part name="parameters" element="tas:CreateCertPathValidationPolicyResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCertPathValidationPolicyRequest">
		<wsdl:part name="parameters" element="tas:GetCertPathValidationPolicy"/>
	</wsdl:message>
	<wsdl:message name="GetCertPathValidationPolicyResponse">
		<wsdl:part name="parameters" element="tas:GetCertPathValidationPolicyResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAllCertPathValidationPoliciesRequest">
		<wsdl:part name="parameters" element="tas:GetAllCertPathValidationPolicies"/>
	</wsdl:message>
	<wsdl:message name="GetAllCertPathValidationPoliciesResponse">
		<wsdl:part name="parameters" element="tas:GetAllCertPathValidationPoliciesResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteCertPathValidationPolicyRequest">
		<wsdl:part name="parameters" element="tas:DeleteCertPathValidationPolicy"/>
	</wsdl:message>
	<wsdl:message name="DeleteCertPathValidationPolicyResponse">
		<wsdl:part name="parameters" element="tas:DeleteCertPathValidationPolicyResponse"/>
	</wsdl:message>
	<wsdl:message name="SetClientAuthenticationRequiredRequest">
		<wsdl:part name="parameters" element="tas:SetClientAuthenticationRequired"/>
	</wsdl:message>
	<wsdl:message name="SetClientAuthenticationRequiredResponse">
		<wsdl:part name="parameters" element="tas:SetClientAuthenticationRequiredResponse"/>
	</wsdl:message>
	<wsdl:message name="GetClientAuthenticationRequiredRequest">
		<wsdl:part name="parameters" element="tas:GetClientAuthenticationRequired"/>
	</wsdl:message>
	<wsdl:message name="GetClientAuthenticationRequiredResponse">
		<wsdl:part name="parameters" element="tas:GetClientAuthenticationRequiredResponse"/>
	</wsdl:message>
	<wsdl:message name="AddCertPathValidationPolicyAssignmentRequest">
		<wsdl:part name="parameters" element="tas:AddCertPathValidationPolicyAssignment"/>
	</wsdl:message>
	<wsdl:message name="AddCertPathValidationPolicyAssignmentResponse">
		<wsdl:part name="parameters" element="tas:AddCertPathValidationPolicyAssignmentResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveCertPathValidationPolicyAssignmentRequest">
		<wsdl:part name="parameters" element="tas:RemoveCertPathValidationPolicyAssignment"/>
	</wsdl:message>
	<wsdl:message name="RemoveCertPathValidationPolicyAssignmentResponse">
		<wsdl:part name="parameters" element="tas:RemoveCertPathValidationPolicyAssignmentResponse"/>
	</wsdl:message>
	<wsdl:message name="ReplaceCertPathValidationPolicyAssignmentRequest">
		<wsdl:part name="parameters" element="tas:ReplaceCertPathValidationPolicyAssignment"/>
	</wsdl:message>
	<wsdl:message name="ReplaceCertPathValidationPolicyAssignmentResponse">
		<wsdl:part name="parameters" element="tas:ReplaceCertPathValidationPolicyAssignmentResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAssignedCertPathValidationPoliciesRequest">
		<wsdl:part name="parameters" element="tas:GetAssignedCertPathValidationPolicies"/>
	</wsdl:message>
	<wsdl:message name="GetAssignedCertPathValidationPoliciesResponse">
		<wsdl:part name="parameters" element="tas:GetAssignedCertPathValidationPoliciesResponse"/>
	</wsdl:message>
	<wsdl:message name="AddDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tas:AddDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tas:AddDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAllDot1XConfigurationsRequest">
		<wsdl:part name="parameters" element="tas:GetAllDot1XConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAllDot1XConfigurationsResponse">
		<wsdl:part name="parameters" element="tas:GetAllDot1XConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tas:GetDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tas:GetDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tas:DeleteDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="DeleteDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tas:DeleteDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetNetworkInterfaceDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tas:SetNetworkInterfaceDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetNetworkInterfaceDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tas:SetNetworkInterfaceDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetNetworkInterfaceDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tas:GetNetworkInterfaceDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetNetworkInterfaceDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tas:GetNetworkInterfaceDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteNetworkInterfaceDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tas:DeleteNetworkInterfaceDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="DeleteNetworkInterfaceDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tas:DeleteNetworkInterfaceDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:portType name="AdvancedSecurityService">
		<wsdl:documentation>Common functionality for all security configuraiton service parts.</wsdl:documentation>
		<wsdl:operation name="GetServiceCapabilities">
			<wsdl:documentation>Returns the capabilities of the security configuraiton service. The result is returned in a typed answer.</wsdl:documentation>
			<wsdl:input message="tas:GetServiceCapabilitiesRequest"/>
			<wsdl:output message="tas:GetServiceCapabilitiesResponse"/>
		</wsdl:operation>
	</wsdl:portType>
	<wsdl:portType name="Keystore">
		<wsdl:documentation>Basic keystore functionality.</wsdl:documentation>
		<wsdl:operation name="CreateRSAKeyPair">
			<wsdl:documentation>
				This operation triggers the asynchronous generation of an RSA key pair of a particular key length (specified as the number of bits) as specified in [RFC 3447], with a suitable key generation mechanism on the device.
				Keys, especially RSA key pairs, are uniquely identified using key IDs.<br/>
				If the device does not have not enough storage capacity for storing the key pair to be created, the maximum number of keys reached fault shall be produced and no key pair shall be generated.
				Otherwise, the operation generates a keyID for the new key and associates the generating status to it.<br/>
				Immediately after key generation has started, the device shall return the keyID to the client and continue to generate the key pair.
				The client may query the device with the GetKeyStatus operation whether the generation has finished.
				The client may also subscribe to Key Status events to be notified about key status changes.<br/>
				The device also returns a best-effort estimate of how much time it requires to create the key pair.
				A client may use this information as an indication how long to wait before querying the device whether key generation is completed.<br/>
				After the key has been successfully created, the device shall assign it the ok status. If the key generation fails, the device shall assign the key the corrupt status.
			</wsdl:documentation>
			<wsdl:input message="tas:CreateRSAKeyPairRequest"/>
			<wsdl:output message="tas:CreateRSAKeyPairResponse"/>
		</wsdl:operation>
		<wsdl:operation name="UploadKeyPairInPKCS8">
			<wsdl:documentation>
        This operation uploads a key pair in a PKCS#8 data structure as specified in [RFC 5958, RFC 5959].<br/>
        If an encryption passphrase ID is supplied in the request, the device shall assume that the KeyPair parameter contains an EncryptedPrivateKeyInfo ASN.1
        structure that is encrypted under the passphrase in the keystore that corresponds to the supplied ID, where the EncryptedPrivateKeyInfo structure contains
        both the private key and the corresponding public key. If no encryption passphrase ID is supplied, the device shall assume that the KeyPair parameter contains a
        OneAsymmetricKey ASN.1 structure which contains both the private key and the corresponding public key.
      </wsdl:documentation>
			<wsdl:input message="tas:UploadKeyPairInPKCS8Request"/>
			<wsdl:output message="tas:UploadKeyPairInPKCS8Response"/>
		</wsdl:operation>
		<wsdl:operation name="UploadCertificateWithPrivateKeyInPKCS12">
			<wsdl:documentation>
        This operation uploads a certification path consisting of X.509 certificates as specified by [RFC 5280] in DER encoding along with a private key to a device’s keystore.
        Certificates and private key are supplied in the form of a PKCS#12 file as specified in [PKCS#12].<br/>
        The device shall support PKCS#12 files that contain the following safe bags:<br/>
        *	one or more instances of CertBag [PKCS#12, Sect. 4.2.3]
        *	either exactly one instance of KeyBag [PKCS#12, Sect. 4.3.1] or exactly one instance of PKCS8ShroudedKeyBag [PKCS#12, Sect. 4.2.2].<br/>
        If the IgnoreAdditionalCertificates parameter has the value true, the device shall behave as if the client had supplied only the first CertBag in the sequence of CertBag instances.
        The device shall support PKCS#12 passphrase integrity mode for integrity protection of the PKCS#12 PFX as specified in [PKCS#12, Sect. 4].
        The device shall support PKCS8ShroudedKeyBags that are encrypted with the same passphrase as the CertBag instances.
        If an integrity passphrase ID is supplied, the device shall use the corresponding passphrase in the keystore to check the integrity of the supplied PKCS#12 PFX.
        If an integrity passphrase ID is supplied, but the supplied PKCS#12 PFX has no integrity protection, the device shall produce a BadPKCS12File fault and shall
        not store the uploaded certificates nor the uploaded key pair in the keystore.
        If an encryption passphrase ID is supplied, the device shall use the corresponding passphrase in the keystore to decrypt the PKCS8ShroudedKeyBag and the CertBag instances.
        If an EncryptionPassphraseID is supplied, but a CertBag is not encrypted, the device shall ignore the supplied EncryptionPassphraseID when processing this CertBag.
        If an EncryptionPassphraseID is supplied, but a KeyBag is provided instead of a PKCS8ShroudedKeyBag, the device shall ignore the supplied EncryptionPassphraseID when processing the KeyBag.
      </wsdl:documentation>
			<wsdl:input message="tas:UploadCertificateWithPrivateKeyInPKCS12Request"/>
			<wsdl:output message="tas:UploadCertificateWithPrivateKeyInPKCS12Response"/>
		</wsdl:operation>
		<wsdl:operation name="GetKeyStatus">
			<wsdl:documentation>
				This operation returns the status of a key.<br/>
				Keys are uniquely identified using key IDs. If no key is stored under the requested key ID in the keystore, an InvalidKeyID fault is produced.
				Otherwise, the status of the key is returned.
			</wsdl:documentation>
			<wsdl:input message="tas:GetKeyStatusRequest"/>
			<wsdl:output message="tas:GetKeyStatusResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetPrivateKeyStatus">
			<wsdl:documentation>
				This operation returns whether a key pair contains a private key.<br/>
				Keys are uniquely identified using key IDs. If no key is stored under the requested key ID in the keystore or the key identified by the requested key ID does not identify a key pair,
				the device shall produce an InvalidKeyID fault.
				Otherwise, this operation returns true if the key pair identified by the key ID contains a private key, and false otherwise.
			</wsdl:documentation>
			<wsdl:input message="tas:GetPrivateKeyStatusRequest"/>
			<wsdl:output message="tas:GetPrivateKeyStatusResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAllKeys">
			<wsdl:documentation>
				This operation returns information about all keys that are stored in the device’s keystore.<br/>
				This operation may be used, e.g., if a client lost track of which keys are present on the device.
				If no key is stored on the device, an empty list is returned.
			</wsdl:documentation>
			<wsdl:input message="tas:GetAllKeysRequest"/>
			<wsdl:output message="tas:GetAllKeysResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteKey">
			<wsdl:documentation>
				This operation deletes a key from the device’s keystore.<br/>
				Keys are uniquely identified using key IDs. If no key is stored under the requested key ID in the keystore, a device shall produce an InvalidArgVal fault.
				If a reference exists for the specified key, a device shall produce the corresponding fault and shall not delete the key.
				If there is a key under the requested key ID stored in the keystore and the key could not be deleted, a device shall produce a KeyDeletion fault.
				If the key has the status generating, a device shall abort the generation of the key and delete from the keystore all data generated for this key.
				After a key is successfully deleted, the device may assign its former ID to other keys.
			</wsdl:documentation>
			<wsdl:input message="tas:DeleteKeyRequest"/>
			<wsdl:output message="tas:DeleteKeyResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreatePKCS10CSR">
			<wsdl:documentation>
				This operation generates a DER-encoded PKCS#10 v1.7 certification request (sometimes also called certificate signing request or CSR) as specified in RFC 2986 
				for a public key on the device.<br/>
				The key pair that contains the public key for which a certification request shall be produced is specified by its key ID.
				If no key is stored under the requested KeyID or the key specified by the requested KeyID is not an asymmetric key pair, an invalid key ID fault shall be produced and 
				no CSR shall be generated.<br/>
				
				A device that supports this command shall as minimum support the sha-1WithRSAEncryption signature algorithm as specified in RFC 3279. 
				If the specified signature algorithm is not supported by the device, an UnsupportedSignatureAlgorithm fault shall be produced and no CSR shall be generated.<br/>
				
				If the public key identified by the requested Key ID is an invalid input to the specified signature algorithm, a KeySignatureAlgorithmMismatch fault shall be produced 
				and no CSR shall be generated.
				If the key pair does not have status ok, a device shall produce an InvalidKeyStatus fault and no CSR shall be generated.
			</wsdl:documentation>
			<wsdl:input message="tas:CreatePKCS10CSRRequest"/>
			<wsdl:output message="tas:CreatePKCS10CSRResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateSelfSignedCertificate">
			<wsdl:documentation>
				This operation generates for a public key on the device a self-signed X.509 certificate that complies to RFC 5280.<br/>
				The X509Version parameter specifies the version of X.509 that the generated certificate shall comply to.
				A device that supports this command shall support the generation of X.509v3 certificates as specified in RFC 5280 and may additionally be able to handle other X.509 certificate formats
				as indicated by the X.509Versions capability.<br/>
				The key pair that contains the public key for which a self-signed certificate shall be produced is specified by its key pair ID.
				The subject parameter describes the entity that the public key belongs to.
				If the key pair does not have status ok, a device shall produce an InvalidKeyStatus fault and no certificate shall be generated.
				
				The signature algorithm parameter determines which signature algorithm shall be used for signing the certification request with the public key specified by the key ID parameter.
				A device that supports this command shall as minimum support the sha-1WithRSAEncryption signature algorithm as specified in RFC 3279. 
				The Extensions parameter specifies potential X509v3 extensions that shall be contained in the certificate.
				A device that supports this command shall support the extensions that are defined in [RFC 5280], Sect. 4.2] as mandatory for CAs that issue self-signed certificates.<br/>
				
				Certificates are uniquely identified using certificate IDs. If the command was successful, the device generates a new ID for the generated certificate and returns this ID.<br/>
				If the device does not have not enough storage capacity for storing the certificate to be created, the maximum number of certificates reached fault shall be produced and no certificate shall be generated.
			</wsdl:documentation>
			<wsdl:input message="tas:CreateSelfSignedCertificateRequest"/>
			<wsdl:output message="tas:CreateSelfSignedCertificateResponse"/>
		</wsdl:operation>
		<wsdl:operation name="UploadCertificate">
			<wsdl:documentation>
				This operation uploads an X.509 certificate as specified by [RFC 5280] in DER encoding and the public key in the certificate to a device’s keystore.<br/>
				A device that supports this command shall be able to handle X.509v3 certificates as specified in RFC 5280 and may additionally be able to handle other X.509 certificate formats as indicated by the X.509Versions capability.
				A device that supports this command shall support sha1-WithRSAEncryption as certificate signature algorithm.<br/>
				
				Certificates are uniquely identified using certificate IDs, and key pairs are uniquely identified using key IDs.
				The device shall generate a new certificate ID for the uploaded certificate.<br/>
				Certain certificate usages, e.g. TLS server authentication, require the private key that corresponds to the public key in the certificate to be present in the keystore.
				In such cases, the client may indicate that it expects the device to produce a fault if the matching private key for
				the uploaded certificate is not present in the keystore by setting the PrivateKeyRequired argument in the upload request to true.<br/>
				
				The uploaded certificate has to be linked to a key pair in the keystore.
				If no private key is required for the public key in the certificate and a key pair exists in the keystore with a public key equal to the public key in the certificate,
				the uploaded certificate is linked to the key pair identified by the supplied key ID by adding a reference from the certificate to the key pair.
				If no private key is required for the public key in the certificate and no key pair exists with the public key equal to the public key in the certificate,
				a new key pair with status ok is created with the public key from the certificate, and this key pair is linked to the uploaded certificate by adding a reference from 
				the certificate to the key pair.
				If a private key is required for the public key in the certificate, and a key pair exists in the keystore with a private key that matches the public key in the certificate,
				the uploaded certificate is linked to this keypair by adding a reference from the certificate to the key pair.
				If a private key is required for the public key and no such keypair exists in the keystore, the NoMatchingPrivateKey fault shall be produced and the certificate
				shall not be stored in the keystore.
				If the key pair that the certificate shall be linked to does not have status ok, an InvalidKeyID fault is produced, and the uploaded certificate is not stored in the keystore.
				If the device cannot process the uploaded certificate, a BadCertificate fault is produced and neither the uploaded certificate nor the public key are stored in the device’s keystore.
				The BadCertificate fault shall not be produced based on the mere fact that the device’s current time lies outside the interval defined by the notBefore and notAfter fields as specified by [RFC 5280], Sect. 4.1 .
				This operation shall not mark the uploaded certificate as trusted.<br/>
				
				If the device does not have not enough storage capacity for storing the certificate to be uploaded, the maximum number of certificates reached fault shall be produced
				and no certificate shall be uploaded.
				If the device does not have not enough storage capacity for storing the key pair that eventually has to be created, the device shall generate a maximum number of keys reached fault.
				Furthermore the device shall not generate a key pair and no certificate shall be stored.
			</wsdl:documentation>
			<wsdl:input message="tas:UploadCertificateRequest"/>
			<wsdl:output message="tas:UploadCertificateResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCertificate">
			<wsdl:documentation>
				This operation returns a specific certificate from the device’s keystore.<br/>
				Certificates are uniquely identified using certificate IDs. If no certificate is stored under the requested certificate ID in the keystore, an InvalidArgVal fault is produced.
				It shall be noted that this command does not return the private key that is associated to the public key in the certificate.
			</wsdl:documentation>
			<wsdl:input message="tas:GetCertificateRequest"/>
			<wsdl:output message="tas:GetCertificateResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAllCertificates">
			<wsdl:documentation>
				This operation returns the IDs of all certificates that are stored in the device’s keystore.<br/>
				This operation may be used, e.g.,  if a client lost track of which certificates are present on the device.
				If no certificate is stored in the device’s keystore, an empty list is returned.
			</wsdl:documentation>
			<wsdl:input message="tas:GetAllCertificatesRequest"/>
			<wsdl:output message="tas:GetAllCertificatesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteCertificate">
			<wsdl:documentation>
				This operation deletes a certificate from the device’s keystore.<br/>
				The operation shall not delete the public key that is contained in the certificate from the keystore.
				Certificates are uniquely identified using certificate IDs. If no certificate is stored under the requested certificate ID in the keystore, an InvalidArgVal fault is produced.
				If there is a certificate under the requested certificate ID stored in the keystore and the certificate could not be deleted, a CertificateDeletion fault is produced.
				If a reference exists for the specified certificate, the certificate shall not be deleted and the corresponding fault shall be produced.
				After a certificate has been  successfully deleted, the device may assign its former ID to other certificates.
			</wsdl:documentation>
			<wsdl:input message="tas:DeleteCertificateRequest"/>
			<wsdl:output message="tas:DeleteCertificateResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateCertificationPath">
			<wsdl:documentation>
				This operation creates a sequence of certificates that may be used, e.g., for certification path validation or for TLS server authentication.<br/>
				Certification paths are uniquely identified using certification path IDs. Certificates are uniquely identified using certificate IDs.
				A certification path contains a sequence of certificate IDs.
				If there is a certificate ID in the sequence of supplied certificate IDs for which no certificate exists in the device’s keystore, the corresponding fault shall be produced
				and no certification path shall be created.<br/>
				
				The signature of each certificate in the certification path except for the last one must be verifiable with the public key contained in the next certificate in the path.
				If there is a certificate ID in the request other than the last ID for which the corresponding certificate cannot be verified with the public key in the certificate identified 
				by the next certificate ID, an InvalidCertificateChain fault shall be produced and no certification path shall be created.
			</wsdl:documentation>
			<wsdl:input message="tas:CreateCertificationPathRequest"/>
			<wsdl:output message="tas:CreateCertificationPathResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCertificationPath">
			<wsdl:documentation>
				This operation returns a specific certification path from the device’s keystore.<br/>
				Certification paths are uniquely identified using certification path IDs.
				If no certification path is stored under the requested ID in the keystore, an InvalidArgVal fault is produced.
			</wsdl:documentation>
			<wsdl:input message="tas:GetCertificationPathRequest"/>
			<wsdl:output message="tas:GetCertificationPathResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAllCertificationPaths">
			<wsdl:documentation>
				This operation returns the IDs of all certification paths that are stored in the device’s keystore.<br/>
				This operation may be used, e.g., if a client lost track of which certificates are present on the device.
				If no certification path is stored on the device, an empty list is returned.
			</wsdl:documentation>
			<wsdl:input message="tas:GetAllCertificationPathsRequest"/>
			<wsdl:output message="tas:GetAllCertificationPathsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteCertificationPath">
			<wsdl:documentation>
				This operation deletes a certification path from the device’s keystore.<br/>
				This operation shall not delete the certificates that are referenced by the certification path.
				Certification paths are uniquely identified using certification path IDs.
				If no certification path is stored under the requested certification path ID in the keystore, an InvalidArgVal fault is produced.
				If there is a certification path under the requested certification path ID stored in the keystore and the certification path could not be deleted,
				a CertificationPathDeletion fault is produced.
				If a reference exists for the specified certification path, the certification path shall not be deleted and the corresponding fault shall be produced.
				After a certification path is successfully deleted, the device may assign its former ID to other certification paths.
			</wsdl:documentation>
			<wsdl:input message="tas:DeleteCertificationPathRequest"/>
			<wsdl:output message="tas:DeleteCertificationPathResponse"/>
		</wsdl:operation>
		<wsdl:operation name="UploadPassphrase">
			<wsdl:documentation>
        This operation uploads a passphrase to the keystore of the device.
      </wsdl:documentation>
			<wsdl:input message="tas:UploadPassphraseRequest"/>
			<wsdl:output message="tas:UploadPassphraseResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAllPassphrases">
			<wsdl:documentation>
        This operation returns information about all passphrases that are stored in the keystore of the device.
        This operation may be used, e.g., if a client lost track of which passphrases are present on the device.
        If no passphrase is stored on the device, the device shall return an empty list.
      </wsdl:documentation>
			<wsdl:input message="tas:GetAllPassphrasesRequest"/>
			<wsdl:output message="tas:GetAllPassphrasesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeletePassphrase">
			<wsdl:documentation>
        This operation deletes a passphrase from the keystore of the device.
      </wsdl:documentation>
			<wsdl:input message="tas:DeletePassphraseRequest"/>
			<wsdl:output message="tas:DeletePassphraseResponse"/>
		</wsdl:operation>
		<!--==========================================-->
		<!-- Begin Cert-based client auth operations  -->
		<!--==========================================-->
		<wsdl:operation name="UploadCRL">
			<wsdl:documentation>
        This operation uploads a certificate revocation list (CRL) as specified in [RFC 5280] to the keystore on the device.
        If the device does not have enough storage space to store the CRL to be uploaded, the device shall produce a MaximumNumberOfCRLsReached fault and shall not store the supplied CRL.
        If the device is not able to process the supplied CRL, the device shall produce a BadCRL fault and shall not store the supplied CRL.
        If the device does not support the signature algorithm that was used to sign the supplied CRL, the device shall produce an UnsupportedSignatureAlgorithm fault and shall not store the supplied CRL.
      </wsdl:documentation>
			<wsdl:input message="tas:UploadCRLRequest"/>
			<wsdl:output message="tas:UploadCRLResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCRL">
			<wsdl:documentation>
        This operation returns a specific certificate revocation list (CRL) from the keystore on the device.
        Certification revocation lists are uniquely identified using CRLIDs. If no CRL is stored under the requested CRLID, the device shall produce a CRLID fault.
      </wsdl:documentation>
			<wsdl:input message="tas:GetCRLRequest"/>
			<wsdl:output message="tas:GetCRLResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAllCRLs">
			<wsdl:documentation>
        This operation returns all certificate revocation lists (CRLs) that are stored in the keystore on the device.
        If no certificate revocation list is stored in the device’s keystore, an empty list is returned.
      </wsdl:documentation>
			<wsdl:input message="tas:GetAllCRLsRequest"/>
			<wsdl:output message="tas:GetAllCRLsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteCRL">
			<wsdl:documentation>
        This operation deletes a certificate revocation list (CRL) from the keystore on the device.
        Certification revocation lists are uniquely identified using CRLIDs. If no CRL is stored under the requested CRLID, the device shall produce a CRLID fault.
        If a reference exists for the specified CRL, the device shall produce a ReferenceExists fault and shall not delete the CRL.
        After a CRL has been successfully deleted, a device may assign its former ID to other CRLs.
      </wsdl:documentation>
			<wsdl:input message="tas:DeleteCRLRequest"/>
			<wsdl:output message="tas:DeleteCRLResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateCertPathValidationPolicy">
			<wsdl:documentation>
        This operation creates a certification path validation policy.
        Certification path validation policies are uniquely identified using certification path validation policy IDs. The device shall generate a new certification path validation policy ID for the created certification path validation policy.
        For the certification path validation parameters that are not represented in the certPathValidationParameters data type, the device shall use the default values specified in Sect. 3.
        If the device does not have enough storage capacity for storing the certification path validation policy to be created, the device shall produce a maximum number of certification path validation policies reached fault and shall not create a certification path validation policy.
        If there is at least one trust anchor certificate ID in the request for which there exists no certificate in the device’s keystore, the device shall produce a CertificateID fault and shall not create a certification path validation policy.
        If the device cannot process the supplied certification path validation parameters, the device shall produce a CertPathValidationParameters fault and shall not create a certification path validation policy.
      </wsdl:documentation>
			<wsdl:input message="tas:CreateCertPathValidationPolicyRequest"/>
			<wsdl:output message="tas:CreateCertPathValidationPolicyResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCertPathValidationPolicy">
			<wsdl:documentation>
        This operation returns a certification path validation policy from the keystore on the device.
        Certification path validation policies are uniquely identified using certification path validation policy IDs. If no certification path validation policy is stored under the requested certification path validation policy ID, the device shall produce a CertPathValidationPolicyID fault.
      </wsdl:documentation>
			<wsdl:input message="tas:GetCertPathValidationPolicyRequest"/>
			<wsdl:output message="tas:GetCertPathValidationPolicyResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAllCertPathValidationPolicies">
			<wsdl:documentation>
        This operation returns all certification path validation policies that are stored in the keystore on the device.
        If no certification path validation policy is stored in the device’s keystore, an empty list is returned.
      </wsdl:documentation>
			<wsdl:input message="tas:GetAllCertPathValidationPoliciesRequest"/>
			<wsdl:output message="tas:GetAllCertPathValidationPoliciesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteCertPathValidationPolicy">
			<wsdl:documentation>
        This operation deletes a certification path validation policy from the keystore on the device.
        Certification path validation policies are uniquely identified using certification path validation policy IDs. If no certification path validation policy is stored under the requested certification path validation policy ID, the device shall produce an InvalidCertPathValidationPolicyID fault.
        If a reference exists for the requested certification path validation policy, the device shall produce a ReferenceExists fault and shall not delete the certification path validation policy.
        After the certification path validation policy has been deleted, the device may assign its former ID to other certification path validation policies.
      </wsdl:documentation>
			<wsdl:input message="tas:DeleteCertPathValidationPolicyRequest"/>
			<wsdl:output message="tas:DeleteCertPathValidationPolicyResponse"/>
		</wsdl:operation>
		<!--==========================================-->
		<!-- End Cert-based client auth operations    -->
		<!--==========================================-->
	</wsdl:portType>
	<wsdl:portType name="TLSServer">
		<wsdl:documentation>TLS server functionality.</wsdl:documentation>
		<wsdl:operation name="AddServerCertificateAssignment">
			<wsdl:documentation>
				This operation assigns a key pair and certificate along with a certification path (certificate chain) to the TLS server on the device.
				The TLS server shall use this information for key exchange during the TLS handshake, particularly for constructing server certificate messages as specified in RFC 4346 and RFC 2246.<br/>
				
				Certification paths are identified by their certification path IDs in the keystore. The first certificate in the certification path must be the TLS server certificate.
				Since each certificate has exactly one associated key pair, a reference to the key pair that is associated with the server certificate is not supplied explicitly.
				Devices shall obtain the private key or results of operations under the private key by suitable internal interaction with the keystore.<br/>
				If a device chooses to perform a TLS key exchange based on the supplied certification path,  it shall use the key pair that is associated with the server certificate for 
				key exchange and transmit the certification path to TLS clients as-is, i.e., the device shall not check conformance of the certification path to RFC 4346 norRFC 2246.
				In order to use the server certificate during the TLS handshake, the corresponding private key is required.
				Therefore, if the key pair that is associated with the server certificate, i.e., the first certificate in the certification path, does not have an associated private key, 
				the NoPrivateKey fault is produced and the certification path is not associated to the TLS server.<br/>
				A TLS server may present different certification paths to different clients during the TLS handshake instead of presenting the same certification path to all clients.
				Therefore more than one certification path may be assigned to the TLS server.<br/>
				If the maximum number of certification paths that may be assigned to the TLS server simultaneously is reached, the device shall generate a MaximumNumberOfCertificationPathsReached 
				fault and the requested certification path shall not be assigned to the TLS server.
			</wsdl:documentation>
			<wsdl:input message="tas:AddServerCertificateAssignmentRequest"/>
			<wsdl:output message="tas:AddServerCertificateAssignmentResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveServerCertificateAssignment">
			<wsdl:documentation>
				This operation removes a key pair and certificate assignment (including certification path) to the TLS server on the device.<br/>
				Certification paths are identified using certification path IDs. If the supplied certification path ID is not associated to the TLS server, an InvalidArgVal fault is produced.
			</wsdl:documentation>
			<wsdl:input message="tas:RemoveServerCertificateAssignmentRequest"/>
			<wsdl:output message="tas:RemoveServerCertificateAssignmentResponse"/>
		</wsdl:operation>
		<wsdl:operation name="ReplaceServerCertificateAssignment">
			<wsdl:documentation>
				This operation replaces an existing key pair and certificate assignment to the TLS server on the device by a new key pair and certificate assignment (including certification paths).<br/>
				
				After the replacement, the TLS server shall use the new certificate and certification path exactly in those cases in which it would have used the old certificate and certification path.
				Therefore, especially in the case that several server certificates are assigned to the TLS server, clients that wish to replace an old certificate assignment by a new assignment
				should use this operation instead of a combination of the Add TLS Server Certificate Assignment and the Remove TLS Server Certificate Assignment operations.<br/>
				
				Certification paths are identified using certification path IDs. If the supplied old certification path ID is not associated to the TLS server, or no certification path exists
				under the new certification path ID, the corresponding InvalidArgVal faults are produced and the associations are unchanged.
				The first certificate in the new certification path must be the TLS server certificate.<br/>
				Since each certificate has exactly one associated key pair, a reference to the key pair that is associated with the new server certificate is not supplied explicitly.
				Devices shall obtain the private key or results of operations under the private key by suitable internal interaction with the keystore.<br/>
				If a device chooses to perform a TLS key exchange based on the new certification path,  it shall use the key pair that is associated with the server certificate
				for key exchange and transmit the certification path to TLS clients as-is, i.e., the device shall not check conformance of the certification path to RFC 4346 norRFC 2246.
				In order to use the server certificate during the TLS handshake, the corresponding private key is required.
				Therefore, if the key pair that is associated with the server certificate, i.e., the first certificate in the certification path, does not have an associated private key,
				the NoPrivateKey fault is produced and the certification path is not associated to the TLS server.
			</wsdl:documentation>
			<wsdl:input message="tas:ReplaceServerCertificateAssignmentRequest"/>
			<wsdl:output message="tas:ReplaceServerCertificateAssignmentResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetEnabledTLSVersions">
			<wsdl:documentation>
				This operation sets the version(s) of TLS which the device shall use.  Valid values are taken from the TLSServerSupported capability.<br/>
				A client initiates a TLS session by sending a ClientHello with the hightest TLS version it supports.  This suggests to the server that the client can accept any TLS version up to and including that version.<br/>
				The server then chooses the TLS version to use.  This is generally the highest TLS version the server supports that is within the range of the client.  For example, if a ClientHello indicates TLS version 1.1, the server can proceed with TLS 1.0 or TLS 1.1.<br/>
				In the event that an ONVIF installation wishes to disable certain version(s) of TLS, it may do so with this operation.  For example, to disable TLS 1.0 on a device signaling support for TLS versions 1.0, 1.1, and 1.2, the enabled version list may be set to "1.1 1.2", omitting 1.0.  If a client then attempts to connect with a ClientHello containing TLS 1.0, the server shall send a "protocol_version" alert message and close the connection.  This handshake indicates to the client that TLS 1.0 is not supported by the server.  The client must try again with a higher TLS version suggestion.<br/>
				An empty list is not permitted.  Disabling all versions of TLS is not the intent of this operation.  See AddServerCertificateAssignment and RemoveServerCertificateAssignment.
			</wsdl:documentation>
			<wsdl:input message="tas:SetEnabledTLSVersionsRequest"/>
			<wsdl:output message="tas:SetEnabledTLSVersionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetEnabledTLSVersions">
			<wsdl:documentation>
				This operation retrieves the version(s) of TLS which are currently enabled on the device.
			</wsdl:documentation>
			<wsdl:input message="tas:GetEnabledTLSVersionsRequest"/>
			<wsdl:output message="tas:GetEnabledTLSVersionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAssignedServerCertificates">
			<wsdl:documentation>
				This operation returns the IDs of all key pairs and certificates (including certification paths) that are assigned to the TLS server on the device.<br/>
				This operation may be used, e.g., if a client lost track of the certification path assignments on the device.
				If no certification path is assigned to the TLS server, an empty list is returned.
			</wsdl:documentation>
			<wsdl:input message="tas:GetAssignedServerCertificatesRequest"/>
			<wsdl:output message="tas:GetAssignedServerCertificatesResponse"/>
		</wsdl:operation>
		<!--==========================================-->
		<!-- Begin Cert-based client auth operations  -->
		<!--==========================================-->
		<wsdl:operation name="SetClientAuthenticationRequired">
			<wsdl:documentation>
        This operation activates or deactivates TLS client authentication for the TLS server on the device.
        The TLS server on the device shall require client authentication if and only if clientAuthenticationRequired is set to true.
        If TLS client authentication is requested to be enabled and no certification path validation policy is assigned to the TLS server, the device shall return an EnablingTLSClientAuthenticationFailed fault and shall not enable TLS client authentication.
        The device shall execute this command regardless of the TLS enabled/disabled state configured in the ONVIF Device Management Service.
      </wsdl:documentation>
			<wsdl:input message="tas:SetClientAuthenticationRequiredRequest"/>
			<wsdl:output message="tas:SetClientAuthenticationRequiredResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetClientAuthenticationRequired">
			<wsdl:documentation>
        This operation returns whether TLS client authentication is active.
      </wsdl:documentation>
			<wsdl:input message="tas:GetClientAuthenticationRequiredRequest"/>
			<wsdl:output message="tas:GetClientAuthenticationRequiredResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddCertPathValidationPolicyAssignment">
			<wsdl:documentation>
        This operation assigns a certification path validation policy to the TLS server on the device. The TLS server shall enforce the policy when authenticating TLS clients and consider a client authentic if and only if the algorithm returns valid.
        If no certification path validation policy is stored under the requested CertPathValidationPolicyID, the device shall produce a CertPathValidationPolicyID fault.
        A TLS server may use different certification path validation policies to authenticate clients. Therefore more than one certification path validation policy may be assigned to the TLS server. If the maximum number of certification path validation policies that may be assigned to the  TLS server simultaneously is reached, the device shall produce a MaximumNumberOfTLSCertPathValidationPoliciesReached fault and shall not assign the requested certification path validation policy to the TLS server.
      </wsdl:documentation>
			<wsdl:input message="tas:AddCertPathValidationPolicyAssignmentRequest"/>
			<wsdl:output message="tas:AddCertPathValidationPolicyAssignmentResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveCertPathValidationPolicyAssignment">
			<wsdl:documentation>
        This operation removes a certification path validation policy assignment from the TLS server on the device.
        If the certification path validation policy identified by the requested CertPathValidationPolicyID is not associated to the TLS server, the device shall produce a CertPathValidationPolicy fault.
      </wsdl:documentation>
			<wsdl:input message="tas:RemoveCertPathValidationPolicyAssignmentRequest"/>
			<wsdl:output message="tas:RemoveCertPathValidationPolicyAssignmentResponse"/>
		</wsdl:operation>
		<wsdl:operation name="ReplaceCertPathValidationPolicyAssignment">
			<wsdl:documentation>
        This operation replaces a certification path validation policy assignment to the TLS server on the device with another certification path validation policy assignment.
        If the certification path validation policy identified by the requested OldCertPathValidationPolicyID is not associated to the TLS server, the device shall produce an OldCertPathValidationPolicyID fault and shall not associate the certification path validation policy identified by the NewCertPathValidationPolicyID to the TLS server.
        If no certification path validation policy exists under the requested NewCertPathValidationPolicyID in the device’s keystore, the device shall produce a NewCertPathValidationPolicyID fault and shall not remove the association of the old certification path validation policy to the TLS server.
      </wsdl:documentation>
			<wsdl:input message="tas:ReplaceCertPathValidationPolicyAssignmentRequest"/>
			<wsdl:output message="tas:ReplaceCertPathValidationPolicyAssignmentResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAssignedCertPathValidationPolicies">
			<wsdl:documentation>
        This operation returns the IDs of all certification path validation policies that are assigned to the TLS server on the device.
      </wsdl:documentation>
			<wsdl:input message="tas:GetAssignedCertPathValidationPoliciesRequest"/>
			<wsdl:output message="tas:GetAssignedCertPathValidationPoliciesResponse"/>
		</wsdl:operation>
		<!--==========================================-->
		<!-- End Cert-based client auth operations    -->
		<!--==========================================-->
	</wsdl:portType>
	<wsdl:portType name="Dot1X">
		<wsdl:documentation>802.1X configuration.</wsdl:documentation>
		<wsdl:operation name="AddDot1XConfiguration">
			<wsdl:documentation>
				(to be written)
			</wsdl:documentation>
			<wsdl:input message="tas:AddDot1XConfigurationRequest"/>
			<wsdl:output message="tas:AddDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAllDot1XConfigurations">
			<wsdl:documentation>
				(to be written)
			</wsdl:documentation>
			<wsdl:input message="tas:GetAllDot1XConfigurationsRequest"/>
			<wsdl:output message="tas:GetAllDot1XConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDot1XConfiguration">
			<wsdl:documentation>
				(to be written)
			</wsdl:documentation>
			<wsdl:input message="tas:GetDot1XConfigurationRequest"/>
			<wsdl:output message="tas:GetDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteDot1XConfiguration">
			<wsdl:documentation>
				(to be written)
			</wsdl:documentation>
			<wsdl:input message="tas:DeleteDot1XConfigurationRequest"/>
			<wsdl:output message="tas:DeleteDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetNetworkInterfaceDot1XConfiguration">
			<wsdl:documentation>
				(to be written)
			</wsdl:documentation>
			<wsdl:input message="tas:SetNetworkInterfaceDot1XConfigurationRequest"/>
			<wsdl:output message="tas:SetNetworkInterfaceDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetNetworkInterfaceDot1XConfiguration">
			<wsdl:documentation>
				(to be written)
			</wsdl:documentation>
			<wsdl:input message="tas:GetNetworkInterfaceDot1XConfigurationRequest"/>
			<wsdl:output message="tas:GetNetworkInterfaceDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteNetworkInterfaceDot1XConfiguration">
			<wsdl:documentation>
				(to be written)
			</wsdl:documentation>
			<wsdl:input message="tas:DeleteNetworkInterfaceDot1XConfigurationRequest"/>
			<wsdl:output message="tas:DeleteNetworkInterfaceDot1XConfigurationResponse"/>
		</wsdl:operation>
	</wsdl:portType>
	<wsdl:binding name="AdvancedSecurityServiceBinding" type="tas:AdvancedSecurityService">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="GetServiceCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetServiceCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
	</wsdl:binding>
	<wsdl:binding name="KeystoreBinding" type="tas:Keystore">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="CreateRSAKeyPair">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/CreateRSAKeyPair"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="UploadKeyPairInPKCS8">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/UploadKeyPairInPKCS8"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="UploadCertificateWithPrivateKeyInPKCS12">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/UploadCertificateWithPrivateKeyInPKCS12"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetKeyStatus">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetKeyStatus"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetPrivateKeyStatus">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetPrivateKeyStatus"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAllKeys">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAllKeys"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteKey">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/DeleteKey"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreatePKCS10CSR">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/CreatePKCS10CSR"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateSelfSignedCertificate">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/CreateSelfSignedCertificate"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="UploadCertificate">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/UploadCertificate"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCertificate">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetCertificate"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAllCertificates">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAllCertificates"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteCertificate">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/DeleteCertificate"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateCertificationPath">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/CreateCertificationPath"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCertificationPath">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetCertificationPath"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAllCertificationPaths">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAllCertificationPaths"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteCertificationPath">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/DeleteCertificationPath"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="UploadPassphrase">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/UploadPassphrase"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAllPassphrases">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAllPassphrases"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeletePassphrase">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/DeletePassphrase"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--==========================================-->
		<!-- Begin Cert-based client auth bindings    -->
		<!--==========================================-->
		<wsdl:operation name="UploadCRL">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/UploadCRL"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCRL">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetCRL"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAllCRLs">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAllCRLs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteCRL">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/DeleteCRL"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateCertPathValidationPolicy">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/CreateCertPathValidationPolicy"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCertPathValidationPolicy">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetCertPathValidationPolicy"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAllCertPathValidationPolicies">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAllCertPathValidationPolicies"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteCertPathValidationPolicy">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/DeleteCertPathValidationPolicy"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--==========================================-->
		<!-- End Cert-based client auth bindings      -->
		<!--==========================================-->
	</wsdl:binding>
	<wsdl:binding name="TLSServerBinding" type="tas:TLSServer">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="AddServerCertificateAssignment">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/AddServerCertificateAssignment"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="RemoveServerCertificateAssignment">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/RemoveServerCertificateAssignment"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="ReplaceServerCertificateAssignment">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/ReplaceServerCertificateAssignment"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAssignedServerCertificates">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAssignedServerCertificates"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetEnabledTLSVersions">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/SetEnabledTLSVersions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetEnabledTLSVersions">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetEnabledTLSVersions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--==========================================-->
		<!-- Begin Cert-based client auth bindings    -->
		<!--==========================================-->
		<wsdl:operation name="SetClientAuthenticationRequired">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/SetClientAuthenticationRequired"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetClientAuthenticationRequired">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetClientAuthenticationRequired"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="AddCertPathValidationPolicyAssignment">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/AddCertPathValidationPolicyAssignment"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="RemoveCertPathValidationPolicyAssignment">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/RemoveCertPathValidationPolicyAssignment"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="ReplaceCertPathValidationPolicyAssignment">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/ReplaceCertPathValidationPolicyAssignment"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAssignedCertPathValidationPolicies">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAssignedCertPathValidationPolicies"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--==========================================-->
		<!-- End Cert-based client auth bindings      -->
		<!--==========================================-->
	</wsdl:binding>
	<wsdl:binding name="Dot1XBinding" type="tas:Dot1X">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="AddDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/AddDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAllDot1XConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetAllDot1XConfigurations"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/DeleteDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetNetworkInterfaceDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/SetNetworkInterfaceDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetNetworkInterfaceDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/GetNetworkInterfaceDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteNetworkInterfaceDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/advancedsecurity/wsdl/DeleteNetworkInterfaceDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
	</wsdl:binding>
	<!--==========================================-->
	<!-- Begin Cert-based client auth messages    -->
	<!--==========================================-->
	<!--==========================================-->
	<!-- End Cert-based client auth messages      -->
	<!--==========================================-->
</wsdl:definitions>