// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";
import * as sc from "./index";

export class ArrayAppendCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncArrayAppend);
	params: sc.MutableArrayAppendParams = new sc.MutableArrayAppendParams(wasmlib.ScView.nilProxy);
}

export class ArrayAppendContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	params: sc.ImmutableArrayAppendParams = new sc.ImmutableArrayAppendParams(wasmlib.paramsProxy());
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class ArrayClearCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncArrayClear);
	params: sc.MutableArrayClearParams = new sc.MutableArrayClearParams(wasmlib.ScView.nilProxy);
}

export class ArrayClearContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	params: sc.ImmutableArrayClearParams = new sc.ImmutableArrayClearParams(wasmlib.paramsProxy());
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class ArraySetCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncArraySet);
	params: sc.MutableArraySetParams = new sc.MutableArraySetParams(wasmlib.ScView.nilProxy);
}

export class ArraySetContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	params: sc.ImmutableArraySetParams = new sc.ImmutableArraySetParams(wasmlib.paramsProxy());
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class MapClearCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncMapClear);
	params: sc.MutableMapClearParams = new sc.MutableMapClearParams(wasmlib.ScView.nilProxy);
}

export class MapClearContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	params: sc.ImmutableMapClearParams = new sc.ImmutableMapClearParams(wasmlib.paramsProxy());
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class MapSetCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncMapSet);
	params: sc.MutableMapSetParams = new sc.MutableMapSetParams(wasmlib.ScView.nilProxy);
}

export class MapSetContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	params: sc.ImmutableMapSetParams = new sc.ImmutableMapSetParams(wasmlib.paramsProxy());
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class ParamTypesCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncParamTypes);
	params: sc.MutableParamTypesParams = new sc.MutableParamTypesParams(wasmlib.ScView.nilProxy);
}

export class ParamTypesContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	params: sc.ImmutableParamTypesParams = new sc.ImmutableParamTypesParams(wasmlib.paramsProxy());
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class RandomCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncRandom);
}

export class RandomContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class TakeAllowanceCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncTakeAllowance);
}

export class TakeAllowanceContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class TakeBalanceCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncTakeBalance);
	results: sc.ImmutableTakeBalanceResults = new sc.ImmutableTakeBalanceResults(wasmlib.ScView.nilProxy);
}

export class TakeBalanceContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	results: sc.MutableTakeBalanceResults = new sc.MutableTakeBalanceResults(wasmlib.ScView.nilProxy);
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class TriggerEventCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncTriggerEvent);
	params: sc.MutableTriggerEventParams = new sc.MutableTriggerEventParams(wasmlib.ScView.nilProxy);
}

export class TriggerEventContext {
	events: sc.TestWasmLibEvents = new sc.TestWasmLibEvents();
	params: sc.ImmutableTriggerEventParams = new sc.ImmutableTriggerEventParams(wasmlib.paramsProxy());
	state: sc.MutableTestWasmLibState = new sc.MutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class ArrayLengthCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewArrayLength);
	params: sc.MutableArrayLengthParams = new sc.MutableArrayLengthParams(wasmlib.ScView.nilProxy);
	results: sc.ImmutableArrayLengthResults = new sc.ImmutableArrayLengthResults(wasmlib.ScView.nilProxy);
}

export class ArrayLengthContext {
	params: sc.ImmutableArrayLengthParams = new sc.ImmutableArrayLengthParams(wasmlib.paramsProxy());
	results: sc.MutableArrayLengthResults = new sc.MutableArrayLengthResults(wasmlib.ScView.nilProxy);
	state: sc.ImmutableTestWasmLibState = new sc.ImmutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class ArrayValueCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewArrayValue);
	params: sc.MutableArrayValueParams = new sc.MutableArrayValueParams(wasmlib.ScView.nilProxy);
	results: sc.ImmutableArrayValueResults = new sc.ImmutableArrayValueResults(wasmlib.ScView.nilProxy);
}

export class ArrayValueContext {
	params: sc.ImmutableArrayValueParams = new sc.ImmutableArrayValueParams(wasmlib.paramsProxy());
	results: sc.MutableArrayValueResults = new sc.MutableArrayValueResults(wasmlib.ScView.nilProxy);
	state: sc.ImmutableTestWasmLibState = new sc.ImmutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class BlockRecordCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewBlockRecord);
	params: sc.MutableBlockRecordParams = new sc.MutableBlockRecordParams(wasmlib.ScView.nilProxy);
	results: sc.ImmutableBlockRecordResults = new sc.ImmutableBlockRecordResults(wasmlib.ScView.nilProxy);
}

export class BlockRecordContext {
	params: sc.ImmutableBlockRecordParams = new sc.ImmutableBlockRecordParams(wasmlib.paramsProxy());
	results: sc.MutableBlockRecordResults = new sc.MutableBlockRecordResults(wasmlib.ScView.nilProxy);
	state: sc.ImmutableTestWasmLibState = new sc.ImmutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class BlockRecordsCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewBlockRecords);
	params: sc.MutableBlockRecordsParams = new sc.MutableBlockRecordsParams(wasmlib.ScView.nilProxy);
	results: sc.ImmutableBlockRecordsResults = new sc.ImmutableBlockRecordsResults(wasmlib.ScView.nilProxy);
}

export class BlockRecordsContext {
	params: sc.ImmutableBlockRecordsParams = new sc.ImmutableBlockRecordsParams(wasmlib.paramsProxy());
	results: sc.MutableBlockRecordsResults = new sc.MutableBlockRecordsResults(wasmlib.ScView.nilProxy);
	state: sc.ImmutableTestWasmLibState = new sc.ImmutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class GetRandomCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewGetRandom);
	results: sc.ImmutableGetRandomResults = new sc.ImmutableGetRandomResults(wasmlib.ScView.nilProxy);
}

export class GetRandomContext {
	results: sc.MutableGetRandomResults = new sc.MutableGetRandomResults(wasmlib.ScView.nilProxy);
	state: sc.ImmutableTestWasmLibState = new sc.ImmutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class IotaBalanceCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewIotaBalance);
	results: sc.ImmutableIotaBalanceResults = new sc.ImmutableIotaBalanceResults(wasmlib.ScView.nilProxy);
}

export class IotaBalanceContext {
	results: sc.MutableIotaBalanceResults = new sc.MutableIotaBalanceResults(wasmlib.ScView.nilProxy);
	state: sc.ImmutableTestWasmLibState = new sc.ImmutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class MapValueCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewMapValue);
	params: sc.MutableMapValueParams = new sc.MutableMapValueParams(wasmlib.ScView.nilProxy);
	results: sc.ImmutableMapValueResults = new sc.ImmutableMapValueResults(wasmlib.ScView.nilProxy);
}

export class MapValueContext {
	params: sc.ImmutableMapValueParams = new sc.ImmutableMapValueParams(wasmlib.paramsProxy());
	results: sc.MutableMapValueResults = new sc.MutableMapValueResults(wasmlib.ScView.nilProxy);
	state: sc.ImmutableTestWasmLibState = new sc.ImmutableTestWasmLibState(wasmlib.ScState.proxy());
}

export class ScFuncs {
	static arrayAppend(_ctx: wasmlib.ScFuncCallContext): ArrayAppendCall {
		const f = new ArrayAppendCall();
		f.params = new sc.MutableArrayAppendParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static arrayClear(_ctx: wasmlib.ScFuncCallContext): ArrayClearCall {
		const f = new ArrayClearCall();
		f.params = new sc.MutableArrayClearParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static arraySet(_ctx: wasmlib.ScFuncCallContext): ArraySetCall {
		const f = new ArraySetCall();
		f.params = new sc.MutableArraySetParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static mapClear(_ctx: wasmlib.ScFuncCallContext): MapClearCall {
		const f = new MapClearCall();
		f.params = new sc.MutableMapClearParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static mapSet(_ctx: wasmlib.ScFuncCallContext): MapSetCall {
		const f = new MapSetCall();
		f.params = new sc.MutableMapSetParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static paramTypes(_ctx: wasmlib.ScFuncCallContext): ParamTypesCall {
		const f = new ParamTypesCall();
		f.params = new sc.MutableParamTypesParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static random(_ctx: wasmlib.ScFuncCallContext): RandomCall {
		return new RandomCall();
	}

	static takeAllowance(_ctx: wasmlib.ScFuncCallContext): TakeAllowanceCall {
		return new TakeAllowanceCall();
	}

	static takeBalance(_ctx: wasmlib.ScFuncCallContext): TakeBalanceCall {
		const f = new TakeBalanceCall();
		f.results = new sc.ImmutableTakeBalanceResults(wasmlib.newCallResultsProxy(f.func));
		return f;
	}

	static triggerEvent(_ctx: wasmlib.ScFuncCallContext): TriggerEventCall {
		const f = new TriggerEventCall();
		f.params = new sc.MutableTriggerEventParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static arrayLength(_ctx: wasmlib.ScViewCallContext): ArrayLengthCall {
		const f = new ArrayLengthCall();
		f.params = new sc.MutableArrayLengthParams(wasmlib.newCallParamsProxy(f.func));
		f.results = new sc.ImmutableArrayLengthResults(wasmlib.newCallResultsProxy(f.func));
		return f;
	}

	static arrayValue(_ctx: wasmlib.ScViewCallContext): ArrayValueCall {
		const f = new ArrayValueCall();
		f.params = new sc.MutableArrayValueParams(wasmlib.newCallParamsProxy(f.func));
		f.results = new sc.ImmutableArrayValueResults(wasmlib.newCallResultsProxy(f.func));
		return f;
	}

	static blockRecord(_ctx: wasmlib.ScViewCallContext): BlockRecordCall {
		const f = new BlockRecordCall();
		f.params = new sc.MutableBlockRecordParams(wasmlib.newCallParamsProxy(f.func));
		f.results = new sc.ImmutableBlockRecordResults(wasmlib.newCallResultsProxy(f.func));
		return f;
	}

	static blockRecords(_ctx: wasmlib.ScViewCallContext): BlockRecordsCall {
		const f = new BlockRecordsCall();
		f.params = new sc.MutableBlockRecordsParams(wasmlib.newCallParamsProxy(f.func));
		f.results = new sc.ImmutableBlockRecordsResults(wasmlib.newCallResultsProxy(f.func));
		return f;
	}

	static getRandom(_ctx: wasmlib.ScViewCallContext): GetRandomCall {
		const f = new GetRandomCall();
		f.results = new sc.ImmutableGetRandomResults(wasmlib.newCallResultsProxy(f.func));
		return f;
	}

	static iotaBalance(_ctx: wasmlib.ScViewCallContext): IotaBalanceCall {
		const f = new IotaBalanceCall();
		f.results = new sc.ImmutableIotaBalanceResults(wasmlib.newCallResultsProxy(f.func));
		return f;
	}

	static mapValue(_ctx: wasmlib.ScViewCallContext): MapValueCall {
		const f = new MapValueCall();
		f.params = new sc.MutableMapValueParams(wasmlib.newCallParamsProxy(f.func));
		f.results = new sc.ImmutableMapValueResults(wasmlib.newCallResultsProxy(f.func));
		return f;
	}
}
