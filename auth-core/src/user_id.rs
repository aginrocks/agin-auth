use std::{fmt::Debug, hash::Hash};

use serde::{Deserialize, Serialize};

mod private {
    /// Used to prevent other crates from implementing [`IdLike`].
    pub trait Sealed {}
}

pub trait IdLike:
    Clone
    + Copy
    + PartialEq
    + Eq
    + Debug
    + Hash
    + PartialOrd
    + Ord
    + Serialize
    + for<'de> Deserialize<'de>
    + private::Sealed
{
    fn as_i32(&self) -> i32;
}

/// Flow state indicates that the user *probably* has the claimed ID.
///
/// Do **NOT** grant privilieges based on this claim.
///
/// This ID is intended to be used in authentication flows.
///
/// See [`UserId`] for the verified counterpart.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ClaimedUserId(i32);

impl ClaimedUserId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
}

impl IdLike for ClaimedUserId {
    fn as_i32(&self) -> i32 {
        self.0
    }
}

/// Presence of this struct implies that the user is fully authenticated.
///
/// See [`ClaimedUserId`] for the unverified counterpart.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct UserId(i32);

impl UserId {
    /// Only construct if you're certain that the user is **fully authenticated**.
    pub fn from_verified(id: i32) -> Self {
        Self(id)
    }
}

impl IdLike for UserId {
    fn as_i32(&self) -> i32 {
        self.0
    }
}

impl private::Sealed for ClaimedUserId {}
impl private::Sealed for UserId {}
