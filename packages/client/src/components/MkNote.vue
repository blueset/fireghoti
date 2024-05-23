<template>
	<div
		v-if="!muted.muted"
		v-show="!isDeleted && renotes?.length !== 0"
		:id="appearNote.historyId || appearNote.id"
		ref="el"
		v-hotkey="keymap"
		v-size="{ max: [500, 350] }"
		v-vibrate="5"
		:aria-label="accessibleLabel"
		class="tkcbzcuz note-container"
		:tabindex="!isDeleted ? '-1' : undefined"
		:class="{ renote: isRenote || (renotes && renotes.length > 0) }"
	>
		<MkNoteSub
			v-if="
				appearNote.reply && !detailedView && !collapsedReply && !parents
			"
			:note="appearNote.reply"
			class="reply-to"
		/>
		<MkNoteSub
			v-for="n of parents"
			v-else-if="!detailedView && !collapsedReply && parents"
			:key="n.id"
			:note="n"
			class="reply-to"
		/>
		<div
			v-if="!detailedView"
			class="note-context"
			:class="{
				collapsedReply: collapsedReply && appearNote.reply,
			}"
			@click="noteClick"
		>
			<XNoteHeaderInfo v-bind="{ appearNote, note, collapsedReply, pinned }" />
			<XRenoteBar
				v-bind="{ appearNote, note, isRenote, renotes }"
				@deleted="isDeleted = true"
			/>
		</div>
		<article
			class="article"
			:style="{
				cursor: expandOnNoteClick && !detailedView ? 'pointer' : '',
			}"
			:class="{
				history: appearNote.historyId,
			}"
			@contextmenu.stop="onContextmenu"
			@click="noteClick"
		>
			<div class="main">
				<div class="header-container">
					<MkAvatar class="avatar" :user="appearNote.user" />
					<XNoteHeader
						class="header"
						:note="appearNote"
						:can-open-server-info="true"
					/>
				</div>
				<div class="body">
					<XNoteContent
						class="text"
						:note="appearNote"
						:detailed="true"
						:detailed-view="detailedView"
						:parent-id="appearNote.id"
						:is-long-judger="isLongJudger"
						@push="(e) => router.push(notePage(e))"
						@focusfooter="footerEl!.focus()"
						@expanded="(e) => setPostExpanded(e)"
					></XNoteContent>
					<XNoteTranslation ref="noteTranslation" :note="note"/>
				</div>
				<XNoteFooterInfo class="info" :note="appearNote" :detailedView />
				<XNoteFooter
					class="footer"
					ref="footerEl"
					:note="appearNote"
					:enableEmojiReactions
					:hideEmojiViewer
					:detailedView
					:note-translation="noteTranslation"
					@deleted="isDeleted = true"
					@event:focus="focus"
					@event:blur="blur"
				/>
			</div>
		</article>
	</div>
	<button
		v-else
		class="muted _button"
		@click="muted.muted = false"
		@contextmenu.stop.prevent
	>
		<I18n :src="softMuteReasonI18nSrc(muted.what)" tag="small">
			<template #name>
				<MkA
					v-user-preview="note.userId"
					class="name"
					:to="userPage(note.user)"
				>
					<MkUserName :user="note.user" />
				</MkA>
			</template>
			<template #reason>
				<b class="_blur_text">{{ muted.matched.join(", ") }}</b>
			</template>
		</I18n>
	</button>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref, watch } from "vue";
import type { entities } from "firefish-js";
import XNoteContent from "@/components/note/MkNoteContent.vue";
import MkNoteSub from "@/components/MkNoteSub.vue";
import XNoteHeader from "@/components/note/MkNoteHeader.vue";
import { focusNext, focusPrev } from "@/scripts/focus";
import { getWordSoftMute } from "@/scripts/check-word-mute";
import { useRouter } from "@/router";
import { userPage } from "@/filters/user";
import { defaultStore, noteViewInterruptors } from "@/store";
import { me } from "@/me";
import { i18n } from "@/i18n";
import { useNoteCapture } from "@/scripts/use-note-capture";
import { notePage } from "@/filters/note";
import { deepClone } from "@/scripts/clone";
import type { NoteType } from "@/types/note";
import { isDeleted as _isDeleted, isRenote as _isRenote } from "@/scripts/note";
import XNoteHeaderInfo from "@/components/note/MkNoteHeaderInfo.vue";
import XNoteFooterInfo from "@/components/note/MkNoteFooterInfo.vue";
import XRenoteBar from "@/components/note/MkRenoteBar.vue";
import XNoteFooter from "./note/MkNoteFooter.vue";
import XNoteTranslation from "./note/MkNoteTranslation.vue";
import { showNoteContextMenu } from "@/scripts/show-note-context-menu";

const props = defineProps<{
	note: NoteType;
	parents?: NoteType[];
	renotes?: entities.Note[];
	pinned?: boolean;
	detailedView?: boolean;
	collapsedReply?: boolean;
	hideFooter?: boolean;
	hideEmojiViewer?: boolean;
	isLongJudger?: (note: entities.Note) => boolean;
}>();

// #region Constants
const router = useRouter();
const keymap = {
	r: () => footerEl.value!.reply(true),
	"e|a|plus": () => footerEl.value!.react(true),
	q: () => footerEl.value!.renote(true),
	"up|k": focusBefore,
	"down|j": focusAfter,
	esc: blur,
	"m|o": () => footerEl.value!.menu(true),
	// FIXME: What's this?
	// s: () => showContent.value !== showContent.value,
};
const el = ref<HTMLElement | null>(null);
const footerEl = ref<InstanceType<typeof XNoteFooter> | null>(null);
const enableEmojiReactions = defaultStore.reactiveState.enableEmojiReactions;
const expandOnNoteClick = defaultStore.reactiveState.expandOnNoteClick;
const noteTranslation = ref<InstanceType<typeof XNoteTranslation> | null>(null);
// #endregion

// #region Variables bound to Notes
let capture: ReturnType<typeof useNoteCapture> | undefined;
const note = ref(deepClone(props.note));
const postIsExpanded = ref(false);
const isDeleted = ref(false);
const renotes = ref(props.renotes?.filter((rn) => !_isDeleted(rn.id)));
const muted = ref(
	getWordSoftMute(
		note.value,
		me?.id,
		defaultStore.reactiveState.mutedWords.value,
		defaultStore.reactiveState.mutedLangs.value,
	),
);
// #endregion

// #region computed
const isRenote = computed(() => _isRenote(note.value));
const appearNote = computed(() =>
	isRenote.value ? (note.value.renote as NoteType) : note.value,
);
const accessibleLabel = computed(() => {
	let label = `${appearNote.value.user.username}; `;
	if (appearNote.value.renote) {
		label += `${i18n.ts.renoted} ${appearNote.value.renote.user.username}; `;
		if (appearNote.value.renote.cw) {
			label += `${i18n.ts.cw}: ${appearNote.value.renote.cw}; `;
			if (postIsExpanded.value) {
				label += `${appearNote.value.renote.text}; `;
			}
		} else {
			label += `${appearNote.value.renote.text}; `;
		}
	} else {
		if (appearNote.value.cw) {
			label += `${i18n.ts.cw}: ${appearNote.value.cw}; `;
			if (postIsExpanded.value) {
				label += `${appearNote.value.text}; `;
			}
		} else {
			label += `${appearNote.value.text}; `;
		}
	}
	const date = new Date(appearNote.value.createdAt);
	label += `${date.toLocaleTimeString()}`;
	return label;
});
// #endregion

async function pluginInit(newNote: NoteType) {
	// plugin
	if (noteViewInterruptors.length > 0) {
		let result = deepClone(newNote);
		for (const interruptor of noteViewInterruptors) {
			result = await interruptor.handler(result);
		}
		note.value = result;
	}
}

function recalculateRenotes() {
	renotes.value = props.renotes?.filter((rn) => !_isDeleted(rn.id));
}

async function init(newNote: NoteType, first = false) {
	if (!first) {
		// plugin
		if (noteViewInterruptors.length > 0) {
			await pluginInit(newNote);
		} else {
			note.value = deepClone(newNote);
		}
	}
	postIsExpanded.value = false;
	isDeleted.value = _isDeleted(note.value.id);
	if (appearNote.value.historyId == null) {
		capture?.close();
		capture = useNoteCapture({
			rootEl: el,
			note: appearNote,
			isDeletedRef: isDeleted,
		});
		if (isRenote.value === true) {
			useNoteCapture({
				rootEl: el,
				note,
				isDeletedRef: isDeleted,
			});
		}
		if (props.renotes) {
			const renoteDeletedTrigger = ref(false);
			for (const renote of props.renotes) {
				useNoteCapture({
					rootEl: el,
					note: ref(renote),
					isDeletedRef: renoteDeletedTrigger,
				});
			}
			watch(renoteDeletedTrigger, recalculateRenotes);
		}
	}
}

init(props.note, true);

onMounted(() => {
	pluginInit(note.value);
});

watch(isDeleted, () => {
	if (isDeleted.value === true) {
		if (props.parents && props.parents.length > 0) {
			let noteTakePlace: NoteType | null = null;
			while (noteTakePlace == null || _isDeleted(noteTakePlace.id)) {
				if (props.parents.length === 0) {
					return;
				}
				noteTakePlace = props.parents[props.parents.length - 1];
				props.parents.pop();
			}
			noteTakePlace.repliesCount -= 1;
			init(noteTakePlace);
			isDeleted.value = false;
		}
	}
});

watch(
	() => props.note.id,
	(o, n) => {
		if (o !== n && _isDeleted(note.value.id) !== true) {
			init(props.note);
		}
	},
);
watch(() => props.renotes?.length, recalculateRenotes);

function softMuteReasonI18nSrc(what?: string) {
	if (what === "note") return i18n.ts.userSaysSomethingReason;
	if (what === "reply") return i18n.ts.userSaysSomethingReasonReply;
	if (what === "renote") return i18n.ts.userSaysSomethingReasonRenote;
	if (what === "quote") return i18n.ts.userSaysSomethingReasonQuote;

	// I don't think here is reachable, but just in case
	return i18n.ts.userSaysSomething;
}

function onContextmenu(ev: MouseEvent): void {
	showNoteContextMenu({
		ev,
		note: appearNote.value,
		react: footerEl.value!.react,
	});
}

function focus() {
	el.value!.focus();
}

function blur() {
	el.value!.blur();
}

function focusBefore() {
	focusPrev(el.value);
}

function focusAfter() {
	focusNext(el.value);
}

function scrollIntoView() {
	el.value!.scrollIntoView();
}

function noteClick(e) {
	if (
		document.getSelection()?.type === "Range" ||
		props.detailedView ||
		!expandOnNoteClick
	) {
		e.stopPropagation();
	} else {
		router.push(notePage(appearNote.value));
	}
}

function setPostExpanded(val: boolean) {
	postIsExpanded.value = val;
}

defineExpose({
	focus,
	blur,
	scrollIntoView,
});
</script>

<style lang="scss" scoped>
.tkcbzcuz {
	position: relative;
	transition: box-shadow 0.1s ease;
	font-size: 1.05em;
	overflow: clip;
	contain: content;
	-webkit-tap-highlight-color: transparent;

	// これらの指定はパフォーマンス向上には有効だが、ノートの高さは一定でないため、
	// 下の方までスクロールすると上のノートの高さがここで決め打ちされたものに変化し、表示しているノートの位置が変わってしまう
	// ノートがマウントされたときに自身の高さを取得し contain-intrinsic-size を設定しなおせばほぼ解決できそうだが、
	// 今度はその処理自体がパフォーマンス低下の原因にならないか懸念される。また、被リアクションでも高さは変化するため、やはり多少のズレは生じる
	// 一度レンダリングされた要素はブラウザがよしなにサイズを覚えておいてくれるような実装になるまで待った方が良さそう(なるのか？)
	//content-visibility: auto;
	//contain-intrinsic-size: 0 128px;

	&:focus-visible {
		outline: none;

		&:after {
			content: "";
			pointer-events: none;
			display: block;
			position: absolute;
			z-index: 10;
			top: 0;
			left: 0;
			right: 0;
			bottom: 0;
			margin: auto;
			width: calc(100% - 8px);
			height: calc(100% - 8px);
			border: solid 1px var(--focus);
			border-radius: var(--radius);
			box-sizing: border-box;
		}
	}

	& > .article > .main {
		&:hover,
		&:focus-within {
			:deep(.footer .button) {
				opacity: 1;
			}
		}
	}

	> .reply-to {
		& + .note-context {
			.line::before {
				content: "";
				display: block;
				margin-bottom: -4px;
				margin-top: 16px;
				border-left: 2px solid currentColor;
				margin-left: calc((var(--avatarSize) / 2) - 1px);
				opacity: 0.25;
			}
		}
	}

	.note-context {
		position: relative;
		padding: 0 32px 0 32px;
		display: flex;
		flex-wrap: wrap;
		z-index: 1;
		&:first-child {
			margin-top: 20px;
		}
		> :not(.line) {
			width: 0;
			flex-grow: 1;
			position: relative;
			line-height: 28px;
		}
		> .line {
			position: relative;
			z-index: 2;
			width: 0;
			display: flex;
			margin-right: 0;
			margin-top: 0;
			flex-grow: 0;
			pointer-events: none;
		}

		> div > i {
			margin-left: -0.5px;
		}

		&.collapsedReply {
			.line {
				opacity: 0.25;
				&::after {
					content: "";
					position: absolute;
					border-left: 2px solid currentColor;
					border-top: 2px solid currentColor;
					margin-left: calc(var(--avatarSize) / 2 - 1px);
					width: calc(var(--avatarSize) / 2 + 14px);
					border-top-left-radius: calc(var(--avatarSize) / 4);
					top: calc(50% - 1px);
					height: calc(50% + 5px);
				}
			}
			.info {
				color: var(--fgTransparentWeak);
				transition: color 0.2s;
			}
			.avatar {
				width: 1.2em;
				height: 1.2em;
				border-radius: 2em;
				overflow: hidden;
				margin-right: 0.4em;
				background: var(--panelHighlight);
			}
			.username {
				font-weight: 700;
				flex-shrink: 0;
				max-width: 30%;
				&::after {
					content: ": ";
				}
			}
			&:hover,
			&:focus-within {
				.info {
					color: var(--fg);
				}
			}
		}
	}

	> .article {
		position: relative;
		overflow: clip;
		padding: 20px 32px 10px;
		margin-top: -16px;
		&.history {
			margin-top: -90px !important;
		}

		&:first-child,
		&:nth-child(2) {
			margin-top: -100px;
			padding-top: 104px;
		}

		@media (pointer: coarse) {
			cursor: default;
		}

		.header-container {
			display: flex;
			position: relative;
			z-index: 2;
			> .avatar {
				flex-shrink: 0;
				display: block;
				margin: 0 14px 0 0;
				width: var(--avatarSize);
				height: var(--avatarSize);
				position: relative;
				top: 0;
				left: 0;
			}
			> .header {
				width: 0;
				flex-grow: 1;
			}
		}
		> .main {
			flex: 1;
			min-width: 0;

			> .body {
				margin-top: 0.7em;
				> .renote {
					padding-top: 8px;
					> * {
						padding: 16px;
						border: solid 1px var(--renote);
						border-radius: 8px;
						transition: background 0.2s;
						&:hover,
						&:focus-within {
							background-color: var(--panelHighlight);
						}
					}
				}
			}
		}
	}

	> .reply {
		border-top: solid 0.5px var(--divider);
	}

	&.max-width_500px {
		font-size: 0.975em;
		--avatarSize: 46px;
		padding-top: 6px;
		> .note-context {
			padding-inline: 16px;
			margin-top: 8px;
			> :not(.line) {
				margin-top: 0px;
			}
			> .line {
				margin-right: 0;
				&::before {
					margin-top: 8px;
				}
			}
		}
		> .article {
			padding: 18px 16px 8px;
			&:first-child,
			&:nth-child(2) {
				padding-top: 104px;
			}
			> .main > .header-container > .avatar {
				margin-right: 10px;
				// top: calc(14px + var(--stickyTop, 0px));
			}
		}
	}

	&.max-width_300px {
		--avatarSize: 40px;
	}
}

.muted {
	padding: 8px;
	text-align: center;
	opacity: 0.7;
	width: 100%;

	._blur_text {
		pointer-events: auto;
	}
	&:active ._blur_text {
		filter: blur(0px);
	}
}
</style>
