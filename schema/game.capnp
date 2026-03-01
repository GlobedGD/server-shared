@0x95684a6f0a0e3cd1;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("globed::schema::game");

using Shared = import "shared.capnp";

# Login messages

struct LoginMessage {
    accountId @0 :Int32;
    token @1 :Text;
    icons @2 :Shared.PlayerIconData;
    settings @3 :Shared.UserSettings;

    # optional fields
    sessionId @4 :UInt64;
    passcode  @5 :UInt32;
    platformer @6 :Bool;
    editorCollab @7 :Bool;
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
    editorCollab @3 :Bool;
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

struct ExtendedPlayerData {
    velocityX @0 :Float32;
    velocityY @1 :Float32;
    accelerating @2 :Bool;
    acceleration @3 :Float32;
    fallStartY @4 :Float32;
    isOnGround2 @5 :Bool;
    gravityMod @6 :Float32;
    gravity    @7 :Float32;
    touchedPad @8 :Bool;
    maybeFalling @9 :Bool;
    fallSpeed @10 :Float32;
    isOnGround4 @11 :Bool;
}

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
    didJustJump   @15 :Bool;
    isFlipped     @16 :Bool;

    extData       @14 :ExtendedPlayerData;
}

struct PlayerData {
    accountId   @0 :Int32;
    timestamp   @1 :Float32;
    frameNumber @2 :UInt8;
    deathCount  @3 :UInt8;
    percentage  @4 :UInt16;

    isDead        @5 :Bool;
    isPaused      @6 :Bool;
    isPracticing  @7 :Bool;
    isInEditor    @8 :Bool;
    isEditorBuilding @9 :Bool;
    isLastDeathReal @10 :Bool;

    # TODO: measure if there is a better, more compact way to lay this out
    union {
        dual :group {
            player1 @11 :PlayerObjectData;
            player2 @12 :PlayerObjectData;
        }

        single :group {
            player1 @13 :PlayerObjectData;
        }

        culled :group {
            nothing @14 :Void;
        }
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
    messageId @6 :UInt16; # wraps
}

struct LevelDataMessage {
    players @0 :List(PlayerData);
    displayDatas @1 :List(Shared.PlayerDisplayData);
    eventData @2 :Data;
    messageId @3 :UInt16; # same as client provided value
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

struct QuickChatMessage {
    id @0 :UInt32;
}

struct UpdateUserSettingsMessage {
    settings @0 :Shared.UserSettings;
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

struct QuickChatBroadcastMessage {
    accountId @0 :Int32;
    id @1 :UInt32;
}

struct ChatNotPermittedMessage {

}

struct Message {
    union {
        # Client messages
        login              @0 :LoginMessage;
        joinSession        @1 :JoinSessionMessage;
        leaveSession       @2 :LeaveSessionMessage;

        playerData         @3 :PlayerDataMessage;
        updateIcons        @4 :UpdateIconsMessage;
        updateUserSettings @16 :UpdateUserSettingsMessage;
        sendLevelScript    @5 :SendLevelScriptMessage;
        voiceData          @6 :VoiceDataMessage;
        quickChat          @17 :QuickChatMessage;

        # Server messages
        loginOk            @7 :LoginOkMessage;
        loginFailed        @8 :LoginFailedMessage;
        joinSessionOk      @9 :JoinSessionOkMessage;
        joinSessionFailed  @10 :JoinSessionFailedMessage;

        levelData          @11 :LevelDataMessage;
        kicked             @12 :KickedMessage;
        scriptLogs         @13 :ScriptLogsMessage;
        voiceBroadcast     @14 :VoiceBroadcastMessage;
        chatNotPermitted   @15 :ChatNotPermittedMessage;
        quickChatBroadcast @18 :QuickChatBroadcastMessage;
    }
}
