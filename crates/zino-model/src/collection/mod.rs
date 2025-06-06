//! The `collection` model and related services.

use crate::{group::Group, source::Source};
use serde::{Deserialize, Serialize};
use zino_core::{
    Map, Uuid,
    datetime::DateTime,
    error::Error,
    extension::JsonObjectExt,
    model::{Model, ModelHooks},
    validation::Validation,
};
use zino_derive::{DecodeRow, Entity, ModelAccessor, Schema};

#[cfg(feature = "tags")]
use crate::tag::Tag;

#[cfg(any(feature = "owner-id", feature = "maintainer-id"))]
use crate::user::User;

#[cfg(feature = "maintainer-id")]
use zino_auth::UserSession;

/// The `collection` model.
#[derive(
    Debug, Clone, Default, Serialize, Deserialize, DecodeRow, Entity, Schema, ModelAccessor,
)]
#[serde(default)]
#[schema(auto_rename)]
pub struct Collection {
    // Basic fields.
    #[schema(read_only)]
    id: Uuid,
    #[schema(not_null)]
    name: String,
    #[cfg(feature = "namespace")]
    #[schema(default_value = "Collection::model_namespace", index_type = "hash")]
    namespace: String,
    #[cfg(feature = "visibility")]
    #[schema(default_value = "Internal")]
    visibility: String,
    #[schema(default_value = "Active", index_type = "hash")]
    status: String,
    description: String,

    // Info fields.
    #[schema(reference = "Group")]
    consumer_id: Option<Uuid>, // group.id
    #[schema(reference = "Source")]
    source_id: Uuid, // source.id
    #[cfg(feature = "tags")]
    #[schema(reference = "Tag", index_type = "gin")]
    tags: Vec<Uuid>, // tag.id, tag.namespace = "*:collection"

    // Extensions.
    extra: Map,

    // Revisions.
    #[cfg(feature = "owner-id")]
    #[schema(reference = "User")]
    owner_id: Option<Uuid>, // user.id
    #[cfg(feature = "maintainer-id")]
    #[schema(reference = "User")]
    maintainer_id: Option<Uuid>, // user.id
    #[schema(read_only, default_value = "now", index_type = "btree")]
    created_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    updated_at: DateTime,
    version: u64,
    #[cfg(feature = "edition")]
    edition: u32,
}

impl Model for Collection {
    const MODEL_NAME: &'static str = "collection";

    #[inline]
    fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            ..Self::default()
        }
    }

    fn read_map(&mut self, data: &Map) -> Validation {
        let mut validation = Validation::new();
        if let Some(result) = data.parse_uuid("id") {
            match result {
                Ok(id) => self.id = id,
                Err(err) => validation.record_fail("id", err),
            }
        }
        if let Some(name) = data.parse_string("name") {
            self.name = name.into_owned();
        }
        if let Some(description) = data.parse_string("description") {
            self.description = description.into_owned();
        }
        #[cfg(feature = "tags")]
        if let Some(result) = data.parse_array("tags") {
            match result {
                Ok(tags) => self.tags = tags,
                Err(err) => validation.record_fail("tags", err),
            }
        }
        #[cfg(feature = "owner-id")]
        if let Some(result) = data.parse_uuid("owner_id") {
            match result {
                Ok(owner_id) => self.owner_id = Some(owner_id),
                Err(err) => validation.record_fail("owner_id", err),
            }
        }
        #[cfg(feature = "maintainer-id")]
        if let Some(result) = data.parse_uuid("maintainer_id") {
            match result {
                Ok(maintainer_id) => self.maintainer_id = Some(maintainer_id),
                Err(err) => validation.record_fail("maintainer_id", err),
            }
        }
        validation
    }
}

impl ModelHooks for Collection {
    type Data = ();
    #[cfg(feature = "maintainer-id")]
    type Extension = UserSession<Uuid, String>;
    #[cfg(not(feature = "maintainer-id"))]
    type Extension = ();

    #[cfg(feature = "maintainer-id")]
    #[inline]
    async fn after_extract(&mut self, session: Self::Extension) -> Result<(), Error> {
        self.maintainer_id = Some(*session.user_id());
        Ok(())
    }

    #[cfg(feature = "maintainer-id")]
    #[inline]
    async fn before_validation(
        data: &mut Map,
        extension: Option<&Self::Extension>,
    ) -> Result<(), Error> {
        if let Some(session) = extension {
            data.upsert("maintainer_id", session.user_id().to_string());
        }
        Ok(())
    }
}
