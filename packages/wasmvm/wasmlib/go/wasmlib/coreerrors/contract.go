// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

package coreerrors

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

type RegisterErrorCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableRegisterErrorParams
	Results ImmutableRegisterErrorResults
}

type GetErrorMessageFormatCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetErrorMessageFormatParams
	Results ImmutableGetErrorMessageFormatResults
}

type Funcs struct{}

var ScFuncs Funcs

// Registers an error message template.
// note that this function must be call()ed
func (sc Funcs) RegisterError(ctx wasmlib.ScFuncCallContext) *RegisterErrorCall {
	f := &RegisterErrorCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncRegisterError)}
	f.Params.Proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	wasmlib.NewCallResultsProxy(&f.Func.ScView, &f.Results.Proxy)
	return f
}

// Returns the message template stored for a given error code.
func (sc Funcs) GetErrorMessageFormat(ctx wasmlib.ScViewCallContext) *GetErrorMessageFormatCall {
	f := &GetErrorMessageFormatCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetErrorMessageFormat)}
	f.Params.Proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.Proxy)
	return f
}

var exportMap = wasmlib.ScExportMap{
	Names: []string{
		FuncRegisterError,
		ViewGetErrorMessageFormat,
	},
	Funcs: []wasmlib.ScFuncContextFunction{
		wasmlib.FuncError,
	},
	Views: []wasmlib.ScViewContextFunction{
		wasmlib.ViewError,
	},
}

func OnDispatch(index int32) *wasmlib.ScExportMap {
	if index < 0 {
		return exportMap.Dispatch(index)
	}

	panic("Calling core contract?")
}
