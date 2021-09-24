// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

// @formatter:off

#![allow(dead_code)]

#![allow(unused_imports)]

use fairroulette::*;
use wasmlib::*;
use wasmlib::host::*;

use crate::consts::*;
use crate::keys::*;
use crate::params::*;
use crate::results::*;
use crate::state::*;

mod consts;
mod contract;
mod keys;
mod params;
mod results;
mod state;
mod types;
mod fairroulette;

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_func(FUNC_PAY_WINNERS, func_pay_winners_thunk);
    exports.add_func(FUNC_PLACE_BET, func_place_bet_thunk);
    exports.add_func(FUNC_PLAY_PERIOD, func_play_period_thunk);
    exports.add_view(VIEW_LAST_WINNING_NUMBER, view_last_winning_number_thunk);
    exports.add_view(VIEW_ROUND_NUMBER, view_round_number_thunk);
    exports.add_view(VIEW_ROUND_STARTED_AT, view_round_started_at_thunk);
    exports.add_view(VIEW_ROUND_STATUS, view_round_status_thunk);

    unsafe {
        for i in 0..KEY_MAP_LEN {
            IDX_MAP[i] = get_key_id_from_string(KEY_MAP[i]);
        }
    }
}

pub struct PayWinnersContext {
    state: MutableFairRouletteState,
}

fn func_pay_winners_thunk(ctx: &ScFuncContext) {
    ctx.log("fairroulette.funcPayWinners");
    // only SC itself can invoke this function
    ctx.require(ctx.caller() == ctx.account_id(), "no permission");

    let f = PayWinnersContext {
        state: MutableFairRouletteState {
            id: OBJ_ID_STATE,
        },
    };
    func_pay_winners(ctx, &f);
    ctx.log("fairroulette.funcPayWinners ok");
}

pub struct PlaceBetContext {
    params: ImmutablePlaceBetParams,
    state:  MutableFairRouletteState,
}

fn func_place_bet_thunk(ctx: &ScFuncContext) {
    ctx.log("fairroulette.funcPlaceBet");
    let f = PlaceBetContext {
        params: ImmutablePlaceBetParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableFairRouletteState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.number().exists(), "missing mandatory number");
    func_place_bet(ctx, &f);
    ctx.log("fairroulette.funcPlaceBet ok");
}

pub struct PlayPeriodContext {
    params: ImmutablePlayPeriodParams,
    state:  MutableFairRouletteState,
}

fn func_play_period_thunk(ctx: &ScFuncContext) {
    ctx.log("fairroulette.funcPlayPeriod");
    // only SC creator can update the play period
    ctx.require(ctx.caller() == ctx.contract_creator(), "no permission");

    let f = PlayPeriodContext {
        params: ImmutablePlayPeriodParams {
            id: OBJ_ID_PARAMS,
        },
        state: MutableFairRouletteState {
            id: OBJ_ID_STATE,
        },
    };
    ctx.require(f.params.play_period().exists(), "missing mandatory playPeriod");
    func_play_period(ctx, &f);
    ctx.log("fairroulette.funcPlayPeriod ok");
}

pub struct LastWinningNumberContext {
    results: MutableLastWinningNumberResults,
    state:   ImmutableFairRouletteState,
}

fn view_last_winning_number_thunk(ctx: &ScViewContext) {
    ctx.log("fairroulette.viewLastWinningNumber");
    let f = LastWinningNumberContext {
        results: MutableLastWinningNumberResults {
            id: OBJ_ID_RESULTS,
        },
        state: ImmutableFairRouletteState {
            id: OBJ_ID_STATE,
        },
    };
    view_last_winning_number(ctx, &f);
    ctx.log("fairroulette.viewLastWinningNumber ok");
}

pub struct RoundNumberContext {
    results: MutableRoundNumberResults,
    state:   ImmutableFairRouletteState,
}

fn view_round_number_thunk(ctx: &ScViewContext) {
    ctx.log("fairroulette.viewRoundNumber");
    let f = RoundNumberContext {
        results: MutableRoundNumberResults {
            id: OBJ_ID_RESULTS,
        },
        state: ImmutableFairRouletteState {
            id: OBJ_ID_STATE,
        },
    };
    view_round_number(ctx, &f);
    ctx.log("fairroulette.viewRoundNumber ok");
}

pub struct RoundStartedAtContext {
    results: MutableRoundStartedAtResults,
    state:   ImmutableFairRouletteState,
}

fn view_round_started_at_thunk(ctx: &ScViewContext) {
    ctx.log("fairroulette.viewRoundStartedAt");
    let f = RoundStartedAtContext {
        results: MutableRoundStartedAtResults {
            id: OBJ_ID_RESULTS,
        },
        state: ImmutableFairRouletteState {
            id: OBJ_ID_STATE,
        },
    };
    view_round_started_at(ctx, &f);
    ctx.log("fairroulette.viewRoundStartedAt ok");
}

pub struct RoundStatusContext {
    results: MutableRoundStatusResults,
    state:   ImmutableFairRouletteState,
}

fn view_round_status_thunk(ctx: &ScViewContext) {
    ctx.log("fairroulette.viewRoundStatus");
    let f = RoundStatusContext {
        results: MutableRoundStatusResults {
            id: OBJ_ID_RESULTS,
        },
        state: ImmutableFairRouletteState {
            id: OBJ_ID_STATE,
        },
    };
    view_round_status(ctx, &f);
    ctx.log("fairroulette.viewRoundStatus ok");
}

// @formatter:on
