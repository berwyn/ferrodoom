use bitflags::bitflags;

bitflags! {
    pub(crate) struct LinedefFlags: u16 {
        const BlocksPlayersAndMonsters = 0x01;
        const BlocksMonsters = 0x02;
        const TwoSided = 0x04;
        const UpperTextureUnpegged = 0x08;
        const LowerTextureUnpegged = 0x10;
        const Secret = 0x20;
        const BlocksSound = 0x40;
        const HiddenOnAutomap = 0x80;
        const AlwaysShownOnAutomap = 0x100;
        // ZDoom / Hexen stuff
        const SuppportsMultipleActivations = 0x200;
        const ActivatedByPlayer = 0x400;
        const ActivatedByMonster = 0x800;
        const ActivatedByProjectile = 0xC00;
        const ActivatedByBumpedPlayer = 0x1000;
        const ActivatedByProjectileCrossing = 0x1400;
        const ActivatedByPlayerCrossthrough = 0x1800;
        const ActivatedByPlayersAndMonsters = 0x2000;
        const Unknown = 0x4000;
        const BlocksAllObjects = 0x8000;
    }
}

pub(crate) struct Linedef {
    start_vertex: i16,
    end_vertex: i16,
    flags: LinedefFlags,
    special_type: i16, // TODO: what is this?
    sector_tag: i16,
    right_sidedef: u16, // TODO: what is this?
    left_sidedef: u16,  // TODO: what is this?
}
