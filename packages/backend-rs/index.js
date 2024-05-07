/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'backend-rs.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./backend-rs.android-arm64.node')
          } else {
            nativeBinding = require('backend-rs-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'backend-rs.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./backend-rs.android-arm-eabi.node')
          } else {
            nativeBinding = require('backend-rs-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'backend-rs.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./backend-rs.win32-x64-msvc.node')
          } else {
            nativeBinding = require('backend-rs-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'backend-rs.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./backend-rs.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('backend-rs-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'backend-rs.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./backend-rs.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('backend-rs-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'backend-rs.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./backend-rs.darwin-universal.node')
      } else {
        nativeBinding = require('backend-rs-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'backend-rs.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./backend-rs.darwin-x64.node')
          } else {
            nativeBinding = require('backend-rs-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'backend-rs.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./backend-rs.darwin-arm64.node')
          } else {
            nativeBinding = require('backend-rs-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'backend-rs.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./backend-rs.freebsd-x64.node')
      } else {
        nativeBinding = require('backend-rs-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'backend-rs.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./backend-rs.linux-x64-musl.node')
            } else {
              nativeBinding = require('backend-rs-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'backend-rs.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./backend-rs.linux-x64-gnu.node')
            } else {
              nativeBinding = require('backend-rs-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'backend-rs.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./backend-rs.linux-arm64-musl.node')
            } else {
              nativeBinding = require('backend-rs-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'backend-rs.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./backend-rs.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('backend-rs-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'backend-rs.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./backend-rs.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('backend-rs-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'backend-rs.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./backend-rs.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('backend-rs-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'backend-rs.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./backend-rs.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('backend-rs-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'backend-rs.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./backend-rs.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('backend-rs-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'backend-rs.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./backend-rs.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('backend-rs-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { SECOND, MINUTE, HOUR, DAY, USER_ONLINE_THRESHOLD, USER_ACTIVE_THRESHOLD, FILE_TYPE_BROWSERSAFE, loadEnv, loadConfig, stringToAcct, acctToString, addNoteToAntenna, isBlockedServer, isSilencedServer, isAllowedServer, checkWordMute, getFullApAccount, isSelfHost, isSameOrigin, extractHost, toPuny, isUnicodeEmoji, sqlLikeEscape, safeForSql, formatMilliseconds, getImageSizeFromUrl, getNoteSummary, isSafeUrl, latestVersion, toMastodonId, fromMastodonId, fetchMeta, metaToPugArgs, nyaify, hashPassword, verifyPassword, isOldPasswordAlgorithm, decodeReaction, countReactions, toDbReaction, removeOldAttestationChallenges, AntennaSrcEnum, DriveFileUsageHintEnum, MutedNoteReasonEnum, NoteVisibilityEnum, NotificationTypeEnum, PageVisibilityEnum, PollNotevisibilityEnum, RelayStatusEnum, UserEmojimodpermEnum, UserProfileFfvisibilityEnum, UserProfileMutingnotificationtypesEnum, initializeRustLogger, fetchNodeinfo, nodeinfo_2_1, nodeinfo_2_0, Protocol, Inbound, Outbound, watchNote, unwatchNote, publishToChannelStream, ChatEvent, publishToChatStream, ChatIndexEvent, publishToChatIndexStream, publishToBroadcastStream, publishToGroupChatStream, publishToModerationStream, getTimestamp, genId, genIdAt, secureRndstr } = nativeBinding

module.exports.SECOND = SECOND
module.exports.MINUTE = MINUTE
module.exports.HOUR = HOUR
module.exports.DAY = DAY
module.exports.USER_ONLINE_THRESHOLD = USER_ONLINE_THRESHOLD
module.exports.USER_ACTIVE_THRESHOLD = USER_ACTIVE_THRESHOLD
module.exports.FILE_TYPE_BROWSERSAFE = FILE_TYPE_BROWSERSAFE
module.exports.loadEnv = loadEnv
module.exports.loadConfig = loadConfig
module.exports.stringToAcct = stringToAcct
module.exports.acctToString = acctToString
module.exports.addNoteToAntenna = addNoteToAntenna
module.exports.isBlockedServer = isBlockedServer
module.exports.isSilencedServer = isSilencedServer
module.exports.isAllowedServer = isAllowedServer
module.exports.checkWordMute = checkWordMute
module.exports.getFullApAccount = getFullApAccount
module.exports.isSelfHost = isSelfHost
module.exports.isSameOrigin = isSameOrigin
module.exports.extractHost = extractHost
module.exports.toPuny = toPuny
module.exports.isUnicodeEmoji = isUnicodeEmoji
module.exports.sqlLikeEscape = sqlLikeEscape
module.exports.safeForSql = safeForSql
module.exports.formatMilliseconds = formatMilliseconds
module.exports.getImageSizeFromUrl = getImageSizeFromUrl
module.exports.getNoteSummary = getNoteSummary
module.exports.isSafeUrl = isSafeUrl
module.exports.latestVersion = latestVersion
module.exports.toMastodonId = toMastodonId
module.exports.fromMastodonId = fromMastodonId
module.exports.fetchMeta = fetchMeta
module.exports.metaToPugArgs = metaToPugArgs
module.exports.nyaify = nyaify
module.exports.hashPassword = hashPassword
module.exports.verifyPassword = verifyPassword
module.exports.isOldPasswordAlgorithm = isOldPasswordAlgorithm
module.exports.decodeReaction = decodeReaction
module.exports.countReactions = countReactions
module.exports.toDbReaction = toDbReaction
module.exports.removeOldAttestationChallenges = removeOldAttestationChallenges
module.exports.AntennaSrcEnum = AntennaSrcEnum
module.exports.DriveFileUsageHintEnum = DriveFileUsageHintEnum
module.exports.MutedNoteReasonEnum = MutedNoteReasonEnum
module.exports.NoteVisibilityEnum = NoteVisibilityEnum
module.exports.NotificationTypeEnum = NotificationTypeEnum
module.exports.PageVisibilityEnum = PageVisibilityEnum
module.exports.PollNotevisibilityEnum = PollNotevisibilityEnum
module.exports.RelayStatusEnum = RelayStatusEnum
module.exports.UserEmojimodpermEnum = UserEmojimodpermEnum
module.exports.UserProfileFfvisibilityEnum = UserProfileFfvisibilityEnum
module.exports.UserProfileMutingnotificationtypesEnum = UserProfileMutingnotificationtypesEnum
module.exports.initializeRustLogger = initializeRustLogger
module.exports.fetchNodeinfo = fetchNodeinfo
module.exports.nodeinfo_2_1 = nodeinfo_2_1
module.exports.nodeinfo_2_0 = nodeinfo_2_0
module.exports.Protocol = Protocol
module.exports.Inbound = Inbound
module.exports.Outbound = Outbound
module.exports.watchNote = watchNote
module.exports.unwatchNote = unwatchNote
module.exports.publishToChannelStream = publishToChannelStream
module.exports.ChatEvent = ChatEvent
module.exports.publishToChatStream = publishToChatStream
module.exports.ChatIndexEvent = ChatIndexEvent
module.exports.publishToChatIndexStream = publishToChatIndexStream
module.exports.publishToBroadcastStream = publishToBroadcastStream
module.exports.publishToGroupChatStream = publishToGroupChatStream
module.exports.publishToModerationStream = publishToModerationStream
module.exports.getTimestamp = getTimestamp
module.exports.genId = genId
module.exports.genIdAt = genIdAt
module.exports.secureRndstr = secureRndstr
