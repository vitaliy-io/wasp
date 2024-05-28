// Code generated by schema tool; DO NOT EDIT.

// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package coreblob

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

type StoreBlobCall struct {
	Func    *wasmlib.ScFunc
	Params  MutableStoreBlobParams
	Results ImmutableStoreBlobResults
}

type GetBlobFieldCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetBlobFieldParams
	Results ImmutableGetBlobFieldResults
}

type GetBlobInfoCall struct {
	Func    *wasmlib.ScView
	Params  MutableGetBlobInfoParams
	Results ImmutableGetBlobInfoResults
}

type Funcs struct{}

var ScFuncs Funcs

// Stores a new blob in the registry.
func (sc Funcs) StoreBlob(ctx wasmlib.ScFuncClientContext) *StoreBlobCall {
	f := &StoreBlobCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncStoreBlob)}
	f.Params.Proxy = wasmlib.NewCallParamsProxy(&f.Func.ScView)
	wasmlib.NewCallResultsProxy(&f.Func.ScView, &f.Results.Proxy)
	return f
}

//Returns the chunk associated with the given blob field name.
func (sc Funcs) GetBlobField(ctx wasmlib.ScViewClientContext) *GetBlobFieldCall {
	f := &GetBlobFieldCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetBlobField)}
	f.Params.Proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.Proxy)
	return f
}

// Returns the size of each chunk of the blob.
func (sc Funcs) GetBlobInfo(ctx wasmlib.ScViewClientContext) *GetBlobInfoCall {
	f := &GetBlobInfoCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetBlobInfo)}
	f.Params.Proxy = wasmlib.NewCallParamsProxy(f.Func)
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.Proxy)
	return f
}

var exportMap = wasmlib.ScExportMap{
	Names: []string{
		FuncStoreBlob,
		ViewGetBlobField,
		ViewGetBlobInfo,
	},
	Funcs: []wasmlib.ScFuncContextFunction{
		wasmlib.FuncError,
	},
	Views: []wasmlib.ScViewContextFunction{
		wasmlib.ViewError,
		wasmlib.ViewError,
	},
}

func OnDispatch(index int32) *wasmlib.ScExportMap {
	if index < 0 {
		return exportMap.Dispatch(index)
	}

	panic("Calling core contract?")
}
