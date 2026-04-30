/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use void_db::{ManifestError, interfaces::void_manifest_manipulator::ManifestController};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VoidPluginManifestMember {
    Name(String),
    Author(String),
    Version(String),
}

pub enum VoidPluginManifestMemberVariant {
    Name,
    Author,
    Version,
}

#[derive(Deserialize)]
struct VoidPluginManifestDeserializer {
    name: String,
    author: String,
    version: String,
}

impl Ord for VoidPluginManifestMember {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            VoidPluginManifestMember::Name(_) => match other {
                VoidPluginManifestMember::Name(_) => Ordering::Equal,
                _ => Ordering::Less,
            },
            VoidPluginManifestMember::Author(_) => match other {
                VoidPluginManifestMember::Name(_) => Ordering::Greater,
                VoidPluginManifestMember::Version(_) => Ordering::Less,
                VoidPluginManifestMember::Author(_) => Ordering::Equal,
            },
            VoidPluginManifestMember::Version(_) => match other {
                VoidPluginManifestMember::Version(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
        }
    }
    fn max(self, other: Self) -> Self {
        match self {
            VoidPluginManifestMember::Name(_) => match other {
                VoidPluginManifestMember::Name(_) => self,
                _ => other,
            },
            VoidPluginManifestMember::Author(_) => match other {
                VoidPluginManifestMember::Name(_) => self,
                VoidPluginManifestMember::Version(_) => other,
                VoidPluginManifestMember::Author(_) => self,
            },
            VoidPluginManifestMember::Version(_) => self,
        }
    }

    fn min(self, other: Self) -> Self {
        match self {
            VoidPluginManifestMember::Name(_) => self,
            VoidPluginManifestMember::Author(_) => match other {
                VoidPluginManifestMember::Name(_) => other,
                VoidPluginManifestMember::Version(_) => self,
                VoidPluginManifestMember::Author(_) => self,
            },
            VoidPluginManifestMember::Version(_) => other,
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl Eq for VoidPluginManifestMember {}

impl PartialEq for VoidPluginManifestMember {
    fn eq(&self, other: &Self) -> bool {
        match self {
            VoidPluginManifestMember::Name(_) => matches!(other, VoidPluginManifestMember::Name(_)),
            VoidPluginManifestMember::Author(_) => {
                matches!(other, VoidPluginManifestMember::Author(_))
            }
            VoidPluginManifestMember::Version(_) => {
                matches!(other, VoidPluginManifestMember::Version(_))
            }
        }
    }
}

impl PartialOrd for VoidPluginManifestMember {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ManifestController for VoidPluginManifestMember {
    type ManifestFieldEnum = VoidPluginManifestMember;
    type ManifestFieldEnumType = VoidPluginManifestMemberVariant;
    fn construct_from_json(
        json: String,
    ) -> Result<Vec<Vec<Self::ManifestFieldEnum>>, void_db::ManifestError> {
        let temp = serde_json::from_str::<Vec<VoidPluginManifestDeserializer>>(&json)
            .map_err(|e| ManifestError::DeserializeError(e.to_string()))?;
        let mut manifest_members = Vec::<Vec<Self::ManifestFieldEnum>>::new();
        for plugin in temp {
            manifest_members.push(vec![
                VoidPluginManifestMember::Name(plugin.name),
                VoidPluginManifestMember::Author(plugin.author),
                VoidPluginManifestMember::Version(plugin.version),
            ]);
        }
        Ok(manifest_members)
    }

    fn sort(unsorted_vec: Vec<Self::ManifestFieldEnum>) -> Vec<Self::ManifestFieldEnum> {
        let mut temp = unsorted_vec;
        temp.sort();
        temp
    }

    fn get_field(
        sorted_vec: Vec<Self::ManifestFieldEnum>,
        field_type: Self::ManifestFieldEnumType,
    ) -> Result<Self::ManifestFieldEnum, ManifestError> {
        if sorted_vec.len() != 3 {
            Err(ManifestError::IncorrectFieldsNumber)
        } else {
            match field_type {
                VoidPluginManifestMemberVariant::Name => Ok(sorted_vec[0].clone()),
                VoidPluginManifestMemberVariant::Author => Ok(sorted_vec[1].clone()),
                VoidPluginManifestMemberVariant::Version => Ok(sorted_vec[2].clone()),
            }
        }
    }
}
