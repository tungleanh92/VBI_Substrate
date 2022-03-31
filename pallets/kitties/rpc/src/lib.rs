// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the transaction payment pallet.

pub use self::gen_client::Client as TransactionPaymentClient;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use pallet_kitties::Kitty;
use pallet_kitties_rpc_runtime_api::KittiesRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_std::vec::Vec;
use std::{sync::Arc, thread::AccessError};
#[rpc]
pub trait KittyInfoApi<BlockHash, Balance, Account, Time>
where
	Balance: std::fmt::Display + std::str::FromStr,
{
	#[rpc(name = "genKitty_push")]
	fn gen_kitty(&self, at: Option<BlockHash>) -> Result<[u8; 16]>;

	#[rpc(name = "kittyInfo_get")]
	fn get_kitty_info(&self, at: Option<BlockHash>) -> Result<Vec<Kitty<Balance, Account, Time>>>;

	#[rpc(name = "kittyQuantity_get")]
	fn get_kitty_quantity(&self, at: Option<BlockHash>) -> Result<u64>;
}

/// A struct that implements the [`TransactionPaymentApi`].
pub struct KittyInfo<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> KittyInfo<C, P> {
	/// Create new `TransactionPayment` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, Balance, Account, Time> KittyInfoApi<<Block as BlockT>::Hash, Balance, Account, Time>
	for KittyInfo<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: KittiesRuntimeApi<Block, Balance, Account, Time>,
	Balance: std::fmt::Display + std::str::FromStr,
	pallet_kitties::Kitty<Balance, Account, Time>: sp_api::Decode,
{
	fn gen_kitty(&self, at: Option<<Block as BlockT>::Hash>) -> Result<[u8; 16]> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let result_api = api.gen_kitty(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

	fn get_kitty_info(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Vec<Kitty<Balance, Account, Time>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let result_api = api.get_kitty_info(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

	fn get_kitty_quantity(&self, at: Option<<Block as BlockT>::Hash>) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let result_api = api.get_kitty_quantity(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}
}
