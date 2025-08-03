@0xb38717d1cd8038ca;

using Shared = import "shared.capnp";

# Login

struct LoginSrvMessage {
    password @0 :Text;
    data     @1 :Shared.GameServer;
}

struct ServerRole {
    id       @0 :UInt8;
    stringId @1 :Text;
}

struct LoginOkMessage {
    tokenKey @0 :Text;
    roles    @1 :List(ServerRole);
}

struct LoginFailedMessage {
    reason @0 :Text;
}

# Rooms

struct NotifyRoomCreatedMessage {
    roomId @0 :UInt32;
    passcode @1 :UInt32;
}

struct NotifyRoomDeletedMessage {
    roomId @0 :UInt32;
}

struct RoomCreatedAckMessage {
    roomId @0 :UInt32;
}

struct Message {
    union {
        # Game server messages
        loginSrv @0 :LoginSrvMessage;
        roomCreatedAck @5 :RoomCreatedAckMessage;

        # Central server messages
        loginOk @1 :LoginOkMessage;
        loginFailed @2 :LoginFailedMessage;
        notifyRoomCreated @3 :NotifyRoomCreatedMessage;
        notifyRoomDeleted @4 :NotifyRoomDeletedMessage;
    }
}
