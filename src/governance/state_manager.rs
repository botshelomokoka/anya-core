use std::sync::Arc;
use tokio::sync::RwLock;
use lru::LruCache;
use serde::{Serialize, Deserialize};
use clarity_sdk::{
    clarity_type::ClarityType,
    types::{Value, ToClarityValue, FromClarityValue},
};
use crate::stacks_client::{StacksContractClient, Error as ClientError};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum StateKey {
    Protocol,
    Proposal(u64),
    Vote(u64, String),
    Token(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolState {
    pub config_parameters: Vec<ConfigParameter>,
    pub active_contracts: Vec<ContractInfo>,
    pub permissions: Vec<PermissionInfo>,
    pub treasury_balance: u64,
}

impl ToClarityValue for ProtocolState {
    fn to_clarity_value(&self) -> Value {
        let mut tuple = Vec::new();
        
        tuple.push(("config-parameters".into(), 
            Value::list(self.config_parameters.iter()
                .map(|p| p.to_clarity_value())
                .collect())
        ));
        
        tuple.push(("active-contracts".into(),
            Value::list(self.active_contracts.iter()
                .map(|c| c.to_clarity_value())
                .collect())
        ));
        
        tuple.push(("permissions".into(),
            Value::list(self.permissions.iter()
                .map(|p| p.to_clarity_value())
                .collect())
        ));
        
        tuple.push(("treasury-balance".into(),
            Value::UInt(self.treasury_balance)
        ));
        
        Value::Tuple(tuple)
    }
}

impl FromClarityValue for ProtocolState {
    fn from_clarity_value(value: Value) -> Option<Self> {
        if let Value::Tuple(tuple) = value {
            let mut state = ProtocolState {
                config_parameters: Vec::new(),
                active_contracts: Vec::new(),
                permissions: Vec::new(),
                treasury_balance: 0,
            };
            
            for (key, value) in tuple {
                match key.as_str() {
                    "config-parameters" => {
                        if let Value::List(params) = value {
                            state.config_parameters = params.iter()
                                .filter_map(|v| ConfigParameter::from_clarity_value(v.clone()))
                                .collect();
                        }
                    },
                    "active-contracts" => {
                        if let Value::List(contracts) = value {
                            state.active_contracts = contracts.iter()
                                .filter_map(|v| ContractInfo::from_clarity_value(v.clone()))
                                .collect();
                        }
                    },
                    "permissions" => {
                        if let Value::List(perms) = value {
                            state.permissions = perms.iter()
                                .filter_map(|v| PermissionInfo::from_clarity_value(v.clone()))
                                .collect();
                        }
                    },
                    "treasury-balance" => {
                        if let Value::UInt(balance) = value {
                            state.treasury_balance = balance;
                        }
                    },
                    _ => {}
                }
            }
            Some(state)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigParameter {
    pub key: String,
    pub value: String,
}

impl ToClarityValue for ConfigParameter {
    fn to_clarity_value(&self) -> Value {
        Value::Tuple(vec![
            ("key".into(), Value::string_ascii(&self.key)?),
            ("value".into(), Value::string_ascii(&self.value)?),
        ])
    }
}

impl FromClarityValue for ConfigParameter {
    fn from_clarity_value(value: Value) -> Option<Self> {
        if let Value::Tuple(tuple) = value {
            let mut param = ConfigParameter {
                key: String::new(),
                value: String::new(),
            };
            
            for (key, value) in tuple {
                match key.as_str() {
                    "key" => {
                        if let Value::String(s) = value {
                            param.key = s;
                        }
                    },
                    "value" => {
                        if let Value::String(s) = value {
                            param.value = s;
                        }
                    },
                    _ => {}
                }
            }
            Some(param)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    pub name: String,
    pub address: String,
    pub version: String,
}

impl ToClarityValue for ContractInfo {
    fn to_clarity_value(&self) -> Value {
        Value::Tuple(vec![
            ("name".into(), Value::string_ascii(&self.name)?),
            ("address".into(), Value::string_ascii(&self.address)?),
            ("version".into(), Value::string_ascii(&self.version)?),
        ])
    }
}

impl FromClarityValue for ContractInfo {
    fn from_clarity_value(value: Value) -> Option<Self> {
        if let Value::Tuple(tuple) = value {
            let mut info = ContractInfo {
                name: String::new(),
                address: String::new(),
                version: String::new(),
            };
            
            for (key, value) in tuple {
                match key.as_str() {
                    "name" => {
                        if let Value::String(s) = value {
                            info.name = s;
                        }
                    },
                    "address" => {
                        if let Value::String(s) = value {
                            info.address = s;
                        }
                    },
                    "version" => {
                        if let Value::String(s) = value {
                            info.version = s;
                        }
                    },
                    _ => {}
                }
            }
            Some(info)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionInfo {
    pub role: String,
    pub address: String,
    pub permissions: Vec<String>,
}

impl ToClarityValue for PermissionInfo {
    fn to_clarity_value(&self) -> Value {
        Value::Tuple(vec![
            ("role".into(), Value::string_ascii(&self.role)?),
            ("address".into(), Value::string_ascii(&self.address)?),
            ("permissions".into(), Value::list(
                self.permissions.iter()
                    .map(|p| Value::string_ascii(p))
                    .collect::<Option<Vec<_>>>()?
            )),
        ])
    }
}

impl FromClarityValue for PermissionInfo {
    fn from_clarity_value(value: Value) -> Option<Self> {
        if let Value::Tuple(tuple) = value {
            let mut info = PermissionInfo {
                role: String::new(),
                address: String::new(),
                permissions: Vec::new(),
            };
            
            for (key, value) in tuple {
                match key.as_str() {
                    "role" => {
                        if let Value::String(s) = value {
                            info.role = s;
                        }
                    },
                    "address" => {
                        if let Value::String(s) = value {
                            info.address = s;
                        }
                    },
                    "permissions" => {
                        if let Value::List(perms) = value {
                            info.permissions = perms.iter()
                                .filter_map(|v| {
                                    if let Value::String(s) = v {
                                        Some(s.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                        }
                    },
                    _ => {}
                }
            }
            Some(info)
        } else {
            None
        }
    }
}

pub struct ProtocolStateManager {
    contract_client: Arc<StacksContractClient>,
    cache: Arc<RwLock<LruCache<StateKey, Box<dyn StateValue>>>>,
}

impl ProtocolStateManager {
    pub fn new(contract_client: StacksContractClient, cache_size: usize) -> Self {
        Self {
            contract_client: Arc::new(contract_client),
            cache: Arc::new(RwLock::new(LruCache::new(cache_size))),
        }
    }

    pub async fn get_protocol_state(&self) -> Result<ProtocolState, ClientError> {
        // Try cache first
        if let Some(state) = self.cache.read().await.get(&StateKey::Protocol) {
            if let Some(protocol_state) = state.as_any().downcast_ref::<ProtocolState>() {
                return Ok(protocol_state.clone());
            }
        }

        // Fetch from blockchain
        let state: Value = self.contract_client
            .call_read_only(
                "protocol",
                "get-protocol-state",
                &[],
            )
            .await?;

        let protocol_state = ProtocolState::from_clarity_value(state)
            .ok_or_else(|| ClientError::DeserializationError)?;

        // Update cache
        self.cache.write().await.put(
            StateKey::Protocol,
            Box::new(protocol_state.clone()),
        );

        Ok(protocol_state)
    }

    pub async fn get_proposal(&self, proposal_id: u64) -> Result<ProposalInfo, ClientError> {
        let key = StateKey::Proposal(proposal_id);

        // Try cache first
        if let Some(state) = self.cache.read().await.get(&key) {
            if let Some(proposal) = state.as_any().downcast_ref::<ProposalInfo>() {
                return Ok(proposal.clone());
            }
        }

        // Fetch from blockchain
        let proposal: Value = self.contract_client
            .call_read_only(
                "dao",
                "get-proposal",
                &[proposal_id],
            )
            .await?;

        let proposal_info = ProposalInfo::from_clarity_value(proposal)
            .ok_or_else(|| ClientError::DeserializationError)?;

        // Update cache
        self.cache.write().await.put(
            key,
            Box::new(proposal_info.clone()),
        );

        Ok(proposal_info)
    }

    pub async fn get_vote(&self, proposal_id: u64, voter: &str) -> Result<VoteInfo, ClientError> {
        let key = StateKey::Vote(proposal_id, voter.to_string());

        // Try cache first
        if let Some(state) = self.cache.read().await.get(&key) {
            if let Some(vote) = state.as_any().downcast_ref::<VoteInfo>() {
                return Ok(vote.clone());
            }
        }

        // Fetch from blockchain
        let vote: Value = self.contract_client
            .call_read_only(
                "dao",
                "get-vote",
                &[proposal_id, voter],
            )
            .await?;

        let vote_info = VoteInfo::from_clarity_value(vote)
            .ok_or_else(|| ClientError::DeserializationError)?;

        // Update cache
        self.cache.write().await.put(
            key,
            Box::new(vote_info.clone()),
        );

        Ok(vote_info)
    }

    pub async fn invalidate_cache(&self, key: StateKey) {
        self.cache.write().await.pop(&key);
    }

    pub async fn clear_cache(&self) {
        self.cache.write().await.clear();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalInfo {
    pub id: u64,
    pub proposer: String,
    pub title: String,
    pub description: String,
    pub start_block: u64,
    pub end_block: u64,
    pub execution_block: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub executed: bool,
    pub canceled: bool,
}

impl ToClarityValue for ProposalInfo {
    fn to_clarity_value(&self) -> Value {
        Value::Tuple(vec![
            ("id".into(), Value::UInt(self.id)),
            ("proposer".into(), Value::string_ascii(&self.proposer)?),
            ("title".into(), Value::string_ascii(&self.title)?),
            ("description".into(), Value::string_ascii(&self.description)?),
            ("start-block".into(), Value::UInt(self.start_block)),
            ("end-block".into(), Value::UInt(self.end_block)),
            ("execution-block".into(), Value::UInt(self.execution_block)),
            ("votes-for".into(), Value::UInt(self.votes_for)),
            ("votes-against".into(), Value::UInt(self.votes_against)),
            ("executed".into(), Value::Bool(self.executed)),
            ("canceled".into(), Value::Bool(self.canceled)),
        ])
    }
}

impl FromClarityValue for ProposalInfo {
    fn from_clarity_value(value: Value) -> Option<Self> {
        if let Value::Tuple(tuple) = value {
            let mut info = ProposalInfo {
                id: 0,
                proposer: String::new(),
                title: String::new(),
                description: String::new(),
                start_block: 0,
                end_block: 0,
                execution_block: 0,
                votes_for: 0,
                votes_against: 0,
                executed: false,
                canceled: false,
            };
            
            for (key, value) in tuple {
                match key.as_str() {
                    "id" => {
                        if let Value::UInt(id) = value {
                            info.id = id;
                        }
                    },
                    "proposer" => {
                        if let Value::String(s) = value {
                            info.proposer = s;
                        }
                    },
                    "title" => {
                        if let Value::String(s) = value {
                            info.title = s;
                        }
                    },
                    "description" => {
                        if let Value::String(s) = value {
                            info.description = s;
                        }
                    },
                    "start-block" => {
                        if let Value::UInt(block) = value {
                            info.start_block = block;
                        }
                    },
                    "end-block" => {
                        if let Value::UInt(block) = value {
                            info.end_block = block;
                        }
                    },
                    "execution-block" => {
                        if let Value::UInt(block) = value {
                            info.execution_block = block;
                        }
                    },
                    "votes-for" => {
                        if let Value::UInt(votes) = value {
                            info.votes_for = votes;
                        }
                    },
                    "votes-against" => {
                        if let Value::UInt(votes) = value {
                            info.votes_against = votes;
                        }
                    },
                    "executed" => {
                        if let Value::Bool(executed) = value {
                            info.executed = executed;
                        }
                    },
                    "canceled" => {
                        if let Value::Bool(canceled) = value {
                            info.canceled = canceled;
                        }
                    },
                    _ => {}
                }
            }
            Some(info)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteInfo {
    pub power: u64,
    pub support: bool,
    pub reason: Option<String>,
    pub timestamp: u64,
}

impl ToClarityValue for VoteInfo {
    fn to_clarity_value(&self) -> Value {
        Value::Tuple(vec![
            ("power".into(), Value::UInt(self.power)),
            ("support".into(), Value::Bool(self.support)),
            ("reason".into(), self.reason.as_ref().map_or(Value::None, |s| Value::string_ascii(s)?)),
            ("timestamp".into(), Value::UInt(self.timestamp)),
        ])
    }
}

impl FromClarityValue for VoteInfo {
    fn from_clarity_value(value: Value) -> Option<Self> {
        if let Value::Tuple(tuple) = value {
            let mut info = VoteInfo {
                power: 0,
                support: false,
                reason: None,
                timestamp: 0,
            };
            
            for (key, value) in tuple {
                match key.as_str() {
                    "power" => {
                        if let Value::UInt(power) = value {
                            info.power = power;
                        }
                    },
                    "support" => {
                        if let Value::Bool(support) = value {
                            info.support = support;
                        }
                    },
                    "reason" => {
                        if let Value::Some(s) = value {
                            if let Value::String(reason) = s {
                                info.reason = Some(reason);
                            }
                        }
                    },
                    "timestamp" => {
                        if let Value::UInt(timestamp) = value {
                            info.timestamp = timestamp;
                        }
                    },
                    _ => {}
                }
            }
            Some(info)
        } else {
            None
        }
    }
}

pub trait StateValue: std::any::Any + Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl StateValue for ProtocolState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl StateValue for ProposalInfo {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl StateValue for VoteInfo {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
