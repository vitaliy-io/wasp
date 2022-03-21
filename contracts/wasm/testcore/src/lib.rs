// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use testcore::*;
use wasmlib::*;

use crate::consts::*;
use crate::params::*;
use crate::results::*;
use crate::state::*;

mod consts;
mod contract;
mod params;
mod results;
mod state;

mod testcore;

const EXPORT_MAP: ScExportMap = ScExportMap {
    names: &[
    	FUNC_CALL_ON_CHAIN,
    	FUNC_CHECK_CONTEXT_FROM_FULL_EP,
    	FUNC_CLAIM_ALLOWANCE,
    	FUNC_DO_NOTHING,
    	FUNC_ESTIMATE_MIN_DUST,
    	FUNC_INC_COUNTER,
    	FUNC_INFINITE_LOOP,
    	FUNC_INIT,
    	FUNC_PASS_TYPES_FULL,
    	FUNC_PING_ALLOWANCE_BACK,
    	FUNC_RUN_RECURSION,
    	FUNC_SEND_LARGE_REQUEST,
    	FUNC_SEND_NF_TS_BACK,
    	FUNC_SEND_TO_ADDRESS,
    	FUNC_SET_INT,
    	FUNC_SPAWN,
    	FUNC_SPLIT_FUNDS,
    	FUNC_SPLIT_FUNDS_NATIVE_TOKENS,
    	FUNC_TEST_BLOCK_CONTEXT1,
    	FUNC_TEST_BLOCK_CONTEXT2,
    	FUNC_TEST_CALL_PANIC_FULL_EP,
    	FUNC_TEST_CALL_PANIC_VIEW_EP_FROM_FULL,
    	FUNC_TEST_CHAIN_OWNER_ID_FULL,
    	FUNC_TEST_EVENT_LOG_DEPLOY,
    	FUNC_TEST_EVENT_LOG_EVENT_DATA,
    	FUNC_TEST_EVENT_LOG_GENERIC_DATA,
    	FUNC_TEST_PANIC_FULL_EP,
    	FUNC_WITHDRAW_FROM_CHAIN,
    	VIEW_CHECK_CONTEXT_FROM_VIEW_EP,
    	VIEW_FIBONACCI,
    	VIEW_GET_COUNTER,
    	VIEW_GET_INT,
    	VIEW_GET_STRING_VALUE,
    	VIEW_INFINITE_LOOP_VIEW,
    	VIEW_JUST_VIEW,
    	VIEW_PASS_TYPES_VIEW,
    	VIEW_TEST_CALL_PANIC_VIEW_EP_FROM_VIEW,
    	VIEW_TEST_CHAIN_OWNER_ID_VIEW,
    	VIEW_TEST_PANIC_VIEW_EP,
    	VIEW_TEST_SANDBOX_CALL,
	],
    funcs: &[
    	func_call_on_chain_thunk,
    	func_check_context_from_full_ep_thunk,
    	func_claim_allowance_thunk,
    	func_do_nothing_thunk,
    	func_estimate_min_dust_thunk,
    	func_inc_counter_thunk,
    	func_infinite_loop_thunk,
    	func_init_thunk,
    	func_pass_types_full_thunk,
    	func_ping_allowance_back_thunk,
    	func_run_recursion_thunk,
    	func_send_large_request_thunk,
    	func_send_nf_ts_back_thunk,
    	func_send_to_address_thunk,
    	func_set_int_thunk,
    	func_spawn_thunk,
    	func_split_funds_thunk,
    	func_split_funds_native_tokens_thunk,
    	func_test_block_context1_thunk,
    	func_test_block_context2_thunk,
    	func_test_call_panic_full_ep_thunk,
    	func_test_call_panic_view_ep_from_full_thunk,
    	func_test_chain_owner_id_full_thunk,
    	func_test_event_log_deploy_thunk,
    	func_test_event_log_event_data_thunk,
    	func_test_event_log_generic_data_thunk,
    	func_test_panic_full_ep_thunk,
    	func_withdraw_from_chain_thunk,
	],
    views: &[
    	view_check_context_from_view_ep_thunk,
    	view_fibonacci_thunk,
    	view_get_counter_thunk,
    	view_get_int_thunk,
    	view_get_string_value_thunk,
    	view_infinite_loop_view_thunk,
    	view_just_view_thunk,
    	view_pass_types_view_thunk,
    	view_test_call_panic_view_ep_from_view_thunk,
    	view_test_chain_owner_id_view_thunk,
    	view_test_panic_view_ep_thunk,
    	view_test_sandbox_call_thunk,
	],
};

#[no_mangle]
fn on_call(index: i32) {
	ScExports::call(index, &EXPORT_MAP);
}

#[no_mangle]
fn on_load() {
    ScExports::export(&EXPORT_MAP);
}

pub struct CallOnChainContext {
	params: ImmutableCallOnChainParams,
	results: MutableCallOnChainResults,
	state: MutableTestCoreState,
}

fn func_call_on_chain_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcCallOnChain");
	let f = CallOnChainContext {
		params: ImmutableCallOnChainParams { proxy: params_proxy() },
		results: MutableCallOnChainResults { proxy: results_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.int_value().exists(), "missing mandatory intValue");
	func_call_on_chain(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.funcCallOnChain ok");
}

pub struct CheckContextFromFullEPContext {
	params: ImmutableCheckContextFromFullEPParams,
	state: MutableTestCoreState,
}

fn func_check_context_from_full_ep_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcCheckContextFromFullEP");
	let f = CheckContextFromFullEPContext {
		params: ImmutableCheckContextFromFullEPParams { proxy: params_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.agent_id().exists(), "missing mandatory agentID");
	ctx.require(f.params.caller().exists(), "missing mandatory caller");
	ctx.require(f.params.chain_id().exists(), "missing mandatory chainID");
	ctx.require(f.params.chain_owner_id().exists(), "missing mandatory chainOwnerID");
	ctx.require(f.params.contract_creator().exists(), "missing mandatory contractCreator");
	func_check_context_from_full_ep(ctx, &f);
	ctx.log("testcore.funcCheckContextFromFullEP ok");
}

pub struct ClaimAllowanceContext {
	state: MutableTestCoreState,
}

fn func_claim_allowance_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcClaimAllowance");
	let f = ClaimAllowanceContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_claim_allowance(ctx, &f);
	ctx.log("testcore.funcClaimAllowance ok");
}

pub struct DoNothingContext {
	state: MutableTestCoreState,
}

fn func_do_nothing_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcDoNothing");
	let f = DoNothingContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_do_nothing(ctx, &f);
	ctx.log("testcore.funcDoNothing ok");
}

pub struct EstimateMinDustContext {
	state: MutableTestCoreState,
}

fn func_estimate_min_dust_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcEstimateMinDust");
	let f = EstimateMinDustContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_estimate_min_dust(ctx, &f);
	ctx.log("testcore.funcEstimateMinDust ok");
}

pub struct IncCounterContext {
	state: MutableTestCoreState,
}

fn func_inc_counter_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcIncCounter");
	let f = IncCounterContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_inc_counter(ctx, &f);
	ctx.log("testcore.funcIncCounter ok");
}

pub struct InfiniteLoopContext {
	state: MutableTestCoreState,
}

fn func_infinite_loop_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcInfiniteLoop");
	let f = InfiniteLoopContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_infinite_loop(ctx, &f);
	ctx.log("testcore.funcInfiniteLoop ok");
}

pub struct InitContext {
	params: ImmutableInitParams,
	state: MutableTestCoreState,
}

fn func_init_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcInit");
	let f = InitContext {
		params: ImmutableInitParams { proxy: params_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_init(ctx, &f);
	ctx.log("testcore.funcInit ok");
}

pub struct PassTypesFullContext {
	params: ImmutablePassTypesFullParams,
	state: MutableTestCoreState,
}

fn func_pass_types_full_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcPassTypesFull");
	let f = PassTypesFullContext {
		params: ImmutablePassTypesFullParams { proxy: params_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.address().exists(), "missing mandatory address");
	ctx.require(f.params.agent_id().exists(), "missing mandatory agentID");
	ctx.require(f.params.chain_id().exists(), "missing mandatory chainID");
	ctx.require(f.params.contract_id().exists(), "missing mandatory contractID");
	ctx.require(f.params.hash().exists(), "missing mandatory hash");
	ctx.require(f.params.hname().exists(), "missing mandatory hname");
	ctx.require(f.params.hname_zero().exists(), "missing mandatory hnameZero");
	ctx.require(f.params.int64().exists(), "missing mandatory int64");
	ctx.require(f.params.int64_zero().exists(), "missing mandatory int64Zero");
	ctx.require(f.params.string().exists(), "missing mandatory string");
	ctx.require(f.params.string_zero().exists(), "missing mandatory stringZero");
	func_pass_types_full(ctx, &f);
	ctx.log("testcore.funcPassTypesFull ok");
}

pub struct PingAllowanceBackContext {
	state: MutableTestCoreState,
}

fn func_ping_allowance_back_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcPingAllowanceBack");
	let f = PingAllowanceBackContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_ping_allowance_back(ctx, &f);
	ctx.log("testcore.funcPingAllowanceBack ok");
}

pub struct RunRecursionContext {
	params: ImmutableRunRecursionParams,
	results: MutableRunRecursionResults,
	state: MutableTestCoreState,
}

fn func_run_recursion_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcRunRecursion");
	let f = RunRecursionContext {
		params: ImmutableRunRecursionParams { proxy: params_proxy() },
		results: MutableRunRecursionResults { proxy: results_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.int_value().exists(), "missing mandatory intValue");
	func_run_recursion(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.funcRunRecursion ok");
}

pub struct SendLargeRequestContext {
	state: MutableTestCoreState,
}

fn func_send_large_request_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcSendLargeRequest");
	let f = SendLargeRequestContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_send_large_request(ctx, &f);
	ctx.log("testcore.funcSendLargeRequest ok");
}

pub struct SendNFTsBackContext {
	state: MutableTestCoreState,
}

fn func_send_nf_ts_back_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcSendNFTsBack");
	let f = SendNFTsBackContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_send_nf_ts_back(ctx, &f);
	ctx.log("testcore.funcSendNFTsBack ok");
}

pub struct SendToAddressContext {
	state: MutableTestCoreState,
}

fn func_send_to_address_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcSendToAddress");
	let f = SendToAddressContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_send_to_address(ctx, &f);
	ctx.log("testcore.funcSendToAddress ok");
}

pub struct SetIntContext {
	params: ImmutableSetIntParams,
	state: MutableTestCoreState,
}

fn func_set_int_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcSetInt");
	let f = SetIntContext {
		params: ImmutableSetIntParams { proxy: params_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.int_value().exists(), "missing mandatory intValue");
	ctx.require(f.params.name().exists(), "missing mandatory name");
	func_set_int(ctx, &f);
	ctx.log("testcore.funcSetInt ok");
}

pub struct SpawnContext {
	state: MutableTestCoreState,
}

fn func_spawn_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcSpawn");
	let f = SpawnContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_spawn(ctx, &f);
	ctx.log("testcore.funcSpawn ok");
}

pub struct SplitFundsContext {
	state: MutableTestCoreState,
}

fn func_split_funds_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcSplitFunds");
	let f = SplitFundsContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_split_funds(ctx, &f);
	ctx.log("testcore.funcSplitFunds ok");
}

pub struct SplitFundsNativeTokensContext {
	state: MutableTestCoreState,
}

fn func_split_funds_native_tokens_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcSplitFundsNativeTokens");
	let f = SplitFundsNativeTokensContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_split_funds_native_tokens(ctx, &f);
	ctx.log("testcore.funcSplitFundsNativeTokens ok");
}

pub struct TestBlockContext1Context {
	state: MutableTestCoreState,
}

fn func_test_block_context1_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestBlockContext1");
	let f = TestBlockContext1Context {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_test_block_context1(ctx, &f);
	ctx.log("testcore.funcTestBlockContext1 ok");
}

pub struct TestBlockContext2Context {
	state: MutableTestCoreState,
}

fn func_test_block_context2_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestBlockContext2");
	let f = TestBlockContext2Context {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_test_block_context2(ctx, &f);
	ctx.log("testcore.funcTestBlockContext2 ok");
}

pub struct TestCallPanicFullEPContext {
	state: MutableTestCoreState,
}

fn func_test_call_panic_full_ep_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestCallPanicFullEP");
	let f = TestCallPanicFullEPContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_test_call_panic_full_ep(ctx, &f);
	ctx.log("testcore.funcTestCallPanicFullEP ok");
}

pub struct TestCallPanicViewEPFromFullContext {
	state: MutableTestCoreState,
}

fn func_test_call_panic_view_ep_from_full_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestCallPanicViewEPFromFull");
	let f = TestCallPanicViewEPFromFullContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_test_call_panic_view_ep_from_full(ctx, &f);
	ctx.log("testcore.funcTestCallPanicViewEPFromFull ok");
}

pub struct TestChainOwnerIDFullContext {
	results: MutableTestChainOwnerIDFullResults,
	state: MutableTestCoreState,
}

fn func_test_chain_owner_id_full_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestChainOwnerIDFull");
	let f = TestChainOwnerIDFullContext {
		results: MutableTestChainOwnerIDFullResults { proxy: results_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_test_chain_owner_id_full(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.funcTestChainOwnerIDFull ok");
}

pub struct TestEventLogDeployContext {
	state: MutableTestCoreState,
}

fn func_test_event_log_deploy_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestEventLogDeploy");
	let f = TestEventLogDeployContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_test_event_log_deploy(ctx, &f);
	ctx.log("testcore.funcTestEventLogDeploy ok");
}

pub struct TestEventLogEventDataContext {
	state: MutableTestCoreState,
}

fn func_test_event_log_event_data_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestEventLogEventData");
	let f = TestEventLogEventDataContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_test_event_log_event_data(ctx, &f);
	ctx.log("testcore.funcTestEventLogEventData ok");
}

pub struct TestEventLogGenericDataContext {
	params: ImmutableTestEventLogGenericDataParams,
	state: MutableTestCoreState,
}

fn func_test_event_log_generic_data_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestEventLogGenericData");
	let f = TestEventLogGenericDataContext {
		params: ImmutableTestEventLogGenericDataParams { proxy: params_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.counter().exists(), "missing mandatory counter");
	func_test_event_log_generic_data(ctx, &f);
	ctx.log("testcore.funcTestEventLogGenericData ok");
}

pub struct TestPanicFullEPContext {
	state: MutableTestCoreState,
}

fn func_test_panic_full_ep_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcTestPanicFullEP");
	let f = TestPanicFullEPContext {
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	func_test_panic_full_ep(ctx, &f);
	ctx.log("testcore.funcTestPanicFullEP ok");
}

pub struct WithdrawFromChainContext {
	params: ImmutableWithdrawFromChainParams,
	state: MutableTestCoreState,
}

fn func_withdraw_from_chain_thunk(ctx: &ScFuncContext) {
	ctx.log("testcore.funcWithdrawFromChain");
	let f = WithdrawFromChainContext {
		params: ImmutableWithdrawFromChainParams { proxy: params_proxy() },
		state: MutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.chain_id().exists(), "missing mandatory chainID");
	ctx.require(f.params.gas_budget().exists(), "missing mandatory gasBudget");
	ctx.require(f.params.iotas_withdrawal().exists(), "missing mandatory iotasWithdrawal");
	func_withdraw_from_chain(ctx, &f);
	ctx.log("testcore.funcWithdrawFromChain ok");
}

pub struct CheckContextFromViewEPContext {
	params: ImmutableCheckContextFromViewEPParams,
	state: ImmutableTestCoreState,
}

fn view_check_context_from_view_ep_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewCheckContextFromViewEP");
	let f = CheckContextFromViewEPContext {
		params: ImmutableCheckContextFromViewEPParams { proxy: params_proxy() },
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.agent_id().exists(), "missing mandatory agentID");
	ctx.require(f.params.chain_id().exists(), "missing mandatory chainID");
	ctx.require(f.params.chain_owner_id().exists(), "missing mandatory chainOwnerID");
	ctx.require(f.params.contract_creator().exists(), "missing mandatory contractCreator");
	view_check_context_from_view_ep(ctx, &f);
	ctx.log("testcore.viewCheckContextFromViewEP ok");
}

pub struct FibonacciContext {
	params: ImmutableFibonacciParams,
	results: MutableFibonacciResults,
	state: ImmutableTestCoreState,
}

fn view_fibonacci_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewFibonacci");
	let f = FibonacciContext {
		params: ImmutableFibonacciParams { proxy: params_proxy() },
		results: MutableFibonacciResults { proxy: results_proxy() },
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.int_value().exists(), "missing mandatory intValue");
	view_fibonacci(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.viewFibonacci ok");
}

pub struct GetCounterContext {
	results: MutableGetCounterResults,
	state: ImmutableTestCoreState,
}

fn view_get_counter_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewGetCounter");
	let f = GetCounterContext {
		results: MutableGetCounterResults { proxy: results_proxy() },
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	view_get_counter(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.viewGetCounter ok");
}

pub struct GetIntContext {
	params: ImmutableGetIntParams,
	results: MutableGetIntResults,
	state: ImmutableTestCoreState,
}

fn view_get_int_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewGetInt");
	let f = GetIntContext {
		params: ImmutableGetIntParams { proxy: params_proxy() },
		results: MutableGetIntResults { proxy: results_proxy() },
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.name().exists(), "missing mandatory name");
	view_get_int(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.viewGetInt ok");
}

pub struct GetStringValueContext {
	params: ImmutableGetStringValueParams,
	results: MutableGetStringValueResults,
	state: ImmutableTestCoreState,
}

fn view_get_string_value_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewGetStringValue");
	let f = GetStringValueContext {
		params: ImmutableGetStringValueParams { proxy: params_proxy() },
		results: MutableGetStringValueResults { proxy: results_proxy() },
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.var_name().exists(), "missing mandatory varName");
	view_get_string_value(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.viewGetStringValue ok");
}

pub struct InfiniteLoopViewContext {
	state: ImmutableTestCoreState,
}

fn view_infinite_loop_view_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewInfiniteLoopView");
	let f = InfiniteLoopViewContext {
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	view_infinite_loop_view(ctx, &f);
	ctx.log("testcore.viewInfiniteLoopView ok");
}

pub struct JustViewContext {
	state: ImmutableTestCoreState,
}

fn view_just_view_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewJustView");
	let f = JustViewContext {
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	view_just_view(ctx, &f);
	ctx.log("testcore.viewJustView ok");
}

pub struct PassTypesViewContext {
	params: ImmutablePassTypesViewParams,
	state: ImmutableTestCoreState,
}

fn view_pass_types_view_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewPassTypesView");
	let f = PassTypesViewContext {
		params: ImmutablePassTypesViewParams { proxy: params_proxy() },
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	ctx.require(f.params.address().exists(), "missing mandatory address");
	ctx.require(f.params.agent_id().exists(), "missing mandatory agentID");
	ctx.require(f.params.chain_id().exists(), "missing mandatory chainID");
	ctx.require(f.params.contract_id().exists(), "missing mandatory contractID");
	ctx.require(f.params.hash().exists(), "missing mandatory hash");
	ctx.require(f.params.hname().exists(), "missing mandatory hname");
	ctx.require(f.params.hname_zero().exists(), "missing mandatory hnameZero");
	ctx.require(f.params.int64().exists(), "missing mandatory int64");
	ctx.require(f.params.int64_zero().exists(), "missing mandatory int64Zero");
	ctx.require(f.params.string().exists(), "missing mandatory string");
	ctx.require(f.params.string_zero().exists(), "missing mandatory stringZero");
	view_pass_types_view(ctx, &f);
	ctx.log("testcore.viewPassTypesView ok");
}

pub struct TestCallPanicViewEPFromViewContext {
	state: ImmutableTestCoreState,
}

fn view_test_call_panic_view_ep_from_view_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewTestCallPanicViewEPFromView");
	let f = TestCallPanicViewEPFromViewContext {
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	view_test_call_panic_view_ep_from_view(ctx, &f);
	ctx.log("testcore.viewTestCallPanicViewEPFromView ok");
}

pub struct TestChainOwnerIDViewContext {
	results: MutableTestChainOwnerIDViewResults,
	state: ImmutableTestCoreState,
}

fn view_test_chain_owner_id_view_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewTestChainOwnerIDView");
	let f = TestChainOwnerIDViewContext {
		results: MutableTestChainOwnerIDViewResults { proxy: results_proxy() },
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	view_test_chain_owner_id_view(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.viewTestChainOwnerIDView ok");
}

pub struct TestPanicViewEPContext {
	state: ImmutableTestCoreState,
}

fn view_test_panic_view_ep_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewTestPanicViewEP");
	let f = TestPanicViewEPContext {
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	view_test_panic_view_ep(ctx, &f);
	ctx.log("testcore.viewTestPanicViewEP ok");
}

pub struct TestSandboxCallContext {
	results: MutableTestSandboxCallResults,
	state: ImmutableTestCoreState,
}

fn view_test_sandbox_call_thunk(ctx: &ScViewContext) {
	ctx.log("testcore.viewTestSandboxCall");
	let f = TestSandboxCallContext {
		results: MutableTestSandboxCallResults { proxy: results_proxy() },
		state: ImmutableTestCoreState { proxy: state_proxy() },
	};
	view_test_sandbox_call(ctx, &f);
	ctx.results(&f.results.proxy.kv_store);
	ctx.log("testcore.viewTestSandboxCall ok");
}
