<template>
	<MkStickyContainer>
		<template #header
			><MkPageHeader
				:display-back-button="true"
		/></template>
		<MkSpacer :content-max="800">
			
			<MkLoading v-if="!loaded" />
			<MkPagination
				v-else
				ref="pagingComponent"
				v-slot="{ items: noteEdits }"
				:pagination="pagination"
			>
				<div ref="tlEl" class="giivymft noGap">
					<XList
						v-slot="{ item: noteEdit }"
						:items="convertNoteEditsToNotes(noteEdits as NoteEdit[])"
						class="notes"
						:no-gap="true"
					>
						<XNote
							:key="noteEdit.id"
							class="qtqtichx"
							:note="noteEdit"
							:hide-footer="true"
						/>
					</XList>
				</div>
			</MkPagination>
				
		</MkSpacer>
	</MkStickyContainer>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref } from "vue";
import MkPagination from "@/components/MkPagination.vue"
import type { Paging } from "@/components/MkPagination.vue"
import { api } from "@/os";
import XList from "@/components/MkDateSeparatedList.vue";
import XNote from "@/components/MkNote.vue";
import { i18n } from "@/i18n";
import { definePageMetadata } from "@/scripts/page-metadata";
import icon from "@/scripts/icon";
import type { Note, NoteEdit } from "firefish-js/src/entities";

const pagingComponent = ref<InstanceType<typeof MkPagination>>();

const props = defineProps<{
	noteId: string;
}>();

const pagination: Paging = {
	endpoint: "notes/history" as const,
	limit: 10,
	offsetMode: true,
	params: computed(() => ({
		noteId: props.noteId
	})),
};

definePageMetadata(
	computed(() => ({
		title: i18n.t("noteEditHistory"),
		icon: `${icon("ph-clock-countdown")}`,
	})),
);

const note = ref<Note>({} as Note);
const loaded = ref(false);

onMounted(() => {
	api("notes/show", {
		noteId: props.noteId,
	}).then((res) => {
		// Remove unnecessary parts
		res.renote = undefined;
		res.renoteId = null;
		res.reply = undefined;
		res.replyId = null;
		
		note.value = res;
		loaded.value = true;
	});
});

function convertNoteEditsToNotes(noteEdits: NoteEdit[]) {
	return [note.value].concat(
		noteEdits.map(e => convertNoteEditToNote(e))
	);
}

function convertNoteEditToNote(noteEdit: NoteEdit): Note {
	return Object.assign({}, note.value, {
		id: crypto.randomUUID(), // Don't use noteId
		createdAt: noteEdit.updatedAt,
		text: noteEdit.text,
		cw: noteEdit.cw,
		_shouldInsertAd_: false,
	});
}
</script>

<style lang="scss" scoped>
.giivymft {
	&.noGap {
		> .notes {
			background: var(--panel) !important;
			border-radius: var(--radius);
		}
	}
	&:not(.noGap) {
		> .notes {
			.qtqtichx {
				background: var(--panel);
				border-radius: var(--radius);
			}
		}
	}
}
</style>
