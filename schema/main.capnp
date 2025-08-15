@0xca2f0996f2ffcf4c;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("globed::schema::main");

using Shared = import "shared.capnp";

# Various structs

struct PlayerAccountData {
    accountId @0 :Int32;
    userId    @1 :Int32;
    username  @2 :Text;
}

# Login messages

struct LoginUTokenMessage {
    accountId @0 :Int32;
    token @1 :Text;
    icons @2 :Shared.PlayerIconData;
}

struct LoginArgonMessage {
    accountId @0 :Int32;
    token @1 :Text;
    icons @2 :Shared.PlayerIconData;
}

struct LoginPlainMessage {
    data @0 :PlayerAccountData;
    icons @1 :Shared.PlayerIconData;
}

struct LoginOkMessage {
    newToken @0 :Text;
    servers  @1 :List(Shared.GameServer);
    allRoles @2 :List(Shared.UserRole);
    userRoles @3 :List(UInt8);
    isModerator @4 :Bool = false;
}

enum LoginFailedReason {
    invalidUserToken    @0;
    invalidArgonToken   @1;
    argonNotSupported   @2;
    argonUnreachable    @3;
    argonInternalError  @4;
    internalDbError     @5;
}

struct LoginFailedMessage {
    reason @0 :LoginFailedReason = invalidUserToken;
}

struct LoginRequiredMessage {
    argonUrl @0 :Text;
}

struct BannedMessage {
    reason @0 :Text;
    expiresAt @1 :Int64;
}

# General messages

struct UpdateOwnDataMessage {
    icons @0 :Shared.PlayerIconData; # nullable
    friendList @1 :List(Int32); # nullable
}

struct RequestPlayerCountsMessage {
    levels @0 :List(UInt64);
}

struct PlayerCountsMessage {
    levelIds @0 :List(UInt64);
    counts @1 :List(UInt16);
}

# Room management messages

struct RoomSettings {
    serverId @9 :UInt8 = 0;
    playerLimit @0 :UInt16 = 0;
    fasterReset @1 :Bool = false;
    hidden @2 :Bool = false;
    privateInvites @3 :Bool = false;
    isFollower @4 :Bool = false;
    levelIntegrity @8 :Bool = false;
    teams @10 :Bool = false;
    lockedTeams @11 :Bool = false;

    collision @5 :Bool = false;
    twoPlayerMode @6 :Bool = false;
    deathlink @7 :Bool = false;
}

struct CreateRoomMessage {
    name @0 :Text;
    passcode @1 :UInt32;
    settings @2 :RoomSettings;
}

struct JoinRoomMessage {
    roomId @0 :UInt32;
    passcode @1 :UInt32;
}

struct RequestRoomListMessage {}

struct AssignTeamMessage {
    accountId @0 :Int32;
    teamId @1 :UInt16;
}

struct CreateTeamMessage {
    color @0 :UInt32;
}

struct DeleteTeamMessage {
    teamId @0 :UInt16;
}

struct UpdateTeamMessage {
    teamId @0 :UInt16;
    color  @1 :UInt32;
}

struct GetTeamMembersMessage {}

struct TeamCreationResultMessage {
    success @0 :Bool;
    teamCount @1 :UInt16;
}

struct TeamChangedMessage {
    teamId @0 :UInt16;
}

struct RoomPlayer {
    accountData @0 :PlayerAccountData;
    cube @1 :Int16;
    color1 @2 :UInt16;
    color2 @3 :UInt16;
    glowColor @4 :UInt16;
    session @5 :UInt64;
    teamId @6 :UInt16;
}

struct TeamMembersMessage {
    members @0 :List(Int32);
    teamIds @1 :List(UInt8);
}

struct RoomStateMessage {
    roomId @0 : UInt32;
    roomOwner @1 :Int32;
    roomName @2 :Text;
    players @3 :List(RoomPlayer); # optional field
    settings @4 :RoomSettings;
    teams @5 :List(UInt32);
}

struct TeamsUpdatedMessage {
    teams @0 :List(UInt32);
}

enum RoomJoinFailedReason {
    notFound @0;
    invalidPasscode @1;
    full @2;
}

struct RoomJoinFailedMessage {
    reason @0 :RoomJoinFailedReason;
}

enum RoomCreateFailedReason {
    invalidName @0;
    invalidSettings @1;
    invalidPasscode @2;
    invalidServer @3;
    serverDown @4;
    inappropriateName @5;
}

struct RoomCreateFailedMessage {
    reason @0 :RoomCreateFailedReason;
}

struct RoomListingInfo {
    roomId @0 :UInt32;
    roomName @1 :Text;
    roomOwner @2 :RoomPlayer;
    playerCount @3 :UInt32;
    hasPassword @4 :Bool;
    settings @5 :RoomSettings;
}

struct RoomBannedMessage {
    reason @0 :Text;
    expiresAt @1 :Int64;
}

struct RoomListMessage {
    rooms @0 :List(RoomListingInfo);
}

# Session management messages

struct JoinSessionMessage {
    sessionId @0 :UInt64;
}

struct LeaveSessionMessage {}

enum JoinSessionFailedReason {
    invalidRoom @0;
    invalidServer @1;
}

struct JoinFailedMessage {
    reason @0 :JoinSessionFailedReason;
}

struct WarpPlayerMessage {
    session @0 :UInt64;
}

# Misc

enum KickReason {
    custom @0;
    duplicateLogin @1;
}

struct KickedMessage {
    reason @0 :KickReason;
    message @1 :Text;
}

struct NoticeMessage {
    senderId @0 :Int32;
    senderName @1 :Text;
    message @2 :Text;
    canReply @3 :Bool = false;
}

struct WarnMessage {
    message @0 :Text;
}

# Admin messages

struct AdminLoginMessage {
    password @0 :Text;
}

struct AdminKickMessage {
    accountId @0 :Int32;
    message @1 :Text;
}

struct AdminNoticeMessage {
    targetUser @0 :Text;
    message @1 :Text;
    roomId @2 :UInt32;
    levelId @3 :Int32;
    canReply @4 :Bool = false;
    showSender @5 :Bool = false;
}

struct AdminNoticeEveryoneMessage {
    message @0 :Text;
}

struct AdminFetchUserMessage {
    query @0 :Text;
}

struct UserPunishment {
    issuedBy @0 :Int32;
    issuedAt @1 :Int64;
    reason @2 :Text;
    expiresAt @3 :Int64;
}

struct AdminFetchResponseMessage {
    accountId @0 :Int32;
    found @1 :Bool;
    whitelisted @2 :Bool;
    roles @3 :List(UInt8);
    activeBan @4 :UserPunishment;
    activeRoomBan @5 :UserPunishment;
    activeMute @6 :UserPunishment;
    punishmentCount @7 :UInt32;
}

struct FetchedMod {
    accountId @0 :Int32;
    username @1 :Text;
    cube @2 :Int16;
    color1 @3 :UInt16;
    color2 @4 :UInt16;
    glowColor @5 :UInt16;
}

struct AdminFetchModsResponseMessage {
    users @0 :List(FetchedMod);
}

struct AdminFetchLogsMessage {
    issuer @0 :Int32;
    target @1 :Int32;
    type @2 :Text;
    before @3 :Int64;
    after @4 :Int64;
    page @5 :UInt32;
}

struct AuditLog {
    id @0 :Int32;
    accountId @1 :Int32;
    targetAccountId @2 :Int32;
    type @3 :Text;
    timestamp @4 :Int64;
    expiresAt @5 :Int64;
    message @6 :Text;
}

struct AdminLogsResponseMessage {
    logs @0 :List(AuditLog);
    accounts @1 :List(PlayerAccountData);
}

struct AdminBanMessage {
    accountId @0 :Int32;
    reason @1 :Text;
    expiresAt @2 :Int64 = 0; # 0 means permanent ban
}

struct AdminUnbanMessage {
    accountId @0 :Int32;
}

struct AdminRoomBanMessage {
    accountId @0 :Int32;
    reason @1 :Text;
    expiresAt @2 :Int64 = 0; # 0 means permanent ban
}

struct AdminRoomUnbanMessage {
    accountId @0 :Int32;
}

# TODO mute

struct AdminEditRolesMessage {
    accountId @0 :Int32;
    roles @1 :List(UInt8);
}

struct AdminSetPasswordMessage {
    accountId @0 :Int32;
    newPassword @1 :Text;
}

struct AdminUpdateUserMessage {
    accountId @0 :Int32;
    username @1 :Text;
    cube @2 :Int16;
    color1 @3 :UInt16;
    color2 @4 :UInt16;
    glowColor @5 :UInt16;
}

struct AdminFetchModsMessage {}

struct AdminResultMessage {
    success @0 :Bool;
    error @1 :Text;
}

struct Message {
    union {
        # Client messages
        loginUToken   @0 :LoginUTokenMessage;
        loginArgon    @1 :LoginArgonMessage;
        loginPlain    @2 :LoginPlainMessage;

        updateOwnData @6 :UpdateOwnDataMessage;
        requestPlayerCounts @17 :RequestPlayerCountsMessage;

        createRoom    @7 :CreateRoomMessage;
        joinRoom      @8 :JoinRoomMessage;
        leaveRoom     @9 :Void; # TODO (high): check if we can change this to a struct without breaking old clients
        checkRoomState @16 :Void;
        requestRoomList @21 :RequestRoomListMessage;
        assignTeam    @42 :AssignTeamMessage;
        createTeam    @43 :CreateTeamMessage;
        deleteTeam    @44 :DeleteTeamMessage;
        updateTeam    @50 :UpdateTeamMessage;
        getTeamMembers @48 :GetTeamMembersMessage;

        joinSession   @12 :JoinSessionMessage;
        leaveSession  @13 :LeaveSessionMessage;

        adminLogin    @25 :AdminLoginMessage;
        adminKick     @26 :AdminKickMessage;
        adminNotice   @27 :AdminNoticeMessage;
        adminNoticeEveryone @28 :AdminNoticeEveryoneMessage;
        adminFetchUser @29 :AdminFetchUserMessage;
        adminFetchLogs @40 :AdminFetchLogsMessage;
        adminBan      @30 :AdminBanMessage;
        adminUnban    @31 :AdminUnbanMessage;
        adminRoomBan  @32 :AdminRoomBanMessage;
        adminRoomUnban @33 :AdminRoomUnbanMessage;
        adminEditRoles @34 :AdminEditRolesMessage;
        adminSetPassword @35 :AdminSetPasswordMessage;
        adminUpdateUser @39 :AdminUpdateUserMessage;
        adminFetchMods @52 :AdminFetchModsMessage;

        # Server messages
        loginOk       @3 :LoginOkMessage;
        loginFailed   @4 :LoginFailedMessage;
        loginRequired @5 :LoginRequiredMessage;
        banned        @23 :BannedMessage;

        playerCounts  @18 :PlayerCountsMessage;

        roomState     @11 :RoomStateMessage;
        roomJoinFailed @19 :RoomJoinFailedMessage;
        roomCreateFailed @20 :RoomCreateFailedMessage;
        roomBanned    @24 :RoomBannedMessage;
        roomList      @22 :RoomListMessage;
        teamCreationResult @46 :TeamCreationResultMessage;
        teamChanged   @47 :TeamChangedMessage;
        teamMembers   @49 :TeamMembersMessage;
        teamsUpdated  @51 :TeamsUpdatedMessage;

        joinFailed    @14 :JoinFailedMessage;
        warpPlayer    @10 :WarpPlayerMessage;

        kicked        @15 :KickedMessage;
        notice        @38 :NoticeMessage;
        warn          @45 :WarnMessage;

        adminResult   @36 :AdminResultMessage;
        adminFetchResponse @37 :AdminFetchResponseMessage;
        adminFetchModsResponse @53 :AdminFetchModsResponseMessage;
        adminLogsResponse @41 :AdminLogsResponseMessage;
    }
}
