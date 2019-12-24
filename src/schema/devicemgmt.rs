// Based on devicemgmt.wsdl.xml

// This file is an example of how generated code will look like. It contains some comments
// to see the relation between xml's and Rust code.

// Quote from ONVIF core spec (5.5 messages):
//
//    The  WSDL  message  part  element  is  used  to  define  the  actual  format  of  the  message.
//    Although there can be multiple parts in a WSDL message, this specification follows the WS-I basic
//    profile  [WS-I  BP  2.0]  and  does  not  allow  more  than  one  part  element  in  a  message.
//    Hence we always use the same name (“parameters”) for the message part name. The following WSDL
//    notation is used for ONVIF specifications:
//
//    <message name="’Operation_Name’Request”>
//       <part name="parameters" element="’prefix’:’Operation_Name’"/>
//    </message>
//
//    respective,
//    <message name="’Operation_Name’Response”>
//        <part name="parameters" element="’prefix’:’Operation_Name’Response”/>
//    </message>
//
//    where 'prefix' is the prefix for the namespace in which the message is defined.
//
//
// So according to the spec we will use only single-part messages.

// Quote from ONVIF core spec (5.6.2 One-way operation type):
//     <operation name=”’Operation_Name’”>
//         <input message=”’prefix’:’Operation_Name’”/>
//     </operation>
//
// Quote from ONVIF core spec (5.6.3 Request-response operation type):
//     <operation name=”’Operation_Name’”>
//         <input message=”’prefix’:’Operation_Name’”/>
//         <output message=”’prefix’:’Operation_Name’Response”/>
//         <fault name> = “Fault” message = “’prefix’:’FaultMessage_Name’”>
//     </operation>
//
//
// So we have 2 types of operations, some with fault message.
// But after some grepping over WSDLs we can state:
//  - Only event.wsdl has one-way operations.
//  - Only event.wsdl has operations with fault messages.
//  - All other wsdl's except event.wsdl (25 in total) have only req-rep operations
//     with exactly 1 input and 1 output message.
//
// For now it looks like we can divide all files into 2 categories:
//  - event.wsdl. It is a little bit more complex case for code generation.
//  - everything else. Simpler case. Let current file (devicegmt.rs) be an example of this group.

use super::transport;
use crate::schema::onvif as tt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

//    <xs:element name="GetSystemDateAndTime">
//        <xs:complexType>
//            <xs:sequence/>
//        </xs:complexType>
//    </xs:element>

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemDateAndTime {}

//    <xs:element name="GetSystemDateAndTimeResponse">
//        <xs:complexType>
//            <xs:sequence>
//                <xs:element name="SystemDateAndTime" type="tt:SystemDateTime">
//                    <xs:annotation>
//                        <xs:documentation>Contains information whether system date and time are set manually or by NTP, daylight savings is on or off, time zone in POSIX 1003.1 format and system date and time in UTC and also local system date and time.</xs:documentation>
//                    </xs:annotation>
//                </xs:element>
//            </xs:sequence>
//        </xs:complexType>
//    </xs:element>

// Contains information whether system date and time are set manually or by NTP,
// daylight savings is on or off, time zone in POSIX 1003.1 format and system date
// and time in UTC and also local system date and time.
#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetSystemDateAndTimeResponse {
    #[yaserde(prefix = "tds", rename = "SystemDateAndTime")]
    pub system_date_and_time: tt::SystemDateAndTime,
}

//    <wsdl:operation name="GetSystemDateAndTime">
//        <wsdl:documentation>This operation gets the device system date and time. The device shall support the return of
//            the daylight saving setting and of the manual system date and time (if applicable) or indication
//            of NTP time (if applicable) through the GetSystemDateAndTime command.<br/>
//            A device shall provide the UTCDateTime information.
//        </wsdl:documentation>
//        <wsdl:input message="tds:GetSystemDateAndTimeRequest"/>
//        <wsdl:output message="tds:GetSystemDateAndTimeResponse"/>
//    </wsdl:operation>
//
//    <wsdl:message name="GetSystemDateAndTimeRequest">
//        <wsdl:part name="parameters" element="tds:GetSystemDateAndTime"/>
//    </wsdl:message>
//
//    <wsdl:message name="GetSystemDateAndTimeResponse">
//        <wsdl:part name="parameters" element="tds:GetSystemDateAndTimeResponse"/>
//    </wsdl:message>

// This operation gets the device system date and time. The device shall support the return of
// the daylight saving setting and of the manual system date and time (if applicable) or
// indication of NTP time (if applicable) through the GetSystemDateAndTime command.
// A device shall provide the UTCDateTime information.

pub fn get_system_date_and_time<T: transport::Transport>(
    transport: &mut T,
    request: &GetSystemDateAndTime,
) -> Result<GetSystemDateAndTimeResponse, transport::Error> {
    transport::request(transport, request)
}

//    <xs:element name="GetDeviceInformation">
//        <xs:complexType>
//            <xs:sequence/>
//        </xs:complexType>
//    </xs:element>

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDeviceInformation {}

//    <xs:element name="GetDeviceInformationResponse">
//        <xs:complexType>
//            <xs:sequence>
//                <xs:element name="Manufacturer" type="xs:string">
//                    <xs:annotation>
//                        <xs:documentation>The manufactor of the device.</xs:documentation>
//                    </xs:annotation>
//                </xs:element>
//                <xs:element name="Model" type="xs:string">
//                    <xs:annotation>
//                        <xs:documentation>The device model.</xs:documentation>
//                    </xs:annotation>
//                </xs:element>
//                <xs:element name="FirmwareVersion" type="xs:string">
//                    <xs:annotation>
//                        <xs:documentation>The firmware version in the device.</xs:documentation>
//                    </xs:annotation>
//                </xs:element>
//                <xs:element name="SerialNumber" type="xs:string">
//                    <xs:annotation>
//                        <xs:documentation>The serial number of the device.</xs:documentation>
//                    </xs:annotation>
//                </xs:element>
//                <xs:element name="HardwareId" type="xs:string">
//                    <xs:annotation>
//                        <xs:documentation>The hardware ID of the device.</xs:documentation>
//                    </xs:annotation>
//                </xs:element>
//            </xs:sequence>
//        </xs:complexType>
//    </xs:element>

#[derive(Default, PartialEq, Debug, YaDeserialize)]
#[yaserde(
    prefix = "tds",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct GetDeviceInformationResponse {
    // The manufactor of the device.
    #[yaserde(prefix = "tds", rename = "Manufacturer")]
    pub manufacturer: String,

    // The device model.
    #[yaserde(prefix = "tds", rename = "Model")]
    pub model: String,

    // The firmware version in the device.
    #[yaserde(prefix = "tds", rename = "FirmwareVersion")]
    pub firmware_version: String,

    // The serial number of the device.
    #[yaserde(prefix = "tds", rename = "SerialNumber")]
    pub serial_number: String,

    // The hardware ID of the device.
    #[yaserde(prefix = "tds", rename = "HardwareId")]
    pub hardware_id: String,
}

//    <wsdl:operation name="GetDeviceInformation">
//        <wsdl:documentation>This operation gets basic device information from the device.</wsdl:documentation>
//        <wsdl:input message="tds:GetDeviceInformationRequest"/>
//        <wsdl:output message="tds:GetDeviceInformationResponse"/>
//    </wsdl:operation>
//
//    <wsdl:message name="GetDeviceInformationRequest">
//        <wsdl:part name="parameters" element="tds:GetDeviceInformation"/>
//    </wsdl:message>
//
//    <wsdl:message name="GetDeviceInformationResponse">
//        <wsdl:part name="parameters" element="tds:GetDeviceInformationResponse"/>
//    </wsdl:message>

// This operation gets basic device information from the device.
pub fn get_device_information<T: transport::Transport>(
    transport: &mut T,
    request: &GetDeviceInformation,
) -> Result<GetDeviceInformationResponse, transport::Error> {
    transport::request(transport, request)
}
