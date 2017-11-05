// Copyright (C) 2020 Annie Lai Investments Limited
// This file is part of Benson.
//
// Benson is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Benson is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Benson.  If not, see <http://www.gnu.org/licenses/>.

#![allow(dead_code)]
use benson_runtime::{constants::asset::*, Runtime, VERSION};
use benson_testing::keyring::*;
use core::convert::TryFrom;
use crml_cennzx_spot::{FeeRate, PerMilli, PerMillion};
use pallet_contracts::{Gas, Schedule};

pub const GENESIS_HASH: [u8; 32] = [69u8; 32];
pub const SPEC_VERSION: u32 = VERSION.spec_version;

#[derive(Default)]
pub struct ExtBuilder {
	initial_balance: u128,
	gas_price: u128,
	// Configurable prices for certain gas metered operations
	gas_sandbox_data_read_cost: Gas,
	gas_regular_op_cost: Gas,
}

impl ExtBuilder {
	pub fn initial_balance(mut self, initial_balance: u128) -> Self {
		self.initial_balance = initial_balance;
		self
	}
	pub fn gas_price(mut self, gas_price: u128) -> Self {
		self.gas_price = gas_price;
		self
	}
	pub fn gas_sandbox_data_read_cost<T: Into<Gas>>(mut self, cost: T) -> Self {
		self.gas_sandbox_data_read_cost = cost.into();
		self
	}
	pub fn gas_regular_op_cost<T: Into<Gas>>(mut self, cost: T) -> Self {
		self.gas_regular_op_cost = cost.into();
		self
	}
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();
		crml_cennzx_spot::GenesisConfig::<Runtime> {
			fee_rate: FeeRate::<PerMillion>::try_from(FeeRate::<PerMilli>::from(3u128)).unwrap(),
			core_asset_id: CENTRAPAY_ASSET_ID,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		// Configure the gas schedule
		let mut gas_price_schedule = Schedule::default();
		gas_price_schedule.sandbox_data_read_cost = self.gas_sandbox_data_read_cost;
		gas_price_schedule.regular_op_cost = self.gas_regular_op_cost;

		pallet_contracts::GenesisConfig::<Runtime> {
			current_schedule: gas_price_schedule,
			gas_price: self.gas_price,
		}
		.assimilate_storage(&mut t)
		.unwrap();
		pallet_generic_asset::GenesisConfig::<Runtime> {
			assets: vec![
				CENNZ_ASSET_ID,
				CENTRAPAY_ASSET_ID,
				PLUG_ASSET_ID,
				SYLO_ASSET_ID,
				CERTI_ASSET_ID,
				ARDA_ASSET_ID,
			],
			initial_balance: self.initial_balance,
			endowed_accounts: vec![alice(), bob(), charlie(), dave(), eve(), ferdie()],
			next_asset_id: NEXT_ASSET_ID,
			staking_asset_id: STAKING_ASSET_ID,
			spending_asset_id: SPENDING_ASSET_ID,
		}
		.assimilate_storage(&mut t)
		.unwrap();
		t.into()
	}
}

/// Test contracts
pub mod contracts {

	/// Contract WABT for reading 32 bytes from memory
	pub const CONTRACT_READ_32_BYTES: &str = r#"
	(module
		(import "env" "ext_scratch_read" (func $ext_scratch_read (param i32 i32 i32)))
		(import "env" "memory" (memory 1 1))
		(func (export "deploy"))
		(func (export "call")
			(call $ext_scratch_read
				(i32.const 0)
				(i32.const 0)
				(i32.const 4)
			)
		)

		;; 32 bytes for reading
		(data (i32.const 4)
			"\09\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
		)
	)"#;

	/// Contract WABT for a contract which will fail during execution
	pub const CONTRACT_WITH_TRAP: &str = r#"
	(module
		(import "env" "ext_scratch_read" (func $ext_scratch_read (param i32 i32 i32)))
		(import "env" "memory" (memory 1 1))
		(func (export "deploy"))
		(func (export "call")
			unreachable
		)
	)"#;

	/// Contract WABT for a contract which dispatches a generic asset transfer of CENNZ to charlie
	pub const CONTRACT_WITH_GA_TRANSFER: &str = r#"
	(module
		(import "env" "ext_dispatch_call" (func $ext_dispatch_call (param i32 i32)))
		(import "env" "memory" (memory 1 1))
		(func (export "call")
			(call $ext_dispatch_call
				(i32.const 8) ;; Pointer to the start of encoded call buffer
				(i32.const 42) ;; Length of the buffer
			)
		)
		(func (export "deploy"))
		(data (i32.const 8) "\06\01\01\FA\90\B5\AB\20\5C\69\74\C9\EA\84\1B\E6\88\86\46\33\DC\9C\A8\A3\57\84\3E\EA\CF\23\14\64\99\65\FE\22\07\00\10\A5\D4\E8")
	)"#;
}
