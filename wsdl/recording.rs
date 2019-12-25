<?xml version="1.0" encoding="utf-8"?>
<?xml-stylesheet type="text/xsl" href="../ver20/util/onvif-wsdl-viewer.xsl"?>
<!--
Copyright (c) 2008-2018 by ONVIF: Open Network Video Interface Forum. All rights reserved.

Recipients of this document may copy, distribute, publish, or display this document so long as this copyright notice, license and disclaimer are retained with all copies of the document. No license is granted to modify this document.

THIS DOCUMENT IS PROVIDED "AS IS," AND THE CORPORATION AND ITS MEMBERS AND THEIR AFFILIATES, MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, OR TITLE; THAT THE CONTENTS OF THIS DOCUMENT ARE SUITABLE FOR ANY PURPOSE; OR THAT THE IMPLEMENTATION OF SUCH CONTENTS WILL NOT INFRINGE ANY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
IN NO EVENT WILL THE CORPORATION OR ITS MEMBERS OR THEIR AFFILIATES BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE OR CONSEQUENTIAL DAMAGES, ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT, WHETHER OR NOT (1) THE CORPORATION, MEMBERS OR THEIR AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES, OR (2) SUCH DAMAGES WERE REASONABLY FORESEEABLE, AND ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT.  THE FOREGOING DISCLAIMER AND LIMITATION ON LIABILITY DO NOT APPLY TO, INVALIDATE, OR LIMIT REPRESENTATIONS AND WARRANTIES MADE BY THE MEMBERS AND THEIR RESPECTIVE AFFILIATES TO THE CORPORATION AND OTHER MEMBERS IN CERTAIN WRITTEN POLICIES OF THE CORPORATION.
-->
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:trc="http://www.onvif.org/ver10/recording/wsdl" targetNamespace="http://www.onvif.org/ver10/recording/wsdl">
	<wsdl:types>
		<xs:schema targetNamespace="http://www.onvif.org/ver10/recording/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified" version="18.06">
			<xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../ver10/schema/onvif.xsd"/>
			<!--  Message Request/Responses elements  -->
			<!--===============================-->
			<xs:element name="GetServiceCapabilities">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetServiceCapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="trc:Capabilities">
							<xs:annotation>
								<xs:documentation>The capabilities for the recording service is returned in the Capabilities element.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="Capabilities">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:attribute name="DynamicRecordings" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indication if the device supports dynamic creation and deletion of recordings</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="DynamicTracks" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indication if the device supports dynamic creation and deletion of tracks</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Encoding" type="trc:EncodingTypes">
					<xs:annotation>
						<xs:documentation>Indication which encodings are supported for recording. The list may contain one or more enumeration values of tt:VideoEncoding and tt:AudioEncoding. For encodings that are neither defined in tt:VideoEncoding nor tt:AudioEncoding the device shall use the <a href="http://www.iana.org/assignments/media-types/media-types.xhtml">IANA</a> defintions. Note, that a device without audio support shall not return audio encodings.  </xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaxRate" type="xs:float">
					<xs:annotation>
						<xs:documentation>Maximum supported bit rate for all tracks of a recording in kBit/s.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaxTotalRate" type="xs:float">
					<xs:annotation>
						<xs:documentation>Maximum supported bit rate for all recordings in kBit/s.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaxRecordings" type="xs:float">
					<xs:annotation>
						<xs:documentation>Maximum number of recordings supported. (Integer values only.)</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaxRecordingJobs" type="xs:int">
					<xs:annotation>
						<xs:documentation>Maximum total number of supported recording jobs by the device.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Options" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indication if the device supports the GetRecordingOptions command.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MetadataRecording" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indication if the device supports recording metadata.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SupportedExportFileFormats" type="tt:StringAttrList">
					<xs:annotation>
						<xs:documentation> 
						Indication that the device supports ExportRecordedData command for the listed export file formats.
						The list shall return at least one export file format value. The value of 'ONVIF' refers to
						ONVIF Export File Format specification.
						</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:simpleType name="EncodingTypes">
				<xs:list itemType="xs:string"/>
			</xs:simpleType>
			<xs:element name="Capabilities" type="trc:Capabilities"/>
			<!--===============================-->
			<xs:element name="CreateRecording">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingConfiguration" type="tt:RecordingConfiguration">
							<xs:annotation>
								<xs:documentation>Initial configuration for the recording.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateRecordingResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>The reference to the created recording.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteRecording">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>The reference of the recording to be deleted.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteRecordingResponse">
				<xs:complexType>
					<xs:sequence>
			          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordings">
				<xs:complexType>
					<xs:sequence>
			          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingItem" type="tt:GetRecordingsResponseItem" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List of recording items.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRecordingConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>Token of the recording that shall be changed.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="RecordingConfiguration" type="tt:RecordingConfiguration">
							<xs:annotation>
								<xs:documentation>The new configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRecordingConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
			          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>Token of the configuration to be retrieved.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingConfiguration" type="tt:RecordingConfiguration">
							<xs:annotation>
								<xs:documentation>Configuration of the recording.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateTrack">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>Identifies the recording to which a track shall be added.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TrackConfiguration" type="tt:TrackConfiguration">
							<xs:annotation>
								<xs:documentation>The configuration of the new track.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateTrackResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="TrackToken" type="tt:TrackReference">
							<xs:annotation>
								<xs:documentation>The TrackToken shall identify the newly created track. The
							TrackToken shall be unique within the recoding to which
							the new track belongs.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteTrack">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>Token of the recording the track belongs to.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TrackToken" type="tt:TrackReference">
							<xs:annotation>
								<xs:documentation>Token of the track to be deleted.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteTrackResponse">
				<xs:complexType>
					<xs:sequence>
			          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetTrackConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>Token of the recording the track belongs to.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TrackToken" type="tt:TrackReference">
							<xs:annotation>
								<xs:documentation>Token of the track.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetTrackConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="TrackConfiguration" type="tt:TrackConfiguration">
							<xs:annotation>
								<xs:documentation>Configuration of the track.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetTrackConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>Token of the recording the track belongs to.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TrackToken" type="tt:TrackReference">
							<xs:annotation>
								<xs:documentation>Token of the track to be modified.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TrackConfiguration" type="tt:TrackConfiguration">
							<xs:annotation>
								<xs:documentation>New configuration for the track.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetTrackConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateRecordingJob">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobConfiguration" type="tt:RecordingJobConfiguration">
							<xs:annotation>
								<xs:documentation>The initial configuration of the new recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateRecordingJobResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobToken" type="tt:RecordingJobReference">
							<xs:annotation>
								<xs:documentation>The JobToken shall identify the created recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="JobConfiguration" type="tt:RecordingJobConfiguration">
							<xs:annotation>
								<xs:documentation>
							The JobConfiguration structure shall be the configuration as it is used by the device. This may be different from the
							JobConfiguration passed to CreateRecordingJob.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteRecordingJob">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobToken" type="tt:RecordingJobReference">
							<xs:annotation>
								<xs:documentation>The token of the job to be deleted.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteRecordingJobResponse">
				<xs:complexType>
					<xs:sequence>
			          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingJobs">
				<xs:complexType>
					<xs:sequence>
			          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingJobsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobItem" type="tt:GetRecordingJobsResponseItem" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List of recording jobs.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRecordingJobConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobToken" type="tt:RecordingJobReference">
							<xs:annotation>
								<xs:documentation>Token of the job to be modified.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="JobConfiguration" type="tt:RecordingJobConfiguration">
							<xs:annotation>
								<xs:documentation>New configuration of the recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRecordingJobConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobConfiguration" type="tt:RecordingJobConfiguration">
							<xs:annotation>
								<xs:documentation>The JobConfiguration structure shall be the configuration
							as it is used by the device. This may be different from the JobConfiguration passed to SetRecordingJobConfiguration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingJobConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobToken" type="tt:RecordingJobReference">
							<xs:annotation>
								<xs:documentation>Token of the recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingJobConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobConfiguration" type="tt:RecordingJobConfiguration">
							<xs:annotation>
								<xs:documentation>Current configuration of the recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRecordingJobMode">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobToken" type="tt:RecordingJobReference">
							<xs:annotation>
								<xs:documentation>Token of the recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Mode" type="tt:RecordingJobMode">
							<xs:annotation>
								<xs:documentation>The new mode for the recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRecordingJobModeResponse">
				<xs:complexType>
					<xs:sequence>
			          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingJobState">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="JobToken" type="tt:RecordingJobReference">
							<xs:annotation>
								<xs:documentation>Token of the recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingJobStateResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="State" type="tt:RecordingJobStateInformation">
							<xs:annotation>
								<xs:documentation>The current state of the recording job.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference">
							<xs:annotation>
								<xs:documentation>Token of the recording.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="trc:RecordingOptions">
							<xs:annotation>
								<xs:documentation>Configuration of the recording.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>

			<!--===============================-->
			<xs:element name="ExportRecordedData">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StartPoint" type="xs:dateTime" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional parameter that specifies start time for the exporting.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="EndPoint" type="xs:dateTime" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional parameter that specifies end time for the exporting.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="SearchScope" type="tt:SearchScope">
							<xs:annotation>
								<xs:documentation>Indicates the selection criterion on the existing recordings. .</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="FileFormat" type="xs:string">
							<xs:annotation>
								<xs:documentation>Indicates which export file format to be used.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="StorageDestination" type="tt:StorageReferencePath">
							<xs:annotation>
								<xs:documentation>Indicates the target storage and relative directory path.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="ExportRecordedDataResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OperationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Unique operation token for client to associate the relevant events.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="FileNames" type="xs:string" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List of exported file names. The device can also use AsyncronousOperationStatus event to publish this list.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Extension" minOccurs="0">
							<xs:complexType>
								<xs:sequence>
									<xs:any namespace="##any" processContents="lax">	 <!-- first Vendor then ONVIF -->
										<xs:annotation>
											<xs:documentation>Extensibility point.</xs:documentation>
										</xs:annotation>
									</xs:any>
								</xs:sequence>
							</xs:complexType>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>

			<xs:element name="StopExportRecordedData">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OperationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Unique ExportRecordedData operation token</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="StopExportRecordedDataResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Progress" type="xs:float">
							<xs:annotation>
								<xs:documentation>Progress percentage of ExportRecordedData operation.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="FileProgressStatus" type="tt:ArrayOfFileProgress">
							<xs:annotation>
								<xs:documentation> </xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax">	 <!-- first Vendor then ONVIF -->
							<xs:annotation>
								<xs:documentation>Extensibility point.</xs:documentation>
							</xs:annotation>
						</xs:any>
					</xs:sequence>
				</xs:complexType>
			</xs:element>

			<xs:element name="GetExportRecordedDataState">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OperationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Unique ExportRecordedData operation token</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetExportRecordedDataStateResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Progress" type="xs:float">
							<xs:annotation>
								<xs:documentation>Progress percentage of ExportRecordedData operation.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="FileProgressStatus" type="tt:ArrayOfFileProgress">
							<xs:annotation>
								<xs:documentation> </xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax">	 <!-- first Vendor then ONVIF -->
							<xs:annotation>
								<xs:documentation>Extensibility point.</xs:documentation>
							</xs:annotation>
						</xs:any>
					</xs:sequence>
				</xs:complexType>
			</xs:element>

			<!--===============================-->
			<xs:complexType name="RecordingOptions">
				<xs:sequence>
					<xs:element name="Job" type="trc:JobOptions"/>
					<xs:element name="Track" type="trc:TrackOptions"/>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
				</xs:sequence>
			</xs:complexType>
			<xs:complexType name="JobOptions">
				<xs:attribute name="Spare" type="xs:int">
					<xs:annotation>
						<xs:documentation>Number of spare jobs that can be created for the recording.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="CompatibleSources" type="tt:StringAttrList">
					<xs:annotation>
						<xs:documentation>A device that supports recording of a restricted set of Media/Media2 Service Profiles returns the list of profiles that can be recorded on the given Recording.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:complexType name="TrackOptions">
				<xs:attribute name="SpareTotal" type="xs:int">
					<xs:annotation>
						<xs:documentation>Total spare number of tracks that can be added to this recording.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SpareVideo" type="xs:int">
					<xs:annotation>
						<xs:documentation>Number of spare Video tracks that can be added to this recording.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SpareAudio" type="xs:int">
					<xs:annotation>
						<xs:documentation>Number of spare Aduio tracks that can be added to this recording.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SpareMetadata" type="xs:int">
					<xs:annotation>
						<xs:documentation>Number of spare Metadata tracks that can be added to this recording.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
		</xs:schema>
	</wsdl:types>
	<wsdl:message name="GetServiceCapabilitiesRequest">
		<wsdl:part name="parameters" element="trc:GetServiceCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesResponse">
		<wsdl:part name="parameters" element="trc:GetServiceCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateRecordingRequest">
		<wsdl:part name="parameters" element="trc:CreateRecording"/>
	</wsdl:message>
	<wsdl:message name="CreateRecordingResponse">
		<wsdl:part name="parameters" element="trc:CreateRecordingResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteRecordingRequest">
		<wsdl:part name="parameters" element="trc:DeleteRecording"/>
	</wsdl:message>
	<wsdl:message name="DeleteRecordingResponse">
		<wsdl:part name="parameters" element="trc:DeleteRecordingResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingsRequest">
		<wsdl:part name="parameters" element="trc:GetRecordings"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingsResponse">
		<wsdl:part name="parameters" element="trc:GetRecordingsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRecordingConfigurationRequest">
		<wsdl:part name="parameters" element="trc:SetRecordingConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetRecordingConfigurationResponse">
		<wsdl:part name="parameters" element="trc:SetRecordingConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingConfigurationRequest">
		<wsdl:part name="parameters" element="trc:GetRecordingConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingConfigurationResponse">
		<wsdl:part name="parameters" element="trc:GetRecordingConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingOptionsRequest">
		<wsdl:part name="parameters" element="trc:GetRecordingOptions"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingOptionsResponse">
		<wsdl:part name="parameters" element="trc:GetRecordingOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateTrackRequest">
		<wsdl:part name="parameters" element="trc:CreateTrack"/>
	</wsdl:message>
	<wsdl:message name="CreateTrackResponse">
		<wsdl:part name="parameters" element="trc:CreateTrackResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteTrackRequest">
		<wsdl:part name="parameters" element="trc:DeleteTrack"/>
	</wsdl:message>
	<wsdl:message name="DeleteTrackResponse">
		<wsdl:part name="parameters" element="trc:DeleteTrackResponse"/>
	</wsdl:message>
	<wsdl:message name="GetTrackConfigurationRequest">
		<wsdl:part name="parameters" element="trc:GetTrackConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetTrackConfigurationResponse">
		<wsdl:part name="parameters" element="trc:GetTrackConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetTrackConfigurationRequest">
		<wsdl:part name="parameters" element="trc:SetTrackConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetTrackConfigurationResponse">
		<wsdl:part name="parameters" element="trc:SetTrackConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateRecordingJobRequest">
		<wsdl:part name="parameters" element="trc:CreateRecordingJob"/>
	</wsdl:message>
	<wsdl:message name="CreateRecordingJobResponse">
		<wsdl:part name="parameters" element="trc:CreateRecordingJobResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteRecordingJobRequest">
		<wsdl:part name="parameters" element="trc:DeleteRecordingJob"/>
	</wsdl:message>
	<wsdl:message name="DeleteRecordingJobResponse">
		<wsdl:part name="parameters" element="trc:DeleteRecordingJobResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingJobsRequest">
		<wsdl:part name="parameters" element="trc:GetRecordingJobs"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingJobsResponse">
		<wsdl:part name="parameters" element="trc:GetRecordingJobsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRecordingJobConfigurationRequest">
		<wsdl:part name="parameters" element="trc:SetRecordingJobConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetRecordingJobConfigurationResponse">
		<wsdl:part name="parameters" element="trc:SetRecordingJobConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingJobConfigurationRequest">
		<wsdl:part name="parameters" element="trc:GetRecordingJobConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingJobConfigurationResponse">
		<wsdl:part name="parameters" element="trc:GetRecordingJobConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRecordingJobModeRequest">
		<wsdl:part name="parameters" element="trc:SetRecordingJobMode"/>
	</wsdl:message>
	<wsdl:message name="SetRecordingJobModeResponse">
		<wsdl:part name="parameters" element="trc:SetRecordingJobModeResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingJobStateRequest">
		<wsdl:part name="parameters" element="trc:GetRecordingJobState"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingJobStateResponse">
		<wsdl:part name="parameters" element="trc:GetRecordingJobStateResponse"/>
	</wsdl:message>

	<wsdl:message name="ExportRecordedDataRequest">
		<wsdl:part name="parameters" element="trc:ExportRecordedData"/>
	</wsdl:message>
	<wsdl:message name="ExportRecordedDataResponse">
		<wsdl:part name="parameters" element="trc:ExportRecordedDataResponse"/>
	</wsdl:message>
	<wsdl:message name="StopExportRecordedDataRequest">
		<wsdl:part name="parameters" element="trc:StopExportRecordedData"/>
	</wsdl:message>
	<wsdl:message name="StopExportRecordedDataResponse">
		<wsdl:part name="parameters" element="trc:StopExportRecordedDataResponse"/>
	</wsdl:message>
	<wsdl:message name="GetExportRecordedDataStateRequest">
		<wsdl:part name="parameters" element="trc:GetExportRecordedDataState"/>
	</wsdl:message>
	<wsdl:message name="GetExportRecordedDataStateResponse">
		<wsdl:part name="parameters" element="trc:GetExportRecordedDataStateResponse"/>
	</wsdl:message>
	
	<wsdl:portType name="RecordingPort">
		<wsdl:operation name="GetServiceCapabilities">
			<wsdl:documentation>Returns the capabilities of the recording service. The result is returned in a typed answer.</wsdl:documentation>
			<wsdl:input message="trc:GetServiceCapabilitiesRequest"/>
			<wsdl:output message="trc:GetServiceCapabilitiesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateRecording">
			<wsdl:documentation>CreateRecording shall create a new recording. The new recording shall be created with a track 
                                for each supported TrackType see Recording Control Spec. <br/>
				This method is optional. It shall be available if the Recording/DynamicRecordings capability is TRUE. <br/>
				When successfully completed, CreateRecording shall have created three tracks with the following configurations: <ul>
					<li>
				TrackToken TrackType</li>
					<li>
					VIDEO001 Video</li>
					<li>
						AUDIO001 Audio</li>
					<li>
					META001 Metadata</li>
				</ul>
				All TrackConfigurations shall have the MaximumRetentionTime set to 0 (unlimited), and the
				Description set to the empty string.
			</wsdl:documentation>
			<wsdl:input message="trc:CreateRecordingRequest"/>
			<wsdl:output message="trc:CreateRecordingResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteRecording">
			<wsdl:documentation>DeleteRecording shall delete a recording object. Whenever a recording is deleted, the device
				shall delete all the tracks that are part of the recording, and it shall delete all the Recording
				Jobs that record into the recording. For each deleted recording job, the device shall also
				delete all the receiver objects associated with the recording job that are automatically created
				using the AutoCreateReceiver field of the recording job configuration structure and are not
				used in any other recording job.<br/>
				This method is optional. It shall be available if the Recording/DynamicRecordings capability is TRUE.
			</wsdl:documentation>
			<wsdl:input message="trc:DeleteRecordingRequest"/>
			<wsdl:output message="trc:DeleteRecordingResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRecordings">
			<wsdl:documentation>GetRecordings shall return a description of all the recordings in the device. This description
				shall include a list of all the tracks for each recording.</wsdl:documentation>
			<wsdl:input message="trc:GetRecordingsRequest"/>
			<wsdl:output message="trc:GetRecordingsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRecordingConfiguration">
			<wsdl:documentation>SetRecordingConfiguration shall change the configuration of a recording.</wsdl:documentation>
			<wsdl:input message="trc:SetRecordingConfigurationRequest"/>
			<wsdl:output message="trc:SetRecordingConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingConfiguration">
			<wsdl:documentation>GetRecordingConfiguration shall retrieve the recording configuration for a recording.</wsdl:documentation>
			<wsdl:input message="trc:GetRecordingConfigurationRequest"/>
			<wsdl:output message="trc:GetRecordingConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingOptions">
			<wsdl:documentation>GetRecordingOptions returns information for a recording identified by the RecordingToken. The information includes the number of additonal tracks as well as recording jobs that can be configured.</wsdl:documentation>
			<wsdl:input message="trc:GetRecordingOptionsRequest"/>
			<wsdl:output message="trc:GetRecordingOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateTrack">
			<wsdl:documentation>This method shall create a new track within a recording.<br/>
				This method is optional. It shall be available if the Recording/DynamicTracks capability is TRUE.<br/>
				A TrackToken in itself does not uniquely identify a specific track. Tracks within different
				recordings may have the same TrackToken.
			</wsdl:documentation>
			<wsdl:input message="trc:CreateTrackRequest"/>
			<wsdl:output message="trc:CreateTrackResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteTrack">
			<wsdl:documentation>DeleteTrack shall remove a track from a recording. All the data in the track shall be deleted.<br/>
				This method is optional. It shall be available if the Recording/DynamicTracks capability is
				TRUE.</wsdl:documentation>
			<wsdl:input message="trc:DeleteTrackRequest"/>
			<wsdl:output message="trc:DeleteTrackResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetTrackConfiguration">
			<wsdl:documentation>GetTrackConfiguration shall retrieve the configuration for a specific track.</wsdl:documentation>
			<wsdl:input message="trc:GetTrackConfigurationRequest"/>
			<wsdl:output message="trc:GetTrackConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetTrackConfiguration">
			<wsdl:documentation>SetTrackConfiguration shall change the configuration of a track.</wsdl:documentation>
			<wsdl:input message="trc:SetTrackConfigurationRequest"/>
			<wsdl:output message="trc:SetTrackConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateRecordingJob">
			<wsdl:documentation>CreateRecordingJob shall create a new recording job.<br/>
				The JobConfiguration returned from CreateRecordingJob shall be identical to the
				JobConfiguration passed into CreateRecordingJob, except for the ReceiverToken and the
				AutoCreateReceiver. In the returned structure, the ReceiverToken shall be present and valid
				and the AutoCreateReceiver field shall be omitted.
			</wsdl:documentation>
			<wsdl:input message="trc:CreateRecordingJobRequest"/>
			<wsdl:output message="trc:CreateRecordingJobResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteRecordingJob">
			<wsdl:documentation>DeleteRecordingJob removes a recording job. It shall also implicitly delete all the receiver
				objects associated with the recording job that are automatically created using the
				AutoCreateReceiver field of the recording job configuration structure and are not used in any
				other recording job.</wsdl:documentation>
			<wsdl:input message="trc:DeleteRecordingJobRequest"/>
			<wsdl:output message="trc:DeleteRecordingJobResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingJobs">
			<wsdl:documentation>GetRecordingJobs shall return a list of all the recording jobs in the device.</wsdl:documentation>
			<wsdl:input message="trc:GetRecordingJobsRequest"/>
			<wsdl:output message="trc:GetRecordingJobsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRecordingJobConfiguration">
			<wsdl:documentation>SetRecordingJobConfiguration shall change the configuration for a recording job.<br/>
				SetRecordingJobConfiguration shall implicitly delete any receiver objects that were created
				automatically if they are no longer used as a result of changing the recording job configuration.
			</wsdl:documentation>
			<wsdl:input message="trc:SetRecordingJobConfigurationRequest"/>
			<wsdl:output message="trc:SetRecordingJobConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingJobConfiguration">
			<wsdl:documentation>GetRecordingJobConfiguration shall return the current configuration for a recording job.</wsdl:documentation>
			<wsdl:input message="trc:GetRecordingJobConfigurationRequest"/>
			<wsdl:output message="trc:GetRecordingJobConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRecordingJobMode">
			<wsdl:documentation>SetRecordingJobMode shall change the mode of the recording job. Using this method shall be
				equivalent to retrieving the recording job configuration, and writing it back with a different
				mode.</wsdl:documentation>
			<wsdl:input message="trc:SetRecordingJobModeRequest"/>
			<wsdl:output message="trc:SetRecordingJobModeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingJobState">
			<wsdl:documentation>GetRecordingJobState returns the state of a recording job. It includes an aggregated state,
				and state for each track of the recording job.</wsdl:documentation>
			<wsdl:input message="trc:GetRecordingJobStateRequest"/>
			<wsdl:output message="trc:GetRecordingJobStateResponse"/>
		</wsdl:operation>

		<wsdl:operation name="ExportRecordedData">
			<wsdl:documentation>
			Exports the selected recordings (from existing recorded data) to the given storage target based on the requested file format. 
			</wsdl:documentation>
			<wsdl:input message="trc:ExportRecordedDataRequest"/>
			<wsdl:output message="trc:ExportRecordedDataResponse"/>
		</wsdl:operation>
		<wsdl:operation name="StopExportRecordedData">
			<wsdl:documentation>
			Stops the selected ExportRecordedData operation. 
			</wsdl:documentation>
			<wsdl:input message="trc:StopExportRecordedDataRequest"/>
			<wsdl:output message="trc:StopExportRecordedDataResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetExportRecordedDataState">
			<wsdl:documentation>
			Retrieves the status of selected ExportRecordedData operation. 
			</wsdl:documentation>
			<wsdl:input message="trc:GetExportRecordedDataStateRequest"/>
			<wsdl:output message="trc:GetExportRecordedDataStateResponse"/>
		</wsdl:operation>
		
	</wsdl:portType>
	<wsdl:binding name="RecordingBinding" type="trc:RecordingPort">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="GetServiceCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetServiceCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateRecording">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/CreateRecording"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteRecording">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/DeleteRecording"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRecordings">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetRecordings"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRecordingConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/SetRecordingConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetRecordingConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetRecordingOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateTrack">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/CreateTrack"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteTrack">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/DeleteTrack"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetTrackConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetTrackConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetTrackConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/SetTrackConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateRecordingJob">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/CreateRecordingJob"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteRecordingJob">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/DeleteRecordingJob"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingJobs">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetRecordingJobs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRecordingJobConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/SetRecordingJobConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingJobConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetRecordingJobConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRecordingJobMode">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/SetRecordingJobMode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRecordingJobState">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetRecordingJobState"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>

		<wsdl:operation name="ExportRecordedData">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/ExportRecordedData"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="StopExportRecordedData">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/StopExportRecordedData"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetExportRecordedDataState">
			<soap:operation soapAction="http://www.onvif.org/ver10/recording/wsdl/GetExportRecordedDataState"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		
	</wsdl:binding>
	<!--===============================-->
</wsdl:definitions>