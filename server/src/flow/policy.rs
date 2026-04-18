use std::fmt::Display;

use auth_core::{FactorMetadataDynamic, FactorRole, FactorSlugDynamic, SecurityLevel};
use itertools::Itertools;
use strum::{Display, IntoEnumIterator};
use thiserror::Error;

use crate::{factors::FactorName, flow::context::FlowContext};

/// A policy engine that is instantiated per step in order to perform a decision.
pub struct PolicyEngine {
    context: FlowContext,

    /// Factors that are enabled by the user
    available_factors: Vec<FactorName>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FactorNeeded {
    one_of: Vec<FactorName>,
}

impl Display for FactorNeeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let factors = &self.one_of.iter().join(",");
        write!(f, "One of the following factors is required: {factors}")
    }
}

#[derive(Display, Debug)]
pub enum ForbiddenReason {
    #[strum(serialize = "Previous factor is required")]
    FactorNeeded(FactorNeeded),
}

#[derive(Error, Debug)]
pub enum PolicyError {
    #[error("Forbidden: {0}")]
    Forbidden(ForbiddenReason),

    #[error("Already authenticated")]
    AlreadyAuthenticated,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuthenticationStep {
    /// User is fully authenticated and can be granted priviledges.
    Authenticated,

    /// Authentication is needed using one of the following factors.
    FactorNeeded(FactorNeeded),
}

/// This policy engine is currently a simple PoC that just enforces 2FA.
///
/// In the future, it will be replaced with a robust flow system.
impl PolicyEngine {
    pub fn from_context(context: FlowContext, available_factors: Vec<FactorName>) -> Self {
        Self {
            context,
            available_factors,
        }
    }

    /// Determines if the user can use a specified factor.
    pub fn can_use<F: FactorSlugDynamic>(&self, factor: &F) -> Result<(), PolicyError> {
        match self.next_step() {
            AuthenticationStep::Authenticated => Err(PolicyError::AlreadyAuthenticated),
            AuthenticationStep::FactorNeeded(needed) => {
                if needed
                    .one_of
                    .iter()
                    .any(|f: &FactorName| f.slug() == factor.slug())
                {
                    Ok(())
                } else {
                    Err(PolicyError::Forbidden(ForbiddenReason::FactorNeeded(
                        needed,
                    )))
                }
            }
        }
    }

    /// Determines the next step in the authentication process.
    pub fn next_step(&self) -> AuthenticationStep {
        let primary_completed = self
            .context
            .completed_steps
            .iter()
            .any(|f| f.factor.role() == FactorRole::Primary);

        // Hardware factors that can be used as a primary factor (like Passkeys) bypass multi-factor authentication
        let bypasses_mfa = self.context.completed_steps.iter().any(|f| {
            f.factor.role() == FactorRole::Primary
                && f.factor.security_level() == SecurityLevel::Hardware
        });
        if bypasses_mfa {
            return AuthenticationStep::Authenticated;
        }

        let next_factors: Vec<FactorName> = if primary_completed {
            // Primary factor completed, so only facotrs with highier security levels are allowed
            self.available_factors
                .iter()
                .copied()
                .filter(|f| {
                    matches!(
                        f.security_level(),
                        SecurityLevel::Hardware
                            | SecurityLevel::Possession
                            | SecurityLevel::OutOfBand
                    )
                })
                .collect()
        } else {
            // Primary factor not completed, so we allow any factor that is not marked as MFA-only
            self.available_factors
                .iter()
                .copied()
                .filter(|f| f.role() == FactorRole::Primary)
                .collect()
        };

        let next_factors = next_factors
            .into_iter()
            .filter(|factor| {
                self.context
                    .completed_steps
                    .iter()
                    .any(|step| step.factor == *factor)
            })
            .collect::<Vec<_>>();

        if next_factors.is_empty() {
            return AuthenticationStep::Authenticated;
        }

        AuthenticationStep::FactorNeeded(FactorNeeded {
            one_of: next_factors,
        })
    }
}
