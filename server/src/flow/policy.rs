use auth_core::{FactorMetadataDynamic, FactorRole};

use crate::flow::context::FlowContext;

/// A policy engine that is instantiated per step in order to perform a decision.
pub struct PolicyEngine {
    context: FlowContext,
}

pub enum PolicyError {}

impl PolicyEngine {
    pub fn from_context(context: FlowContext) -> Self {
        Self { context }
    }

    /// Determines if the user can use a specified factor.
    pub fn can_use<F: FactorMetadataDynamic>(&self, factor: &F) -> Result<(), PolicyError> {
        if factor.role() == FactorRole::MultiFactorOnly {}
        Ok(())
    }
}
