/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecorderError {
    #[error("recorder initialization error: `{0}`")]
    InitError(String),
    #[error("recording start error: `{0}`")]
    StartError(String),
    #[error("recording stop error: `{0}`")]
    StopError(String),
    #[error("recorder not initialized properly")]
    NotInitialized,
}
