/**
 * Firefish Entry Point
 */

import { EventEmitter } from "node:events";
import boot from "./boot/index.js";
import { inspect } from "node:util";

Error.stackTraceLimit = Infinity;
EventEmitter.defaultMaxListeners = 128;

boot().catch((err) => {
	console.error(inspect(err));
});
