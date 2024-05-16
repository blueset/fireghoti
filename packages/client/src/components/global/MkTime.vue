<template>
	<time :title="absolute">
		<template v-if="invalid">{{ i18n.ts._ago.invalid }}</template>
		<template v-else-if="mode === 'relative'">{{ relative }}</template>
		<template v-else-if="mode === 'absolute'">{{ absolute }}</template>
		<template v-else-if="mode === 'detail'"
			>{{ absolute }} ({{ relative }})</template
		>
	</time>
</template>

<script lang="ts" setup>
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { i18n } from "@/i18n";
import { dateTimeFormat } from "@/scripts/intl-const";

const props = withDefaults(
	defineProps<{
		time: Date | string | number | null;
		origin?: Date | null;
		mode?: "relative" | "absolute" | "detail";
	}>(),
	{
		mode: "relative",
	},
);

function getDateSafe(n: Date | string | number) {
	try {
		if (n instanceof Date) {
			return n;
		}
		return new Date(n);
	} catch (err) {
		return {
			getTime: () => Number.NaN,
		};
	}
}

const _time = computed(() =>
	props.time == null ? Number.NaN : getDateSafe(props.time).getTime(),
);
const invalid = computed(() => Number.isNaN(_time.value));
const absolute = computed(() =>
	!invalid.value ? dateTimeFormat.format(_time.value) : i18n.ts._ago.invalid,
);

const now = ref(props.origin?.getTime() ?? Date.now());

const relative = computed<string>(() => {
	if (props.mode === "absolute") return ""; // absoluteではrelativeを使わないので計算しない
	if (invalid.value) return i18n.ts._ago.invalid;

	const ago = Math.abs(now.value - _time.value) / 1000; /* ms */
	const agoType = now.value > _time.value ? "_ago" : "_later";

	if (ago >= 31536000) {
		return i18n.t(`${agoType}.yearsAgo`, {
			n: Math.floor(ago / 31536000).toString(),
		});
	}
	if (ago >= 2592000) {
		return i18n.t(`${agoType}.monthsAgo`, {
			n: Math.floor(ago / 2592000).toString(),
		});
	}
	if (ago >= 604800) {
		return i18n.t(`${agoType}.weeksAgo`, {
			n: Math.floor(ago / 604800).toString(),
		});
	}
	if (ago >= 86400) {
		return i18n.t(`${agoType}.daysAgo`, {
			n: Math.floor(ago / 86400).toString(),
		});
	}
	if (ago >= 3600) {
		return i18n.t(`${agoType}.hoursAgo`, {
			n: Math.floor(ago / 3600).toString(),
		});
	}
	if (ago >= 60) {
		return i18n.t(`${agoType}.minutesAgo`, {
			n: (~~(ago / 60)).toString(),
		});
	}
	if (ago >= 10) {
		return i18n.t(`${agoType}.secondsAgo`, {
			n: (~~(ago % 60)).toString(),
		});
	}
	if (ago >= -1) {
		return i18n.ts[agoType].justNow;
	}
	return i18n.ts[agoType].future;
});

let tickId: number | undefined;

function tick(forceUpdateTicker = false) {
	if (
		invalid.value ||
		props.origin ||
		(props.mode !== "relative" && props.mode !== "detail")
	) {
		if (tickId) window.clearInterval(tickId);
		tickId = undefined;
		return;
	}

	const _now = Date.now();
	const agoPrev = (now.value - _time.value) / 1000; /* ms */ // 現状のinterval

	now.value = _now;

	const ago = (now.value - _time.value) / 1000; /* ms */ // 次のinterval
	const prev = agoPrev < 60 ? 10000 : agoPrev < 3600 ? 60000 : 180000;
	const next = ago < 60 ? 10000 : ago < 3600 ? 60000 : 180000;

	if (!tickId) {
		tickId = window.setInterval(tick, next);
	} else if (prev < next || forceUpdateTicker) {
		window.clearInterval(tickId);
		tickId = window.setInterval(tick, next);
	}
}

watch(
	() => props.time,
	() => tick(true),
);

onMounted(() => {
	tick();
});

onUnmounted(() => {
	if (tickId) window.clearInterval(tickId);
});
</script>
