import type { Account } from "@/account";
import { reactive } from "vue";

const accountData = localStorage.getItem("account");

// TODO: 外部からはreadonlyに
export const me = accountData
	? reactive(JSON.parse(accountData) as Account)
	: null;

export const isSignedIn = me != null;
export const isModerator = me != null && (me.isModerator || me.isAdmin);
export const isEmojiMod = isModerator || me?.emojiModPerm !== "unauthorized";
export const isAdmin = me?.isAdmin;
