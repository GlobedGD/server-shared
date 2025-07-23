@0xca2f0996f2ffcf4c;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("globed::schema::main");

using Shared = import "shared.capnp";

# Various structs

struct PlayerIconData {
    cube        @0 :Int16;
    ship        @1 :Int16;
    ball        @2 :Int16;
    ufo         @3 :Int16;
    wave        @4 :Int16;
    robot       @5 :Int16;
    spider      @6 :Int16;
    swing       @7 :Int16;
    jetpack     @8 :Int16;
    color1      @9 :UInt16;
    color2      @10 :UInt16;
    glowColor   @11 :UInt16;
    deathEffect @12 :UInt8 = 1;     # 255 means none/default
    trail       @13 :UInt8 = 255;
    shipTrail   @14 :UInt8 = 255;
}

struct PlayerAccountData {
    accountId @0 :Int32;
    userId    @1 :Int32;
    username  @2 :Text;
}

struct LevelSession {
    sessionId @0 :UInt64;
}

# Login messages

struct LoginUTokenMessage {
    accountId @0 :Int32;
    token @1 :Text;
}

struct LoginArgonMessage {
    accountId @0 :Int32;
    token @1 :Text;
}

struct LoginPlainMessage {
    data @0 :PlayerAccountData;
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
    icons @0 :PlayerIconData;
}

# Room management messages

struct CreateRoomMessage {
    name @0 :Text;
}

struct JoinRoomMessage {
    roomId @0 :UInt32;
}

struct LeaveRoomMessage {
    # empty
}

struct RoomPlayer {
    accountData @0 :PlayerAccountData;
    cube @1 :Int16;
    level @2 :LevelSession;
}

struct RoomStateMessage {
    roomId @0 : UInt32;
    name @1 :Text;
    players @2 :List(RoomPlayer);
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
    session @0 :LevelSession;
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

        createRoom    @7 :CreateRoomMessage;
        joinRoom      @8 :JoinRoomMessage;
        leaveRoom     @9 :LeaveRoomMessage;

        joinSession   @12 :JoinSessionMessage;
        leaveSession  @13 :LeaveSessionMessage;

        # Server messages
        loginOk       @3 :LoginOkMessage;
        loginFailed   @4 :LoginFailedMessage;
        loginRequired @5 :LoginRequiredMessage;

        roomState     @11 :RoomStateMessage;

        joinFailed    @14 :JoinFailedMessage;
        warpPlayer    @10 :WarpPlayerMessage;

        kicked        @15 :KickedMessage;
    }
}
