use crate::zinit::errors::ZInitError;
use anyhow::Result;
use nix::sys::wait::WaitStatus;

/// Target state for a service
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// Service should be running
    Up,
    /// Service should be stopped
    Down,
}

/// Service state
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    /// Service is in an unknown state
    Unknown,

    /// Blocked means one or more dependencies hasn't been met yet. Service can stay in
    /// this state as long as at least one dependency is not in either Running, or Success
    Blocked,

    /// Service has been started, but it didn't exit yet, or we didn't run the test command.
    Spawned,

    /// Service has been started, and test command passed.
    Running,

    /// Service has exited with success state, only one-shot can stay in this state
    Success,

    /// Service exited with this error, only one-shot can stay in this state
    Error(WaitStatus),

    /// The service test command failed, this might (or might not) be replaced
    /// with an Error state later on once the service process itself exits
    TestFailure,

    /// Failure means the service has failed to spawn in a way that retrying
    /// won't help, like command line parsing error or failed to fork
    Failure,
}

impl State {
    /// Validate if a transition from the current state to the new state is valid
    #[must_use]
    pub const fn can_transition_to(&self, new_state: &Self) -> bool {
        match (self, new_state) {
            // From Unknown state, any transition is valid
            (Self::Unknown, _) => true,

            // From Blocked state
            (Self::Blocked, Self::Spawned) => true,
            (Self::Blocked, Self::Failure) => true,

            // From Spawned state
            (Self::Spawned, Self::Running) => true,
            (Self::Spawned, Self::TestFailure) => true,
            (Self::Spawned, Self::Error(_)) => true,
            (Self::Spawned, Self::Success) => true,

            // From Running state
            (Self::Running, Self::Success) => true,
            (Self::Running, Self::Error(_)) => true,

            // To Unknown or Blocked state is always valid
            (_, Self::Unknown) => true,
            (_, Self::Blocked) => true,

            // Any other transition is invalid
            _ => false,
        }
    }

    /// Attempt to transition to a new state, validating the transition
    pub fn transition_to(&self, new_state: Self) -> Result<Self, ZInitError> {
        if self.can_transition_to(&new_state) {
            Ok(new_state)
        } else {
            Err(ZInitError::invalid_state_transition(format!(
                "Invalid transition from {self:?} to {new_state:?}"
            )))
        }
    }

    /// Check if the state is considered "active" (running or in progress)
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self, Self::Running | Self::Spawned)
    }

    /// Check if the state is considered "terminal" (success or failure)
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::Success | Self::Error(_) | Self::Failure)
    }

    /// Check if the state is considered "successful"
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        matches!(self, Self::Success | Self::Running)
    }

    /// Check if the state is considered "failed"
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        matches!(self, Self::Error(_) | Self::Failure | Self::TestFailure)
    }
}
