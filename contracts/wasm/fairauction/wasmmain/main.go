// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

// +build wasm

package main

import "github.com/iotaledger/wasp/packages/vm/wasmclient"
import "github.com/iotaledger/wasp/contracts/wasm/fairauction"

func main() {
}

//export on_load
func OnLoad() {
	h := &wasmclient.WasmVMHost{}
	h.ConnectWasmHost()
	fairauction.OnLoad()
}
