// Auto-generated from Chrome at version 146.0.7680.165 domain: BluetoothEmulation
#![allow(dead_code)]
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CentralState {
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "powered-off")]
    PoweredOff,
    #[serde(rename = "powered-on")]
    PoweredOn,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GattOperationType {
    #[serde(rename = "connection")]
    Connection,
    #[serde(rename = "discovery")]
    Discovery,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CharacteristicWriteType {
    #[serde(rename = "write-default-deprecated")]
    WriteDefaultDeprecated,
    #[serde(rename = "write-with-response")]
    WriteWithResponse,
    #[serde(rename = "write-without-response")]
    WriteWithoutResponse,
}
#[allow(deprecated)]
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
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DescriptorOperationType {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Stores the manufacturer data"]
pub struct ManufacturerData {
    #[serde(default)]
    #[doc = "Company identifier\n <https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml>\n <https://usb.org/developers>"]
    pub key: JsUInt,
    #[doc = "Manufacturer-specific data"]
    pub data: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Stores the byte data of the advertisement packet sent by a Bluetooth device."]
pub struct ScanRecord {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub uuids: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Stores the external appearance description of the device."]
    pub appearance: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Stores the transmission power of a broadcasting device."]
    pub tx_power: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Key is the company identifier and the value is an array of bytes of\n manufacturer specific data."]
    pub manufacturer_data: Option<Vec<ManufacturerData>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Stores the advertisement packet information that is sent by a Bluetooth device."]
pub struct ScanEntry {
    #[serde(default)]
    pub device_address: String,
    #[serde(default)]
    pub rssi: JsUInt,
    pub scan_record: ScanRecord,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Describes the properties of a characteristic. This follows Bluetooth Core\n Specification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties."]
pub struct CharacteristicProperties {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub broadcast: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub read: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub write_without_response: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub write: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub notify: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub indicate: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub authenticated_signed_writes: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub extended_properties: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable the BluetoothEmulation domain."]
pub struct Enable {
    #[doc = "State of the simulated central."]
    pub state: CentralState,
    #[serde(default)]
    #[doc = "If the simulated central supports low-energy."]
    pub le_supported: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set the state of the simulated central."]
pub struct SetSimulatedCentralState {
    #[doc = "State of the simulated central."]
    pub state: CentralState,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Simulates a peripheral with |address|, |name| and |knownServiceUuids|\n that has already been connected to the system."]
pub struct SimulatePreconnectedPeripheral {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub name: String,
    pub manufacturer_data: Vec<ManufacturerData>,
    #[serde(default)]
    pub known_service_uuids: Vec<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Simulates an advertisement packet described in |entry| being received by\n the central."]
pub struct SimulateAdvertisement {
    pub entry: ScanEntry,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Simulates the response code from the peripheral with |address| for a\n GATT operation of |type|. The |code| value follows the HCI Error Codes from\n Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes."]
pub struct SimulateGATTOperationResponse {
    #[serde(default)]
    pub address: String,
    pub r#type: GattOperationType,
    #[serde(default)]
    pub code: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Simulates the response from the characteristic with |characteristicId| for a\n characteristic operation of |type|. The |code| value follows the Error\n Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.\n The |data| is expected to exist when simulating a successful read operation\n response."]
pub struct SimulateCharacteristicOperationResponse {
    #[serde(default)]
    pub characteristic_id: String,
    pub r#type: CharacteristicOperationType,
    #[serde(default)]
    pub code: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Simulates the response from the descriptor with |descriptorId| for a\n descriptor operation of |type|. The |code| value follows the Error\n Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.\n The |data| is expected to exist when simulating a successful read operation\n response."]
pub struct SimulateDescriptorOperationResponse {
    #[serde(default)]
    pub descriptor_id: String,
    pub r#type: DescriptorOperationType,
    #[serde(default)]
    pub code: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Adds a service with |serviceUuid| to the peripheral with |address|."]
pub struct AddService {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub service_uuid: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes the service respresented by |serviceId| from the simulated central."]
pub struct RemoveService {
    #[serde(default)]
    pub service_id: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Adds a characteristic with |characteristicUuid| and |properties| to the\n service represented by |serviceId|."]
pub struct AddCharacteristic {
    #[serde(default)]
    pub service_id: String,
    #[serde(default)]
    pub characteristic_uuid: String,
    pub properties: CharacteristicProperties,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes the characteristic respresented by |characteristicId| from the\n simulated central."]
pub struct RemoveCharacteristic {
    #[serde(default)]
    pub characteristic_id: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Adds a descriptor with |descriptorUuid| to the characteristic respresented\n by |characteristicId|."]
pub struct AddDescriptor {
    #[serde(default)]
    pub characteristic_id: String,
    #[serde(default)]
    pub descriptor_uuid: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes the descriptor with |descriptorId| from the simulated central."]
pub struct RemoveDescriptor {
    #[serde(default)]
    pub descriptor_id: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Simulates a GATT disconnection from the peripheral with |address|."]
pub struct SimulateGATTDisconnection {
    #[serde(default)]
    pub address: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable the BluetoothEmulation domain."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set the state of the simulated central."]
pub struct SetSimulatedCentralStateReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disable the BluetoothEmulation domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Simulates a peripheral with |address|, |name| and |knownServiceUuids|\n that has already been connected to the system."]
pub struct SimulatePreconnectedPeripheralReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Simulates an advertisement packet described in |entry| being received by\n the central."]
pub struct SimulateAdvertisementReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Simulates the response code from the peripheral with |address| for a\n GATT operation of |type|. The |code| value follows the HCI Error Codes from\n Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes."]
pub struct SimulateGATTOperationResponseReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Simulates the response from the characteristic with |characteristicId| for a\n characteristic operation of |type|. The |code| value follows the Error\n Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.\n The |data| is expected to exist when simulating a successful read operation\n response."]
pub struct SimulateCharacteristicOperationResponseReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Simulates the response from the descriptor with |descriptorId| for a\n descriptor operation of |type|. The |code| value follows the Error\n Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.\n The |data| is expected to exist when simulating a successful read operation\n response."]
pub struct SimulateDescriptorOperationResponseReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Adds a service with |serviceUuid| to the peripheral with |address|."]
pub struct AddServiceReturnObject {
    #[serde(default)]
    #[doc = "An identifier that uniquely represents this service."]
    pub service_id: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes the service respresented by |serviceId| from the simulated central."]
pub struct RemoveServiceReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Adds a characteristic with |characteristicUuid| and |properties| to the\n service represented by |serviceId|."]
pub struct AddCharacteristicReturnObject {
    #[serde(default)]
    #[doc = "An identifier that uniquely represents this characteristic."]
    pub characteristic_id: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes the characteristic respresented by |characteristicId| from the\n simulated central."]
pub struct RemoveCharacteristicReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Adds a descriptor with |descriptorUuid| to the characteristic respresented\n by |characteristicId|."]
pub struct AddDescriptorReturnObject {
    #[serde(default)]
    #[doc = "An identifier that uniquely represents this descriptor."]
    pub descriptor_id: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes the descriptor with |descriptorId| from the simulated central."]
pub struct RemoveDescriptorReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Simulates a GATT disconnection from the peripheral with |address|."]
pub struct SimulateGATTDisconnectionReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "BluetoothEmulation.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for SetSimulatedCentralState {
    const NAME: &'static str = "BluetoothEmulation.setSimulatedCentralState";
    type ReturnObject = SetSimulatedCentralStateReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "BluetoothEmulation.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for SimulatePreconnectedPeripheral {
    const NAME: &'static str = "BluetoothEmulation.simulatePreconnectedPeripheral";
    type ReturnObject = SimulatePreconnectedPeripheralReturnObject;
}
#[allow(deprecated)]
impl Method for SimulateAdvertisement {
    const NAME: &'static str = "BluetoothEmulation.simulateAdvertisement";
    type ReturnObject = SimulateAdvertisementReturnObject;
}
#[allow(deprecated)]
impl Method for SimulateGATTOperationResponse {
    const NAME: &'static str = "BluetoothEmulation.simulateGATTOperationResponse";
    type ReturnObject = SimulateGATTOperationResponseReturnObject;
}
#[allow(deprecated)]
impl Method for SimulateCharacteristicOperationResponse {
    const NAME: &'static str = "BluetoothEmulation.simulateCharacteristicOperationResponse";
    type ReturnObject = SimulateCharacteristicOperationResponseReturnObject;
}
#[allow(deprecated)]
impl Method for SimulateDescriptorOperationResponse {
    const NAME: &'static str = "BluetoothEmulation.simulateDescriptorOperationResponse";
    type ReturnObject = SimulateDescriptorOperationResponseReturnObject;
}
#[allow(deprecated)]
impl Method for AddService {
    const NAME: &'static str = "BluetoothEmulation.addService";
    type ReturnObject = AddServiceReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveService {
    const NAME: &'static str = "BluetoothEmulation.removeService";
    type ReturnObject = RemoveServiceReturnObject;
}
#[allow(deprecated)]
impl Method for AddCharacteristic {
    const NAME: &'static str = "BluetoothEmulation.addCharacteristic";
    type ReturnObject = AddCharacteristicReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveCharacteristic {
    const NAME: &'static str = "BluetoothEmulation.removeCharacteristic";
    type ReturnObject = RemoveCharacteristicReturnObject;
}
#[allow(deprecated)]
impl Method for AddDescriptor {
    const NAME: &'static str = "BluetoothEmulation.addDescriptor";
    type ReturnObject = AddDescriptorReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveDescriptor {
    const NAME: &'static str = "BluetoothEmulation.removeDescriptor";
    type ReturnObject = RemoveDescriptorReturnObject;
}
#[allow(deprecated)]
impl Method for SimulateGATTDisconnection {
    const NAME: &'static str = "BluetoothEmulation.simulateGATTDisconnection";
    type ReturnObject = SimulateGATTDisconnectionReturnObject;
}
#[allow(dead_code)]
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct GattOperationReceivedEvent {
        pub params: GattOperationReceivedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct GattOperationReceivedEventParams {
        #[serde(default)]
        pub address: String,
        pub r#type: super::GattOperationType,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CharacteristicOperationReceivedEvent {
        pub params: CharacteristicOperationReceivedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct CharacteristicOperationReceivedEventParams {
        #[serde(default)]
        pub characteristic_id: String,
        pub r#type: super::CharacteristicOperationType,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub data: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub write_type: Option<super::CharacteristicWriteType>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DescriptorOperationReceivedEvent {
        pub params: DescriptorOperationReceivedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DescriptorOperationReceivedEventParams {
        #[serde(default)]
        pub descriptor_id: String,
        pub r#type: super::DescriptorOperationType,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub data: Option<String>,
    }
}
