pub enum SectorType {
    Normal,
    LightBlinkRandom,
    LightBlinkHalfSecond,
    LightBlinkSecond,
    Damage20WithBlinkSecond,
    Damage10,
    Damage5,
    LightOscillates,
    Secret,
    CeilingDescendsAfter30,
    LevelEnd,
    BlinkHalfSecondSync,
    BlinkSecondSync,
    CeilingAscendsAfter300,
    Damage20,
    LightFlicker,
}

pub struct Sector {
    floor_height: i16,
    ceiling_height: i16,
    floor_texture_name: String,
    ceiling_texture_name: String,
    light_level: i16,
    sector_type: SectorType,
    tag_number: i16,
}
