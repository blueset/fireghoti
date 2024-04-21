<template>
	<MkStickyContainer>
		<template #header><MkPageHeader /></template>
		<MkSpacer :content-max="800">
			<MkPagination ref="paginationComponent" :pagination="pagination">
				<template #empty>
					<div class="_fullinfo">
						<img
							src="/static-assets/badges/info.webp"
							aria-label="none"
							class="_ghost"
						/>
						<div>{{ i18n.ts.noSentFollowRequests }}</div>
					</div>
				</template>
				<template #default="{ items }">
					<div class="mk-follow-requests">
						<div
							v-for="req in items"
							:key="req.id"
							class="user _panel"
						>
							<MkAvatar
								class="avatar"
								:user="req.followee"
								:show-indicator="true"
								disable-link
							/>
							<div class="body">
								<div class="name">
									<MkA
										v-user-preview="req.followee.id"
										class="name"
										:to="userPage(req.followee)"
										><MkUserName :user="req.followee"
									/></MkA>
									<p class="acct">
										@{{ acct.toString(req.followee) }}
									</p>
								</div>
								<div
									v-if="req.followee.description"
									class="description"
									:title="req.followee.description"
								>
									<Mfm
										:text="req.followee.description"
										:is-note="false"
										:author="req.followee"
										:i="me"
										:custom-emojis="req.followee.emojis"
										:plain="true"
										:nowrap="true"
									/>
								</div>
							</div>
						</div>
					</div>
				</template>
			</MkPagination>
		</MkSpacer>
	</MkStickyContainer>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import { acct } from "firefish-js";
import MkPagination from "@/components/MkPagination.vue";
import type { MkPaginationType } from "@/components/MkPagination.vue";
import { userPage } from "@/filters/user";
// import * as os from "@/os";
import { i18n } from "@/i18n";
import { definePageMetadata } from "@/scripts/page-metadata";
import { me } from "@/me";
import icon from "@/scripts/icon";

const paginationComponent = ref<MkPaginationType<
	typeof pagination.endpoint
> | null>(null);

const pagination = {
	endpoint: "following/requests/sent" as const,
	limit: 10,
	noPaging: true,
};

definePageMetadata(
	computed(() => ({
		title: i18n.ts.sentFollowRequests,
		icon: `${icon("ph-hand-waving")}`,
	})),
);
</script>

<style lang="scss" scoped>
.mk-follow-requests {
	> .user {
		display: flex;
		padding: 16px;
		margin: 10px 0 auto;

		> .avatar {
			display: block;
			flex-shrink: 0;
			margin: 0 12px 0 0;
			width: 42px;
			height: 42px;
			border-radius: 8px;
		}

		> .body {
			display: flex;
			width: calc(100% - 54px);
			position: relative;

			> .name {
				width: 45%;

				@media (max-width: 500px) {
					width: 100%;
				}

				> .name,
				> .acct {
					display: block;
					white-space: nowrap;
					text-overflow: ellipsis;
					overflow: hidden;
					margin: 0;
				}

				> .name {
					font-size: 16px;
					line-height: 24px;
				}

				> .acct {
					font-size: 15px;
					line-height: 16px;
					opacity: 0.7;
				}
			}

			> .description {
				width: 55%;
				line-height: 42px;
				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;
				opacity: 0.7;
				font-size: 14px;
				padding-right: 40px;
				padding-left: 8px;
				box-sizing: border-box;

				@media (max-width: 500px) {
					display: none;
				}
			}
		}
	}
}
</style>
