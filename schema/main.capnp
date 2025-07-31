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
    # TODO: roles or something
}

enum LoginFailedReason {
    invalidUserToken    @0;
    invalidArgonToken   @1;
    argonNotSupported   @2;
    argonUnreachable    @3;
    argonInternalError  @4;
}

struct LoginFailedMessage {
    reason @0 :LoginFailedReason = invalidUserToken;
}

struct LoginRequiredMessage {
    argonUrl @0 :Text;
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

struct RoomPlayer {
    accountData @0 :PlayerAccountData;
    cube @1 :Int16;
    color1 @2 :UInt16;
    color2 @3 :UInt16;
    glowColor @5 :UInt16;
    session @4 :UInt64;
}

struct RoomStateMessage {
    roomId @0 : UInt32;
    roomOwner @1 :Int32;
    roomName @2 :Text;
    players @3 :List(RoomPlayer); # optional field
    settings @4 :RoomSettings;
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

        joinSession   @12 :JoinSessionMessage;
        leaveSession  @13 :LeaveSessionMessage;

        # Server messages
        loginOk       @3 :LoginOkMessage;
        loginFailed   @4 :LoginFailedMessage;
        loginRequired @5 :LoginRequiredMessage;

        playerCounts  @18 :PlayerCountsMessage;

        roomState     @11 :RoomStateMessage;
        roomJoinFailed @19 :RoomJoinFailedMessage;
        roomCreateFailed @20 :RoomCreateFailedMessage;
        roomList      @22 :RoomListMessage;

        joinFailed    @14 :JoinFailedMessage;
        warpPlayer    @10 :WarpPlayerMessage;

        kicked        @15 :KickedMessage;
    }
}
