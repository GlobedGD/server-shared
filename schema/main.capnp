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

struct LoginMessage {
    accountId @0 :Int32;
    icons @1 :Shared.PlayerIconData;
    uident @2 :Data;
    settings @3 :Shared.UserSettings;

    union {
        utoken @4 :Text;
        argon  @5 :Text;
        plain  @6 :PlayerAccountData;
    }
}

struct LoginOkMessage {
    newToken @0 :Text;
    servers  @1 :List(Shared.GameServer);
    allRoles @2 :List(Shared.UserRole);
    userRoles @3 :List(UInt8);
    nameColor @4 :Data;
    isModerator @5 :Bool;
    canMute @6 :Bool;
    canBan @7 :Bool;
    canSetPassword @8 :Bool;
    canEditRoles @9 :Bool;
    canSendFeatures @10 :Bool;
    canRateFeatures @11 :Bool;

    featuredLevel @12 :Int32;
    featuredLevelTier @13 :UInt8;
    featuredLevelEdition @14 :UInt32;
}

enum LoginFailedReason {
    invalidUserToken    @0;
    invalidArgonToken   @1;
    argonNotSupported   @2;
    argonUnreachable    @3;
    argonInternalError  @4;
    internalDbError     @5;
    invalidAccountData  @6;
    notWhitelisted      @7;
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

struct MutedMessage {
    reason @0 :Text;
    expiresAt @1 :Int64;
}

struct ServersChangedMessage {
    servers @0 :List(Shared.GameServer);
}

struct UserDataChangedMessage {
    roles @0 :List(UInt8);
    nameColor @1 :Data;
    isModerator @2 :Bool;
    canMute @3 :Bool;
    canBan @4 :Bool;
    canSetPassword @5 :Bool;
    canEditRoles @6 :Bool;
    canSendFeatures @7 :Bool;
    canRateFeatures @8 :Bool;
}

# General messages

struct UpdateOwnDataMessage {
    icons @0 :Shared.PlayerIconData; # nullable
    friendList @1 :List(Int32); # nullable
}

struct RequestPlayerCountsMessage {
    levels @0 :List(UInt64);
}

struct RequestGlobalPlayerListMessage {
    nameFilter @0 :Text;
}

struct UpdateUserSettingsMessage {
    settings @0 :Shared.UserSettings;
}

# Room management messages

struct RoomSettings {
    serverId @0 :UInt8 = 0;
    playerLimit @1 :UInt16 = 0;
    fasterReset @2 :Bool = false;
    hidden @3 :Bool = false;
    privateInvites @4 :Bool = false;
    isFollower @5 :Bool = false;
    levelIntegrity @6 :Bool = false;
    teams @7 :Bool = false;
    lockedTeams @8 :Bool = false;

    collision @9 :Bool = false;
    twoPlayerMode @10 :Bool = false;
    deathlink @11 :Bool = false;
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

struct JoinRoomByTokenMessage {
    token @0 :UInt64;
}

struct RequestRoomPlayersMessage {
    nameFilter @0 :Text;
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

enum RoomOwnerActionType {
    banUser @0;
    kickUser @1;
    closeRoom @2;
}

struct RoomOwnerActionMessage {
    type @0 :RoomOwnerActionType;
    target @1 :Int32;
}

struct UpdateRoomSettingsMessage {
    settings @0 :RoomSettings;
}

struct InvitePlayerMessage {
    player @0 :Int32;
}

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
    specialData @7 :Shared.SpecialUserData;
}

struct MinimalRoomPlayer {
    accountData @0 :PlayerAccountData;
    cube @1 :Int16;
    color1 @2 :UInt16;
    color2 @3 :UInt16;
    glowColor @4 :UInt16;
}

struct TeamMembersMessage {
    members @0 :List(Int32);
    teamIds @1 :List(UInt8);
}

struct RoomStateMessage {
    roomId @0 : UInt32;
    roomOwner @1 :Int32;
    roomName @2 :Text;
    players @3 :List(RoomPlayer);
    settings @4 :RoomSettings;
    teams @5 :List(UInt32);
    passcode @6 :UInt32;
    playerCount @7 :UInt32;
}

struct RoomPlayersMessage {
    players @0 :List(RoomPlayer);
}

struct TeamsUpdatedMessage {
    teams @0 :List(UInt32);
}

struct RoomSettingsUpdatedMessage {
    settings @0 :RoomSettings;
}

enum RoomJoinFailedReason {
    notFound @0;
    invalidPasscode @1;
    full @2;
    banned @3;
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

struct InvitedMessage {
    invitedBy @0 :PlayerAccountData;
    token @1 :UInt64;
}

struct InviteTokenCreatedMessage {
    token @0 :UInt64;
}

# Misc general messages

struct JoinSessionMessage {
    sessionId @0 :UInt64;
}

struct LeaveSessionMessage {}

struct RequestLevelListMessage {}

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

struct PlayerCountsMessage {
    levelIds @0 :List(UInt64);
    counts @1 :List(UInt16);
}

struct GlobalPlayersMessage {
    players @0 :List(MinimalRoomPlayer);
}

struct LevelListMessage {
    levelIds @0 :List(UInt64);
    playerCounts @1 :List(UInt16);
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

struct FetchCreditsMessage {}

struct CreditsUser {
    accountId @0 :Int32;
    userId @1 :Int32;
    username @2 :Text;
    displayName @3 :Text;
    cube @4 :Int16;
    color1 @5 :UInt16;
    color2 @6 :UInt16;
    glowColor @7 :UInt16;
}

struct CreditsCategory {
    name @0 :Text;
    users @1 :List(CreditsUser);
}

struct CreditsMessage {
    categories @0 :List(CreditsCategory);
    unavailable @1 :Bool;
}

struct GetDiscordLinkStateMessage {}

struct SetDiscordPairingStateMessage {
    state @0 :Bool;
}

struct DiscordLinkConfirmMessage {
    id @0 :UInt64;
    accept @1 :Bool;
}

struct GetFeaturedListMessage {
    page @0 :UInt32;
}

struct SendFeaturedLevelMessage {
    levelId     @0 :Int32;
    levelName   @1 :Text;
    authorId    @2 :Int32;
    authorName  @3 :Text;
    rateTier    @4 :UInt8;
    note        @5 :Text;
    queue       @6 :Bool;
}

struct DiscordLinkStateMessage {
    id @0 :UInt64;
    username @1 :Text;
    avatarUrl @2 :Text;
}

struct DiscordLinkAttemptMessage {
    id @0 :UInt64;
    username @1 :Text;
    avatarUrl @2 :Text;
}

struct FeaturedLevelMessage {
    levelId  @0 :Int32;
    rateTier @1 :UInt8;
    edition  @2 :UInt32;
}

struct FeaturedListMessage {
    levelIds @0 :List(Int32);
    rateTiers @1 :List(UInt8);

    page @2 :UInt32;
    totalPages @3 :UInt32;
}

struct FetchUserMessage {
	accountId @0 :Int32;
}

struct FetchUserResponseMessage {
    accountId @0 :Int32;
    found @1 :Bool;
    roles @2 :List(UInt8);
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

struct AdminMuteMessage {
    accountId @0 :Int32;
    reason @1 :Text;
    expiresAt @2 :Int64 = 0; # 0 means permanent mute
}

struct AdminUnmuteMessage {
    accountId @0 :Int32;
}

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
        ### Client messages

        login                       @0 :LoginMessage;

        updateOwnData               @1 :UpdateOwnDataMessage;
        requestPlayerCounts         @2 :RequestPlayerCountsMessage;
        requestGlobalPlayerList     @3 :RequestGlobalPlayerListMessage;
        updateUserSettings          @4 :UpdateUserSettingsMessage;

        createRoom                  @5 :CreateRoomMessage;
        joinRoom                    @6 :JoinRoomMessage;
        joinRoomByToken             @7 :JoinRoomByTokenMessage;
        leaveRoom                   @8 :Void; # TODO (high): check if we can change this to a struct without breaking old clients
        checkRoomState              @9 :Void;
        requestRoomPlayers          @10 :RequestRoomPlayersMessage;
        requestRoomList             @11 :RequestRoomListMessage;
        assignTeam                  @12 :AssignTeamMessage;
        createTeam                  @13 :CreateTeamMessage;
        deleteTeam                  @14 :DeleteTeamMessage;
        updateTeam                  @15 :UpdateTeamMessage;
        getTeamMembers              @16 :GetTeamMembersMessage;
        roomOwnerAction             @17 :RoomOwnerActionMessage;
        updateRoomSettings          @18 :UpdateRoomSettingsMessage;
        invitePlayer                @19 :InvitePlayerMessage;

        joinSession                 @20 :JoinSessionMessage;
        leaveSession                @21 :LeaveSessionMessage;
        requestLevelList            @22 :RequestLevelListMessage;

        adminLogin                  @23 :AdminLoginMessage;
        adminKick                   @24 :AdminKickMessage;
        adminNotice                 @25 :AdminNoticeMessage;
        adminNoticeEveryone         @26 :AdminNoticeEveryoneMessage;
        adminFetchUser              @27 :AdminFetchUserMessage;
        adminFetchLogs              @28 :AdminFetchLogsMessage;
        adminBan                    @29 :AdminBanMessage;
        adminUnban                  @30 :AdminUnbanMessage;
        adminRoomBan                @31 :AdminRoomBanMessage;
        adminRoomUnban              @32 :AdminRoomUnbanMessage;
        adminMute                   @33 :AdminMuteMessage;
        adminUnmute                 @34 :AdminUnmuteMessage;
        adminEditRoles              @35 :AdminEditRolesMessage;
        adminSetPassword            @36 :AdminSetPasswordMessage;
        adminUpdateUser             @37 :AdminUpdateUserMessage;
        adminFetchMods              @38 :AdminFetchModsMessage;

        fetchCredits                @39 :FetchCreditsMessage;
        getDiscordLinkState         @40 :GetDiscordLinkStateMessage;
        setDiscordPairingState      @41 :SetDiscordPairingStateMessage;
        discordLinkConfirm          @42 :DiscordLinkConfirmMessage;
        getFeaturedLevel            @43 :Void;
        getFeaturedList             @44 :GetFeaturedListMessage;
        sendFeaturedLevel           @45 :SendFeaturedLevelMessage;

        fetchUser                   @46 :FetchUserMessage;

        ### Server messages



        loginOk                     @47 :LoginOkMessage;
        loginFailed                 @48 :LoginFailedMessage;
        loginRequired               @49 :LoginRequiredMessage;
        banned                      @50 :BannedMessage;
        muted                       @51 :MutedMessage;
        serversChanged              @52 :ServersChangedMessage;
        userDataChanged             @53 :UserDataChangedMessage;

        roomState                   @54 :RoomStateMessage;
        roomPlayers                 @55 :RoomPlayersMessage;
        roomJoinFailed              @56 :RoomJoinFailedMessage;
        roomCreateFailed            @57 :RoomCreateFailedMessage;
        roomBanned                  @58 :RoomBannedMessage;
        roomList                    @59 :RoomListMessage;
        teamCreationResult          @60 :TeamCreationResultMessage;
        teamChanged                 @61 :TeamChangedMessage;
        teamMembers                 @62 :TeamMembersMessage;
        teamsUpdated                @63 :TeamsUpdatedMessage;
        roomSettingsUpdated         @64 :RoomSettingsUpdatedMessage;
        invited                     @65 :InvitedMessage;
        inviteTokenCreated          @66 :InviteTokenCreatedMessage;

        joinFailed                  @67 :JoinFailedMessage;
        warpPlayer                  @68 :WarpPlayerMessage;
        playerCounts                @69 :PlayerCountsMessage;
        globalPlayers               @70 :GlobalPlayersMessage;
        levelList                   @71 :LevelListMessage;

        kicked                      @72 :KickedMessage;
        notice                      @73 :NoticeMessage;
        warn                        @74 :WarnMessage;

        adminResult                 @75 :AdminResultMessage;
        adminFetchResponse          @76 :AdminFetchResponseMessage;
        adminFetchModsResponse      @77 :AdminFetchModsResponseMessage;
        adminLogsResponse           @78 :AdminLogsResponseMessage;

        credits                     @79 :CreditsMessage;
        discordLinkState            @80 :DiscordLinkStateMessage;
        discordLinkAttempt          @81 :DiscordLinkAttemptMessage;
        featuredLevel               @82 :FeaturedLevelMessage;
        featuredList                @83 :FeaturedListMessage;

		fetchUserResponse           @84 :FetchUserResponseMessage;
    }
}
