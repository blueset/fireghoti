<template>
	<canvas
		v-if="!loaded"
		ref="canvas"
		:width="size"
		:height="size"
		:title="title"
	/>
	<img
		v-if="src"
		:src="src"
		:title="title"
		:type="type"
		:alt="alt"
		:class="{
			cover,
			wide: largestDimension === 'width',
			tall: largestDimension === 'height',
		}"
		:style="{ 'object-fit': cover ? 'cover' : null }"
		loading="lazy"
		@load="onLoad"
	/>
	<i
		class="alt-indicator"
		:class="icon('ph-subtitles')"
		v-if="alt && showAltIndicator"
		v-tooltip.noLabel="
			`${i18n.ts.alt}: ${
				alt.length > 200 ? alt.trim().slice(0, 200) + '...' : alt.trim()
			}`
		"
	></i>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { decodeBlurHash } from "fast-blurhash";
import icon from "@/scripts/icon";
import { i18n } from "@/i18n";

const props = withDefaults(
	defineProps<{
		src?: string | null;
		hash?: string;
		alt?: string | null;
		type?: string | null;
		title?: string | null;
		size?: number;
		cover?: boolean;
		largestDimension?: "width" | "height";
		showAltIndicator?: boolean;
	}>(),
	{
		src: null,
		type: null,
		alt: null,
		title: null,
		size: 64,
		cover: true,
		showAltIndicator: false,
	},
);

const canvas = ref<HTMLCanvasElement>();
const loaded = ref(false);

function draw() {
	if (props.hash == null || canvas.value == null) return;
	const pixels = decodeBlurHash(props.hash, props.size, props.size);
	const ctx = canvas.value.getContext("2d");
	const imageData = ctx!.createImageData(props.size, props.size);
	imageData.data.set(pixels);
	ctx!.putImageData(imageData, 0, 0);
}

function onLoad() {
	loaded.value = true;
}

onMounted(() => {
	draw();
});
</script>

<style lang="scss" scoped>
canvas,
img {
	display: block;
	max-width: 100%;
	max-height: 100%;
}

canvas {
	position: absolute;
	inset: 0;
	object-fit: cover;
	width: 100%;
	height: 100%;
}

img {
	object-fit: contain;

	&.wide {
		width: 100%;
	}

	&.tall {
		height: 100%;
	}
}

i.alt-indicator {
	display: flex;
	gap: 4px;
	position: absolute;
	border-radius: 6px;
	overflow: hidden;
	top: 0;
	right: 0;
	background-color: var(--accentedBg);
	-webkit-backdrop-filter: var(--blur, blur(15px));
	backdrop-filter: var(--blur, blur(15px));
	color: var(--accent);
	font-size: 1em;
	padding: 6px 8px;
	text-align: center;
}
</style>
