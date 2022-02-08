// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmtypes from "wasmlib/wasmtypes";
import * as sc from "./index";

export class ImmutableApproveParams extends wasmtypes.ScProxy {
	approved(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamApproved));
	}

	tokenID(): wasmtypes.ScImmutableHash {
		return new wasmtypes.ScImmutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class MutableApproveParams extends wasmtypes.ScProxy {
	approved(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamApproved));
	}

	tokenID(): wasmtypes.ScMutableHash {
		return new wasmtypes.ScMutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class ImmutableBurnParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScImmutableHash {
		return new wasmtypes.ScImmutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class MutableBurnParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScMutableHash {
		return new wasmtypes.ScMutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class ImmutableInitParams extends wasmtypes.ScProxy {
	name(): wasmtypes.ScImmutableString {
		return new wasmtypes.ScImmutableString(this.proxy.root(sc.ParamName));
	}

	symbol(): wasmtypes.ScImmutableString {
		return new wasmtypes.ScImmutableString(this.proxy.root(sc.ParamSymbol));
	}
}

export class MutableInitParams extends wasmtypes.ScProxy {
	name(): wasmtypes.ScMutableString {
		return new wasmtypes.ScMutableString(this.proxy.root(sc.ParamName));
	}

	symbol(): wasmtypes.ScMutableString {
		return new wasmtypes.ScMutableString(this.proxy.root(sc.ParamSymbol));
	}
}

export class ImmutableMintParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScImmutableHash {
		return new wasmtypes.ScImmutableHash(this.proxy.root(sc.ParamTokenID));
	}

	tokenURI(): wasmtypes.ScImmutableString {
		return new wasmtypes.ScImmutableString(this.proxy.root(sc.ParamTokenURI));
	}
}

export class MutableMintParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScMutableHash {
		return new wasmtypes.ScMutableHash(this.proxy.root(sc.ParamTokenID));
	}

	tokenURI(): wasmtypes.ScMutableString {
		return new wasmtypes.ScMutableString(this.proxy.root(sc.ParamTokenURI));
	}
}

export class ImmutableSafeTransferFromParams extends wasmtypes.ScProxy {
	data(): wasmtypes.ScImmutableBytes {
		return new wasmtypes.ScImmutableBytes(this.proxy.root(sc.ParamData));
	}

	from(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamFrom));
	}

	to(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamTo));
	}

	tokenID(): wasmtypes.ScImmutableHash {
		return new wasmtypes.ScImmutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class MutableSafeTransferFromParams extends wasmtypes.ScProxy {
	data(): wasmtypes.ScMutableBytes {
		return new wasmtypes.ScMutableBytes(this.proxy.root(sc.ParamData));
	}

	from(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamFrom));
	}

	to(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamTo));
	}

	tokenID(): wasmtypes.ScMutableHash {
		return new wasmtypes.ScMutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class ImmutableSetApprovalForAllParams extends wasmtypes.ScProxy {
	approval(): wasmtypes.ScImmutableBool {
		return new wasmtypes.ScImmutableBool(this.proxy.root(sc.ParamApproval));
	}

	operator(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamOperator));
	}
}

export class MutableSetApprovalForAllParams extends wasmtypes.ScProxy {
	approval(): wasmtypes.ScMutableBool {
		return new wasmtypes.ScMutableBool(this.proxy.root(sc.ParamApproval));
	}

	operator(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamOperator));
	}
}

export class ImmutableTransferFromParams extends wasmtypes.ScProxy {
	from(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamFrom));
	}

	to(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamTo));
	}

	tokenID(): wasmtypes.ScImmutableHash {
		return new wasmtypes.ScImmutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class MutableTransferFromParams extends wasmtypes.ScProxy {
	from(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamFrom));
	}

	to(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamTo));
	}

	tokenID(): wasmtypes.ScMutableHash {
		return new wasmtypes.ScMutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class ImmutableBalanceOfParams extends wasmtypes.ScProxy {
	owner(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamOwner));
	}
}

export class MutableBalanceOfParams extends wasmtypes.ScProxy {
	owner(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamOwner));
	}
}

export class ImmutableGetApprovedParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScImmutableHash {
		return new wasmtypes.ScImmutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class MutableGetApprovedParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScMutableHash {
		return new wasmtypes.ScMutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class ImmutableIsApprovedForAllParams extends wasmtypes.ScProxy {
	operator(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamOperator));
	}

	owner(): wasmtypes.ScImmutableAgentID {
		return new wasmtypes.ScImmutableAgentID(this.proxy.root(sc.ParamOwner));
	}
}

export class MutableIsApprovedForAllParams extends wasmtypes.ScProxy {
	operator(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamOperator));
	}

	owner(): wasmtypes.ScMutableAgentID {
		return new wasmtypes.ScMutableAgentID(this.proxy.root(sc.ParamOwner));
	}
}

export class ImmutableOwnerOfParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScImmutableHash {
		return new wasmtypes.ScImmutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class MutableOwnerOfParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScMutableHash {
		return new wasmtypes.ScMutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class ImmutableTokenURIParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScImmutableHash {
		return new wasmtypes.ScImmutableHash(this.proxy.root(sc.ParamTokenID));
	}
}

export class MutableTokenURIParams extends wasmtypes.ScProxy {
	tokenID(): wasmtypes.ScMutableHash {
		return new wasmtypes.ScMutableHash(this.proxy.root(sc.ParamTokenID));
	}
}
