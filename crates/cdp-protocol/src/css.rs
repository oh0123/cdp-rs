// Auto-generated from Chrome at version 140.0.7339.186 domain: CSS
use super::dom;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type StyleSheetId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StyleSheetOrigin {
    #[serde(rename = "injected")]
    Injected,
    #[serde(rename = "user-agent")]
    UserAgent,
    #[serde(rename = "inspector")]
    Inspector,
    #[serde(rename = "regular")]
    Regular,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CssRuleType {
    #[serde(rename = "MediaRule")]
    MediaRule,
    #[serde(rename = "SupportsRule")]
    SupportsRule,
    #[serde(rename = "ContainerRule")]
    ContainerRule,
    #[serde(rename = "LayerRule")]
    LayerRule,
    #[serde(rename = "ScopeRule")]
    ScopeRule,
    #[serde(rename = "StyleRule")]
    StyleRule,
    #[serde(rename = "StartingStyleRule")]
    StartingStyleRule,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CssMediaSource {
    #[serde(rename = "mediaRule")]
    MediaRule,
    #[serde(rename = "importRule")]
    ImportRule,
    #[serde(rename = "linkedSheet")]
    LinkedSheet,
    #[serde(rename = "inlineSheet")]
    InlineSheet,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PseudoElementMatches {
    #[serde(rename = "pseudoType")]
    pub pseudo_type: dom::PseudoType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pseudoIdentifier")]
    pub pseudo_identifier: Option<String>,
    #[serde(rename = "matches")]
    pub matches: Vec<RuleMatch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssAnimationStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "style")]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InheritedStyleEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inlineStyle")]
    pub inline_style: Option<CssStyle>,
    #[serde(rename = "matchedCSSRules")]
    pub matched_css_rules: Vec<RuleMatch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InheritedAnimatedStyleEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "animationStyles")]
    pub animation_styles: Option<Vec<CssAnimationStyle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transitionsStyle")]
    pub transitions_style: Option<CssStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InheritedPseudoElementMatches {
    #[serde(rename = "pseudoElements")]
    pub pseudo_elements: Vec<PseudoElementMatches>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RuleMatch {
    #[serde(rename = "rule")]
    pub rule: CssRule,
    #[serde(default)]
    #[serde(rename = "matchingSelectors")]
    pub matching_selectors: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Value {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "specificity")]
    pub specificity: Option<Specificity>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Specificity {
    #[serde(default)]
    #[serde(rename = "a")]
    pub a: JsUInt,
    #[serde(default)]
    #[serde(rename = "b")]
    pub b: JsUInt,
    #[serde(default)]
    #[serde(rename = "c")]
    pub c: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SelectorList {
    #[serde(rename = "selectors")]
    pub selectors: Vec<Value>,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssStyleSheetHeader {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
    #[serde(default)]
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sourceMapURL")]
    pub source_map_url: Option<String>,
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[serde(default)]
    #[serde(rename = "title")]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ownerNode")]
    pub owner_node: Option<dom::BackendNodeId>,
    #[serde(default)]
    #[serde(rename = "disabled")]
    pub disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hasSourceURL")]
    pub has_source_url: Option<bool>,
    #[serde(default)]
    #[serde(rename = "isInline")]
    pub is_inline: bool,
    #[serde(default)]
    #[serde(rename = "isMutable")]
    pub is_mutable: bool,
    #[serde(default)]
    #[serde(rename = "isConstructed")]
    pub is_constructed: bool,
    #[serde(default)]
    #[serde(rename = "startLine")]
    pub start_line: JsFloat,
    #[serde(default)]
    #[serde(rename = "startColumn")]
    pub start_column: JsFloat,
    #[serde(default)]
    #[serde(rename = "length")]
    pub length: JsFloat,
    #[serde(default)]
    #[serde(rename = "endLine")]
    pub end_line: JsFloat,
    #[serde(default)]
    #[serde(rename = "endColumn")]
    pub end_column: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "loadingFailed")]
    pub loading_failed: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(rename = "selectorList")]
    pub selector_list: SelectorList,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "nestingSelectors")]
    pub nesting_selectors: Option<Vec<String>>,
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[serde(rename = "style")]
    pub style: CssStyle,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "media")]
    pub media: Option<Vec<CssMedia>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerQueries")]
    pub container_queries: Option<Vec<CssContainerQuery>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supports")]
    pub supports: Option<Vec<CssSupports>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "layers")]
    pub layers: Option<Vec<CssLayer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<CssScope>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ruleTypes")]
    pub rule_types: Option<Vec<CssRuleType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startingStyles")]
    pub starting_styles: Option<Vec<CssStartingStyle>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RuleUsage {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(default)]
    #[serde(rename = "startOffset")]
    pub start_offset: JsFloat,
    #[serde(default)]
    #[serde(rename = "endOffset")]
    pub end_offset: JsFloat,
    #[serde(default)]
    #[serde(rename = "used")]
    pub used: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SourceRange {
    #[serde(default)]
    #[serde(rename = "startLine")]
    pub start_line: JsUInt,
    #[serde(default)]
    #[serde(rename = "startColumn")]
    pub start_column: JsUInt,
    #[serde(default)]
    #[serde(rename = "endLine")]
    pub end_line: JsUInt,
    #[serde(default)]
    #[serde(rename = "endColumn")]
    pub end_column: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShorthandEntry {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "important")]
    pub important: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssComputedStyleProperty {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(rename = "cssProperties")]
    pub css_properties: Vec<CssProperty>,
    #[serde(rename = "shorthandEntries")]
    pub shorthand_entries: Vec<ShorthandEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "cssText")]
    pub css_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssProperty {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "important")]
    pub important: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "implicit")]
    pub implicit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "parsedOk")]
    pub parsed_ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "disabled")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "longhandProperties")]
    pub longhand_properties: Option<Vec<CssProperty>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssMedia {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "source")]
    pub source: CssMediaSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sourceURL")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mediaList")]
    pub media_list: Option<Vec<MediaQuery>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MediaQuery {
    #[serde(rename = "expressions")]
    pub expressions: Vec<MediaQueryExpression>,
    #[serde(default)]
    #[serde(rename = "active")]
    pub active: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MediaQueryExpression {
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: JsFloat,
    #[serde(default)]
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(default)]
    #[serde(rename = "feature")]
    pub feature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueRange")]
    pub value_range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "computedLength")]
    pub computed_length: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssContainerQuery {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "physicalAxes")]
    pub physical_axes: Option<dom::PhysicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logicalAxes")]
    pub logical_axes: Option<dom::LogicalAxes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "queriesScrollState")]
    pub queries_scroll_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "queriesAnchored")]
    pub queries_anchored: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssSupports {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(default)]
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssScope {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssLayer {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssStartingStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssLayerData {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subLayers")]
    pub sub_layers: Option<Vec<CssLayerData>>,
    #[serde(default)]
    #[serde(rename = "order")]
    pub order: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PlatformFontUsage {
    #[serde(default)]
    #[serde(rename = "familyName")]
    pub family_name: String,
    #[serde(default)]
    #[serde(rename = "postScriptName")]
    pub post_script_name: String,
    #[serde(default)]
    #[serde(rename = "isCustomFont")]
    pub is_custom_font: bool,
    #[serde(default)]
    #[serde(rename = "glyphCount")]
    pub glyph_count: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FontVariationAxis {
    #[serde(default)]
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "minValue")]
    pub min_value: JsFloat,
    #[serde(default)]
    #[serde(rename = "maxValue")]
    pub max_value: JsFloat,
    #[serde(default)]
    #[serde(rename = "defaultValue")]
    pub default_value: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FontFace {
    #[serde(default)]
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(default)]
    #[serde(rename = "fontStyle")]
    pub font_style: String,
    #[serde(default)]
    #[serde(rename = "fontVariant")]
    pub font_variant: String,
    #[serde(default)]
    #[serde(rename = "fontWeight")]
    pub font_weight: String,
    #[serde(default)]
    #[serde(rename = "fontStretch")]
    pub font_stretch: String,
    #[serde(default)]
    #[serde(rename = "fontDisplay")]
    pub font_display: String,
    #[serde(default)]
    #[serde(rename = "unicodeRange")]
    pub unicode_range: String,
    #[serde(default)]
    #[serde(rename = "src")]
    pub src: String,
    #[serde(default)]
    #[serde(rename = "platformFontFamily")]
    pub platform_font_family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontVariationAxes")]
    pub font_variation_axes: Option<Vec<FontVariationAxis>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssTryRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[serde(rename = "style")]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssPositionTryRule {
    #[serde(rename = "name")]
    pub name: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[serde(rename = "style")]
    pub style: CssStyle,
    #[serde(default)]
    #[serde(rename = "active")]
    pub active: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssKeyframesRule {
    #[serde(rename = "animationName")]
    pub animation_name: Value,
    #[serde(rename = "keyframes")]
    pub keyframes: Vec<CssKeyframeRule>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssPropertyRegistration {
    #[serde(default)]
    #[serde(rename = "propertyName")]
    pub property_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initialValue")]
    pub initial_value: Option<Value>,
    #[serde(default)]
    #[serde(rename = "inherits")]
    pub inherits: bool,
    #[serde(default)]
    #[serde(rename = "syntax")]
    pub syntax: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssFontPaletteValuesRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[serde(rename = "fontPaletteName")]
    pub font_palette_name: Value,
    #[serde(rename = "style")]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssPropertyRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[serde(rename = "propertyName")]
    pub property_name: Value,
    #[serde(rename = "style")]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssFunctionParameter {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssFunctionConditionNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "media")]
    pub media: Option<CssMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerQueries")]
    pub container_queries: Option<CssContainerQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supports")]
    pub supports: Option<CssSupports>,
    #[serde(rename = "children")]
    pub children: Vec<CssFunctionNode>,
    #[serde(default)]
    #[serde(rename = "conditionText")]
    pub condition_text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssFunctionNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "condition")]
    pub condition: Option<CssFunctionConditionNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "style")]
    pub style: Option<CssStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssFunctionRule {
    #[serde(rename = "name")]
    pub name: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[serde(rename = "parameters")]
    pub parameters: Vec<CssFunctionParameter>,
    #[serde(rename = "children")]
    pub children: Vec<CssFunctionNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CssKeyframeRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: Option<StyleSheetId>,
    #[serde(rename = "origin")]
    pub origin: StyleSheetOrigin,
    #[serde(rename = "keyText")]
    pub key_text: Value,
    #[serde(rename = "style")]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StyleDeclarationEdit {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddRule {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(default)]
    #[serde(rename = "ruleText")]
    pub rule_text: String,
    #[serde(rename = "location")]
    pub location: SourceRange,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeForPropertySyntaxValidation")]
    pub node_for_property_syntax_validation: Option<dom::NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CollectClassNames {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateStyleSheet {
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "force")]
    pub force: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ForcePseudoState {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
    #[serde(default)]
    #[serde(rename = "forcedPseudoClasses")]
    pub forced_pseudo_classes: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ForceStartingStyle {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
    #[serde(default)]
    #[serde(rename = "forced")]
    pub forced: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBackgroundColors {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetComputedStyleForNode {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolveValues {
    #[serde(default)]
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "propertyName")]
    pub property_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pseudoType")]
    pub pseudo_type: Option<dom::PseudoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "pseudoIdentifier")]
    pub pseudo_identifier: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetLonghandProperties {
    #[serde(default)]
    #[serde(rename = "shorthandName")]
    pub shorthand_name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetInlineStylesForNode {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAnimatedStylesForNode {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetMatchedStylesForNode {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetEnvironmentVariables(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMediaQueries(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPlatformFontsForNode {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetStyleSheetText {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetLayersForNode {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetLocationForSelector {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(default)]
    #[serde(rename = "selectorText")]
    pub selector_text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrackComputedStyleUpdatesForNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<dom::NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrackComputedStyleUpdates {
    #[serde(rename = "propertiesToTrack")]
    pub properties_to_track: Vec<CssComputedStyleProperty>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakeComputedStyleUpdates(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetEffectivePropertyValueForNode {
    #[serde(rename = "nodeId")]
    pub node_id: dom::NodeId,
    #[serde(default)]
    #[serde(rename = "propertyName")]
    pub property_name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPropertyRulePropertyName {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[serde(default)]
    #[serde(rename = "propertyName")]
    pub property_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetKeyframeKey {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[serde(default)]
    #[serde(rename = "keyText")]
    pub key_text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetMediaText {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetContainerQueryText {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSupportsText {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetScopeText {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetRuleSelector {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(rename = "range")]
    pub range: SourceRange,
    #[serde(default)]
    #[serde(rename = "selector")]
    pub selector: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetStyleSheetText {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetStyleTexts {
    #[serde(rename = "edits")]
    pub edits: Vec<StyleDeclarationEdit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeForPropertySyntaxValidation")]
    pub node_for_property_syntax_validation: Option<dom::NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartRuleUsageTracking(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopRuleUsageTracking(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TakeCoverageDelta(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetLocalFontsEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddRuleReturnObject {
    #[serde(rename = "rule")]
    pub rule: CssRule,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CollectClassNamesReturnObject {
    #[serde(rename = "classNames")]
    pub class_names: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateStyleSheetReturnObject {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: StyleSheetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ForcePseudoStateReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ForceStartingStyleReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetBackgroundColorsReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backgroundColors")]
    pub background_colors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "computedFontSize")]
    pub computed_font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "computedFontWeight")]
    pub computed_font_weight: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetComputedStyleForNodeReturnObject {
    #[serde(rename = "computedStyle")]
    pub computed_style: Vec<CssComputedStyleProperty>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolveValuesReturnObject {
    #[serde(rename = "results")]
    pub results: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetLonghandPropertiesReturnObject {
    #[serde(rename = "longhandProperties")]
    pub longhand_properties: Vec<CssProperty>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetInlineStylesForNodeReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inlineStyle")]
    pub inline_style: Option<CssStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributesStyle")]
    pub attributes_style: Option<CssStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAnimatedStylesForNodeReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "animationStyles")]
    pub animation_styles: Option<Vec<CssAnimationStyle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transitionsStyle")]
    pub transitions_style: Option<CssStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inherited")]
    pub inherited: Option<Vec<InheritedAnimatedStyleEntry>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetMatchedStylesForNodeReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inlineStyle")]
    pub inline_style: Option<CssStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributesStyle")]
    pub attributes_style: Option<CssStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "matchedCSSRules")]
    pub matched_css_rules: Option<Vec<RuleMatch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pseudoElements")]
    pub pseudo_elements: Option<Vec<PseudoElementMatches>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inherited")]
    pub inherited: Option<Vec<InheritedStyleEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inheritedPseudoElements")]
    pub inherited_pseudo_elements: Option<Vec<InheritedPseudoElementMatches>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cssKeyframesRules")]
    pub css_keyframes_rules: Option<Vec<CssKeyframesRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cssPositionTryRules")]
    pub css_position_try_rules: Option<Vec<CssPositionTryRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "activePositionFallbackIndex")]
    pub active_position_fallback_index: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cssPropertyRules")]
    pub css_property_rules: Option<Vec<CssPropertyRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cssPropertyRegistrations")]
    pub css_property_registrations: Option<Vec<CssPropertyRegistration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cssFontPaletteValuesRule")]
    pub css_font_palette_values_rule: Option<CssFontPaletteValuesRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentLayoutNodeId")]
    pub parent_layout_node_id: Option<dom::NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cssFunctionRules")]
    pub css_function_rules: Option<Vec<CssFunctionRule>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetEnvironmentVariablesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetMediaQueriesReturnObject {
    #[serde(rename = "medias")]
    pub medias: Vec<CssMedia>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPlatformFontsForNodeReturnObject {
    #[serde(rename = "fonts")]
    pub fonts: Vec<PlatformFontUsage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetStyleSheetTextReturnObject {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetLayersForNodeReturnObject {
    #[serde(rename = "rootLayer")]
    pub root_layer: CssLayerData,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetLocationForSelectorReturnObject {
    #[serde(rename = "ranges")]
    pub ranges: Vec<SourceRange>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrackComputedStyleUpdatesForNodeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrackComputedStyleUpdatesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeComputedStyleUpdatesReturnObject {
    #[serde(rename = "nodeIds")]
    pub node_ids: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetEffectivePropertyValueForNodeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPropertyRulePropertyNameReturnObject {
    #[serde(rename = "propertyName")]
    pub property_name: Value,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetKeyframeKeyReturnObject {
    #[serde(rename = "keyText")]
    pub key_text: Value,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetMediaTextReturnObject {
    #[serde(rename = "media")]
    pub media: CssMedia,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetContainerQueryTextReturnObject {
    #[serde(rename = "containerQuery")]
    pub container_query: CssContainerQuery,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSupportsTextReturnObject {
    #[serde(rename = "supports")]
    pub supports: CssSupports,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetScopeTextReturnObject {
    #[serde(rename = "scope")]
    pub scope: CssScope,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetRuleSelectorReturnObject {
    #[serde(rename = "selectorList")]
    pub selector_list: SelectorList,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetStyleSheetTextReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "sourceMapURL")]
    pub source_map_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetStyleTextsReturnObject {
    #[serde(rename = "styles")]
    pub styles: Vec<CssStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartRuleUsageTrackingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopRuleUsageTrackingReturnObject {
    #[serde(rename = "ruleUsage")]
    pub rule_usage: Vec<RuleUsage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeCoverageDeltaReturnObject {
    #[serde(rename = "coverage")]
    pub coverage: Vec<RuleUsage>,
    #[serde(default)]
    #[serde(rename = "timestamp")]
    pub timestamp: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetLocalFontsEnabledReturnObject {}
impl Method for AddRule {
    const NAME: &'static str = "CSS.addRule";
    type ReturnObject = AddRuleReturnObject;
}
impl Method for CollectClassNames {
    const NAME: &'static str = "CSS.collectClassNames";
    type ReturnObject = CollectClassNamesReturnObject;
}
impl Method for CreateStyleSheet {
    const NAME: &'static str = "CSS.createStyleSheet";
    type ReturnObject = CreateStyleSheetReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "CSS.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "CSS.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for ForcePseudoState {
    const NAME: &'static str = "CSS.forcePseudoState";
    type ReturnObject = ForcePseudoStateReturnObject;
}
impl Method for ForceStartingStyle {
    const NAME: &'static str = "CSS.forceStartingStyle";
    type ReturnObject = ForceStartingStyleReturnObject;
}
impl Method for GetBackgroundColors {
    const NAME: &'static str = "CSS.getBackgroundColors";
    type ReturnObject = GetBackgroundColorsReturnObject;
}
impl Method for GetComputedStyleForNode {
    const NAME: &'static str = "CSS.getComputedStyleForNode";
    type ReturnObject = GetComputedStyleForNodeReturnObject;
}
impl Method for ResolveValues {
    const NAME: &'static str = "CSS.resolveValues";
    type ReturnObject = ResolveValuesReturnObject;
}
impl Method for GetLonghandProperties {
    const NAME: &'static str = "CSS.getLonghandProperties";
    type ReturnObject = GetLonghandPropertiesReturnObject;
}
impl Method for GetInlineStylesForNode {
    const NAME: &'static str = "CSS.getInlineStylesForNode";
    type ReturnObject = GetInlineStylesForNodeReturnObject;
}
impl Method for GetAnimatedStylesForNode {
    const NAME: &'static str = "CSS.getAnimatedStylesForNode";
    type ReturnObject = GetAnimatedStylesForNodeReturnObject;
}
impl Method for GetMatchedStylesForNode {
    const NAME: &'static str = "CSS.getMatchedStylesForNode";
    type ReturnObject = GetMatchedStylesForNodeReturnObject;
}
impl Method for GetEnvironmentVariables {
    const NAME: &'static str = "CSS.getEnvironmentVariables";
    type ReturnObject = GetEnvironmentVariablesReturnObject;
}
impl Method for GetMediaQueries {
    const NAME: &'static str = "CSS.getMediaQueries";
    type ReturnObject = GetMediaQueriesReturnObject;
}
impl Method for GetPlatformFontsForNode {
    const NAME: &'static str = "CSS.getPlatformFontsForNode";
    type ReturnObject = GetPlatformFontsForNodeReturnObject;
}
impl Method for GetStyleSheetText {
    const NAME: &'static str = "CSS.getStyleSheetText";
    type ReturnObject = GetStyleSheetTextReturnObject;
}
impl Method for GetLayersForNode {
    const NAME: &'static str = "CSS.getLayersForNode";
    type ReturnObject = GetLayersForNodeReturnObject;
}
impl Method for GetLocationForSelector {
    const NAME: &'static str = "CSS.getLocationForSelector";
    type ReturnObject = GetLocationForSelectorReturnObject;
}
impl Method for TrackComputedStyleUpdatesForNode {
    const NAME: &'static str = "CSS.trackComputedStyleUpdatesForNode";
    type ReturnObject = TrackComputedStyleUpdatesForNodeReturnObject;
}
impl Method for TrackComputedStyleUpdates {
    const NAME: &'static str = "CSS.trackComputedStyleUpdates";
    type ReturnObject = TrackComputedStyleUpdatesReturnObject;
}
impl Method for TakeComputedStyleUpdates {
    const NAME: &'static str = "CSS.takeComputedStyleUpdates";
    type ReturnObject = TakeComputedStyleUpdatesReturnObject;
}
impl Method for SetEffectivePropertyValueForNode {
    const NAME: &'static str = "CSS.setEffectivePropertyValueForNode";
    type ReturnObject = SetEffectivePropertyValueForNodeReturnObject;
}
impl Method for SetPropertyRulePropertyName {
    const NAME: &'static str = "CSS.setPropertyRulePropertyName";
    type ReturnObject = SetPropertyRulePropertyNameReturnObject;
}
impl Method for SetKeyframeKey {
    const NAME: &'static str = "CSS.setKeyframeKey";
    type ReturnObject = SetKeyframeKeyReturnObject;
}
impl Method for SetMediaText {
    const NAME: &'static str = "CSS.setMediaText";
    type ReturnObject = SetMediaTextReturnObject;
}
impl Method for SetContainerQueryText {
    const NAME: &'static str = "CSS.setContainerQueryText";
    type ReturnObject = SetContainerQueryTextReturnObject;
}
impl Method for SetSupportsText {
    const NAME: &'static str = "CSS.setSupportsText";
    type ReturnObject = SetSupportsTextReturnObject;
}
impl Method for SetScopeText {
    const NAME: &'static str = "CSS.setScopeText";
    type ReturnObject = SetScopeTextReturnObject;
}
impl Method for SetRuleSelector {
    const NAME: &'static str = "CSS.setRuleSelector";
    type ReturnObject = SetRuleSelectorReturnObject;
}
impl Method for SetStyleSheetText {
    const NAME: &'static str = "CSS.setStyleSheetText";
    type ReturnObject = SetStyleSheetTextReturnObject;
}
impl Method for SetStyleTexts {
    const NAME: &'static str = "CSS.setStyleTexts";
    type ReturnObject = SetStyleTextsReturnObject;
}
impl Method for StartRuleUsageTracking {
    const NAME: &'static str = "CSS.startRuleUsageTracking";
    type ReturnObject = StartRuleUsageTrackingReturnObject;
}
impl Method for StopRuleUsageTracking {
    const NAME: &'static str = "CSS.stopRuleUsageTracking";
    type ReturnObject = StopRuleUsageTrackingReturnObject;
}
impl Method for TakeCoverageDelta {
    const NAME: &'static str = "CSS.takeCoverageDelta";
    type ReturnObject = TakeCoverageDeltaReturnObject;
}
impl Method for SetLocalFontsEnabled {
    const NAME: &'static str = "CSS.setLocalFontsEnabled";
    type ReturnObject = SetLocalFontsEnabledReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FontsUpdatedEvent {
        pub params: FontsUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FontsUpdatedEventParams {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "font")]
        pub font: Option<super::FontFace>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct MediaQueryResultChangedEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetAddedEvent {
        pub params: StyleSheetAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetAddedEventParams {
        #[serde(rename = "header")]
        pub header: super::CssStyleSheetHeader,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetChangedEvent {
        pub params: StyleSheetChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetChangedEventParams {
        #[serde(rename = "styleSheetId")]
        pub style_sheet_id: super::StyleSheetId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetRemovedEvent {
        pub params: StyleSheetRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetRemovedEventParams {
        #[serde(rename = "styleSheetId")]
        pub style_sheet_id: super::StyleSheetId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ComputedStyleUpdatedEvent {
        pub params: ComputedStyleUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ComputedStyleUpdatedEventParams {
        #[serde(rename = "nodeId")]
        pub node_id: super::super::dom::NodeId,
    }
}
