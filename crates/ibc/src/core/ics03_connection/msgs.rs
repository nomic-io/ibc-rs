//! Message definitions for the connection handshake datagrams.
//!
//! We define each of the four messages in the connection handshake protocol as a `struct`.
//! Each such message comprises the same fields as the datagrams defined in ICS3 English spec:
//! <https://github.com/cosmos/ibc/tree/master/spec/core/ics-003-connection-semantics>.
//!
//! One departure from ICS3 is that we abstract the three counterparty fields (connection id,
//! prefix, and client id) into a single field of type `Counterparty`; this applies to messages
//! `MsgConnectionOpenInit` and `MsgConnectionOpenTry`. One other difference with regards to
//! abstraction is that all proof-related attributes in a message are encapsulated in `Proofs` type.
//!
//! Another difference to ICS3 specs is that each message comprises an additional field called
//! `signer` which is specific to Cosmos-SDK.

use crate::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck;
use crate::core::ics03_connection::msgs::conn_open_confirm::MsgConnectionOpenConfirm;
use crate::core::ics03_connection::msgs::conn_open_init::MsgConnectionOpenInit;
use crate::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry;
use crate::prelude::*;

pub mod conn_open_ack;
pub mod conn_open_confirm;
pub mod conn_open_init;
pub mod conn_open_try;

/// Enumeration of all possible messages that the ICS3 protocol processes.
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConnectionMsg {
    OpenInit(MsgConnectionOpenInit),
    OpenTry(MsgConnectionOpenTry),
    OpenAck(MsgConnectionOpenAck),
    OpenConfirm(MsgConnectionOpenConfirm),
}

#[cfg(test)]
pub mod test_util {

    use ibc_proto::ibc::core::commitment::v1::MerklePrefix;
    use ibc_proto::ibc::core::connection::v1::Counterparty as RawCounterparty;

    use crate::core::ics24_host::identifier::{ClientId, ConnectionId};
    use crate::prelude::*;

    pub fn get_dummy_raw_counterparty(conn_id: Option<u64>) -> RawCounterparty {
        let connection_id = match conn_id {
            Some(id) => ConnectionId::new(id).to_string(),
            None => "".to_string(),
        };
        RawCounterparty {
            client_id: ClientId::default().to_string(),
            connection_id,
            prefix: Some(MerklePrefix {
                key_prefix: b"ibc".to_vec(),
            }),
        }
    }
}
