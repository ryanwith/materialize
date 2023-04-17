// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

// BEGIN LINT CONFIG
// DO NOT EDIT. Automatically generated by bin/gen-lints.
// Have complaints about the noise? See the note in misc/python/materialize/cli/gen-lints.py first.
#![allow(clippy::style)]
#![allow(clippy::complexity)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::mutable_key_type)]
#![allow(clippy::stable_sort_primitive)]
#![allow(clippy::map_entry)]
#![allow(clippy::box_default)]
#![warn(clippy::bool_comparison)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::no_effect)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::zero_prefixed_literal)]
#![warn(clippy::borrowed_box)]
#![warn(clippy::deref_addrof)]
#![warn(clippy::double_must_use)]
#![warn(clippy::double_parens)]
#![warn(clippy::extra_unused_lifetimes)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_question_mark)]
#![warn(clippy::needless_return)]
#![warn(clippy::redundant_pattern)]
#![warn(clippy::redundant_slicing)]
#![warn(clippy::redundant_static_lifetimes)]
#![warn(clippy::single_component_path_imports)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::useless_asref)]
#![warn(clippy::useless_conversion)]
#![warn(clippy::builtin_type_shadow)]
#![warn(clippy::duplicate_underscore_argument)]
#![warn(clippy::double_neg)]
#![warn(clippy::unnecessary_mut_passed)]
#![warn(clippy::wildcard_in_or_patterns)]
#![warn(clippy::collapsible_if)]
#![warn(clippy::collapsible_else_if)]
#![warn(clippy::crosspointer_transmute)]
#![warn(clippy::excessive_precision)]
#![warn(clippy::overflow_check_conditional)]
#![warn(clippy::as_conversions)]
#![warn(clippy::match_overlapping_arm)]
#![warn(clippy::zero_divided_by_zero)]
#![warn(clippy::must_use_unit)]
#![warn(clippy::suspicious_assignment_formatting)]
#![warn(clippy::suspicious_else_formatting)]
#![warn(clippy::suspicious_unary_op_formatting)]
#![warn(clippy::mut_mutex_lock)]
#![warn(clippy::print_literal)]
#![warn(clippy::same_item_push)]
#![warn(clippy::useless_format)]
#![warn(clippy::write_literal)]
#![warn(clippy::redundant_closure)]
#![warn(clippy::redundant_closure_call)]
#![warn(clippy::unnecessary_lazy_evaluations)]
#![warn(clippy::partialeq_ne_impl)]
#![warn(clippy::redundant_field_names)]
#![warn(clippy::transmutes_expressible_as_ptr_casts)]
#![warn(clippy::unused_async)]
#![warn(clippy::disallowed_methods)]
#![warn(clippy::disallowed_macros)]
#![warn(clippy::disallowed_types)]
#![warn(clippy::from_over_into)]
// END LINT CONFIG

//! Audit log data structures.
//!
//! The audit log is logging that is produced by user actions and consumed
//! by users in the form of the `mz_catalog.mz_audit_events` SQL table and
//! by the cloud management layer for billing and introspection. This crate
//! is designed to make the production and consumption of the logs type
//! safe. Events and their metadata are versioned and the data structures
//! replicated here so that if the data change in some other crate, a
//! new version here can be made. This avoids needing to poke at the data
//! when reading it to determine what it means and should have full backward
//! compatibility. This is its own crate so that production and consumption can
//! be in different processes and production is not allowed to specify private
//! data structures unknown to the reader.

use serde::{Deserialize, Serialize};

use mz_ore::now::EpochMillis;

/// New version variants should be added if fields need to be added, changed, or removed.
#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum VersionedEvent {
    V1(EventV1),
}

impl VersionedEvent {
    /// Create a new event. This function must always require and produce the most
    /// recent variant of VersionedEvent. `id` must be a globally increasing,
    /// ordered number such that sorting by it on all events yields the order
    /// of events by users. It is insufficient to use `occurred_at` (even at
    /// nanosecond precision) due to clock unpredictability.
    pub fn new(
        id: u64,
        event_type: EventType,
        object_type: ObjectType,
        details: EventDetails,
        user: Option<String>,
        occurred_at: EpochMillis,
    ) -> Self {
        Self::V1(EventV1::new(
            id,
            event_type,
            object_type,
            details,
            user,
            occurred_at,
        ))
    }

    // Implement deserialize and serialize so writers and readers don't have to
    // coordinate about which Serializer to use.
    pub fn deserialize(data: &[u8]) -> Result<Self, anyhow::Error> {
        Ok(serde_json::from_slice(data)?)
    }

    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("must serialize")
    }

    /// Returns a globally sortable event order. All event versions must have this
    /// field.
    pub fn sortable_id(&self) -> u64 {
        match self {
            VersionedEvent::V1(ev) => ev.id,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum EventType {
    Create,
    Drop,
    Alter,
    Grant,
    Revoke,
}

impl EventType {
    pub fn as_title_case(&self) -> &'static str {
        match self {
            EventType::Create => "Created",
            EventType::Drop => "Dropped",
            EventType::Alter => "Altered",
            EventType::Grant => "Granted",
            EventType::Revoke => "Revoked",
        }
    }
}

serde_plain::derive_display_from_serialize!(EventType);

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ObjectType {
    Cluster,
    ClusterReplica,
    Connection,
    Database,
    Func,
    Index,
    MaterializedView,
    Role,
    Secret,
    Schema,
    Sink,
    Source,
    Table,
    Type,
    View,
}

impl ObjectType {
    pub fn as_title_case(&self) -> &'static str {
        match self {
            ObjectType::Cluster => "Cluster",
            ObjectType::ClusterReplica => "Cluster Replica",
            ObjectType::Connection => "Connection",
            ObjectType::Database => "Database",
            ObjectType::Func => "Function",
            ObjectType::Index => "Index",
            ObjectType::MaterializedView => "Materialized View",
            ObjectType::Role => "Role",
            ObjectType::Schema => "Schema",
            ObjectType::Secret => "Secret",
            ObjectType::Sink => "Sink",
            ObjectType::Source => "Source",
            ObjectType::Table => "Table",
            ObjectType::Type => "Type",
            ObjectType::View => "View",
        }
    }
}

serde_plain::derive_display_from_serialize!(ObjectType);

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum EventDetails {
    #[serde(rename = "CreateComputeReplicaV1")] // historical name
    CreateClusterReplicaV1(CreateClusterReplicaV1),
    #[serde(rename = "DropComputeReplicaV1")] // historical name
    DropClusterReplicaV1(DropClusterReplicaV1),
    CreateSourceSinkV1(CreateSourceSinkV1),
    CreateSourceSinkV2(CreateSourceSinkV2),
    AlterSourceSinkV1(AlterSourceSinkV1),
    GrantRoleV1(GrantRoleV1),
    GrantRoleV2(GrantRoleV2),
    RevokeRoleV1(RevokeRoleV1),
    RevokeRoleV2(RevokeRoleV2),
    IdFullNameV1(IdFullNameV1),
    RenameItemV1(RenameItemV1),
    IdNameV1(IdNameV1),
    SchemaV1(SchemaV1),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct IdFullNameV1 {
    pub id: String,
    #[serde(flatten)]
    pub name: FullNameV1,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FullNameV1 {
    pub database: String,
    pub schema: String,
    pub item: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct IdNameV1 {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct RenameItemV1 {
    pub id: String,
    pub old_name: FullNameV1,
    pub new_name: FullNameV1,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct DropClusterReplicaV1 {
    pub cluster_id: String,
    pub cluster_name: String,
    // Events that predate v0.32.0 will not have this field set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<String>,
    pub replica_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct CreateClusterReplicaV1 {
    pub cluster_id: String,
    pub cluster_name: String,
    // Events that predate v0.32.0 will not have this field set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<String>,
    pub replica_name: String,
    pub logical_size: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct CreateSourceSinkV1 {
    pub id: String,
    #[serde(flatten)]
    pub name: FullNameV1,
    pub size: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct CreateSourceSinkV2 {
    pub id: String,
    #[serde(flatten)]
    pub name: FullNameV1,
    pub size: Option<String>,
    #[serde(rename = "type")]
    pub external_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct AlterSourceSinkV1 {
    pub id: String,
    #[serde(flatten)]
    pub name: FullNameV1,
    pub old_size: Option<String>,
    pub new_size: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct GrantRoleV1 {
    pub role_id: String,
    pub member_id: String,
    pub grantor_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct GrantRoleV2 {
    pub role_id: String,
    pub member_id: String,
    pub grantor_id: String,
    pub executed_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct RevokeRoleV1 {
    pub role_id: String,
    pub member_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct RevokeRoleV2 {
    pub role_id: String,
    pub member_id: String,
    pub grantor_id: String,
    pub executed_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct SchemaV1 {
    pub id: String,
    pub name: String,
    pub database_name: String,
}

impl EventDetails {
    pub fn as_json(&self) -> serde_json::Value {
        match self {
            EventDetails::CreateClusterReplicaV1(v) => {
                serde_json::to_value(v).expect("must serialize")
            }
            EventDetails::DropClusterReplicaV1(v) => {
                serde_json::to_value(v).expect("must serialize")
            }
            EventDetails::IdFullNameV1(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::RenameItemV1(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::IdNameV1(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::SchemaV1(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::CreateSourceSinkV1(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::CreateSourceSinkV2(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::AlterSourceSinkV1(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::GrantRoleV1(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::GrantRoleV2(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::RevokeRoleV1(v) => serde_json::to_value(v).expect("must serialize"),
            EventDetails::RevokeRoleV2(v) => serde_json::to_value(v).expect("must serialize"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct EventV1 {
    pub id: u64,
    pub event_type: EventType,
    pub object_type: ObjectType,
    pub details: EventDetails,
    pub user: Option<String>,
    pub occurred_at: EpochMillis,
}

impl EventV1 {
    fn new(
        id: u64,
        event_type: EventType,
        object_type: ObjectType,
        details: EventDetails,
        user: Option<String>,
        occurred_at: EpochMillis,
    ) -> EventV1 {
        EventV1 {
            id,
            event_type,
            object_type,
            details,
            user,
            occurred_at,
        }
    }
}

// Test all versions of events. This test hard codes bytes so that
// programmers are not able to change data structures here without this test
// failing. Instead of changing data structures, add new variants.
#[test]
fn test_audit_log() -> Result<(), anyhow::Error> {
    let cases: Vec<(VersionedEvent, &'static str)> = vec![(
        VersionedEvent::V1(EventV1::new(
            2,
            EventType::Drop,
            ObjectType::ClusterReplica,
            EventDetails::IdNameV1(IdNameV1 {
                id: "u1".to_string(),
                name: "name".into(),
            }),
            None,
            2,
        )),
        r#"{"V1":{"id":2,"event_type":"drop","object_type":"cluster-replica","details":{"IdNameV1":{"id":"u1","name":"name"}},"user":null,"occurred_at":2}}"#,
    )];

    for (event, expected_bytes) in cases {
        let event_bytes = serde_json::to_vec(&event).unwrap();
        assert_eq!(
            event_bytes,
            expected_bytes.as_bytes(),
            "expected bytes {}, got {}",
            expected_bytes,
            std::str::from_utf8(&event_bytes).unwrap(),
        );
    }

    Ok(())
}

/// Describes the environment's storage usage at a point in time.
///
/// This type is persisted in the catalog across restarts, so any updates to the
/// schema will require a new version.
#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum VersionedStorageUsage {
    V1(StorageUsageV1),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct StorageUsageV1 {
    pub id: u64,
    pub shard_id: Option<String>,
    pub size_bytes: u64,
    pub collection_timestamp: EpochMillis,
}

impl StorageUsageV1 {
    pub fn new(
        id: u64,
        shard_id: Option<String>,
        size_bytes: u64,
        collection_timestamp: EpochMillis,
    ) -> StorageUsageV1 {
        StorageUsageV1 {
            id,
            shard_id,
            size_bytes,
            collection_timestamp,
        }
    }
}

impl VersionedStorageUsage {
    /// Create a new metric snapshot.
    /// This function must always require and produce the most
    /// recent variant of VersionedStorageMetrics.
    pub fn new(
        id: u64,
        object_id: Option<String>,
        size_bytes: u64,
        collection_timestamp: EpochMillis,
    ) -> Self {
        Self::V1(StorageUsageV1::new(
            id,
            object_id,
            size_bytes,
            collection_timestamp,
        ))
    }

    // Implement deserialize and serialize so writers and readers don't have to
    // coordinate about which Serializer to use.
    pub fn deserialize(data: &[u8]) -> Result<Self, anyhow::Error> {
        Ok(serde_json::from_slice(data)?)
    }

    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("must serialize")
    }

    pub fn timestamp(&self) -> EpochMillis {
        match self {
            VersionedStorageUsage::V1(StorageUsageV1 {
                collection_timestamp,
                ..
            }) => *collection_timestamp,
        }
    }
}
