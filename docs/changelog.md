# Changelog

Critical security updates are indicated by the :warning: icon.

## v20240301

- Add a page (`/my/follow-requests/sent`) to check your follow requests that haven't been approved
- Add ability to hide replies from certain users in timelines
- Admins are now allowed to migrate their account
	- This was requested by personal server admins
- Change default client settings (you can restore the previous settings)
	- Use system's font
		- This is for accessibility reasons (related discussion: <https://github.com/misskey-dev/misskey/issues/10192>)
	- Disable vibrations
	- Don't show gaps between posts in timelines
	- Show the instance ticker on local posts
- Change default user settings (existing users are not affected)
	- Reject crawler indexing
	- Set reaction history to public
- Change default server settings (existing servers are not affected)
	- Disable new user registration

## v20240229

- Add ability to pull-down-to-refresh timelines in PWA
- Make passkey/security key independent of TOTP (!10670)
- Fix bugs

## v20240228

- Update "About Firefish" page (!10673)
- Fix bugs (!10675 !10676 !10678 !10679)
- Remove charts generation to improve performance (#10611)

## v20240225

- Fix bugs
- Add syntax highlighting in MFM code blocks in various programming languages

## v20240222

- Enhance Mastodon post import feature (!10652)
- Minor style change in the web client
- Refactoring

## v20240221-1

- Fix a bug

## v20240221

- Add the ability to give regular (non-moderator) users permission to manage custom emojis
- Fix a bug that made impossible to update user profiles under some conditions
- Add "private" (only me) post visibility
	- It's just a paraphrase of DMs without recipients
	- You can also convert your existing public posts to private posts

## :warning: v20240217-1

- Fix a [security issue](https://github.com/misskey-dev/misskey/security/advisories/GHSA-qqrm-9grj-6v32)

## v20240217

- Add ability to specify the search engine used in the search bar MFM
- Remove auto NSFW media detection
- The "Hide NSFW media" config is now per device and per account
- Increase the max number of pinned posts from 5 to 15
- Change the second tab on the notifications page from "unread" to "reactions"
- Add ability to show a huge post button on the posting form
	- This is a joke feature inspired by https://mstdn.poyo.me/@prime/110668364208741253
- Bug fix
- Add `/api/emojis` endpoint (compatible with Misskey v13) for better experiences with Misskey clients
	- This does not mean we will continue to maintain API compatibility with Misskey. Instead, we plan to improve the compatibility with the Mastodon API.

## v20240216

- Style changes in the web client (a770ef4314e21f17fdce1f19feb3758953b04486 ab39ff5954a392cc6688a02f1723e1702df5e35c 4eefd534d8150e2cd5cf31dddd327edceb5b84dc)
- Clicking the "like" button now sends the actual emoji reaction (star, good, heart, etc.) instead of an empty "like"

## v20240215

- Separate settings for displaying rounded avatars for cat and non-cat accounts
- Add a toggleable setting to replace the chat button with account menu on mobile
- Reduce the size of the container image (!10667)

## v20240214

- Fix container images

## v20240213

- Bug fix
- Refactoring

## v20240212

- Refactoring
- Add a toggleable setting to hide follow buttons in a misclickable position
- Add a toggleable setting to show preview in posting form by default

## v20240210

- Security update (cf5b42a160ae8a4d94bf3dcea04ce12935ca4f76)
- Refactoring

## v20240208

- Bug fix (!10654 !10665)
- Enlarge profile picture by clicking it (!10659)
- Support Pleroma chat (!10660)
- [Add documentation about downgrading](./docs/downgrade.md)

## v20240206

- Many bug fixes
- Per-post language selector (!10616)
