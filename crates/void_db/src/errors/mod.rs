/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use thiserror::Error;

#[derive(PartialEq, Eq, Debug, Error)]
pub enum DbError {
    #[error("error while creating entity: `{0}`")]
    CreateError(String),
    #[error("error reading database: `{0}`")]
    ReadError(String),
    #[error("error updating entity: `{0}`")]
    UpdateError(String),
    #[error("error deleting entity: `{0}`")]
    DeleteError(String),
    #[error("fuck")]
    OperationNotPermitted,
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("error while deserializing entity: `{0}`")]
    DeserializeError(String),
    #[error("error in manifest, object doesn't have required fields or have too much fields")]
    IncorrectFieldsNumber,
}
