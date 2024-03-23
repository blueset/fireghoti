import OAuth from "@/server/api/mastodon/entities/oauth/oauth.js";
import { secureRndstr } from "@/misc/secure-rndstr.js";
import { Apps, AccessTokens } from "@/models/index.js";
import { genId } from "@/misc/gen-id.js";
import { fetchMeta } from "@/misc/fetch-meta.js";
import { MastoContext } from "@/server/api/mastodon/index.js";
import { MastoApiError } from "@/server/api/mastodon/middleware/catch-errors.js";
import { difference, toSingleLast, unique } from "@/prelude/array.js";
import { ILocalUser } from "@/models/entities/user.js";
import { splitCamelCaseIntoWords } from "@redocly/openapi-core/lib/utils";

export class AuthHelpers {
    public static async registerApp(ctx: MastoContext): Promise<OAuth.Application> {
        const body: any = ctx.request.body || ctx.request.query;
        const scopes = (typeof body.scopes === "string" ? body.scopes.split(' ') : body.scopes) ?? ['read'];
        const redirect_uris = body.redirect_uris?.split('\n') as string[] | undefined;
        const client_name = body.client_name;
        const website = body.website;

        if (client_name == null) throw new MastoApiError(400, 'Missing client_name param');
        if (redirect_uris == null || redirect_uris.length < 1) throw new MastoApiError(400, 'Missing redirect_uris param');

        try {
            redirect_uris.every(u => this.validateRedirectUri(u));
        } catch {
            throw new MastoApiError(400, 'Invalid redirect_uris');
        }

        const id = genId();

        const app = await Apps.insert({
            id,
            secret: secureRndstr(32),
            createdAt: new Date(),
            name: client_name,
            description: website,
            permission: scopes,
            callbackUrl: redirect_uris?.join('\n'),
        }).then((x) => Apps.findOneByOrFail(x.identifiers[0]));

        return {
            id: app.id,
            name: app.name,
            website: app.description,
            redirect_uri: app.callbackUrl ?? "",
            client_id: app.id,
            client_secret: app.secret,
            vapid_key: await fetchMeta().then(meta => meta.swPublicKey) ?? undefined,
        };
    }

    public static async getAuthCode(ctx: MastoContext) {
        const user = ctx.miauth[0] as ILocalUser;
        if (!user) throw new MastoApiError(401, "Unauthorized");

        const body = ctx.request.body as any;
        const scopes: string[] = (typeof body.scopes === "string" ? body.scopes.split(' ') : body.scopes) ?? ['read'];
        const clientId = toSingleLast(body.client_id);

        if (clientId == null) throw new MastoApiError(400, "Invalid client_id (1)");

        const app = await Apps.findOneBy({ id: clientId });

        this.validateRedirectUri(body.redirect_uri);
        if (!app) throw new MastoApiError(400, "Invalid client_id (2)");
        if (!scopes.every(p => app.permission.includes(p))) throw new MastoApiError(400, "Cannot request more scopes than application");
        if (!app.callbackUrl?.startsWith(body.redirect_uri)) throw new MastoApiError(400, "Redirect URI not in list");
        const secret = secureRndstr(32);
        const token = await AccessTokens.insert({
            id: genId(),
            token: secret,
            hash: secret,
            appId: app.id,
            userId: user.id,
            permission: scopes,
            createdAt: new Date(),
            fetched: false,
        }).then((x) => AccessTokens.findOneByOrFail(x.identifiers[0]));

        return { code: token.token };
    }

    public static async getAppInfo(ctx: MastoContext) {
        const body = ctx.request.body as any;
        const clientId = toSingleLast(body.client_id);
        console.log("body", body);

        if (clientId == null) throw new MastoApiError(400, "Invalid client_id (3)");

        const app = await Apps.findOneBy({ id: clientId });

        if (!app) throw new MastoApiError(400, "Invalid client_id (4)");

        return { name: app.name };
    }

    public static async getAuthToken(ctx: MastoContext) {
        const body: any = ctx.request.body || ctx.request.query;
        const scopes: string[] = (typeof body.scope === "string" ? body.scope.split(' ') : body.scope) ?? ['read'];
        const clientId = toSingleLast(body.client_id);
        const code = toSingleLast(body.code);

        const invalidScopeError = new MastoApiError(400, "invalid_scope", "The requested scope is invalid, unknown, or malformed.");
        const invalidClientError = new MastoApiError(401, "invalid_client", "Client authentication failed due to unknown client, no client authentication included, or unsupported authentication method.");

        if (clientId == null) throw invalidClientError;
        if (code == null) throw new MastoApiError(401, "Invalid code");

        const app = await Apps.findOneBy({ id: clientId });
        const token = await AccessTokens.findOneBy({ token: code });

        this.validateRedirectUri(body.redirect_uri);
        if (body.grant_type !== 'authorization_code') throw new MastoApiError(400, "Invalid grant_type");
        if (!app || body.client_secret !== app.secret) throw invalidClientError;
        if (!token || app.id !== token.appId) throw new MastoApiError(401, "Invalid code");
        if (difference(scopes, app.permission).length > 0) throw invalidScopeError;
        if (!app.callbackUrl?.split('\n').includes(body.redirect_uri)) throw new MastoApiError(400, "Redirect URI not in list");

        await AccessTokens.update(token.id, { fetched: true });

        return {
            "access_token": token.token,
            "token_type": "Bearer",
            "scope": app.permission.join(' '),
            "created_at": Math.floor(token.createdAt.getTime() / 1000)
        };
    }

    public static async revokeAuthToken(ctx: MastoContext) {
        const error = new MastoApiError(403, "unauthorized_client", "You are not authorized to revoke this token");
        const body: any = ctx.request.body || ctx.request.query;
        const clientId = toSingleLast(body.client_id);
        const clientSecret = toSingleLast(body.client_secret);
        const token = toSingleLast(body.token);

        if (clientId == null || clientSecret == null || token == null) throw error;

        const app = await Apps.findOneBy({ id: clientId, secret: clientSecret });
        const oatoken = await AccessTokens.findOneBy({ token: token });

        if (!app || !oatoken || app.id !== oatoken.appId) throw error;

        await AccessTokens.delete(oatoken.id);

        return {};
    }

    public static async verifyAppCredentials(ctx: MastoContext) {
        console.log(ctx.appId);
        if (!ctx.appId) throw new MastoApiError(401, "The access token is invalid");
        const app = await Apps.findOneByOrFail({ id: ctx.appId });
        return {
            name: app.name,
            website: app.description,
            vapid_key: await fetchMeta().then(meta => meta.swPublicKey ?? undefined),
        }
    }

    private static validateRedirectUri(redirectUri: string): void {
        const error = new MastoApiError(400, "Invalid redirect_uri");
        if (redirectUri == null) throw error;
        if (redirectUri === 'urn:ietf:wg:oauth:2.0:oob') return;
        try {
            const url = new URL(redirectUri);
            if (["javascript:", "file:", "data:", "mailto:", "tel:"].includes(url.protocol)) throw error;
        } catch {
            throw error;
        }
    }

    private static readScopes = [
        "read:accounts",
        "read:blocks",
        "read:bookmarks",
        "read:favourites",
        "read:filters",
        "read:follows",
        "read:lists",
        "read:mutes",
        "read:notifications",
        "read:search",
        "read:statuses",
    ];
    private static writeScopes = [
        "write:accounts",
        "write:blocks",
        "write:bookmarks",
        "write:conversations",
        "write:favourites",
        "write:filters",
        "write:follows",
        "write:lists",
        "write:media",
        "write:mutes",
        "write:notifications",
        "write:reports",
        "write:statuses",
    ];
    private static followScopes = [
        "read:follows",
        "read:blocks",
        "read:mutes",
        "write:follows",
        "write:blocks",
        "write:mutes",
    ];

    public static expandScopes(scopes: string[]): string[] {
        const res: string[] = [];

        for (const scope of scopes) {
            if (scope === "read")
                res.push(...this.readScopes);
            else if (scope === "write")
                res.push(...this.writeScopes);
            else if (scope === "follow")
                res.push(...this.followScopes);

			res.push(scope);
        }

        return unique(res);
    }
}
