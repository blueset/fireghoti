import { ILocalUser } from "@/models/entities/user.js";
import { Notes, Notifications, SwSubscriptions } from "@/models/index.js";
import { PaginationHelpers } from "@/server/api/mastodon/helpers/pagination.js";
import { Notification } from "@/models/entities/notification.js";
import { MastoApiError } from "@/server/api/mastodon/middleware/catch-errors.js";
import { MastoContext } from "@/server/api/mastodon/index.js";
import { SwSubscription } from "@/models/entities/sw-subscription";

export class NotificationHelpers {
    public static async getNotifications(maxId: string | undefined, sinceId: string | undefined, minId: string | undefined, limit: number = 40, types: string[] | undefined, excludeTypes: string[] | undefined, accountId: string | undefined, ctx: MastoContext): Promise<Notification[]> {
        if (limit > 80) limit = 80;

        const user = ctx.user as ILocalUser;
        let requestedTypes = types
            ? this.decodeTypes(types)
            : ['follow', 'mention', 'reply', 'renote', 'quote', 'reaction', 'pollEnded', 'receiveFollowRequest'];

        if (excludeTypes) {
            const excludedTypes = this.decodeTypes(excludeTypes);
            requestedTypes = requestedTypes.filter(p => !excludedTypes.includes(p));
        }

        const query = PaginationHelpers.makePaginationQuery(
            Notifications.createQueryBuilder("notification"),
            sinceId,
            maxId,
            minId
        )
            .andWhere("notification.notifieeId = :userId", { userId: user.id })
            .andWhere("notification.type IN (:...types)", { types: requestedTypes });

        if (accountId !== undefined)
            query.andWhere("notification.notifierId = :notifierId", { notifierId: accountId });

        query
			.leftJoinAndSelect("notification.note", "note")
			.leftJoinAndSelect("notification.notifier", "notifier")
			.leftJoinAndSelect("notification.notifiee", "notifiee");

        return PaginationHelpers.execQueryLinkPagination(query, limit, minId !== undefined, ctx);
    }

    public static async getNotification(id: string, ctx: MastoContext): Promise<Notification | null> {
        const user = ctx.user as ILocalUser;
        return Notifications.findOneBy({ id: id, notifieeId: user.id });
    }

    public static async getNotificationOr404(id: string, ctx: MastoContext): Promise<Notification> {
        return this.getNotification(id, ctx).then(p => {
            if (p) return p;
            throw new MastoApiError(404);
        });
    }

    public static async dismissNotification(id: string, ctx: MastoContext): Promise<void> {
        const user = ctx.user as ILocalUser;
        await Notifications.update({ id: id, notifieeId: user.id }, { isRead: true });
    }

    public static async clearAllNotifications(ctx: MastoContext): Promise<void> {
        const user = ctx.user as ILocalUser;
        await Notifications.update({ notifieeId: user.id }, { isRead: true });
    }

    public static async markConversationAsRead(id: string, ctx: MastoContext): Promise<void> {
        const user = ctx.user as ILocalUser;
        const notesQuery = Notes.createQueryBuilder("note")
            .select("note.id")
            .andWhere("COALESCE(note.threadId, note.id) = :conversationId");

        await Notifications.createQueryBuilder("notification")
            .where(`notification."noteId" IN (${notesQuery.getQuery()})`)
            .andWhere(`notification."notifieeId" = :userId`)
            .andWhere(`notification."isRead" = FALSE`)
            .andWhere("notification.type IN (:...types)")
            .setParameter("userId", user.id)
            .setParameter("conversationId", id)
            .setParameter("types", ['reply', 'mention'])
            .update()
            .set({ isRead: true })
            .execute();
    }

    public static async getPushSubscription(ctx: MastoContext): Promise<SwSubscription | null> {
        const user = ctx.user as ILocalUser;
        const tokenId = ctx.tokenId as string;
        const subscription = await SwSubscriptions.findOneBy({ userId: user.id, appAccessTokenId: tokenId });
        return subscription;
    }

    public static async getPushSubscriptionOr404(ctx: MastoContext): Promise<SwSubscription> {
        const subscription = await this.getPushSubscription(ctx);
        if (subscription) return subscription;
        throw new MastoApiError(404);
    }

    public static async setPushSubscription(ctx: MastoContext): Promise<SwSubscription> {
        const user = ctx.user as ILocalUser;
        const tokenId = ctx.tokenId as string;
        const body = ctx.request.body as any;
        const subscription = body.subscription as { endpoint: string; keys: { p256dh: string; auth: string } };

        const existing = await SwSubscriptions.findOneBy({ userId: user.id, appAccessTokenId: tokenId });
        if (existing) {
            await SwSubscriptions.update(
                { userId: user.id, appAccessTokenId: tokenId },
                { 
                    endpoint: subscription.endpoint,
                    publickey: subscription.keys.p256dh,
                    auth: subscription.keys.auth,
                    sendReadMessage: false,
                    appAccessTokenId: tokenId,
                }
            );
        } else {
            await SwSubscriptions.insert({
                id: tokenId,
                userId: user.id,
                createdAt: new Date(),
                endpoint: subscription.endpoint,
                publickey: subscription.keys.p256dh,
                auth: subscription.keys.auth,
                sendReadMessage: false,
                appAccessTokenId: tokenId,
            });
        }
        return SwSubscriptions.findOneByOrFail({ userId: user.id, appAccessTokenId: tokenId });
    }

    public static async deletePushSubscription(ctx: MastoContext): Promise<void> {
        const user = ctx.user as ILocalUser;
        const tokenId = ctx.tokenId as string;
        await SwSubscriptions.delete({ userId: user.id, appAccessTokenId: tokenId });
    }

    private static decodeTypes(types: string[]) {
        const result: string[] = [];
        if (types.includes('follow')) result.push('follow');
        if (types.includes('mention')) result.push('mention', 'reply');
        if (types.includes('reblog')) result.push('renote', 'quote');
        if (types.includes('favourite')) result.push('reaction');
        if (types.includes('poll')) result.push('pollEnded');
        if (types.includes('follow_request')) result.push('receiveFollowRequest');
        return result;
    }
}
