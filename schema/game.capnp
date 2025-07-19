@0x95684a6f0a0e3cd1;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("globed::schema::game");

# Login messages

struct LoginUTokenMessage {
    accountId @0 :Int32;
    token @1 :Text;
}

struct LoginUTokenAndJoinMessage {
    accountId @0 :Int32;
    token @1 :Text;
    sessionId @2 :UInt64;
    passcode  @3 :UInt32;
}

struct LoginOkMessage {
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
}

struct JoinSessionOkMessage {
    sessionId @0 :UInt64;
}

enum JoinSessionFailedReason {
    notFound @0;
    invalidPasscode @1;
}

struct JoinSessionFailedMessage {
    reason @0 :JoinSessionFailedReason;
}

struct LeaveSessionMessage {
}

struct Message {
    union {
        # Client messages
        loginUToken        @0 :LoginUTokenMessage;
        loginUTokenAndJoin @3 :LoginUTokenAndJoinMessage;
        joinSession        @4 :JoinSessionMessage;
        leaveSession       @5 :LeaveSessionMessage;

        # Server messages
        loginOk            @1 :LoginOkMessage;
        loginFailed        @2 :LoginFailedMessage;
    }
}
