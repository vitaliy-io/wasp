// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]

use wasmlib::*;
use crate::*;

pub struct ArrayAppendCall {
	pub func: ScFunc,
	pub params: MutableArrayAppendParams,
}

pub struct ArrayClearCall {
	pub func: ScFunc,
	pub params: MutableArrayClearParams,
}

pub struct ArraySetCall {
	pub func: ScFunc,
	pub params: MutableArraySetParams,
}

pub struct MapClearCall {
	pub func: ScFunc,
	pub params: MutableMapClearParams,
}

pub struct MapSetCall {
	pub func: ScFunc,
	pub params: MutableMapSetParams,
}

pub struct ParamTypesCall {
	pub func: ScFunc,
	pub params: MutableParamTypesParams,
}

pub struct RandomCall {
	pub func: ScFunc,
}

pub struct TakeAllowanceCall {
	pub func: ScFunc,
}

pub struct TakeBalanceCall {
	pub func: ScFunc,
	pub results: ImmutableTakeBalanceResults,
}

pub struct TriggerEventCall {
	pub func: ScFunc,
	pub params: MutableTriggerEventParams,
}

pub struct ArrayLengthCall {
	pub func: ScView,
	pub params: MutableArrayLengthParams,
	pub results: ImmutableArrayLengthResults,
}

pub struct ArrayValueCall {
	pub func: ScView,
	pub params: MutableArrayValueParams,
	pub results: ImmutableArrayValueResults,
}

pub struct BlockRecordCall {
	pub func: ScView,
	pub params: MutableBlockRecordParams,
	pub results: ImmutableBlockRecordResults,
}

pub struct BlockRecordsCall {
	pub func: ScView,
	pub params: MutableBlockRecordsParams,
	pub results: ImmutableBlockRecordsResults,
}

pub struct GetRandomCall {
	pub func: ScView,
	pub results: ImmutableGetRandomResults,
}

pub struct IotaBalanceCall {
	pub func: ScView,
	pub results: ImmutableIotaBalanceResults,
}

pub struct MapValueCall {
	pub func: ScView,
	pub params: MutableMapValueParams,
	pub results: ImmutableMapValueResults,
}

pub struct ScFuncs {
}

impl ScFuncs {
    pub fn array_append(_ctx: &dyn ScFuncCallContext) -> ArrayAppendCall {
        let mut f = ArrayAppendCall {
            func: ScFunc::new(HSC_NAME, HFUNC_ARRAY_APPEND),
            params: MutableArrayAppendParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn array_clear(_ctx: &dyn ScFuncCallContext) -> ArrayClearCall {
        let mut f = ArrayClearCall {
            func: ScFunc::new(HSC_NAME, HFUNC_ARRAY_CLEAR),
            params: MutableArrayClearParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn array_set(_ctx: &dyn ScFuncCallContext) -> ArraySetCall {
        let mut f = ArraySetCall {
            func: ScFunc::new(HSC_NAME, HFUNC_ARRAY_SET),
            params: MutableArraySetParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn map_clear(_ctx: &dyn ScFuncCallContext) -> MapClearCall {
        let mut f = MapClearCall {
            func: ScFunc::new(HSC_NAME, HFUNC_MAP_CLEAR),
            params: MutableMapClearParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn map_set(_ctx: &dyn ScFuncCallContext) -> MapSetCall {
        let mut f = MapSetCall {
            func: ScFunc::new(HSC_NAME, HFUNC_MAP_SET),
            params: MutableMapSetParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn param_types(_ctx: &dyn ScFuncCallContext) -> ParamTypesCall {
        let mut f = ParamTypesCall {
            func: ScFunc::new(HSC_NAME, HFUNC_PARAM_TYPES),
            params: MutableParamTypesParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn random(_ctx: &dyn ScFuncCallContext) -> RandomCall {
        RandomCall {
            func: ScFunc::new(HSC_NAME, HFUNC_RANDOM),
        }
    }

    pub fn take_allowance(_ctx: &dyn ScFuncCallContext) -> TakeAllowanceCall {
        TakeAllowanceCall {
            func: ScFunc::new(HSC_NAME, HFUNC_TAKE_ALLOWANCE),
        }
    }

    pub fn take_balance(_ctx: &dyn ScFuncCallContext) -> TakeBalanceCall {
        let mut f = TakeBalanceCall {
            func: ScFunc::new(HSC_NAME, HFUNC_TAKE_BALANCE),
            results: ImmutableTakeBalanceResults { proxy: Proxy::nil() },
        };
        ScFunc::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn trigger_event(_ctx: &dyn ScFuncCallContext) -> TriggerEventCall {
        let mut f = TriggerEventCall {
            func: ScFunc::new(HSC_NAME, HFUNC_TRIGGER_EVENT),
            params: MutableTriggerEventParams { proxy: Proxy::nil() },
        };
        ScFunc::link_params(&mut f.params.proxy, &f.func);
        f
    }

    pub fn array_length(_ctx: &dyn ScViewCallContext) -> ArrayLengthCall {
        let mut f = ArrayLengthCall {
            func: ScView::new(HSC_NAME, HVIEW_ARRAY_LENGTH),
            params: MutableArrayLengthParams { proxy: Proxy::nil() },
            results: ImmutableArrayLengthResults { proxy: Proxy::nil() },
        };
        ScView::link_params(&mut f.params.proxy, &f.func);
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn array_value(_ctx: &dyn ScViewCallContext) -> ArrayValueCall {
        let mut f = ArrayValueCall {
            func: ScView::new(HSC_NAME, HVIEW_ARRAY_VALUE),
            params: MutableArrayValueParams { proxy: Proxy::nil() },
            results: ImmutableArrayValueResults { proxy: Proxy::nil() },
        };
        ScView::link_params(&mut f.params.proxy, &f.func);
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn block_record(_ctx: &dyn ScViewCallContext) -> BlockRecordCall {
        let mut f = BlockRecordCall {
            func: ScView::new(HSC_NAME, HVIEW_BLOCK_RECORD),
            params: MutableBlockRecordParams { proxy: Proxy::nil() },
            results: ImmutableBlockRecordResults { proxy: Proxy::nil() },
        };
        ScView::link_params(&mut f.params.proxy, &f.func);
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn block_records(_ctx: &dyn ScViewCallContext) -> BlockRecordsCall {
        let mut f = BlockRecordsCall {
            func: ScView::new(HSC_NAME, HVIEW_BLOCK_RECORDS),
            params: MutableBlockRecordsParams { proxy: Proxy::nil() },
            results: ImmutableBlockRecordsResults { proxy: Proxy::nil() },
        };
        ScView::link_params(&mut f.params.proxy, &f.func);
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_random(_ctx: &dyn ScViewCallContext) -> GetRandomCall {
        let mut f = GetRandomCall {
            func: ScView::new(HSC_NAME, HVIEW_GET_RANDOM),
            results: ImmutableGetRandomResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn iota_balance(_ctx: &dyn ScViewCallContext) -> IotaBalanceCall {
        let mut f = IotaBalanceCall {
            func: ScView::new(HSC_NAME, HVIEW_IOTA_BALANCE),
            results: ImmutableIotaBalanceResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn map_value(_ctx: &dyn ScViewCallContext) -> MapValueCall {
        let mut f = MapValueCall {
            func: ScView::new(HSC_NAME, HVIEW_MAP_VALUE),
            params: MutableMapValueParams { proxy: Proxy::nil() },
            results: ImmutableMapValueResults { proxy: Proxy::nil() },
        };
        ScView::link_params(&mut f.params.proxy, &f.func);
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }
}
