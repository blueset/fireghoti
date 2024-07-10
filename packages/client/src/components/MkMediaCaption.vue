<template>
	<MkModal ref="modal" @click="done(true)" @closed="emit('closed')">
		<div class="container">
			<div class="fullwidth top-caption">
				<div class="mk-dialog">
					<header>
						<Mfm v-if="title" class="title" :text="title" />
						<span
							class="text-count"
							:class="{ over: remainingLength < 0 }"
							>{{ remainingLength }}</span
						>
						<br />
					</header>
					<textarea
						id="captioninput"
						v-model="inputValue"
						autofocus
						:placeholder="input.placeholder"
						@keydown="onInputKeydown"
					></textarea>
					<div
						v-if="
							showOkButton ||
							showCaptionButton ||
							showCancelButton
						"
						class="buttons"
					>
						<MkButton
							inline
							primary
							:disabled="remainingLength < 0"
							@click="ok"
							>{{ i18n.ts.ok }}</MkButton
						>
						<MkButton inline @click="caption">{{
							i18n.ts.caption
						}}</MkButton>
						<MkButton inline @click="cancel">{{
							i18n.ts.cancel
						}}</MkButton>
					</div>
				</div>
			</div>
			<div class="hdrwpsaf fullwidth">
				<header>{{ image.name }}</header>
				<img
					id="imgtocaption"
					:src="image.url"
					:alt="image.comment || undefined"
					:title="image.comment || undefined"
					@click="modal!.close()"
				/>
				<footer>
					<span>{{ image.type }}</span>
					<span>{{ bytes(image.size) }}</span>
					<span v-if="image.properties && image.properties.width"
						>{{ number(image.properties.width) }}px ×
						{{ number(image.properties.height) }}px</span
					>
				</footer>
			</div>
		</div>
	</MkModal>
</template>

<script lang="ts" setup>
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import insertTextAtCursor from "insert-text-at-cursor";
import { length } from "stringz";
import type { entities } from "firefish-js";
import * as os from "@/os";
import MkModal from "@/components/MkModal.vue";
import MkButton from "@/components/MkButton.vue";
import bytes from "@/filters/bytes";
import number from "@/filters/number";
import { i18n } from "@/i18n";
import { getInstanceInfo } from "@/instance";

const props = withDefaults(
	defineProps<{
		image: entities.DriveFile;
		input: {
			placeholder: string;
			default: string;
		};
		title?: string;
		showOkButton?: boolean;
		showCaptionButton?: boolean;
		showCancelButton?: boolean;
		cancelableByBgClick?: boolean;
	}>(),
	{
		showOkButton: true,
		showCaptionButton: true,
		showCancelButton: true,
		cancelableByBgClick: true,
	},
);

const emit = defineEmits<{
	done: [result: { canceled: boolean; result?: string | null }];
	closed: [];
}>();

const modal = ref<InstanceType<typeof MkModal> | null>(null);

const inputValue = ref(props.input.default ? props.input.default : null);

const remainingLength = computed(() => {
	const maxCaptionLength = getInstanceInfo().maxCaptionTextLength ?? 512;
	if (typeof inputValue.value !== "string") return maxCaptionLength;
	return maxCaptionLength - length(inputValue.value);
});

function done(canceled: boolean, result?: string | null) {
	emit("done", { canceled, result });
	modal.value?.close();
}

async function ok() {
	if (!props.showOkButton) return;

	const result = inputValue.value;
	done(false, result);
}

function cancel() {
	done(true);
}

// function onBgClick() {
// 	if (props.cancelableByBgClick) {
// 		cancel();
// 	}
// }

function onKeydown(evt) {
	if (evt.which === 27) {
		// ESC
		cancel();
	}
}

function onInputKeydown(evt) {
	if (evt.which === 13) {
		// Enter
		if (evt.ctrlKey) {
			evt.preventDefault();
			evt.stopPropagation();
			ok();
		}
	}
}

function caption() {
	const img = document.getElementById("imgtocaption") as HTMLImageElement;
	const ta = document.getElementById("captioninput") as HTMLTextAreaElement;
	os.api("drive/files/caption-image", {
		url: img.src,
	}).then((text) => {
		insertTextAtCursor(ta, text.slice(0, 512 - ta.value.length));
	});
}

onMounted(() => {
	document.addEventListener("keydown", onKeydown);
});

onBeforeUnmount(() => {
	document.removeEventListener("keydown", onKeydown);
});
</script>

<style lang="scss" scoped>
.container {
	display: flex;
	width: 100%;
	height: 100%;
	flex-direction: row;
	overflow: scroll;
	position: fixed;
	left: 0;
	top: 0;
}
@media (max-width: 850px) {
	.container {
		flex-direction: column;
	}
	.top-caption {
		padding-bottom: 8px;
	}
}
.fullwidth {
	width: 100%;
	margin: auto;
}
.mk-dialog {
	position: relative;
	padding: 32px;
	min-width: 320px;
	max-width: 480px;
	box-sizing: border-box;
	text-align: center;
	background: var(--panel);
	border-radius: var(--radius);
	margin: auto;

	> header {
		margin: 0 0 8px 0;
		position: relative;

		> .title {
			font-weight: bold;
			font-size: 20px;
		}

		> .text-count {
			opacity: 0.7;
			position: absolute;
			right: 0;
		}
	}

	> .buttons {
		margin-top: 16px;

		> * {
			margin: 0 8px;
		}
	}

	> textarea {
		display: block;
		box-sizing: border-box;
		padding: 0 24px;
		margin: 0;
		width: 100%;
		font-size: 16px;
		border: none;
		border-radius: 0;
		background: transparent;
		color: var(--fg);
		font-family: inherit;
		max-width: 100%;
		min-width: 100%;
		min-height: 90px;

		&:focus-visible {
			outline: none;
		}

		&:disabled {
			opacity: 0.5;
		}
	}
}
.hdrwpsaf {
	display: flex;
	flex-direction: column;
	height: 100%;

	> header,
	> footer {
		align-self: center;
		display: inline-block;
		padding: 6px 9px;
		font-size: 90%;
		background: rgba(0, 0, 0, 0.5);
		border-radius: 6px;
		color: #fff;
	}

	> header {
		margin-bottom: 8px;
		opacity: 0.9;
	}

	> img {
		display: block;
		flex: 1;
		min-height: 0;
		object-fit: contain;
		width: 100%;
		cursor: zoom-out;
		image-orientation: from-image;
	}

	> footer {
		margin-top: 8px;
		opacity: 0.8;

		> span + span {
			margin-left: 0.5em;
			padding-left: 0.5em;
			border-left: solid 1px rgba(255, 255, 255, 0.5);
		}
	}
}
</style>
