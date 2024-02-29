<template>
	<div ref="rootEl">
		<div
			v-if="pullStarted"
			:class="$style.frame"
			:style="`--frame-min-height: ${
				pullDistance /
				(PULL_BRAKE_BASE + pullDistance / PULL_BRAKE_FACTOR)
			}px;`"
		>
			<div :class="$style.frameContent">
				<div :class="$style.text">
					<template v-if="pullEnded">{{
						i18n.ts.releaseToReload
					}}</template>
					<template v-else-if="isRefreshing">{{
						i18n.ts.reloading
					}}</template>
					<template v-else>{{ i18n.ts.pullDownToReload }}</template>
				</div>
			</div>
		</div>
		<div :class="{ [$style.slotClip]: pullStarted }">
			<slot />
		</div>
	</div>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref, shallowRef } from "vue";
// import { deviceKind } from "@/scripts/device-kind";
import { i18n } from "@/i18n";
import { defaultStore } from "@/store";

const SCROLL_STOP = 10;
const MAX_PULL_DISTANCE = Infinity;
const FIRE_THRESHOLD = defaultStore.state.pullToRefreshThreshold;
const RELEASE_TRANSITION_DURATION = 120;
const PULL_BRAKE_BASE = 1.5;
const PULL_BRAKE_FACTOR = 100;

const pullStarted = ref(false);
const pullEnded = ref(false);
const isRefreshing = ref(false);
const pullDistance = ref(0);

let disabled = false;

let supportPointerDesktop = false;
let startScreenY: number | null = null;

const rootEl = shallowRef<HTMLDivElement>();
let scrollEl: HTMLElement | null = null;

const props = withDefaults(
	defineProps<{
		refresher: () => Promise<void>;
	}>(),
	{
		refresher: () => Promise.resolve(),
	},
);

const emits = defineEmits<{
	(ev: "refresh"): void;
}>();

function getScrollableParentElement(node) {
	if (node == null) return null;
	if (node.scrollHeight > node.clientHeight) return node;

	return getScrollableParentElement(node.parentNode);
}

function getScreenY(event) {
	if (supportPointerDesktop) return event.screenY;
	return event.touches[0].screenY;
}

function moveStart(event) {
	if (!pullStarted.value && !isRefreshing.value && !disabled) {
		pullStarted.value = true;
		startScreenY = getScreenY(event);
		pullDistance.value = 0;
	}
}

function moveBySystem(to: number): Promise<void> {
	return new Promise((r) => {
		const initialHeight = pullDistance.value;
		const overHeight = pullDistance.value - to;
		if (overHeight < 1) {
			r();
			return;
		}
		const startTime = Date.now();
		let intervalId = setInterval(() => {
			const time = Date.now() - startTime;
			if (time > RELEASE_TRANSITION_DURATION) {
				pullDistance.value = to;
				clearInterval(intervalId);
				r();
				return;
			}
			const nextHeight =
				initialHeight -
				(overHeight / RELEASE_TRANSITION_DURATION) * time;
			if (pullDistance.value < nextHeight) return;
			pullDistance.value = nextHeight;
		}, 1);
	});
}

async function fixOverContent() {
	if (pullDistance.value > FIRE_THRESHOLD) await moveBySystem(FIRE_THRESHOLD);
}

async function closeContent() {
	if (pullDistance.value > 0) await moveBySystem(0);
}

function moveEnd() {
	if (pullStarted.value && !isRefreshing.value) {
		startScreenY = null;
		if (pullEnded.value) {
			pullEnded.value = false;
			isRefreshing.value = true;
			fixOverContent().then(() => {
				emits("refresh");
				props.refresher().then(() => {
					refreshFinished();
				});
			});
		} else {
			closeContent().then(() => (pullStarted.value = false));
		}
	}
}

function moving(event) {
	if (!pullStarted.value || isRefreshing.value || disabled) return;
	if (scrollEl == null) scrollEl = getScrollableParentElement(rootEl);
	if (
		(scrollEl?.scrollTop ?? 0) >
		(supportPointerDesktop ? SCROLL_STOP : SCROLL_STOP + pullDistance.value)
	) {
		pullDistance.value = 0;
		pullEnded.value = false;
		moveEnd();
		return;
	}
	if (startScreenY === null) {
		startScreenY = getScreenY(event);
	}
	const moveScreenY = getScreenY(event);
	const moveHeight = moveScreenY - startScreenY!;
	pullDistance.value = Math.min(Math.max(moveHeight, 0), MAX_PULL_DISTANCE);
	pullEnded.value = pullDistance.value >= FIRE_THRESHOLD;
}

function refreshFinished() {
	closeContent().then(() => {
		pullStarted.value = false;
		isRefreshing.value = false;
	});
}

function setDisabled(value) {
	disabled = value;
}

onMounted(() => {
	// supportPointerDesktop = !!window.PointerEvent && deviceKind === "desktop";

	if (supportPointerDesktop) {
		rootEl.value?.addEventListener("pointerdown", moveStart);
		// "up" event won't be emmitted by mouse pointer on desktop
		window.addEventListener("pointerup", moveEnd);
		rootEl.value?.addEventListener("pointermove", moving, {
			passive: true,
		});
	} else {
		rootEl.value?.addEventListener("touchstart", moveStart);
		rootEl.value?.addEventListener("touchend", moveEnd);
		rootEl.value?.addEventListener("touchmove", moving, { passive: true });
	}
});

onUnmounted(() => {
	if (supportPointerDesktop) window.removeEventListener("pointerup", moveEnd);
});

defineExpose({
	setDisabled,
});
</script>

<style lang="scss" module>
.frame {
	position: relative;
	overflow: clip;
	width: 100%;
	min-height: var(--frame-min-height, 0px);
	mask-image: linear-gradient(90deg, #000 0%, #000 80%, transparent);
	-webkit-mask-image: -webkit-linear-gradient(
		90deg,
		#000 0%,
		#000 80%,
		transparent
	);
	pointer-events: none;
}
.frameContent {
	position: absolute;
	bottom: 0;
	width: 100%;
	margin: 5px 0;
	display: flex;
	flex-direction: column;
	align-items: center;
	font-size: 14px;
	> .icon,
	> .loader {
		margin: 6px 0;
	}
	> .icon {
		transition: transform 0.25s;
		&.refresh {
			transform: rotate(180deg);
		}
	}
	> .text {
		margin: 5px 0;
	}
}
.slotClip {
	overflow-y: clip;
}
</style>
