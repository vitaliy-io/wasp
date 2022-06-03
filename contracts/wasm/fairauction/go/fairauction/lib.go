// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package fairauction

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

var exportMap = wasmlib.ScExportMap{
	Names: []string{
		FuncFinalizeAuction,
		FuncPlaceBid,
		FuncSetOwnerMargin,
		FuncStartAuction,
		ViewGetAuctionInfo,
	},
	Funcs: []wasmlib.ScFuncContextFunction{
		funcFinalizeAuctionThunk,
		funcPlaceBidThunk,
		funcSetOwnerMarginThunk,
		funcStartAuctionThunk,
	},
	Views: []wasmlib.ScViewContextFunction{
		viewGetAuctionInfoThunk,
	},
}

func OnLoad(index int32) {
	if index >= 0 {
		wasmlib.ScExportsCall(index, &exportMap)
		return
	}

	wasmlib.ScExportsExport(&exportMap)
}

type FinalizeAuctionContext struct {
	Params ImmutableFinalizeAuctionParams
	State  MutableFairAuctionState
}

func funcFinalizeAuctionThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("fairauction.funcFinalizeAuction")
	f := &FinalizeAuctionContext{
		Params: ImmutableFinalizeAuctionParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableFairAuctionState{
			proxy: wasmlib.NewStateProxy(),
		},
	}

	// only SC itself can invoke this function
	ctx.Require(ctx.Caller() == ctx.AccountID(), "no permission")

	ctx.Require(f.Params.Nft().Exists(), "missing mandatory nft")
	funcFinalizeAuction(ctx, f)
	ctx.Log("fairauction.funcFinalizeAuction ok")
}

type PlaceBidContext struct {
	Params ImmutablePlaceBidParams
	State  MutableFairAuctionState
}

func funcPlaceBidThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("fairauction.funcPlaceBid")
	f := &PlaceBidContext{
		Params: ImmutablePlaceBidParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableFairAuctionState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Nft().Exists(), "missing mandatory nft")
	funcPlaceBid(ctx, f)
	ctx.Log("fairauction.funcPlaceBid ok")
}

type SetOwnerMarginContext struct {
	Params ImmutableSetOwnerMarginParams
	State  MutableFairAuctionState
}

func funcSetOwnerMarginThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("fairauction.funcSetOwnerMargin")
	f := &SetOwnerMarginContext{
		Params: ImmutableSetOwnerMarginParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableFairAuctionState{
			proxy: wasmlib.NewStateProxy(),
		},
	}

	// only SC creator can set owner margin
	ctx.Require(ctx.Caller() == ctx.ContractCreator(), "no permission")

	ctx.Require(f.Params.OwnerMargin().Exists(), "missing mandatory ownerMargin")
	funcSetOwnerMargin(ctx, f)
	ctx.Log("fairauction.funcSetOwnerMargin ok")
}

type StartAuctionContext struct {
	Params ImmutableStartAuctionParams
	State  MutableFairAuctionState
}

func funcStartAuctionThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("fairauction.funcStartAuction")
	f := &StartAuctionContext{
		Params: ImmutableStartAuctionParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		State: MutableFairAuctionState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.MinimumBid().Exists(), "missing mandatory minimumBid")
	funcStartAuction(ctx, f)
	ctx.Log("fairauction.funcStartAuction ok")
}

type GetAuctionInfoContext struct {
	Params  ImmutableGetAuctionInfoParams
	Results MutableGetAuctionInfoResults
	State   ImmutableFairAuctionState
}

func viewGetAuctionInfoThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("fairauction.viewGetAuctionInfo")
	results := wasmlib.NewScDict()
	f := &GetAuctionInfoContext{
		Params: ImmutableGetAuctionInfoParams{
			proxy: wasmlib.NewParamsProxy(),
		},
		Results: MutableGetAuctionInfoResults{
			proxy: results.AsProxy(),
		},
		State: ImmutableFairAuctionState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	ctx.Require(f.Params.Nft().Exists(), "missing mandatory nft")
	viewGetAuctionInfo(ctx, f)
	ctx.Results(results)
	ctx.Log("fairauction.viewGetAuctionInfo ok")
}
