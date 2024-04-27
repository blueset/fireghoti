import * as acct from "./acct.js";
import type { Acct } from "./acct.js";
import type { Endpoints } from "./api.types.js";
import type * as ApiTypes from "./api.types.js";
import * as consts from "./consts.js";
import Stream, { Connection } from "./streaming.js";
import * as StreamTypes from "./streaming.types.js";
import type * as TypeUtils from "./type-utils.js";

import type * as SchemaTypes from "./misc/schema.js";
import * as Schema from "./misc/schema.js";

export {
	type Endpoints,
	type ApiTypes,
	Stream,
	Connection as ChannelConnection,
	StreamTypes,
	acct,
	type Acct,
	type TypeUtils,
	Schema,
	type SchemaTypes,
};

export const permissions = consts.permissions;
export const notificationTypes = consts.notificationTypes;
export const noteVisibilities = consts.noteVisibilities;
export const mutedNoteReasons = consts.mutedNoteReasons;
export const languages = consts.languages;
export const ffVisibility = consts.ffVisibility;
export const instanceSortParam = consts.instanceSortParam;

import { langmap, type PostLanguage } from "./misc/langmap.js";
export { langmap, type PostLanguage };

// api extractor not supported yet
//export * as api from './api';
//export * as entities from './entities';
import * as api from "./api.js";
import * as entities from "./entities.js";
export { api, entities };
