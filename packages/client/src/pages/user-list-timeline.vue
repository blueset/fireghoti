<template>
	<MkStickyContainer>
		<template #header
			><MkPageHeader :actions="headerActions" :tabs="headerTabs"
		/></template>
		<div ref="rootEl" v-size="{ min: [800] }" class="eqqrhokj">
			<div class="tl _block">
				<XTimeline
					ref="tlEl"
					:key="listId"
					class="tl"
					src="list"
					:list="listId"
					:sound="true"
				/>
			</div>
		</div>
	</MkStickyContainer>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from "vue";
import XTimeline from "@/components/MkTimeline.vue";
import * as os from "@/os";
import { useRouter } from "@/router";
import { definePageMetadata } from "@/scripts/page-metadata";
import { i18n } from "@/i18n";
import icon from "@/scripts/icon";

const router = useRouter();

const props = defineProps<{
	listId: string;
}>();

const list = ref(null);
const tlEl = ref<InstanceType<typeof XTimeline>>();
const rootEl = ref<HTMLElement>();

watch(
	() => props.listId,
	async () => {
		list.value = await os.api("users/lists/show", {
			listId: props.listId,
		});
	},
	{ immediate: true },
);

function settings() {
	router.push(`/my/lists/${props.listId}`);
}

async function timetravel() {
	const { canceled, result: date } = await os.inputDate({
		title: i18n.ts.date,
	});
	if (canceled) return;

	tlEl.value.timetravel(date);
}

const headerActions = computed(() =>
	list.value
		? [
				{
					icon: `${icon("ph-calendar-blank")}`,
					text: i18n.ts.jumpToSpecifiedDate,
					handler: timetravel,
				},
				{
					icon: `${icon("ph-list-bullets")}`,
					title: i18n.ts.lists,
					text: i18n.ts.lists,
					iconOnly: true,
					handler: chooseList,
				},
				{
					icon: `${icon("ph-gear-six")}`,
					text: i18n.ts.settings,
					handler: settings,
				},
			]
		: [],
);

const headerTabs = computed(() => []);

async function chooseList(ev: MouseEvent) {
	await os.api("users/lists/list").then((res) => {
		const items = [
			{
				type: "link" as const,
				text: i18n.ts.manageLists,
				icon: `${icon("ph-faders-horizontal")}`,
				to: "/my/lists",
			},
		].concat(
			res.map((list) => ({
				type: "link" as const,
				text: list.name,
				icon: list.id === props.listId ? icon("ph-check") : "",
				to: `/timeline/list/${list.id}`,
			})),
		);
		os.popupMenu(items, ev.currentTarget ?? ev.target);
	});
}

definePageMetadata(
	computed(() =>
		list.value
			? {
					title: list.value.name,
					icon: `${icon("ph-list-bullets")}`,
				}
			: null,
	),
);
</script>

<style lang="scss" scoped>
.eqqrhokj {
	padding: var(--margin);
	> .tl {
		background: none;
		border-radius: var(--radius);
	}

	&.min-width_800px {
		max-width: 800px;
		margin: 0 auto;
	}
}
</style>
