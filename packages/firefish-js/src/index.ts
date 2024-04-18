import * as acct from "./acct";
import type { Acct } from "./acct";
import { Endpoints } from "./api.types";
import type * as ApiTypes from "./api.types";
import * as consts from "./consts";
import Stream, { Connection } from "./streaming";
import * as StreamTypes from "./streaming.types";
import type * as TypeUtils from "./type-utils";

export {
	Endpoints,
	type ApiTypes,
	Stream,
	Connection as ChannelConnection,
	StreamTypes,
	acct,
	type Acct,
	type TypeUtils,
};

export const permissions = consts.permissions;
export const notificationTypes = consts.notificationTypes;
export const noteVisibilities = consts.noteVisibilities;
export const mutedNoteReasons = consts.mutedNoteReasons;
export const languages = consts.languages;
export const ffVisibility = consts.ffVisibility;
export const instanceSortParam = consts.instanceSortParam;

// api extractor not supported yet
//export * as api from './api';
//export * as entities from './entities';
import * as api from "./api";
import * as entities from "./entities";
export { api, entities };
