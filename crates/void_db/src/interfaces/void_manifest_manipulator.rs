/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::ManifestError;

/// # Simple trait that required for those types of data that uses manifests for storing data in
/// databases
pub trait ManifestController {
    /// PartialOrd, PartialEq, Ord traits required for simplier sort functions
    type ManifestFieldEnum: PartialEq + PartialOrd + Ord;

    /// Simple type for better function contracts
    type ManifestFieldEnumType;

    /// This function should basically perform deserealization for your manifest.
    /// Doesn't matter you use json yaml or smth different.
    fn construct_from_json(
        json: String,
    ) -> Result<Vec<Vec<Self::ManifestFieldEnum>>, ManifestError>;

    /// This function is basically a wrapper around <vec>.sort() function which will be provided
    /// from Ord implementation
    fn sort(unsorted_vec: Vec<Self::ManifestFieldEnum>) -> Vec<Self::ManifestFieldEnum>;

    /// This function You will use a lot later for simplier database structures constructions
    fn get_field(
        sorted_vec: Vec<Self::ManifestFieldEnum>,
        field_type: Self::ManifestFieldEnumType,
    ) -> Result<Self::ManifestFieldEnum, ManifestError>;
}
