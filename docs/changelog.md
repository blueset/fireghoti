# Changelog

Critical security updates are indicated by the :warning: icon.

- Server administrators should check [notice-for-admins.md](./notice-for-admins.md) as well.
- Third-party client/bot developers may want to check [api-change.md](./api-change.md) as well.

## Unreleased

- Add features to share links to an account in the three dots menu on the profile page
- Improve server logs
- Fix bugs

## [v20240424](https://firefish.dev/firefish/firefish/-/merge_requests/10765/commits)

- Improve the usability of the feature to prevent forgetting to write alt texts
- Add a server-wide setting for the maximum number of antennas each user can create
- Fix bugs

## [v20240421](https://firefish.dev/firefish/firefish/-/merge_requests/10756/commits)

- Fix bugs

## [v20240413](https://firefish.dev/firefish/firefish/-/merge_requests/10741/commits)

- Add "Media" tab to user page
- Improve federation and rendering of mathematical expressions
- Remove donor information from the web client
	- See also: https://info.firefish.dev/notes/9s1n283sb10rh869
- Fix bugs

## [v20240405](https://firefish.dev/firefish/firefish/-/merge_requests/10733/commits)

- Add ability to view the history of post edits (!10714)
- Fix bugs

## [v20240401](https://firefish.dev/firefish/firefish/-/merge_requests/10724/commits)

- Fix bugs

## :warning: [v20240330](https://firefish.dev/firefish/firefish/-/merge_requests/10719/commits)

- Fix bugs (including a critical security issue)
	- We are very thankful to Oneric (the reporter of the security issue) and Laura Hausmann (Iceshrimp maintainer) for kindly and securely sharing the information to fix the issue.

## [v20240326](https://firefish.dev/firefish/firefish/-/merge_requests/10713/commits)

- Fix bugs
- Add an icon in the posting form to indicate that attached files have alt text
- Add a toggleable setting to show a warning if the post language setting might be incorrect

## [v20240319](https://firefish.dev/firefish/firefish/-/compare/v20240301...v20240319?from_project_id=7&straight=false)

- Introduce new full-text search engine and post search filters
- Refactoring
- Show unlisted posts from following users in antennas (similar to [Fedibird](https://github.com/fedibird/mastodon/tree/fedibird) and [kmyblue](https://github.com/kmycode/mastodon), unlisted posts from people you don't follow won't be shown)
- Add ability to publish the Local and Global timelines on `/timeline` page
- Add langage annotation to post contents (!10687)
- Add a toggleable setting to show a warning when you attempt to post files without alt text
- Fix bugs
- Update documents and example config files
- Added `/authorize_interaction` page, allowing users to jump from a remote Mastodon post/user page to the corresponding page in Firefish (!10702)

## [v20240301](https://firefish.dev/firefish/firefish/-/compare/v20240229...v20240301?from_project_id=7&straight=false)

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
- Fix bugs

## [v20240229](https://firefish.dev/firefish/firefish/-/compare/v20240228...v20240229?from_project_id=7&straight=false)

- Add ability to pull-down-to-refresh timelines in PWA
- Make passkey/security key independent of TOTP (!10670)
- Fix bugs

## [v20240228](https://firefish.dev/firefish/firefish/-/compare/v20240225...v20240228?from_project_id=7&straight=false)

- Update "About Firefish" page (!10673)
- Fix bugs (!10675 !10676 !10678 !10679)
- Remove charts generation to improve performance (#10611)

## [v20240225](https://firefish.dev/firefish/firefish/-/compare/v20240222...v20240225?from_project_id=7&straight=false)

- Fix bugs
- Add syntax highlighting in MFM code blocks in various programming languages

## [v20240222](https://firefish.dev/firefish/firefish/-/compare/v20240221-1...v20240222?from_project_id=7&straight=false)

- Enhance Mastodon post import feature (!10652)
- Minor style change in the web client
- Refactoring

## [v20240221-1](https://firefish.dev/firefish/firefish/-/compare/v20240221...v20240221-1?from_project_id=7&straight=false)

- Fix a bug

## [v20240221](https://firefish.dev/firefish/firefish/-/compare/v20240217-1...v20240221?from_project_id=7&straight=false)

- Add the ability to give regular (non-moderator) users permission to manage custom emojis
- Fix a bug that made impossible to update user profiles under some conditions
- Add "private" (only me) post visibility
	- It's just a paraphrase of DMs without recipients
	- You can also convert your existing public posts to private posts

## :warning: [v20240217-1](https://firefish.dev/firefish/firefish/-/compare/v20240217...v20240217-1?from_project_id=7&straight=false)

- Fix a [security issue](https://github.com/misskey-dev/misskey/security/advisories/GHSA-qqrm-9grj-6v32)

## [v20240217](https://firefish.dev/firefish/firefish/-/compare/v20240216...v20240217?from_project_id=7&straight=false)

- Add ability to specify the search engine used in the search bar MFM
- Remove auto NSFW media detection
- The "Hide NSFW media" config is now per device and per account
- Increase the max number of pinned posts from 5 to 15
- Change the second tab on the notifications page from "unread" to "reactions"
- Add ability to show a huge post button on the posting form
	- This is a joke feature inspired by https://mstdn.poyo.me/@prime/110668364208741253
- Fix bugs
- Add `/api/emojis` endpoint (compatible with Misskey v13) for better experiences with Misskey clients
	- This does not mean we will continue to maintain API compatibility with Misskey. Instead, we plan to improve the compatibility with the Mastodon API.

## [v20240216](https://firefish.dev/firefish/firefish/-/compare/v20240215...v20240216?from_project_id=7&straight=false)

- Style changes in the web client (a770ef4314e21f17fdce1f19feb3758953b04486 ab39ff5954a392cc6688a02f1723e1702df5e35c 4eefd534d8150e2cd5cf31dddd327edceb5b84dc)
- Clicking the "like" button now sends the actual emoji reaction (star, good, heart, etc.) instead of an empty "like"

## [v20240215](https://firefish.dev/firefish/firefish/-/compare/v20240214...v20240215?from_project_id=7&straight=false)

- Separate settings for displaying rounded avatars for cat and non-cat accounts
- Add a toggleable setting to replace the chat button with account menu on mobile
- Reduce the size of the container image (!10667)

## [v20240214](https://firefish.dev/firefish/firefish/-/compare/v20240213...v20240214?from_project_id=7&straight=false)

- Fix container images

## [v20240213](https://firefish.dev/firefish/firefish/-/compare/v20240212...v20240213?from_project_id=7&straight=false)

- Fix bugs
- Refactoring

## [v20240212](https://firefish.dev/firefish/firefish/-/compare/v20240210...v20240212?from_project_id=7&straight=false)

- Refactoring
- Add a toggleable setting to hide follow buttons in a misclickable position
- Add a toggleable setting to show preview in posting form by default

## [v20240210](https://firefish.dev/firefish/firefish/-/compare/v20240208...v20240210?from_project_id=7&straight=false)

- Security update (cf5b42a160ae8a4d94bf3dcea04ce12935ca4f76)
- Refactoring

## [v20240208](https://firefish.dev/firefish/firefish/-/compare/v20240206...v20240208?from_project_id=7&straight=false)

- Fix bugs (!10654 !10665)
- Enlarge profile picture by clicking it (!10659)
- Support Pleroma chat (!10660)
- [Add documentation about downgrading](./docs/downgrade.md)

## [v20240206](https://firefish.dev/firefish/firefish/-/compare/v1.0.5-rc...v20240206?from_project_id=7&straight=false)

- Fix many bugs
- Per-post language selector (!10616)
