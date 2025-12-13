// Auto-generated from Chrome at version 143.0.7499.110 domain: BluetoothEmulation
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CentralState {
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "powered-off")]
    PoweredOff,
    #[serde(rename = "powered-on")]
    PoweredOn,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GattOperationType {
    #[serde(rename = "connection")]
    Connection,
    #[serde(rename = "discovery")]
    Discovery,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CharacteristicWriteType {
    #[serde(rename = "write-default-deprecated")]
    WriteDefaultDeprecated,
    #[serde(rename = "write-with-response")]
    WriteWithResponse,
    #[serde(rename = "write-without-response")]
    WriteWithoutResponse,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CharacteristicOperationType {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "subscribe-to-notifications")]
    SubscribeToNotifications,
    #[serde(rename = "unsubscribe-from-notifications")]
    UnsubscribeFromNotifications,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DescriptorOperationType {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ManufacturerData {
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: JsUInt,
    #[serde(rename = "data")]
    pub data: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScanRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "uuids")]
    pub uuids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "appearance")]
    pub appearance: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "txPower")]
    pub tx_power: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "manufacturerData")]
    pub manufacturer_data: Option<Vec<ManufacturerData>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScanEntry {
    #[serde(default)]
    #[serde(rename = "deviceAddress")]
    pub device_address: String,
    #[serde(default)]
    #[serde(rename = "rssi")]
    pub rssi: JsUInt,
    #[serde(rename = "scanRecord")]
    pub scan_record: ScanRecord,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CharacteristicProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "broadcast")]
    pub broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "read")]
    pub read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "writeWithoutResponse")]
    pub write_without_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "write")]
    pub write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "notify")]
    pub notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "indicate")]
    pub indicate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "authenticatedSignedWrites")]
    pub authenticated_signed_writes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "extendedProperties")]
    pub extended_properties: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(rename = "state")]
    pub state: CentralState,
    #[serde(default)]
    #[serde(rename = "leSupported")]
    pub le_supported: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSimulatedCentralState {
    #[serde(rename = "state")]
    pub state: CentralState,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SimulatePreconnectedPeripheral {
    #[serde(default)]
    #[serde(rename = "address")]
    pub address: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "manufacturerData")]
    pub manufacturer_data: Vec<ManufacturerData>,
    #[serde(default)]
    #[serde(rename = "knownServiceUuids")]
    pub known_service_uuids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SimulateAdvertisement {
    #[serde(rename = "entry")]
    pub entry: ScanEntry,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SimulateGATTOperationResponse {
    #[serde(default)]
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: GattOperationType,
    #[serde(default)]
    #[serde(rename = "code")]
    pub code: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SimulateCharacteristicOperationResponse {
    #[serde(default)]
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
    #[serde(rename = "type")]
    pub r#type: CharacteristicOperationType,
    #[serde(default)]
    #[serde(rename = "code")]
    pub code: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SimulateDescriptorOperationResponse {
    #[serde(default)]
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
    #[serde(rename = "type")]
    pub r#type: DescriptorOperationType,
    #[serde(default)]
    #[serde(rename = "code")]
    pub code: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddService {
    #[serde(default)]
    #[serde(rename = "address")]
    pub address: String,
    #[serde(default)]
    #[serde(rename = "serviceUuid")]
    pub service_uuid: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveService {
    #[serde(default)]
    #[serde(rename = "serviceId")]
    pub service_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddCharacteristic {
    #[serde(default)]
    #[serde(rename = "serviceId")]
    pub service_id: String,
    #[serde(default)]
    #[serde(rename = "characteristicUuid")]
    pub characteristic_uuid: String,
    #[serde(rename = "properties")]
    pub properties: CharacteristicProperties,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveCharacteristic {
    #[serde(default)]
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddDescriptor {
    #[serde(default)]
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
    #[serde(default)]
    #[serde(rename = "descriptorUuid")]
    pub descriptor_uuid: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveDescriptor {
    #[serde(default)]
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SimulateGATTDisconnection {
    #[serde(default)]
    #[serde(rename = "address")]
    pub address: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSimulatedCentralStateReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePreconnectedPeripheralReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimulateAdvertisementReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGATTOperationResponseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimulateCharacteristicOperationResponseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimulateDescriptorOperationResponseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddServiceReturnObject {
    #[serde(default)]
    #[serde(rename = "serviceId")]
    pub service_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveServiceReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddCharacteristicReturnObject {
    #[serde(default)]
    #[serde(rename = "characteristicId")]
    pub characteristic_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCharacteristicReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddDescriptorReturnObject {
    #[serde(default)]
    #[serde(rename = "descriptorId")]
    pub descriptor_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDescriptorReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimulateGATTDisconnectionReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "BluetoothEmulation.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for SetSimulatedCentralState {
    const NAME: &'static str = "BluetoothEmulation.setSimulatedCentralState";
    type ReturnObject = SetSimulatedCentralStateReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "BluetoothEmulation.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for SimulatePreconnectedPeripheral {
    const NAME: &'static str = "BluetoothEmulation.simulatePreconnectedPeripheral";
    type ReturnObject = SimulatePreconnectedPeripheralReturnObject;
}
impl Method for SimulateAdvertisement {
    const NAME: &'static str = "BluetoothEmulation.simulateAdvertisement";
    type ReturnObject = SimulateAdvertisementReturnObject;
}
impl Method for SimulateGATTOperationResponse {
    const NAME: &'static str = "BluetoothEmulation.simulateGATTOperationResponse";
    type ReturnObject = SimulateGATTOperationResponseReturnObject;
}
impl Method for SimulateCharacteristicOperationResponse {
    const NAME: &'static str = "BluetoothEmulation.simulateCharacteristicOperationResponse";
    type ReturnObject = SimulateCharacteristicOperationResponseReturnObject;
}
impl Method for SimulateDescriptorOperationResponse {
    const NAME: &'static str = "BluetoothEmulation.simulateDescriptorOperationResponse";
    type ReturnObject = SimulateDescriptorOperationResponseReturnObject;
}
impl Method for AddService {
    const NAME: &'static str = "BluetoothEmulation.addService";
    type ReturnObject = AddServiceReturnObject;
}
impl Method for RemoveService {
    const NAME: &'static str = "BluetoothEmulation.removeService";
    type ReturnObject = RemoveServiceReturnObject;
}
impl Method for AddCharacteristic {
    const NAME: &'static str = "BluetoothEmulation.addCharacteristic";
    type ReturnObject = AddCharacteristicReturnObject;
}
impl Method for RemoveCharacteristic {
    const NAME: &'static str = "BluetoothEmulation.removeCharacteristic";
    type ReturnObject = RemoveCharacteristicReturnObject;
}
impl Method for AddDescriptor {
    const NAME: &'static str = "BluetoothEmulation.addDescriptor";
    type ReturnObject = AddDescriptorReturnObject;
}
impl Method for RemoveDescriptor {
    const NAME: &'static str = "BluetoothEmulation.removeDescriptor";
    type ReturnObject = RemoveDescriptorReturnObject;
}
impl Method for SimulateGATTDisconnection {
    const NAME: &'static str = "BluetoothEmulation.simulateGATTDisconnection";
    type ReturnObject = SimulateGATTDisconnectionReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct GattOperationReceivedEvent {
        pub params: GattOperationReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct GattOperationReceivedEventParams {
        #[serde(default)]
        #[serde(rename = "address")]
        pub address: String,
        #[serde(rename = "type")]
        pub r#type: super::GattOperationType,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CharacteristicOperationReceivedEvent {
        pub params: CharacteristicOperationReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CharacteristicOperationReceivedEventParams {
        #[serde(default)]
        #[serde(rename = "characteristicId")]
        pub characteristic_id: String,
        #[serde(rename = "type")]
        pub r#type: super::CharacteristicOperationType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "data")]
        pub data: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "writeType")]
        pub write_type: Option<super::CharacteristicWriteType>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DescriptorOperationReceivedEvent {
        pub params: DescriptorOperationReceivedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DescriptorOperationReceivedEventParams {
        #[serde(default)]
        #[serde(rename = "descriptorId")]
        pub descriptor_id: String,
        #[serde(rename = "type")]
        pub r#type: super::DescriptorOperationType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "data")]
        pub data: Option<u8>,
    }
}
