@0x95684a6f0a0e3cd1;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("globed::schema::game");

using Shared = import "shared.capnp";

# Login messages

struct LoginUTokenMessage {
    accountId @0 :Int32;
    token @1 :Text;
    icons @2 :Shared.PlayerIconData;
}

struct LoginUTokenAndJoinMessage {
    accountId @0 :Int32;
    token @1 :Text;
    icons @2 :Shared.PlayerIconData;
    sessionId @3 :UInt64;
    passcode  @4 :UInt32;
    platformer @5 :Bool;
}

struct LoginOkMessage {
    tickrate @0 :UInt16;
}

enum LoginFailedReason {
    invalidUserToken @0;
    centralServerUnreachable @1;
}

struct LoginFailedMessage {
    reason @0 :LoginFailedReason;
}

# Session messages

struct JoinSessionMessage {
    sessionId @0 :UInt64;
    passcode  @1 :UInt32;
    platformer @2 :Bool;
}

struct JoinSessionOkMessage {
    sessionId @0 :UInt64;
}

enum JoinSessionFailedReason {
    invalidPasscode @0;
    invalidRoom @1;
}

struct JoinSessionFailedMessage {
    reason @0 :JoinSessionFailedReason;
}

struct LeaveSessionMessage {
}

# Player data messages

struct PlayerObjectData { # aka SpecificIconData in globed v1
    positionX @0 :Float32;
    positionY @1 :Float32;
    rotation  @2 :Float32;
    iconType  @3 :Shared.IconType;

    isVisible     @4 :Bool;
    isLookingLeft @5 :Bool;
    isUpsideDown  @6 :Bool;
    isDashing     @7 :Bool;
    isMini        @8 :Bool;
    isGrounded    @9 :Bool;
    isStationary  @10 :Bool;
    isFalling     @11 :Bool;
    isRotating    @12 :Bool;
    isSideways    @13 :Bool;
}

struct PlayerData {
    accountId   @0 :Int32;
    timestamp   @1 :Float32;
    frameNumber @2 :UInt8;
    deathCount  @3 :UInt8;
    percentage  @13 :UInt16;

    isDead        @4 :Bool;
    isPaused      @5 :Bool;
    isPracticing  @6 :Bool;
    isInEditor    @7 :Bool;
    isEditorBuilding @8 :Bool;
    isLastDeathReal @9 :Bool;

    # TODO: measure if there is a better, more compact way to lay this out
    union {
        dual :group {
            player1 @10 :PlayerObjectData;
            player2 @11 :PlayerObjectData;
        }

        single :group {
            player1 @12 :PlayerObjectData;
        }

        culled :group {
            nothing @14 :Void;
        }

        # TODO: more complete data for spectator
    }
}

struct Event {
    type @0 :UInt16;
    data @1 :Data;
}

struct PlayerDataMessage {
    data @0 :PlayerData;
    dataRequests @1 :List(Int32);
    eventData @2 :Data;
    cameraX @3 :Float32;
    cameraY @4 :Float32;
    cameraRadius @5 :Float32;
}

struct LevelDataMessage {
    players @0 :List(PlayerData);
    displayDatas @1 :List(Shared.PlayerDisplayData);
    eventData @2 :Data;
}

# Misc

struct UpdateIconsMessage {
    icons @0 :Shared.PlayerIconData;
}

struct LevelScript {
    content @0 :Text;
    filename @1 :Text;
    main @2 :Bool;
    signature @3 :Data;
}

struct SendLevelScriptMessage {
    scripts @0 :List(LevelScript);
}

struct VoiceDataMessage {
    frames @0 :List(Data);
}

enum KickReason {
    custom @0;
    duplicateLogin @1;
}

struct KickedMessage {
    reason @0 :KickReason;
    message @1 :Text;
}

struct ScriptLogsMessage {
    logs @0 :List(Text);
    ramUsage @1 :Float32;
}

struct VoiceBroadcastMessage {
    accountId @0 :Int32;
    frames @1 :List(Data);
}

struct Message {
    union {
        # Client messages
        loginUToken        @0 :LoginUTokenMessage;
        loginUTokenAndJoin @3 :LoginUTokenAndJoinMessage;
        joinSession        @4 :JoinSessionMessage;
        leaveSession       @5 :LeaveSessionMessage;

        playerData         @6 :PlayerDataMessage;
        updateIcons        @11 :UpdateIconsMessage;
        sendLevelScript    @12 :SendLevelScriptMessage;
        voiceData          @14 :VoiceDataMessage;

        # Server messages
        loginOk            @1 :LoginOkMessage;
        loginFailed        @2 :LoginFailedMessage;
        joinSessionOk      @7 :JoinSessionOkMessage;
        joinSessionFailed  @8 :JoinSessionFailedMessage;

        levelData          @9 :LevelDataMessage;
        kicked             @10 :KickedMessage;
        scriptLogs         @13 :ScriptLogsMessage;
        voiceBroadcast     @15 :VoiceBroadcastMessage;
    }
}
