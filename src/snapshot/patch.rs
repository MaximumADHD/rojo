//! Defines the data structures used for describing instance patches.

use std::collections::HashMap;

use rbx_dom_weak::types::{Ref, Variant};
use serde::{Deserialize, Serialize};

use super::{InstanceMetadata, InstanceSnapshot};

/// A set of different kinds of patches that can be applied to an WeakDom.
///
/// These patches shouldn't be persisted: there's no mechanism in place to make
/// sure that another patch wasn't applied before this one that could cause a
/// conflict!
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchSet {
    pub removed_instances: Vec<Ref>,
    pub added_instances: Vec<PatchAdd>,
    pub updated_instances: Vec<PatchUpdate>,
}

impl PatchSet {
    pub fn new() -> Self {
        PatchSet {
            removed_instances: Vec::new(),
            added_instances: Vec::new(),
            updated_instances: Vec::new(),
        }
    }
}

/// A patch containing an instance that was added to the tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchAdd {
    pub parent_id: Ref,
    pub instance: InstanceSnapshot,
}

/// A patch indicating that properties of an instance changed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchUpdate {
    pub id: Ref,
    pub changed_name: Option<String>,
    pub changed_class_name: Option<String>,

    /// Contains all changed properties. If a property is assigned to `None`,
    /// then that property has been removed.
    pub changed_properties: HashMap<String, Option<Variant>>,

    /// Changed Rojo-specific metadata, if any of it changed.
    pub changed_metadata: Option<InstanceMetadata>,
}

/// Applied patch sets have the same rough shape as PatchSet, but are
/// descriptive of the operation that happened instead of prescribing what
/// mutations to apply to the tree.
///
/// Applied patch sets are generated by applying a patch to a tree, and are
/// suitable for sending over the network to a synchronized tree like the Rojo
/// Studio plugin.
///
// TODO: Introduce machinery to detect conflicts, like keeping previous +
// current values in all fields.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppliedPatchSet {
    pub removed: Vec<Ref>,
    pub added: Vec<Ref>,
    pub updated: Vec<AppliedPatchUpdate>,
}

impl AppliedPatchSet {
    pub fn new() -> Self {
        AppliedPatchSet {
            removed: Vec::new(),
            added: Vec::new(),
            updated: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.added.is_empty() && self.updated.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedPatchUpdate {
    pub id: Ref,

    // TODO: Store previous values in order to detect application conflicts
    pub changed_name: Option<String>,
    pub changed_class_name: Option<String>,
    pub changed_properties: HashMap<String, Option<Variant>>,
    pub changed_metadata: Option<InstanceMetadata>,
}

impl AppliedPatchUpdate {
    pub fn new(id: Ref) -> Self {
        Self {
            id,
            changed_name: None,
            changed_class_name: None,
            changed_properties: HashMap::new(),
            changed_metadata: None,
        }
    }
}
