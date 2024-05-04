# Changes to the Firefish API

Breaking changes are indicated by the :warning: icon.

## v20240504

- :warning: Removed `release` endpoint.

## v20240424

- Added `antennaLimit` field to the response of `meta` and `admin/meta`, and the request of `admin/update-meta` (optional).
- Added `filter` optional parameter to `notes/renotes` endpoint to filter the types of renotes. It can take the following values:
	- `all` (default)
  - `renote`
  - `quote`
- :warning: Removed the following optional parameters in `notes/reactions`, as they were never taken into account due to a bug:
	- `sinceId`
	- `untilId`

## v20240413

- :warning: Removed `patrons` endpoint.

## v20240405

- Added `notes/history` endpoint.

## v20240319

- :warning: `followingCount` and `followersCount` in `users/show` will be `null` (instead of 0) if these values are unavailable.
- :warning: `admin/search/index-all` is removed since posts are now indexed automatically.
- New optional parameters are added to `notes/search` endpoint:
	- `sinceDate`
	- `untilDate`
	- `withFiles`
	- `searchCwAndAlt`
- Added `enableGuestTimeline` field to the response of `meta` and `admin/meta`, and the request of `admin/update-meta` (optional).

## v20240301

- With the addition of new features, the following endpoints are added:
	- check your follow requests that haven't been approved
		- `following/requests/sent`
	- per-user reply mutes
		- `reply-mute/create`
		- `reply-mute/delete`
		- `reply-mute/list`
- :warning: The following (horrible) endpoints are removed:
	- `admin/vacuum`
	- `reset-db`

## v20240228

- :warning: The following endpoints are removed:
	- `charts/ap-request`
	- `charts/drive`
	- `charts/federation`
	- `charts/hashtag`
	- `charts/instance`
	- `charts/notes`
	- `charts/user/drive`
	- `charts/user/following`
	- `charts/user/notes`
	- `charts/user/reactions`
	- `charts/users`

## v20240221

- Added `admin/set-emoji-moderator` endpoint, where moderators can give these permissions to regular users:
	- `add`: Add new custom emojis, set tag/category/license to newly added custom emojis
	- `mod`: `add` permission + edit the name/category/tag/license of the existing custom emojis
	- `full`: `mod` permission + delete existing custom emojis
- Emoji moderators are able to access to the endpoints under `admin/emoji/`
- Removed `lang` from the response of `i` and the request parameter of `i/update`.
- Added `notes/make-private` endpoint.

## v20240217

- :warning: Since the auto NSFW media detection has been removed, these endpoints are affected:
  - `admin/meta`
    - These parameter(s) are removed from the response field:
      - `sensitiveMediaDetection`
      - `sensitiveMediaDetectionSensitivity`
      - `setSensitiveFlagAutomatically`
      - `enableSensitiveMediaDetectionForVideos`
  - `admin/update-meta`
    - These parameter(s) are removed from the request field:
      - `sensitiveMediaDetection`
      - `sensitiveMediaDetectionSensitivity`
      - `setSensitiveFlagAutomatically`
      - `enableSensitiveMediaDetectionForVideos`
  - `admin/show-user`
    - These parameter(s) are removed from the response field:
      - `autoSensitive`
  - `i/update`
    - These parameter(s) are removed from the request field:
      - `autoSensitive`
- `/api/emojis` endpoint has been added.

## v20240212

- :warning: The field name of the response of `latest-version` has been changed from `tag_name` to `latest_version`.

## v1.0.5-rc

- `admin/update-meta` can now take `moreUrls` parameter, and response of `admin/meta` now includes `moreUrls`
  - These URLs are used for the help menu ([related merge request](https://firefish.dev/firefish/firefish/-/merge_requests/10640))
- :warning: response of `meta` no longer includes the following:
  - `enableTwitterIntegration`
  - `enableGithubIntegration`
  - `enableDiscordIntegration`
- :warning: parameter of `admin/update-meta` and response of `admin/meta` no longer include the following:
  - `enableTwitterIntegration`
  - `enableGithubIntegration`
  - `enableDiscordIntegration`
  - `twitterConsumerKey`
  - `twitterConsumerSecret`
  - `githubClientId`
  - `githubClientSecret`
  - `discordClientId`
  - `discordClientSecret`
- :warning: response of `admin/show-user` no longer includes `integrations`.
- Added `lang` parameter to `notes/create` and `notes/edit`.
- :warning: `notes/translate` now requires credentials.
