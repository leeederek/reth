use reth_primitives::{serde_helper::num::from_int_or_hex_opt, Address, Bytes, B256, U256};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// <https://github.com/ethereum/go-ethereum/blob/91cb6f863a965481e51d5d9c0e5ccd54796fd967/eth/tracers/native/prestate.go#L38>
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PreStateFrame {
    Default(PreStateMode),
    Diff(DiffMode),
}

impl PreStateFrame {
    /// Returns true if this trace was requested without diffmode.
    pub fn is_default(&self) -> bool {
        matches!(self, PreStateFrame::Default(_))
    }

    /// Returns true if this trace was requested with diffmode.
    pub fn is_diff(&self) -> bool {
        matches!(self, PreStateFrame::Diff(_))
    }

    /// Returns the account states after the transaction is executed if this trace was requested
    /// without diffmode.
    pub fn as_default(&self) -> Option<&PreStateMode> {
        match self {
            PreStateFrame::Default(mode) => Some(mode),
            _ => None,
        }
    }

    /// Returns the account states before and after the transaction is executed if this trace was
    /// requested with diffmode.
    pub fn as_diff(&self) -> Option<&DiffMode> {
        match self {
            PreStateFrame::Diff(mode) => Some(mode),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreStateMode(pub BTreeMap<Address, AccountState>);

/// Represents the account states before and after the transaction is executed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiffMode {
    /// The account states after the transaction is executed.
    pub post: BTreeMap<Address, AccountState>,
    /// The account states before the transaction is executed.
    pub pre: BTreeMap<Address, AccountState>,
}

/// Represents the state of an account
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountState {
    #[serde(
        default,
        deserialize_with = "from_int_or_hex_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance: Option<U256>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<Bytes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub storage: BTreeMap<B256, B256>,
}

impl AccountState {
    /// Creates a new `AccountState` with the given account info.
    ///
    /// If balance is zero, it will be omitted.
    /// If nonce is zero, it will be omitted.
    /// If code is empty, it will be omitted.
    pub fn from_account_info(nonce: u64, balance: U256, code: Option<Bytes>) -> Self {
        Self {
            balance: (balance != U256::ZERO).then_some(balance),
            code: code.filter(|code| !code.is_empty()),
            nonce: (nonce != 0).then_some(nonce),
            storage: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreStateConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_mode: Option<bool>,
}

impl PreStateConfig {
    pub fn is_diff_mode(&self) -> bool {
        self.diff_mode.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace::geth::*;

    // See <https://github.com/ethereum/go-ethereum/tree/master/eth/tracers/internal/tracetest/testdata>
    const DEFAULT: &str = include_str!("../../../../test_data/pre_state_tracer/default.json");
    const LEGACY: &str = include_str!("../../../../test_data/pre_state_tracer/legacy.json");
    const DIFF_MODE: &str = include_str!("../../../../test_data/pre_state_tracer/diff_mode.json");

    #[test]
    fn test_serialize_pre_state_trace() {
        let mut opts = GethDebugTracingCallOptions::default();
        opts.tracing_options.config.disable_storage = Some(false);
        opts.tracing_options.tracer =
            Some(GethDebugTracerType::BuiltInTracer(GethDebugBuiltInTracerType::PreStateTracer));
        opts.tracing_options.tracer_config =
            serde_json::to_value(PreStateConfig { diff_mode: Some(true) }).unwrap().into();

        assert_eq!(
            serde_json::to_string(&opts).unwrap(),
            r#"{"disableStorage":false,"tracer":"prestateTracer","tracerConfig":{"diffMode":true}}"#
        );
    }

    #[test]
    fn test_deserialize_pre_state_trace() {
        let trace: PreStateFrame = serde_json::from_str(DEFAULT).unwrap();
        match trace {
            PreStateFrame::Default(PreStateMode(_)) => {}
            _ => unreachable!(),
        }
        let _trace: PreStateFrame = serde_json::from_str(LEGACY).unwrap();
        let trace: PreStateFrame = serde_json::from_str(DIFF_MODE).unwrap();
        match trace {
            PreStateFrame::Diff(DiffMode { pre: _pre, post: _post }) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_is_diff_mode() {
        assert!(PreStateConfig { diff_mode: Some(true) }.is_diff_mode());
        assert!(!PreStateConfig { diff_mode: Some(false) }.is_diff_mode());
        assert!(!PreStateConfig { diff_mode: None }.is_diff_mode());
    }

    #[test]
    fn parse_prestate_default_resp() {
        let s = r#"{
  "0x0000000000000000000000000000000000000002": {
    "balance": "0x0"
  },
  "0x008b3b2f992c0e14edaa6e2c662bec549caa8df1": {
    "balance": "0x2638035a26d133809"
  },
  "0x35a9f94af726f07b5162df7e828cc9dc8439e7d0": {
    "balance": "0x7a48734599f7284",
    "nonce": 1133
  },
  "0xc8ba32cab1757528daf49033e3673fae77dcf05d": {
    "balance": "0x0",
    "code": "0x",
    "nonce": 1,
    "storage": {
      "0x0000000000000000000000000000000000000000000000000000000000000000": "0x000000000000000000000000000000000000000000000000000000000024aea6",
      "0x59fb7853eb21f604d010b94c123acbeae621f09ce15ee5d7616485b1e78a72e9": "0x00000000000000c42b56a52aedf18667c8ae258a0280a8912641c80c48cd9548",
      "0x8d8ebb65ec00cb973d4fe086a607728fd1b9de14aa48208381eed9592f0dee9a": "0x00000000000000784ae4881e40b1f5ebb4437905fbb8a5914454123b0293b35f",
      "0xff896b09014882056009dedb136458f017fcef9a4729467d0d00b4fd413fb1f1": "0x000000000000000e78ac39cb1c20e9edc753623b153705d0ccc487e31f9d6749"
    }
  }
}
"#;
        let pre_state: PreStateFrame = serde_json::from_str(s).unwrap();
        assert!(pre_state.is_default());
    }
    #[test]
    fn parse_prestate_diff_resp() {
        let s = r#"{
  "post": {
    "0x35a9f94af726f07b5162df7e828cc9dc8439e7d0": {
      "nonce": 1135
    }
  },
  "pre": {
    "0x35a9f94af726f07b5162df7e828cc9dc8439e7d0": {
      "balance": "0x7a48429e177130a",
      "nonce": 1134
    }
  }
}
"#;
        let pre_state: PreStateFrame = serde_json::from_str(s).unwrap();
        assert!(pre_state.is_diff());
    }
}
