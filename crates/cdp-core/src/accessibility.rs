use crate::error::{CdpError, Result};
use crate::page::{Page, element::ElementHandle};
use cdp_protocol::accessibility as ax;
use cdp_protocol::accessibility::{
    Disable as AccessibilityDisable, DisableReturnObject as AccessibilityDisableReturnObject,
    Enable as AccessibilityEnable, EnableReturnObject as AccessibilityEnableReturnObject,
    GetFullAXTree, GetFullAXTreeReturnObject, GetPartialAXTree, GetPartialAXTreeReturnObject,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// High level controller for interacting with the Chrome Accessibility domain.
pub struct AccessibilityController {
    page: Arc<Page>,
}

impl AccessibilityController {
    pub(crate) fn new(page: Arc<Page>) -> Self {
        Self { page }
    }

    /// Captures an accessibility tree snapshot for the current page or a specific element.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{AccessibilitySnapshotOptions, Page};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let snapshot = page.accessibility().snapshot(None).await?;
    /// assert!(!snapshot.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn snapshot(
        &self,
        options: Option<AccessibilitySnapshotOptions>,
    ) -> Result<AccessibilitySnapshot> {
        let opts = options.unwrap_or_default();
        self.enable().await?;

        let (nodes, root_backend_id) = if let Some(element) = opts.root.as_ref() {
            self.ensure_same_page(element)?;
            let method = GetPartialAXTree {
                node_id: element.node_id,
                backend_node_id: Some(element.backend_node_id),
                object_id: None,
                fetch_relatives: Some(true),
            };
            let response: GetPartialAXTreeReturnObject =
                self.page.session.send_command(method, None).await?;
            (response.nodes, Some(element.backend_node_id))
        } else {
            let method = GetFullAXTree {
                depth: opts.max_depth,
                frame_id: None,
            };
            let response: GetFullAXTreeReturnObject =
                self.page.session.send_command(method, None).await?;
            (response.nodes, None)
        };

        Ok(AccessibilitySnapshot::from_ax_nodes(
            nodes,
            root_backend_id,
            opts.interesting_only,
        ))
    }

    /// Enables the accessibility domain. Safe to call multiple times.
    pub async fn enable(&self) -> Result<()> {
        let method = AccessibilityEnable(None);
        let _: AccessibilityEnableReturnObject =
            self.page.session.send_command(method, None).await?;
        Ok(())
    }

    /// Disables the accessibility domain.
    pub async fn disable(&self) -> Result<()> {
        let method = AccessibilityDisable(None);
        let _: AccessibilityDisableReturnObject =
            self.page.session.send_command(method, None).await?;
        Ok(())
    }

    fn ensure_same_page(&self, element: &ElementHandle) -> Result<()> {
        if !Arc::ptr_eq(&self.page, &element.page) {
            return Err(CdpError::page(
                "Accessibility snapshot root element must belong to the same page".to_string(),
            ));
        }
        Ok(())
    }
}

/// Options used when generating accessibility snapshots.
#[derive(Clone)]
pub struct AccessibilitySnapshotOptions {
    pub root: Option<ElementHandle>,
    pub interesting_only: bool,
    pub max_depth: Option<u32>,
}

impl Default for AccessibilitySnapshotOptions {
    fn default() -> Self {
        Self {
            root: None,
            interesting_only: true,
            max_depth: None,
        }
    }
}

impl AccessibilitySnapshotOptions {
    pub fn with_root(mut self, element: ElementHandle) -> Self {
        self.root = Some(element);
        self
    }

    pub fn with_interesting_only(mut self, interesting_only: bool) -> Self {
        self.interesting_only = interesting_only;
        self
    }

    pub fn with_max_depth(mut self, depth: u32) -> Self {
        self.max_depth = Some(depth);
        self
    }
}

/// Structured accessibility tree snapshot.
#[derive(Clone, Debug, Default)]
pub struct AccessibilitySnapshot {
    pub roots: Vec<AccessibilityNode>,
}

impl AccessibilitySnapshot {
    fn from_ax_nodes(
        nodes: Vec<ax::AxNode>,
        root_backend_id: Option<u32>,
        interesting_only: bool,
    ) -> Self {
        let mut map: HashMap<String, ax::AxNode> = HashMap::new();
        for node in nodes {
            map.insert(node.node_id.clone(), node);
        }

        let mut referenced_children: HashSet<String> = HashSet::new();
        for node in map.values() {
            if let Some(children) = &node.child_ids {
                for child in children {
                    referenced_children.insert(child.clone());
                }
            }
        }

        let mut candidate_roots: Vec<String> = Vec::new();
        if let Some(backend_id) = root_backend_id
            && let Some(root) = map
                .values()
                .find(|node| node.backend_dom_node_id == Some(backend_id))
            {
                candidate_roots.push(root.node_id.clone());
            }

        if candidate_roots.is_empty() {
            for node in map.values() {
                let parent_known = node.parent_id.as_ref().and_then(|parent| map.get(parent));
                if parent_known.is_none() && !referenced_children.contains(&node.node_id) {
                    candidate_roots.push(node.node_id.clone());
                }
            }
        }

        if candidate_roots.is_empty()
            && let Some(first) = map.keys().next() {
                candidate_roots.push(first.clone());
            }

        let mut visited: HashSet<String> = HashSet::new();
        let mut roots: Vec<AccessibilityNode> = Vec::new();
        for root_id in candidate_roots {
            roots.extend(build_subtree(
                &root_id,
                &map,
                interesting_only,
                &mut visited,
            ));
        }

        Self { roots }
    }

    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }
}

fn build_subtree(
    node_id: &str,
    nodes: &HashMap<String, ax::AxNode>,
    interesting_only: bool,
    visited: &mut HashSet<String>,
) -> Vec<AccessibilityNode> {
    if !visited.insert(node_id.to_string()) {
        return Vec::new();
    }

    let node = match nodes.get(node_id) {
        Some(node) => node.clone(),
        None => return Vec::new(),
    };

    let child_ids = node.child_ids.clone().unwrap_or_default();
    let mut children: Vec<AccessibilityNode> = Vec::new();
    for child_id in child_ids {
        children.extend(build_subtree(&child_id, nodes, interesting_only, visited));
    }

    let interesting = !interesting_only || !node.ignored;
    if interesting {
        vec![AccessibilityNode::from_ax_node(node, children)]
    } else {
        children
    }
}

/// Represents a single accessibility node in the snapshot.
#[derive(Clone, Debug)]
pub struct AccessibilityNode {
    pub node_id: String,
    pub ignored: bool,
    pub ignored_reasons: Vec<AccessibilityProperty>,
    pub role: Option<ax::AxValue>,
    pub chrome_role: Option<ax::AxValue>,
    pub name: Option<ax::AxValue>,
    pub description: Option<ax::AxValue>,
    pub value: Option<ax::AxValue>,
    pub properties: Vec<AccessibilityProperty>,
    pub backend_dom_node_id: Option<u32>,
    pub frame_id: Option<String>,
    pub children: Vec<AccessibilityNode>,
}

impl AccessibilityNode {
    fn from_ax_node(node: ax::AxNode, children: Vec<AccessibilityNode>) -> Self {
        Self {
            node_id: node.node_id,
            ignored: node.ignored,
            ignored_reasons: convert_properties(node.ignored_reasons),
            role: node.role,
            chrome_role: node.chrome_role,
            name: node.name,
            description: node.description,
            value: node.value,
            properties: convert_properties(node.properties),
            backend_dom_node_id: node.backend_dom_node_id,
            frame_id: node.frame_id,
            children,
        }
    }
}

/// Name/value pair extracted from an `AxProperty`.
#[derive(Clone, Debug)]
pub struct AccessibilityProperty {
    pub name: String,
    pub value: ax::AxValue,
}

fn convert_properties(input: Option<Vec<ax::AxProperty>>) -> Vec<AccessibilityProperty> {
    input
        .unwrap_or_default()
        .into_iter()
        .map(|prop| AccessibilityProperty {
            name: serialize_property_name(&prop.name),
            value: prop.value,
        })
        .collect()
}

fn serialize_property_name(name: &ax::AxPropertyName) -> String {
    serde_json::to_value(name)
        .ok()
        .and_then(|val| val.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", name))
}
