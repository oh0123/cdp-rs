// Auto-generated from Chrome at version 146.0.7680.165 domain: CSS
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
pub enum CssAtRuleType {
    #[serde(rename = "font-face")]
    FontFace,
    #[serde(rename = "font-feature-values")]
    FontFeatureValues,
    #[serde(rename = "font-palette-values")]
    FontPaletteValues,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CssAtRuleSubsection {
    #[serde(rename = "swash")]
    Swash,
    #[serde(rename = "annotation")]
    Annotation,
    #[serde(rename = "ornaments")]
    Ornaments,
    #[serde(rename = "stylistic")]
    Stylistic,
    #[serde(rename = "styleset")]
    Styleset,
    #[serde(rename = "character-variant")]
    CharacterVariant,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS rule collection for a single pseudo style."]
pub struct PseudoElementMatches {
    #[doc = "Pseudo element type."]
    pub pseudo_type: dom::PseudoType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Pseudo element custom ident."]
    pub pseudo_identifier: Option<String>,
    #[doc = "Matches of CSS rules applicable to the pseudo style."]
    pub matches: Vec<RuleMatch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS style coming from animations with the name of the animation."]
pub struct CssAnimationStyle {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The name of the animation."]
    pub name: Option<String>,
    #[doc = "The style coming from the animation."]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Inherited CSS rule collection from ancestor node."]
pub struct InheritedStyleEntry {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The ancestor node's inline style, if any, in the style inheritance chain."]
    pub inline_style: Option<CssStyle>,
    #[doc = "Matches of CSS rules matching the ancestor node in the style inheritance chain."]
    #[serde(rename = "matchedCSSRules")]
    pub matched_css_rules: Vec<RuleMatch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Inherited CSS style collection for animated styles from ancestor node."]
pub struct InheritedAnimatedStyleEntry {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Styles coming from the animations of the ancestor, if any, in the style inheritance chain."]
    pub animation_styles: Option<Vec<CssAnimationStyle>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The style coming from the transitions of the ancestor, if any, in the style inheritance chain."]
    pub transitions_style: Option<CssStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Inherited pseudo element matches from pseudos of an ancestor node."]
pub struct InheritedPseudoElementMatches {
    #[doc = "Matches of pseudo styles from the pseudos of an ancestor node."]
    pub pseudo_elements: Vec<PseudoElementMatches>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Match data for a CSS rule."]
pub struct RuleMatch {
    #[doc = "CSS rule in the match."]
    pub rule: CssRule,
    #[serde(default)]
    #[doc = "Matching selector indices in the rule's selectorList selectors (0-based)."]
    pub matching_selectors: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Data for a simple selector (these are delimited by commas in a selector list)."]
pub struct Value {
    #[serde(default)]
    #[doc = "Value text."]
    pub text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Value range in the underlying resource (if available)."]
    pub range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specificity of the selector."]
    pub specificity: Option<Specificity>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Specificity:\n https://drafts.csswg.org/selectors/#specificity-rules"]
pub struct Specificity {
    #[serde(default)]
    #[doc = "The a component, which represents the number of ID selectors."]
    pub a: JsUInt,
    #[serde(default)]
    #[doc = "The b component, which represents the number of class selectors, attributes selectors, and\n pseudo-classes."]
    pub b: JsUInt,
    #[serde(default)]
    #[doc = "The c component, which represents the number of type selectors and pseudo-elements."]
    pub c: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Selector list data."]
pub struct SelectorList {
    #[doc = "Selectors in the list."]
    pub selectors: Vec<Value>,
    #[serde(default)]
    #[doc = "Rule selector text."]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS stylesheet metainformation."]
pub struct CssStyleSheetHeader {
    #[doc = "The stylesheet identifier."]
    pub style_sheet_id: dom::StyleSheetId,
    #[doc = "Owner frame identifier."]
    pub frame_id: page::FrameId,
    #[serde(default)]
    #[doc = "Stylesheet resource URL. Empty if this is a constructed stylesheet created using\n new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported\n as a CSS module script)."]
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL of source map associated with the stylesheet (if any)."]
    #[serde(rename = "sourceMapURL")]
    pub source_map_url: Option<String>,
    #[doc = "Stylesheet origin."]
    pub origin: StyleSheetOrigin,
    #[serde(default)]
    #[doc = "Stylesheet title."]
    pub title: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The backend id for the owner node of the stylesheet."]
    pub owner_node: Option<dom::BackendNodeId>,
    #[serde(default)]
    #[doc = "Denotes whether the stylesheet is disabled."]
    pub disabled: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the sourceURL field value comes from the sourceURL comment."]
    #[serde(rename = "hasSourceURL")]
    pub has_source_url: Option<bool>,
    #[serde(default)]
    #[doc = "Whether this stylesheet is created for STYLE tag by parser. This flag is not set for\n document.written STYLE tags."]
    pub is_inline: bool,
    #[serde(default)]
    #[doc = "Whether this stylesheet is mutable. Inline stylesheets become mutable\n after they have been modified via CSSOM API.\n `<link>` element's stylesheets become mutable only if DevTools modifies them.\n Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation."]
    pub is_mutable: bool,
    #[serde(default)]
    #[doc = "True if this stylesheet is created through new CSSStyleSheet() or imported as a\n CSS module script."]
    pub is_constructed: bool,
    #[serde(default)]
    #[doc = "Line offset of the stylesheet within the resource (zero based)."]
    pub start_line: JsFloat,
    #[serde(default)]
    #[doc = "Column offset of the stylesheet within the resource (zero based)."]
    pub start_column: JsFloat,
    #[serde(default)]
    #[doc = "Size of the content (in characters)."]
    pub length: JsFloat,
    #[serde(default)]
    #[doc = "Line offset of the end of the stylesheet within the resource (zero based)."]
    pub end_line: JsFloat,
    #[serde(default)]
    #[doc = "Column offset of the end of the stylesheet within the resource (zero based)."]
    pub end_column: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If the style sheet was loaded from a network resource, this indicates when the resource failed to load"]
    pub loading_failed: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS rule representation."]
pub struct CssRule {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[doc = "Rule selector data."]
    pub selector_list: SelectorList,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Array of selectors from ancestor style rules, sorted by distance from the current rule."]
    pub nesting_selectors: Option<Vec<String>>,
    #[doc = "Parent stylesheet's origin."]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated style declaration."]
    pub style: CssStyle,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule."]
    pub origin_tree_scope_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Media list array (for rules involving media queries). The array enumerates media queries\n starting with the innermost one, going outwards."]
    pub media: Option<Vec<CssMedia>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Container query list array (for rules involving container queries).\n The array enumerates container queries starting with the innermost one, going outwards."]
    pub container_queries: Option<Vec<CssContainerQuery>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "@supports CSS at-rule array.\n The array enumerates @supports at-rules starting with the innermost one, going outwards."]
    pub supports: Option<Vec<CssSupports>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Cascade layer array. Contains the layer hierarchy that this rule belongs to starting\n with the innermost layer and going outwards."]
    pub layers: Option<Vec<CssLayer>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "@scope CSS at-rule array.\n The array enumerates @scope at-rules starting with the innermost one, going outwards."]
    pub scopes: Option<Vec<CssScope>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The array keeps the types of ancestor CSSRules from the innermost going outwards."]
    pub rule_types: Option<Vec<CssRuleType>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "@starting-style CSS at-rule array.\n The array enumerates @starting-style at-rules starting with the innermost one, going outwards."]
    pub starting_styles: Option<Vec<CssStartingStyle>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS coverage information."]
pub struct RuleUsage {
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: dom::StyleSheetId,
    #[serde(default)]
    #[doc = "Offset of the start of the rule (including selector) from the beginning of the stylesheet."]
    pub start_offset: JsFloat,
    #[serde(default)]
    #[doc = "Offset of the end of the rule body from the beginning of the stylesheet."]
    pub end_offset: JsFloat,
    #[serde(default)]
    #[doc = "Indicates whether the rule was actually used by some element in the page."]
    pub used: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Text range within a resource. All numbers are zero-based."]
pub struct SourceRange {
    #[serde(default)]
    #[doc = "Start line of range."]
    pub start_line: JsUInt,
    #[serde(default)]
    #[doc = "Start column of range (inclusive)."]
    pub start_column: JsUInt,
    #[serde(default)]
    #[doc = "End line of range"]
    pub end_line: JsUInt,
    #[serde(default)]
    #[doc = "End column of range (exclusive)."]
    pub end_column: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ShorthandEntry {
    #[serde(default)]
    #[doc = "Shorthand name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Shorthand value."]
    pub value: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the property has \"!important\" annotation (implies `false` if absent)."]
    pub important: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct CssComputedStyleProperty {
    #[serde(default)]
    #[doc = "Computed style property name."]
    pub name: String,
    #[serde(default)]
    #[doc = "Computed style property value."]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyleExtraFields {
    #[serde(default)]
    #[doc = "Returns whether or not this node is being rendered with base appearance,\n which happens when it has its appearance property set to base/base-select\n or it is in the subtree of an element being rendered with base appearance."]
    pub is_appearance_base: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS style representation."]
pub struct CssStyle {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[doc = "CSS properties in the style."]
    pub css_properties: Vec<CssProperty>,
    #[doc = "Computed values for all shorthands found in the style."]
    pub shorthand_entries: Vec<ShorthandEntry>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Style declaration text (if available)."]
    pub css_text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style declaration range in the enclosing stylesheet (if available)."]
    pub range: Option<SourceRange>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS property declaration data."]
pub struct CssProperty {
    #[serde(default)]
    #[doc = "The property name."]
    pub name: String,
    #[serde(default)]
    #[doc = "The property value."]
    pub value: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the property has \"!important\" annotation (implies `false` if absent)."]
    pub important: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the property is implicit (implies `false` if absent)."]
    pub implicit: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The full property text as specified in the style."]
    pub text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the property is understood by the browser (implies `true` if absent)."]
    pub parsed_ok: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the property is disabled by the user (present for source-based properties only)."]
    pub disabled: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The entire property range in the enclosing style declaration (if available)."]
    pub range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Parsed longhand components of this property if it is a shorthand.\n This field will be empty if the given property is not a shorthand."]
    pub longhand_properties: Option<Vec<CssProperty>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS media rule descriptor."]
pub struct CssMedia {
    #[serde(default)]
    #[doc = "Media query text."]
    pub text: String,
    #[doc = "Source of the media query: \"mediaRule\" if specified by a @media rule, \"importRule\" if\n specified by an @import rule, \"linkedSheet\" if specified by a \"media\" attribute in a linked\n stylesheet's LINK tag, \"inlineSheet\" if specified by a \"media\" attribute in an inline\n stylesheet's STYLE tag."]
    pub source: CssMediaSource,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL of the document containing the media query description."]
    #[serde(rename = "sourceURL")]
    pub source_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The associated rule (@media or @import) header range in the enclosing stylesheet (if\n available)."]
    pub range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Array of media queries."]
    pub media_list: Option<Vec<MediaQuery>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Media query descriptor."]
pub struct MediaQuery {
    #[doc = "Array of media query expressions."]
    pub expressions: Vec<MediaQueryExpression>,
    #[serde(default)]
    #[doc = "Whether the media query condition is satisfied."]
    pub active: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Media query expression descriptor."]
pub struct MediaQueryExpression {
    #[serde(default)]
    #[doc = "Media query expression value."]
    pub value: JsFloat,
    #[serde(default)]
    #[doc = "Media query expression units."]
    pub unit: String,
    #[serde(default)]
    #[doc = "Media query expression feature."]
    pub feature: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The associated range of the value text in the enclosing stylesheet (if available)."]
    pub value_range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Computed length of media query expression (if applicable)."]
    pub computed_length: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS container query rule descriptor."]
pub struct CssContainerQuery {
    #[serde(default)]
    #[doc = "Container query text."]
    pub text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The associated rule header range in the enclosing stylesheet (if\n available)."]
    pub range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Optional name for the container."]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Optional physical axes queried for the container."]
    pub physical_axes: Option<dom::PhysicalAxes>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Optional logical axes queried for the container."]
    pub logical_axes: Option<dom::LogicalAxes>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "true if the query contains scroll-state() queries."]
    pub queries_scroll_state: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "true if the query contains anchored() queries."]
    pub queries_anchored: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS Supports at-rule descriptor."]
pub struct CssSupports {
    #[serde(default)]
    #[doc = "Supports rule text."]
    pub text: String,
    #[serde(default)]
    #[doc = "Whether the supports condition is satisfied."]
    pub active: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The associated rule header range in the enclosing stylesheet (if\n available)."]
    pub range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS Scope at-rule descriptor."]
pub struct CssScope {
    #[serde(default)]
    #[doc = "Scope rule text."]
    pub text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The associated rule header range in the enclosing stylesheet (if\n available)."]
    pub range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS Layer at-rule descriptor."]
pub struct CssLayer {
    #[serde(default)]
    #[doc = "Layer name."]
    pub text: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The associated rule header range in the enclosing stylesheet (if\n available)."]
    pub range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS Starting Style at-rule descriptor."]
pub struct CssStartingStyle {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The associated rule header range in the enclosing stylesheet (if\n available)."]
    pub range: Option<SourceRange>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the stylesheet containing this object (if exists)."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS Layer data."]
pub struct CssLayerData {
    #[serde(default)]
    #[doc = "Layer name."]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Direct sub-layers"]
    pub sub_layers: Option<Vec<CssLayerData>>,
    #[serde(default)]
    #[doc = "Layer order. The order determines the order of the layer in the cascade order.\n A higher number has higher priority in the cascade order."]
    pub order: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about amount of glyphs that were rendered with given font."]
pub struct PlatformFontUsage {
    #[serde(default)]
    #[doc = "Font's family name reported by platform."]
    pub family_name: String,
    #[serde(default)]
    #[doc = "Font's PostScript name reported by platform."]
    pub post_script_name: String,
    #[serde(default)]
    #[doc = "Indicates if the font was downloaded or resolved locally."]
    pub is_custom_font: bool,
    #[serde(default)]
    #[doc = "Amount of glyphs that were rendered with this font."]
    pub glyph_count: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about font variation axes for variable fonts"]
pub struct FontVariationAxis {
    #[serde(default)]
    #[doc = "The font-variation-setting tag (a.k.a. \"axis tag\")."]
    pub tag: String,
    #[serde(default)]
    #[doc = "Human-readable variation name in the default language (normally, \"en\")."]
    pub name: String,
    #[serde(default)]
    #[doc = "The minimum value (inclusive) the font supports for this tag."]
    pub min_value: JsFloat,
    #[serde(default)]
    #[doc = "The maximum value (inclusive) the font supports for this tag."]
    pub max_value: JsFloat,
    #[serde(default)]
    #[doc = "The default value."]
    pub default_value: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions\n and additional information such as platformFontFamily and fontVariationAxes."]
pub struct FontFace {
    #[serde(default)]
    #[doc = "The font-family."]
    pub font_family: String,
    #[serde(default)]
    #[doc = "The font-style."]
    pub font_style: String,
    #[serde(default)]
    #[doc = "The font-variant."]
    pub font_variant: String,
    #[serde(default)]
    #[doc = "The font-weight."]
    pub font_weight: String,
    #[serde(default)]
    #[doc = "The font-stretch."]
    pub font_stretch: String,
    #[serde(default)]
    #[doc = "The font-display."]
    pub font_display: String,
    #[serde(default)]
    #[doc = "The unicode-range."]
    pub unicode_range: String,
    #[serde(default)]
    #[doc = "The src."]
    pub src: String,
    #[serde(default)]
    #[doc = "The resolved platform font family"]
    pub platform_font_family: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Available variation settings (a.k.a. \"axes\")."]
    pub font_variation_axes: Option<Vec<FontVariationAxis>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS try rule representation."]
pub struct CssTryRule {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated style declaration."]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS @position-try rule representation."]
pub struct CssPositionTryRule {
    #[doc = "The prelude dashed-ident name"]
    pub name: Value,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated style declaration."]
    pub style: CssStyle,
    #[serde(default)]
    pub active: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS keyframes rule representation."]
pub struct CssKeyframesRule {
    #[doc = "Animation name."]
    pub animation_name: Value,
    #[doc = "List of keyframes."]
    pub keyframes: Vec<CssKeyframeRule>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Representation of a custom property registration through CSS.registerProperty"]
pub struct CssPropertyRegistration {
    #[serde(default)]
    pub property_name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<Value>,
    #[serde(default)]
    pub inherits: bool,
    #[serde(default)]
    pub syntax: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS generic @rule representation."]
pub struct CssAtRule {
    #[doc = "Type of at-rule."]
    pub r#type: CssAtRuleType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Subsection of font-feature-values, if this is a subsection."]
    pub subsection: Option<CssAtRuleSubsection>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)\n Associated name, if applicable."]
    pub name: Option<Value>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated style declaration."]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS property at-rule representation."]
pub struct CssPropertyRule {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated property name."]
    pub property_name: Value,
    #[doc = "Associated style declaration."]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS function argument representation."]
pub struct CssFunctionParameter {
    #[serde(default)]
    #[doc = "The parameter name."]
    pub name: String,
    #[serde(default)]
    #[doc = "The parameter type."]
    pub r#type: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS function conditional block representation."]
pub struct CssFunctionConditionNode {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Media query for this conditional block. Only one type of condition should be set."]
    pub media: Option<CssMedia>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Container query for this conditional block. Only one type of condition should be set."]
    pub container_queries: Option<CssContainerQuery>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "@supports CSS at-rule condition. Only one type of condition should be set."]
    pub supports: Option<CssSupports>,
    #[doc = "Block body."]
    pub children: Vec<CssFunctionNode>,
    #[serde(default)]
    #[doc = "The condition text."]
    pub condition_text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Section of the body of a CSS function rule."]
pub struct CssFunctionNode {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A conditional block. If set, style should not be set."]
    pub condition: Option<CssFunctionConditionNode>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Values set by this node. If set, condition should not be set."]
    pub style: Option<CssStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS function at-rule representation."]
pub struct CssFunctionRule {
    #[doc = "Name of the function."]
    pub name: Value,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    pub origin: StyleSheetOrigin,
    #[doc = "List of parameters."]
    pub parameters: Vec<CssFunctionParameter>,
    #[doc = "Function body."]
    pub children: Vec<CssFunctionNode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "CSS keyframe rule representation."]
pub struct CssKeyframeRule {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The css style sheet identifier (absent for user agent stylesheet and user-specified\n stylesheet rules) this rule came from."]
    pub style_sheet_id: Option<dom::StyleSheetId>,
    #[doc = "Parent stylesheet's origin."]
    pub origin: StyleSheetOrigin,
    #[doc = "Associated key text."]
    pub key_text: Value,
    #[doc = "Associated style declaration."]
    pub style: CssStyle,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A descriptor of operation to mutate style declaration text."]
pub struct StyleDeclarationEdit {
    #[doc = "The css style sheet identifier."]
    pub style_sheet_id: dom::StyleSheetId,
    #[doc = "The range of the style text in the enclosing stylesheet."]
    pub range: SourceRange,
    #[serde(default)]
    #[doc = "New style text."]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the\n position specified by `location`."]
pub struct AddRule {
    #[doc = "The css style sheet identifier where a new rule should be inserted."]
    pub style_sheet_id: dom::StyleSheetId,
    #[serde(default)]
    #[doc = "The text of a new rule."]
    pub rule_text: String,
    #[doc = "Text position of a new rule in the target style sheet."]
    pub location: SourceRange,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "NodeId for the DOM node in whose context custom property declarations for registered properties should be\n validated. If omitted, declarations in the new rule text can only be validated statically, which may produce\n incorrect results if the declaration contains a var() for example."]
    pub node_for_property_syntax_validation: Option<dom::NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all class names from specified stylesheet."]
pub struct CollectClassNames {
    pub style_sheet_id: dom::StyleSheetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Creates a new special \"via-inspector\" stylesheet in the frame with given `frameId`."]
pub struct CreateStyleSheet {
    #[doc = "Identifier of the frame where \"via-inspector\" stylesheet should be created."]
    pub frame_id: page::FrameId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, creates a new stylesheet for every call. If false,\n returns a stylesheet previously created by a call with force=false\n for the frame's document if it exists or creates a new stylesheet\n (default: false)."]
    pub force: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Ensures that the given node will have specified pseudo-classes whenever its style is computed by\n the browser."]
pub struct ForcePseudoState {
    #[doc = "The element id for which to force the pseudo state."]
    pub node_id: dom::NodeId,
    #[serde(default)]
    #[doc = "Element pseudo classes to force when computing the element's style."]
    pub forced_pseudo_classes: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Ensures that the given node is in its starting-style state."]
pub struct ForceStartingStyle {
    #[doc = "The element id for which to force the starting-style state."]
    pub node_id: dom::NodeId,
    #[serde(default)]
    #[doc = "Boolean indicating if this is on or off."]
    pub forced: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GetBackgroundColors {
    #[doc = "Id of the node to get background colors for."]
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the computed style for a DOM node identified by `nodeId`."]
pub struct GetComputedStyleForNode {
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Resolve the specified values in the context of the provided element.\n For example, a value of '1em' is evaluated according to the computed\n 'font-size' of the element and a value 'calc(1px + 2px)' will be\n resolved to '3px'.\n If the `propertyName` was specified the `values` are resolved as if\n they were property's declaration. If a value cannot be parsed according\n to the provided property syntax, the value is parsed using combined\n syntax as if null `propertyName` was provided. If the value cannot be\n resolved even then, return the provided value without any changes.\n Note: this function currently does not resolve CSS random() function,\n it returns unmodified random() function parts.`"]
pub struct ResolveValues {
    #[serde(default)]
    #[doc = "Cascade-dependent keywords (revert/revert-layer) do not work."]
    pub values: Vec<String>,
    #[doc = "Id of the node in whose context the expression is evaluated"]
    pub node_id: dom::NodeId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Only longhands and custom property names are accepted."]
    pub property_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Pseudo element type, only works for pseudo elements that generate\n elements in the tree, such as ::before and ::after."]
    pub pseudo_type: Option<dom::PseudoType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Pseudo element custom ident."]
    pub pseudo_identifier: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GetLonghandProperties {
    #[serde(default)]
    pub shorthand_name: String,
    #[serde(default)]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the styles defined inline (explicitly in the \"style\" attribute and implicitly, using DOM\n attributes) for a DOM node identified by `nodeId`."]
pub struct GetInlineStylesForNode {
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the styles coming from animations & transitions\n including the animation & transition styles coming from inheritance chain."]
pub struct GetAnimatedStylesForNode {
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns requested styles for a DOM node identified by `nodeId`."]
pub struct GetMatchedStylesForNode {
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetEnvironmentVariables(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetMediaQueries(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Requests information about platform fonts which we used to render child TextNodes in the given\n node."]
pub struct GetPlatformFontsForNode {
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the current textual content for a stylesheet."]
pub struct GetStyleSheetText {
    pub style_sheet_id: dom::StyleSheetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all layers parsed by the rendering engine for the tree scope of a node.\n Given a DOM element identified by nodeId, getLayersForNode returns the root\n layer for the nearest ancestor document or shadow root. The layer root contains\n the full layer tree for the tree scope and their ordering."]
pub struct GetLayersForNode {
    pub node_id: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Given a CSS selector text and a style sheet ID, getLocationForSelector\n returns an array of locations of the CSS selector in the style sheet."]
pub struct GetLocationForSelector {
    pub style_sheet_id: dom::StyleSheetId,
    #[serde(default)]
    pub selector_text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Starts tracking the given node for the computed style updates\n and whenever the computed style is updated for node, it queues\n a `computedStyleUpdated` event with throttling.\n There can only be 1 node tracked for computed style updates\n so passing a new node id removes tracking from the previous node.\n Pass `undefined` to disable tracking."]
pub struct TrackComputedStyleUpdatesForNode {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<dom::NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Starts tracking the given computed styles for updates. The specified array of properties\n replaces the one previously specified. Pass empty array to disable tracking.\n Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.\n The changes to computed style properties are only tracked for nodes pushed to the front-end\n by the DOM agent. If no changes to the tracked properties occur after the node has been pushed\n to the front-end, no updates will be issued for the node."]
pub struct TrackComputedStyleUpdates {
    pub properties_to_track: Vec<CssComputedStyleProperty>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeComputedStyleUpdates(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Find a rule with the given active property for the given node and set the new value for this\n property"]
pub struct SetEffectivePropertyValueForNode {
    #[doc = "The element id for which to set property."]
    pub node_id: dom::NodeId,
    #[serde(default)]
    pub property_name: String,
    #[serde(default)]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the property rule property name."]
pub struct SetPropertyRulePropertyName {
    pub style_sheet_id: dom::StyleSheetId,
    pub range: SourceRange,
    #[serde(default)]
    pub property_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the keyframe rule key text."]
pub struct SetKeyframeKey {
    pub style_sheet_id: dom::StyleSheetId,
    pub range: SourceRange,
    #[serde(default)]
    pub key_text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the rule selector."]
pub struct SetMediaText {
    pub style_sheet_id: dom::StyleSheetId,
    pub range: SourceRange,
    #[serde(default)]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the expression of a container query."]
pub struct SetContainerQueryText {
    pub style_sheet_id: dom::StyleSheetId,
    pub range: SourceRange,
    #[serde(default)]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the expression of a supports at-rule."]
pub struct SetSupportsText {
    pub style_sheet_id: dom::StyleSheetId,
    pub range: SourceRange,
    #[serde(default)]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the expression of a scope at-rule."]
pub struct SetScopeText {
    pub style_sheet_id: dom::StyleSheetId,
    pub range: SourceRange,
    #[serde(default)]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the rule selector."]
pub struct SetRuleSelector {
    pub style_sheet_id: dom::StyleSheetId,
    pub range: SourceRange,
    #[serde(default)]
    pub selector: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets the new stylesheet text."]
pub struct SetStyleSheetText {
    pub style_sheet_id: dom::StyleSheetId,
    #[serde(default)]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Applies specified style edits one after another in the given order."]
pub struct SetStyleTexts {
    pub edits: Vec<StyleDeclarationEdit>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "NodeId for the DOM node in whose context custom property declarations for registered properties should be\n validated. If omitted, declarations in the new rule text can only be validated statically, which may produce\n incorrect results if the declaration contains a var() for example."]
    pub node_for_property_syntax_validation: Option<dom::NodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StartRuleUsageTracking(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StopRuleUsageTracking(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeCoverageDelta(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables/disables rendering of local CSS fonts (enabled by default)."]
pub struct SetLocalFontsEnabled {
    #[serde(default)]
    #[doc = "Whether rendering of local fonts is enabled."]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the\n position specified by `location`."]
pub struct AddRuleReturnObject {
    #[doc = "The newly created rule."]
    pub rule: CssRule,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all class names from specified stylesheet."]
pub struct CollectClassNamesReturnObject {
    #[doc = "Class name list."]
    pub class_names: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Creates a new special \"via-inspector\" stylesheet in the frame with given `frameId`."]
pub struct CreateStyleSheetReturnObject {
    #[doc = "Identifier of the created \"via-inspector\" stylesheet."]
    pub style_sheet_id: dom::StyleSheetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables the CSS agent for the given page."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been\n enabled until the result of this command is received."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Ensures that the given node will have specified pseudo-classes whenever its style is computed by\n the browser."]
pub struct ForcePseudoStateReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Ensures that the given node is in its starting-style state."]
pub struct ForceStartingStyleReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetBackgroundColorsReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The range of background colors behind this element, if it contains any visible text. If no\n visible text is present, this will be undefined. In the case of a flat background color,\n this will consist of simply that color. In the case of a gradient, this will consist of each\n of the color stops. For anything more complicated, this will be an empty array. Images will\n be ignored (as if the image had failed to load)."]
    pub background_colors: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The computed font size for this node, as a CSS computed value string (e.g. '12px')."]
    pub computed_font_size: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or\n '100')."]
    pub computed_font_weight: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the computed style for a DOM node identified by `nodeId`."]
pub struct GetComputedStyleForNodeReturnObject {
    #[doc = "Computed style for the specified DOM node."]
    pub computed_style: Vec<CssComputedStyleProperty>,
    #[doc = "A list of non-standard \"extra fields\" which blink stores alongside each\n computed style."]
    pub extra_fields: ComputedStyleExtraFields,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Resolve the specified values in the context of the provided element.\n For example, a value of '1em' is evaluated according to the computed\n 'font-size' of the element and a value 'calc(1px + 2px)' will be\n resolved to '3px'.\n If the `propertyName` was specified the `values` are resolved as if\n they were property's declaration. If a value cannot be parsed according\n to the provided property syntax, the value is parsed using combined\n syntax as if null `propertyName` was provided. If the value cannot be\n resolved even then, return the provided value without any changes.\n Note: this function currently does not resolve CSS random() function,\n it returns unmodified random() function parts.`"]
pub struct ResolveValuesReturnObject {
    pub results: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetLonghandPropertiesReturnObject {
    pub longhand_properties: Vec<CssProperty>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the styles defined inline (explicitly in the \"style\" attribute and implicitly, using DOM\n attributes) for a DOM node identified by `nodeId`."]
pub struct GetInlineStylesForNodeReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Inline style for the specified DOM node."]
    pub inline_style: Option<CssStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Attribute-defined element style (e.g. resulting from \"width=20 height=100%\")."]
    pub attributes_style: Option<CssStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the styles coming from animations & transitions\n including the animation & transition styles coming from inheritance chain."]
pub struct GetAnimatedStylesForNodeReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Styles coming from animations."]
    pub animation_styles: Option<Vec<CssAnimationStyle>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Style coming from transitions."]
    pub transitions_style: Option<CssStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Inherited style entries for animationsStyle and transitionsStyle from\n the inheritance chain of the element."]
    pub inherited: Option<Vec<InheritedAnimatedStyleEntry>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns requested styles for a DOM node identified by `nodeId`."]
pub struct GetMatchedStylesForNodeReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Inline style for the specified DOM node."]
    pub inline_style: Option<CssStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Attribute-defined element style (e.g. resulting from \"width=20 height=100%\")."]
    pub attributes_style: Option<CssStyle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "CSS rules matching this node, from all applicable stylesheets."]
    #[serde(rename = "matchedCSSRules")]
    pub matched_css_rules: Option<Vec<RuleMatch>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Pseudo style matches for this node."]
    pub pseudo_elements: Option<Vec<PseudoElementMatches>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A chain of inherited styles (from the immediate node parent up to the DOM tree root)."]
    pub inherited: Option<Vec<InheritedStyleEntry>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root)."]
    pub inherited_pseudo_elements: Option<Vec<InheritedPseudoElementMatches>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A list of CSS keyframed animations matching this node."]
    pub css_keyframes_rules: Option<Vec<CssKeyframesRule>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property."]
    pub css_position_try_rules: Option<Vec<CssPositionTryRule>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Index of the active fallback in the applied position-try-fallback property,\n will not be set if there is no active position-try fallback."]
    pub active_position_fallback_index: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A list of CSS at-property rules matching this node."]
    pub css_property_rules: Option<Vec<CssPropertyRule>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A list of CSS property registrations matching this node."]
    pub css_property_registrations: Option<Vec<CssPropertyRegistration>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A list of simple @rules matching this node or its pseudo-elements."]
    pub css_at_rules: Option<Vec<CssAtRule>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Id of the first parent element that does not have display: contents."]
    pub parent_layout_node_id: Option<dom::NodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A list of CSS at-function rules referenced by styles of this node."]
    pub css_function_rules: Option<Vec<CssFunctionRule>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the values of the default UA-defined environment variables used in env()"]
pub struct GetEnvironmentVariablesReturnObject {
    #[serde(default)]
    pub environment_variables: Json,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all media queries parsed by the rendering engine."]
pub struct GetMediaQueriesReturnObject {
    pub medias: Vec<CssMedia>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Requests information about platform fonts which we used to render child TextNodes in the given\n node."]
pub struct GetPlatformFontsForNodeReturnObject {
    #[doc = "Usage statistics for every employed platform font."]
    pub fonts: Vec<PlatformFontUsage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the current textual content for a stylesheet."]
pub struct GetStyleSheetTextReturnObject {
    #[serde(default)]
    #[doc = "The stylesheet text."]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all layers parsed by the rendering engine for the tree scope of a node.\n Given a DOM element identified by nodeId, getLayersForNode returns the root\n layer for the nearest ancestor document or shadow root. The layer root contains\n the full layer tree for the tree scope and their ordering."]
pub struct GetLayersForNodeReturnObject {
    pub root_layer: CssLayerData,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Given a CSS selector text and a style sheet ID, getLocationForSelector\n returns an array of locations of the CSS selector in the style sheet."]
pub struct GetLocationForSelectorReturnObject {
    pub ranges: Vec<SourceRange>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Starts tracking the given node for the computed style updates\n and whenever the computed style is updated for node, it queues\n a `computedStyleUpdated` event with throttling.\n There can only be 1 node tracked for computed style updates\n so passing a new node id removes tracking from the previous node.\n Pass `undefined` to disable tracking."]
pub struct TrackComputedStyleUpdatesForNodeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Starts tracking the given computed styles for updates. The specified array of properties\n replaces the one previously specified. Pass empty array to disable tracking.\n Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.\n The changes to computed style properties are only tracked for nodes pushed to the front-end\n by the DOM agent. If no changes to the tracked properties occur after the node has been pushed\n to the front-end, no updates will be issued for the node."]
pub struct TrackComputedStyleUpdatesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Polls the next batch of computed style updates."]
pub struct TakeComputedStyleUpdatesReturnObject {
    #[doc = "The list of node Ids that have their tracked computed styles updated."]
    pub node_ids: dom::NodeId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Find a rule with the given active property for the given node and set the new value for this\n property"]
pub struct SetEffectivePropertyValueForNodeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the property rule property name."]
pub struct SetPropertyRulePropertyNameReturnObject {
    #[doc = "The resulting key text after modification."]
    pub property_name: Value,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the keyframe rule key text."]
pub struct SetKeyframeKeyReturnObject {
    #[doc = "The resulting key text after modification."]
    pub key_text: Value,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the rule selector."]
pub struct SetMediaTextReturnObject {
    #[doc = "The resulting CSS media rule after modification."]
    pub media: CssMedia,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the expression of a container query."]
pub struct SetContainerQueryTextReturnObject {
    #[doc = "The resulting CSS container query rule after modification."]
    pub container_query: CssContainerQuery,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the expression of a supports at-rule."]
pub struct SetSupportsTextReturnObject {
    #[doc = "The resulting CSS Supports rule after modification."]
    pub supports: CssSupports,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the expression of a scope at-rule."]
pub struct SetScopeTextReturnObject {
    #[doc = "The resulting CSS Scope rule after modification."]
    pub scope: CssScope,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Modifies the rule selector."]
pub struct SetRuleSelectorReturnObject {
    #[doc = "The resulting selector list after modification."]
    pub selector_list: SelectorList,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sets the new stylesheet text."]
pub struct SetStyleSheetTextReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL of source map associated with script (if any)."]
    #[serde(rename = "sourceMapURL")]
    pub source_map_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Applies specified style edits one after another in the given order."]
pub struct SetStyleTextsReturnObject {
    #[doc = "The resulting styles after modification."]
    pub styles: Vec<CssStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables the selector recording."]
pub struct StartRuleUsageTrackingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Stop tracking rule usage and return the list of rules that were used since last call to\n `takeCoverageDelta` (or since start of coverage instrumentation)."]
pub struct StopRuleUsageTrackingReturnObject {
    pub rule_usage: Vec<RuleUsage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Obtain list of rules that became used since last call to this method (or since start of coverage\n instrumentation)."]
pub struct TakeCoverageDeltaReturnObject {
    pub coverage: Vec<RuleUsage>,
    #[serde(default)]
    #[doc = "Monotonically increasing time, in seconds."]
    pub timestamp: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables/disables rendering of local CSS fonts (enabled by default)."]
pub struct SetLocalFontsEnabledReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct FontsUpdatedEvent {
        pub params: FontsUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct FontsUpdatedEventParams {
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The web font that has loaded."]
        pub font: Option<super::FontFace>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct MediaQueryResultChangedEvent(pub Option<Json>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetAddedEvent {
        pub params: StyleSheetAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct StyleSheetAddedEventParams {
        #[doc = "Added stylesheet metainfo."]
        pub header: super::CssStyleSheetHeader,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetChangedEvent {
        pub params: StyleSheetChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct StyleSheetChangedEventParams {
        pub style_sheet_id: super::super::dom::StyleSheetId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StyleSheetRemovedEvent {
        pub params: StyleSheetRemovedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct StyleSheetRemovedEventParams {
        #[doc = "Identifier of the removed stylesheet."]
        pub style_sheet_id: super::super::dom::StyleSheetId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ComputedStyleUpdatedEvent {
        pub params: ComputedStyleUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ComputedStyleUpdatedEventParams {
        #[doc = "The node id that has updated computed styles."]
        pub node_id: super::super::dom::NodeId,
    }
}
