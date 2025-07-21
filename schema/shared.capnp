@0xb27cd19ebf0acb82;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("globed::schema::shared");

enum IconType {
    unknown @0;
    cube    @1;
    ship    @2;
    ball    @3;
    ufo     @4;
    wave    @5;
    robot   @6;
    spider  @7;
    swing   @8;
    jetpack @9;
}

struct GameServer {
    address  @0 :Text;  # Qunet URL
    stringId @1 :Text;  # Permanently unique server ID
    id       @2 :UInt8; # Temporary server ID, invalidated after that server restarts
    name     @3 :Text;  # Human-readable name
    region   @4 :Text;  # Region string
}
