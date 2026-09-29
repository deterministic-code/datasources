#![allow(non_local_definitions)]

use crate::sabi_value::{SabiCell, SabiResponse, SabiValue};
use abi_stable::std_types::{RBox, ROption, RResult, RString, RVec};

#[abi_stable::sabi_trait]
pub trait SabiReader: Send + Sync {
    fn current(&self) -> ROption<RVec<SabiCell>>;
    fn next(&self) -> RResult<bool, RString>;
    fn next_result(&self) -> RResult<bool, RString>;
}

pub type SabiReaderBox = SabiReader_TO<'static, RBox<()>>;

#[abi_stable::sabi_trait]
pub trait SabiTransaction: Send + Sync {
    fn isolation(&self) -> u8;
    fn commit(&self) -> RResult<(), RString>;
    fn abort(&self) -> RResult<(), RString>;
}

pub type SabiTransactionBox = SabiTransaction_TO<'static, RBox<()>>;

#[abi_stable::sabi_trait]
pub trait SabiCommand: Send + Sync {
    fn execute(
        &self,
        text: RString,
        parameters: ROption<RVec<crate::sabi_value::SabiParameter>>,
    ) -> RResult<SabiResponse, RString>;
    fn execute_reader(
        &self,
        text: RString,
        parameters: ROption<RVec<crate::sabi_value::SabiParameter>>,
    ) -> RResult<SabiReaderBox, RString>;
}

pub type SabiCommandBox = SabiCommand_TO<'static, RBox<()>>;

#[abi_stable::sabi_trait]
pub trait SabiConnection: Send + Sync {
    fn slot(&self) -> u64;
    fn provider(&self) -> RString;
    fn is_open(&self) -> bool;
    fn create_command(&self) -> SabiCommandBox;
    fn open(&self) -> RResult<(), RString>;
    fn close(&self) -> RResult<(), RString>;
    fn start_transaction(&self, isolation: ROption<u8>) -> RResult<SabiTransactionBox, RString>;
}

pub type SabiConnectionBox = SabiConnection_TO<'static, RBox<()>>;

#[abi_stable::sabi_trait]
pub trait SabiDriver: Send + Sync {
    fn connection(&self, options: RVec<SabiCell>) -> SabiConnectionBox;
    fn command(&self, slot: u64) -> SabiCommandBox;
    fn transaction(&self, slot: u64, isolation: u8) -> SabiTransactionBox;
    fn column_type(&self, name: RString) -> RResult<RString, RString>;
    fn converter_names(&self) -> RVec<RString>;
    fn convert(&self, name: RString, value: SabiValue, reverse: bool) -> RResult<SabiValue, RString>;
}

pub type SabiDriverBox = SabiDriver_TO<'static, RBox<()>>;
