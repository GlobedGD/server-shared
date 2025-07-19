@0xb38717d1cd8038ca;

using Shared = import "shared.capnp";

struct LoginSrvMessage {
    password @0 :Text;
    data     @1 :Shared.GameServer;
}

struct LoginOkMessage {
    tokenKey @0 :Text;
}

struct LoginFailedMessage {
    reason @0 :Text;
}

struct Message {
    union {
        # Game server messages
        loginSrv @0 :LoginSrvMessage;

        # Central server messages
        loginOk @1 :LoginOkMessage;
        loginFailed @2 :LoginFailedMessage;
    }
}
