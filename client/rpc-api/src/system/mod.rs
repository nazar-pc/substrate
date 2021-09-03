// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate system API.

use jsonrpsee::{
	proc_macros::rpc,
	types::{JsonRpcResult, JsonValue},
};

pub use self::helpers::{Health, NodeRole, PeerInfo, SyncState, SystemInfo};

pub mod error;
pub mod helpers;

/// Substrate system RPC API
#[rpc(client, server, namespace = "system")]
pub trait SystemApi<Hash, Number> {
	/// Get the node's implementation name. Plain old string.
	#[method(name = "name")]
	fn system_name(&self) -> JsonRpcResult<String>;

	/// Get the node implementation's version. Should be a semver string.
	#[method(name = "version")]
	fn system_version(&self) -> JsonRpcResult<String>;

	/// Get the chain's name. Given as a string identifier.
	#[method(name = "chain")]
	fn system_chain(&self) -> JsonRpcResult<String>;

	/// Get the chain's type.
	#[method(name = "chainType")]
	fn system_type(&self) -> JsonRpcResult<sc_chain_spec::ChainType>;

	/// Get a custom set of properties as a JSON object, defined in the chain spec.
	#[method(name = "properties")]
	fn system_properties(&self) -> JsonRpcResult<sc_chain_spec::Properties>;

	/// Return health status of the node.
	///
	/// Node is considered healthy if it is:
	/// - connected to some peers (unless running in dev mode)
	/// - not performing a major sync
	#[method(name = "health")]
	async fn system_health(&self) -> JsonRpcResult<Health>;

	/// Returns the base58-encoded PeerId of the node.
	#[method(name = "localPeerId")]
	async fn system_local_peer_id(&self) -> JsonRpcResult<String>;

	/// Returns the multi-addresses that the local node is listening on
	///
	/// The addresses include a trailing `/p2p/` with the local PeerId, and are thus suitable to
	/// be passed to `addReservedPeer` or as a bootnode address for example.
	#[method(name = "localListenAddresses")]
	async fn system_local_listen_addresses(&self) -> JsonRpcResult<Vec<String>>;

	/// Returns currently connected peers
	#[method(name = "peers")]
	async fn system_peers(&self) -> JsonRpcResult<Vec<PeerInfo<Hash, Number>>>;

	/// Returns current state of the network.
	///
	/// **Warning**: This API is not stable. Please do not programmatically interpret its output,
	/// as its format might change at any time.
	// TODO: the future of this call is uncertain: https://github.com/paritytech/substrate/issues/1890
	// https://github.com/paritytech/substrate/issues/5541
	#[method(name = "unstable_networkState")]
	async fn system_network_state(&self) -> JsonRpcResult<JsonValue>;

	/// Adds a reserved peer. Returns the empty string or an error. The string
	/// parameter should encode a `p2p` multiaddr.
	///
	/// `/ip4/198.51.100.19/tcp/30333/p2p/QmSk5HQbn6LhUwDiNMseVUjuRYhEtYj4aUZ6WfWoGURpdV`
	/// is an example of a valid, passing multiaddr with PeerId attached.
	#[method(name = "addReservedPeer")]
	async fn system_add_reserved_peer(&self, peer: String) -> JsonRpcResult<()>;

	/// Remove a reserved peer. Returns the empty string or an error. The string
	/// should encode only the PeerId e.g. `QmSk5HQbn6LhUwDiNMseVUjuRYhEtYj4aUZ6WfWoGURpdV`.
	#[method(name = "removeReservedPeer")]
	async fn system_remove_reserved_peer(&self, peer_id: String) -> JsonRpcResult<()>;

	/// Returns the list of reserved peers
	#[method(name = "reservedPeers")]
	async fn system_reserved_peers(&self) -> JsonRpcResult<Vec<String>>;

	/// Returns the roles the node is running as.
	#[method(name = "nodeRoles")]
	async fn system_node_roles(&self) -> JsonRpcResult<Vec<NodeRole>>;

	/// Returns the state of the syncing of the node: starting block, current best block, highest
	/// known block.
	#[method(name = "syncState")]
	async fn system_sync_state(&self) -> JsonRpcResult<SyncState<Number>>;

	/// Adds the supplied directives to the current log filter
	///
	/// The syntax is identical to the CLI `<target>=<level>`:
	///
	/// `sync=debug,state=trace`
	#[method(name = "addLogFilter")]
	fn system_add_log_filter(&self, directives: String) -> JsonRpcResult<()>;

	/// Resets the log filter to Substrate defaults
	#[method(name = "resetLogFilter")]
	fn system_reset_log_filter(&self) -> JsonRpcResult<()>;
}
