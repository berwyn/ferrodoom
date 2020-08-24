use bitflags::bitflags;

pub(crate) enum ThingType {}

bitflags! {
    pub(crate) struct ThingFlags: u8 {
        const SkillLevel1And2 = 0x01;
        const SkillLevel3 = 0x02;
        const SkillLevel4And5 = 0x04;
        const IsDeaf = 0x08;
        const MultiplayerOnly = 0x10;
        const BoomNotInDeathmatch = 0x20;
        const BoomNotInCoop = 0x40;
        const MBFFriendlyMonster = 0x80;
    }
}

pub(crate) struct Thing {
    position: (i16, i16),
    facing: i16,
    doom_editor_type: ThingType,
    flags: ThingFlags,
}
