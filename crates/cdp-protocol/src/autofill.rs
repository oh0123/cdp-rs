// Auto-generated from Chrome at version 140.0.7339.186 domain: Autofill
use super::dom;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FillingStrategy {
    #[serde(rename = "autocompleteAttribute")]
    AutocompleteAttribute,
    #[serde(rename = "autofillInferred")]
    AutofillInferred,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreditCard {
    #[serde(default)]
    #[serde(rename = "number")]
    pub number: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "expiryMonth")]
    pub expiry_month: String,
    #[serde(default)]
    #[serde(rename = "expiryYear")]
    pub expiry_year: String,
    #[serde(default)]
    #[serde(rename = "cvc")]
    pub cvc: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddressField {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddressFields {
    #[serde(rename = "fields")]
    pub fields: Vec<AddressField>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Address {
    #[serde(rename = "fields")]
    pub fields: Vec<AddressField>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddressUi {
    #[serde(rename = "addressFields")]
    pub address_fields: Vec<AddressFields>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FilledField {
    #[serde(default)]
    #[serde(rename = "htmlType")]
    pub html_type: String,
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
    #[serde(default)]
    #[serde(rename = "autofillType")]
    pub autofill_type: String,
    #[serde(rename = "fillingStrategy")]
    pub filling_strategy: FillingStrategy,
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
    #[serde(rename = "fieldId")]
    pub field_id: dom::BackendNodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Trigger {
    #[serde(rename = "fieldId")]
    pub field_id: dom::BackendNodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frameId")]
    pub frame_id: Option<page::FrameId>,
    #[serde(rename = "card")]
    pub card: CreditCard,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAddresses {
    #[serde(rename = "addresses")]
    pub addresses: Vec<Address>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TriggerReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAddressesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl Method for Trigger {
    const NAME: &'static str = "Autofill.trigger";
    type ReturnObject = TriggerReturnObject;
}
impl Method for SetAddresses {
    const NAME: &'static str = "Autofill.setAddresses";
    type ReturnObject = SetAddressesReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Autofill.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Autofill.enable";
    type ReturnObject = EnableReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AddressFormFilledEvent {
        pub params: AddressFormFilledEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AddressFormFilledEventParams {
        #[serde(rename = "filledFields")]
        pub filled_fields: Vec<super::FilledField>,
        #[serde(rename = "addressUi")]
        pub address_ui: super::AddressUi,
    }
}
