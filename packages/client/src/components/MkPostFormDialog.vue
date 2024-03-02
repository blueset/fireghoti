<template>
	<MkModal
		ref="modal"
		:prefer-type="'dialog'"
		@click="modal.close()"
		@closed="onModalClosed()"
	>
		<MkPostForm
			ref="form"
			style="margin: 0 auto auto auto"
			v-bind="props"
			autofocus
			freeze-after-posted
			@posted="onPosted"
			@cancel="modal.close()"
			@esc="modal.close()"
		/>
	</MkModal>
</template>

<script lang="ts" setup>
import { shallowRef } from "vue";

import type { entities, languages, noteVisibilities } from "firefish-js";
import MkModal from "@/components/MkModal.vue";
import MkPostForm from "@/components/MkPostForm.vue";

const props = defineProps<{
	reply?: entities.Note;
	renote?: entities.Note;
	channel?: any; // TODO
	mention?: entities.User;
	specified?: entities.User;
	initialText?: string;
	initialVisibility?: typeof noteVisibilities;
	initialLanguage?: typeof languages;
	initialFiles?: entities.DriveFile[];
	initialLocalOnly?: boolean;
	initialVisibleUsers?: entities.User[];
	initialNote?: entities.Note;
	instant?: boolean;
	fixed?: boolean;
	autofocus?: boolean;
	editId?: entities.Note["id"];
}>();

const emit = defineEmits<{
	(ev: "closed"): void;
}>();

const modal = shallowRef<InstanceType<typeof MkModal>>();
const form = shallowRef<InstanceType<typeof MkPostForm>>();

function onPosted() {
	modal.value.close({
		useSendAnimation: true,
	});
}

function onModalClosed() {
	emit("closed");
}
</script>
