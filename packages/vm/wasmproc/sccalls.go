// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package wasmproc

import (
	"github.com/iotaledger/goshimmer/dapps/valuetransfers/packages/balance"
	"github.com/iotaledger/wasp/packages/coretypes"
	"github.com/iotaledger/wasp/packages/coretypes/cbalances"
	"github.com/iotaledger/wasp/packages/kv"
	"github.com/iotaledger/wasp/packages/kv/codec"
	"github.com/iotaledger/wasp/packages/kv/dict"
	"github.com/iotaledger/wasp/packages/vm/wasmhost"
)

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

type ScCalls struct {
	ScSandboxObject
}

func NewScCalls(vm *wasmProcessor) *ScCalls {
	a := &ScCalls{}
	a.vm = vm
	return a
}

func (a *ScCalls) GetObjectId(keyId int32, typeId int32) int32 {
	return GetArrayObjectId(a, keyId, typeId, func() WaspObject {
		return NewScCallInfo(a.vm)
	})
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

type ScCallInfo struct {
	ScSandboxObject
	chainId   coretypes.ChainID
	contract  coretypes.Hname
	function  coretypes.Hname
	params    int32
	transfers int32
}

func NewScCallInfo(vm *wasmProcessor) *ScCallInfo {
	o := &ScCallInfo{}
	o.vm = vm
	return o
}

func (o *ScCallInfo) Exists(keyId int32, typeId int32) bool {
	return o.GetTypeId(keyId) > 0
}

func (o *ScCallInfo) GetObjectId(keyId int32, typeId int32) int32 {
	return GetMapObjectId(o, keyId, typeId, ObjFactories{
		wasmhost.KeyResults: func() WaspObject { return NewScDict(o.vm) },
	})
}

func (o *ScCallInfo) GetTypeId(keyId int32) int32 {
	switch keyId {
	case wasmhost.KeyChain:
		return wasmhost.OBJTYPE_CHAIN
	case wasmhost.KeyContract:
		return wasmhost.OBJTYPE_HNAME
	case wasmhost.KeyDelay:
		return wasmhost.OBJTYPE_INT
	case wasmhost.KeyFunction:
		return wasmhost.OBJTYPE_HNAME
	case wasmhost.KeyParams:
		return wasmhost.OBJTYPE_INT
	case wasmhost.KeyResults:
		return wasmhost.OBJTYPE_MAP
	case wasmhost.KeyTransfers:
		return wasmhost.OBJTYPE_INT
	}
	return 0
}

func (o *ScCallInfo) Invoke(delay int64) {
	params := dict.New()
	if o.params != 0 {
		params = o.host.FindObject(o.params).(*ScDict).kvStore.(dict.Dict)
		params.MustIterate("", func(key kv.Key, value []byte) bool {
			o.Trace("  PARAM '%s'", key)
			return true
		})
	}

	transfer := map[balance.Color]int64(nil)
	if o.transfers != 0 {
		transfer = make(map[balance.Color]int64)
		transferDict := o.host.FindObject(o.transfers).(*ScDict).kvStore.(dict.Dict)
		transferDict.MustIterate("", func(key kv.Key, value []byte) bool {
			color, _, err := codec.DecodeColor([]byte(key))
			if err != nil {
				o.Panic(err.Error())
			}
			amount, _, err := codec.DecodeInt64(value)
			if err != nil {
				o.Panic(err.Error())
			}
			o.Trace("  XFER %d '%s'", amount, color.String())
			transfer[color] = amount
			return true
		})
	}

	if delay >= 0 {
		o.Trace("POST ch'%s' c'%s' f'%s'", o.chainId.String(), o.contract.String(), o.function.String())
		o.vm.ctx.PostRequest(coretypes.PostRequestParams{
			TargetContractID: coretypes.NewContractID(o.chainId, o.contract),
			EntryPoint:       o.function,
			TimeLock:         uint32(delay),
			Params:           params,
			Transfer:         cbalances.NewFromMap(transfer),
		})
		return
	}

	o.Trace("CALL c'%s' f'%s'", o.contract.String(), o.function.String())
	var err error
	var results dict.Dict
	if o.vm.ctx != nil {
		results, err = o.vm.ctx.Call(o.contract, o.function, params, cbalances.NewFromMap(transfer))
	} else {
		results, err = o.vm.ctxView.Call(o.contract, o.function, params)
	}
	if err != nil {
		o.Panic("failed to invoke call: %v", err)
	}
	resultsId := o.GetObjectId(wasmhost.KeyResults, wasmhost.OBJTYPE_MAP)
	o.host.FindObject(resultsId).(*ScDict).kvStore = results
}

func (o *ScCallInfo) SetBytes(keyId int32, typeId int32, bytes []byte) {
	var err error
	switch keyId {
	case wasmhost.KeyChain:
		o.chainId, err = coretypes.NewChainIDFromBytes(bytes)
	case wasmhost.KeyContract:
		o.contract, err = coretypes.NewHnameFromBytes(bytes)
	case wasmhost.KeyFunction:
		o.function, err = coretypes.NewHnameFromBytes(bytes)
	case wasmhost.KeyDelay:
		value := o.MustInt64(bytes)
		if value < -1 {
			o.Panic("invalid delay: %d", value)
		}
		o.Invoke(value)
	case wasmhost.KeyParams:
		o.params = int32(o.MustInt64(bytes))
	case wasmhost.KeyTransfers:
		o.transfers = int32(o.MustInt64(bytes))
	default:
		o.invalidKey(keyId)
	}
	if err != nil {
		o.Panic(err.Error())
	}
}
