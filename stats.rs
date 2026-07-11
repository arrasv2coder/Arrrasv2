struct Entitymotion {
    xpos : f32,
    ypos : f32,
    movementspeedstat : f32,
    xspeed : f32,
    yspeed : f32,
    forcex : f32,
    forcey : f32,
    mousex : f32,
    mousey : f32,
    visibilityspecialabilities : u8, //this is a bitmask replacing the previous bool, it takes the same size but will allow for funnier invisibility modes, like invisible to AI turrets, or invisible to everyone not on team, or minimap jamming, or radar jamming if we ever implement a radar tank
    ownsbots : [u16;5],
}
struct Communicatedentitydata {
    angle : f32,
    entitytype : i32,
    name : [u8;30],
    position : (f32,f32),
    radius : f32,
    score : i32,
    team : i32,
    squad : i32,
    firingbitmask : u16,
}
struct Collidingentity {
    damagesbitmask : i64,
    knockbackbitmask : i64,
    hackschatbitmask : i64,
    radius : f32,
    squadid : i32,
    dmgstat : f32,
    penstat : f32,
    healrate : f32,
    hacklevel : f32,
    hp : f32,
    shield : f32,
    knockback : f32,
    damagecountedfor : i32,
    messagecarried : i32,
    isasquareorhealsbitmask : i8, //other bool replacement, smaller and more useful
    killcount : i8, //might as well put it here, this was wasted padding before. Besides this is only used when sending to client (and there is another struct for that, i can always fit it in the first 10 bits of the entity type, there are not 4 billion entity types, and 4 million eneity types with 1023 possible kills will likely be enough), or when an entity collides with another maybe dreadnought forge block, bot spawning forge to recycle the same entity system and keep a "realistic" war feel, if something else is needed it can be fetched from the communication array
    teamid : i8,
    courierid : i8,
    damagelogs : [(i32,f32);8],
}
struct Couriermessage {
    chatseen : [i32;10],
    entitiesseen : [Collidingentity;50],
    names : [[u8;30];50],
    timestamp : f32,
    position : (f32,f32),
    seenbybitmask : i64,
}
struct Botdata {
    goto : (f32,f32),
    behaviortype : i8,
}
struct Spawningentities {
    spawnstype : [i32;16],
    firingbitmask : u16,
    cooldown : [f32;16],
    reloadpertick : [f32;16],
    offsetfromspawnedx : [f32;16],
    offsetfromspawny : [f32;16],
    knockback : [f32;16],
}
struct Healthvaryingentity {
    hpregenpertick : f32,
    shieldregenpertick : f32,
}
struct Entities{
    namelist : [[u8;30];5000],
    typelist : [i32;5000],
    scorelist : [i32;5000],
    entitylistformotion : [Entitymotion;5000],
    entitylistforhp : [Healthvaryingentity;5000],
    entitylistforcollision : [Collidingentity;5000],
    entitylistfornetworkpackets : [Communicatedentitydata;5000],
    entitylistforspawn : [Spawningentities;5000],
    bots : [Botdata;5000],
    messagelist : [Couriermessage;64],
    chat : [[u8;100];100],
    recycledidlist : [i32;5000],
    listcontainssomethingupto : i16,
}
