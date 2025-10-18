@0xb27cd19ebf0acb82;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("globed::schema::shared");

struct SpecialUserData {
    roles @0 :List(UInt8); # can be empty
    nameColor @1 :Data;    # optional
}

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

struct PlayerDisplayData {
    accountId @0 :Int32;
    userId    @1 :Int32;
    username  @2 :Text;
    icons     @3 :PlayerIconData;
    specialData @4 :SpecialUserData;
}

struct GameServer {
    address  @0 :Text;  # Qunet URL
    stringId @1 :Text;  # Permanently unique server ID
    id       @2 :UInt8; # Temporary server ID, invalidated after that server restarts
    name     @3 :Text;  # Human-readable name
    region   @4 :Text;  # Region string
}

struct UserRole {
    stringId   @0 :Text;
    icon       @1 :Text;
    nameColor  @2 :Data;
}

struct UserSettings {
    hideInLevel @0 :Bool = false;
    hideInMenus @1 :Bool = false;
    hideRoles   @2 :Bool = false;
}
