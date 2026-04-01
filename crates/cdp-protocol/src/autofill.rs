// Auto-generated from Chrome at version 146.0.7680.165 domain: Autofill
use super::dom;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct CreditCard {
    #[serde(default)]
    #[doc = "16-digit credit card number."]
    pub number: String,
    #[serde(default)]
    #[doc = "Name of the credit card owner."]
    pub name: String,
    #[serde(default)]
    #[doc = "2-digit expiry month."]
    pub expiry_month: String,
    #[serde(default)]
    #[doc = "4-digit expiry year."]
    pub expiry_year: String,
    #[serde(default)]
    #[doc = "3-digit card verification code."]
    pub cvc: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AddressField {
    #[serde(default)]
    #[doc = "address field name, for example GIVEN_NAME.\n The full list of supported field names:\n https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38"]
    pub name: String,
    #[serde(default)]
    #[doc = "address field value, for example Jon Doe."]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A list of address fields."]
pub struct AddressFields {
    pub fields: Vec<AddressField>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[doc = "fields and values defining an address."]
    pub fields: Vec<AddressField>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Defines how an address can be displayed like in chrome://settings/addresses.\n Address UI is a two dimensional array, each inner array is an \"address information line\", and when rendered in a UI surface should be displayed as such.\n The following address UI for instance:\n [[{name: \"GIVE_NAME\", value: \"Jon\"}, {name: \"FAMILY_NAME\", value: \"Doe\"}], [{name: \"CITY\", value: \"Munich\"}, {name: \"ZIP\", value: \"81456\"}]]\n should allow the receiver to render:\n Jon Doe\n Munich 81456"]
pub struct AddressUi {
    #[doc = "A two dimension array containing the representation of values from an address profile."]
    pub address_fields: Vec<AddressFields>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FilledField {
    #[serde(default)]
    #[doc = "The type of the field, e.g text, password etc."]
    pub html_type: String,
    #[serde(default)]
    #[doc = "the html id"]
    pub id: String,
    #[serde(default)]
    #[doc = "the html name"]
    pub name: String,
    #[serde(default)]
    #[doc = "the field value"]
    pub value: String,
    #[serde(default)]
    #[doc = "The actual field type, e.g FAMILY_NAME"]
    pub autofill_type: String,
    #[doc = "The filling strategy"]
    pub filling_strategy: FillingStrategy,
    #[doc = "The frame the field belongs to"]
    pub frame_id: page::FrameId,
    #[doc = "The form field's DOM node"]
    pub field_id: dom::BackendNodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Trigger autofill on a form identified by the fieldId.\n If the field and related form cannot be autofilled, returns an error."]
pub struct Trigger {
    #[doc = "Identifies a field that serves as an anchor for autofill."]
    pub field_id: dom::BackendNodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifies the frame that field belongs to."]
    pub frame_id: Option<page::FrameId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with `address`."]
    pub card: Option<CreditCard>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Address to fill out the form. Address data is not saved. Mutually exclusive with `card`."]
    pub address: Option<Address>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set addresses so that developers can verify their forms implementation."]
pub struct SetAddresses {
    pub addresses: Vec<Address>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Trigger autofill on a form identified by the fieldId.\n If the field and related form cannot be autofilled, returns an error."]
pub struct TriggerReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set addresses so that developers can verify their forms implementation."]
pub struct SetAddressesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables autofill domain notifications."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables autofill domain notifications."]
pub struct EnableReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AddressFormFilledEvent {
        pub params: AddressFormFilledEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AddressFormFilledEventParams {
        #[doc = "Information about the fields that were filled"]
        pub filled_fields: Vec<super::FilledField>,
        #[doc = "An UI representation of the address used to fill the form.\n Consists of a 2D array where each child represents an address/profile line."]
        pub address_ui: super::AddressUi,
    }
}
