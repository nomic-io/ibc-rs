//! Definition of domain type message `MsgCreateClient`.

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgCreateClient as RawMsgCreateClient;
use ibc_proto::protobuf::Protobuf;

use crate::core::ics02_client::error::ClientError;
use crate::core::Msg;
use crate::prelude::*;
use crate::signer::Signer;

pub(crate) const TYPE_URL: &str = "/ibc.core.client.v1.MsgCreateClient";

/// A type of message that triggers the creation of a new on-chain (IBC) client.
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MsgCreateClient {
    #[cfg_attr(feature = "schema", schemars(with = "crate::utils::schema::AnySchema"))]
    pub client_state: Any,
    #[cfg_attr(feature = "schema", schemars(with = "crate::utils::schema::AnySchema"))]
    pub consensus_state: Any,
    pub signer: Signer,
}

impl MsgCreateClient {
    pub fn new(client_state: Any, consensus_state: Any, signer: Signer) -> Self {
        MsgCreateClient {
            client_state,
            consensus_state,
            signer,
        }
    }
}

impl Msg for MsgCreateClient {
    type Raw = RawMsgCreateClient;

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
}

impl Protobuf<RawMsgCreateClient> for MsgCreateClient {}

impl TryFrom<RawMsgCreateClient> for MsgCreateClient {
    type Error = ClientError;

    fn try_from(raw: RawMsgCreateClient) -> Result<Self, Self::Error> {
        let raw_client_state = raw.client_state.ok_or(ClientError::MissingRawClientState)?;

        let raw_consensus_state = raw
            .consensus_state
            .ok_or(ClientError::MissingRawConsensusState)?;

        Ok(MsgCreateClient::new(
            raw_client_state,
            raw_consensus_state,
            raw.signer.into(),
        ))
    }
}

impl From<MsgCreateClient> for RawMsgCreateClient {
    fn from(ics_msg: MsgCreateClient) -> Self {
        RawMsgCreateClient {
            client_state: Some(ics_msg.client_state),
            consensus_state: Some(ics_msg.consensus_state),
            signer: ics_msg.signer.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use ibc_proto::ibc::core::client::v1::MsgCreateClient as RawMsgCreateClient;
    use test_log::test;

    use crate::clients::ics07_tendermint::client_state::ClientState as TmClientState;
    use crate::clients::ics07_tendermint::consensus_state::ConsensusState as TmConsensusState;
    use crate::clients::ics07_tendermint::header::test_util::get_dummy_tendermint_header;
    use crate::core::ics02_client::msgs::create_client::MsgCreateClient;
    use crate::test_utils::get_dummy_account_id;

    #[test]
    fn msg_create_client_serialization() {
        let signer = get_dummy_account_id();

        let tm_header = get_dummy_tendermint_header();
        let tm_client_state = TmClientState::new_dummy_from_header(tm_header.clone()).into();

        let msg = MsgCreateClient::new(
            tm_client_state,
            TmConsensusState::try_from(tm_header).unwrap().into(),
            signer,
        );

        let raw = RawMsgCreateClient::from(msg.clone());
        let msg_back = MsgCreateClient::try_from(raw.clone()).unwrap();
        let raw_back = RawMsgCreateClient::from(msg_back.clone());
        assert_eq!(msg, msg_back);
        assert_eq!(raw, raw_back);
    }
}
