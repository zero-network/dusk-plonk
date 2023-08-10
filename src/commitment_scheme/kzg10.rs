// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! Implementation of the KZG10 polynomial commitment scheme.

pub mod key;

pub(crate) use proof::AggregateProof;

pub use key::OpeningKey;

pub(crate) mod proof;
