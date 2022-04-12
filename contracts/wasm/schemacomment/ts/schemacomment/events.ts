// COPYRIGHT OF A TEST SCHEMA DEFINITION 1
// COPYRIGHT OF A TEST SCHEMA DEFINITION 2


// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";
import * as wasmtypes from "wasmlib/wasmtypes";

export class SchemaCommentEvents {

	// header comment for TestEvent 1
// header comment for TestEvent 2
	testEvent(eventParam1: string, eventParam2: string): void {
		const evt = new wasmlib.EventEncoder("schemacomment.testEvent");
		evt.encode(wasmtypes.stringToString(eventParam1));
		evt.encode(wasmtypes.stringToString(eventParam2));
		evt.emit();
	}
}
