#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct AgentKey {
    pub(crate) domain: String,
    pub(crate) name: String,
}

impl AgentKey {
    pub fn key(&self) -> String {
        format!("{}:{}", self.domain, self.name)
    }
}

#[derive(Debug)]
pub enum AgentError {
    AgentAlreadyExists,
}

/// # Agent is a struct that represents a running agent.
/// - It contains the agent's key, version, description, and active status.
/// - The key is generated by the AgentManager when the agent is registered.
/// - The version is the version of the agent.
/// - The description is a description of the agent.
/// - The active status is a boolean that indicates whether the agent is active or not.
/// - An active agent is one that is running and can be used to execute tasks.
/// - An inactive agent is one that is not running and cannot be used to execute tasks.
/// - An inactive agent can be activated by the AgentManager.
/// - An active agent can be deactivated by the AgentManager.
/// - An agent can be removed by the AgentManager.
/// - An agent can be updated by the AgentManager.
pub struct Agent {
    key: Option<AgentKey>,
    version: String,
    description: String,
    active: bool,
}

impl Agent {
    pub fn new(version: String, description: String) -> Agent {
        Agent {
            key: None,
            version,
            description,
            active: true,
        }
    }
}