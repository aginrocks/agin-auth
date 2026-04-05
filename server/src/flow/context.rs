use auth_core::FlowClaims;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::factors::FactorName;

/// A single step in a flow, bound to a single [`Factor`].
///
/// A step represents a factor invocation.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Step {
    pub factor: FactorName,

    /// Claims made by that specific factor.
    pub claims: FlowClaims,
}

/// Context and history of the specified flow.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FlowContext {
    /// Previously completed steps in this flow.
    pub completed_steps: Vec<Step>,

    /// Latest claims returned from the last step.
    pub current_claims: FlowClaims,
}

impl FlowContext {
    const STORAGE_KEY: &str = "flow";

    pub async fn from_session(session: &Session) -> Result<Self, tower_sessions::session::Error> {
        let context = session.get::<Self>(Self::STORAGE_KEY).await?;
        Ok(context.unwrap_or_default())
    }
}
