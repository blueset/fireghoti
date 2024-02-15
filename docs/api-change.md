# Changes to the Firefish API

Breaking changes are indicated by the :warning: icon.

## Unreleased

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
