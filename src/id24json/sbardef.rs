#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct NumberFont {
    name: String,
    numberfont_type: NumberFontType,
    stem: String
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[repr(u8)]
enum NumberFontType {
    MonoSpacedZero = 0,
    MonoSpaceWidest = 1,
    Proportional = 2
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct StatusBar {
    height: u16,
    fullscreenrender: bool,
    fillflat: String,
    children: Vec<SBarElem>
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct SBarElem {
    // TODO: spec says all these can be undefined but does *not* say they can be null, currently this works for writing but when reading this will allow null to be accepted
    // I suppose it's maybe okay if we let bad json be fixed up a bit
    // Already any extra fields not part of the spec will just be thrown away
    #[serde(skip_serializing_if = "Option::is_none")]
    canvas: Option<Canvas>,
    #[serde(skip_serializing_if = "Option::is_none")]
    graphic: Option<Graphic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    face: Option<Face>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facebackground: Option<FaceBG>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<Percent>
}

type Face = Canvas;
type FaceBG = Canvas;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Canvas {
    x: u16,
    y: u16,
    alignment: u32, // TODO: find out width of the bitfield
    conditions: Option<Vec<Condition>>,
    children: Option<Vec<SBarElem>>
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Graphic {
    x: u16,
    y: u16,
    alignment: u32, // TODO: find out width of the bitfield
    tranmap: String,
    translation: String,
    conditions: Option<Vec<Condition>>,
    children: Option<Vec<SBarElem>>,
    patch: String
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Animation {
    x: u16,
    y: u16,
    alignment: u32, // TODO: find out width of the bitfield
    tranmap: String,
    translation: String,
    conditions: Option<Vec<Condition>>,
    children: Option<Vec<SBarElem>>,
    frames: Vec<Frame>
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Frame {
    lump: String,
    duration: f32
}

type Percent = Number;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Number {
    x: u16,
    y: u16,
    alignment: u32, // TODO: find out width of the bitfield
    tranmap: String,
    translation: String,
    conditions: Option<Vec<Condition>>,
    children: Option<Vec<SBarElem>>,
    font: String,
    #[serde(rename = "type")]
    num_type: NumberType,
    param: u8,
    maxlength: u8
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[repr(u8)]
enum NumberType {
    // TODO: maybe make the ammo names clearer
    Health = 0,
    Armor = 1,
    Frags = 2,
    AmmoParam = 3,
    AmmoCurrWeapon = 4,
    MaxAmmoParam = 5,
    AmmoParamWeapon = 6,
    MaxAmmoParamWeapon = 7
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Condition {
    condition: ConditionType,
    param: u8
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[repr(u8)]
enum ConditionType {
    WeaponOwned           = 0, // Whether the weapon defined by param is owned
    WeaponSelected        = 1, // Whether the weapon defined by param is selected
    WeaponNotSelected     = 2, // Whether the weapon defined by param is not selected
    WeaponValidAmmo       = 3, // Whether the weapon defined by param has a valid ammo type
    CurrWeaponValidAmmo   = 4, // Whether the selected weapon has a valid ammo type
    MatchesCurrWeaponAmmo = 5, // Whether the ammo type defined by param matches the selected weapon’s ammo type
    AnyWeaponOwned        = 6, // Whether any weapon in a slot defined by param is owned
    AnyWeaponNotOwned     = 7, // Whether any weapon in a slot defined by param not owned
    AnyWeaponSelected     = 8, // Whether any weapon in a slot defined by param is selected
    AnyWeaponNotSelected  = 9, // Whether any weapon in a slot defined by param is not selected
    ItemOwned             = 10, // Whether the item defined by param is owned
    ItemNotOwned          = 11, // Whether the item defined by param is not owned
    GameVersionGreaterEq  = 12, // Whether the current game version is greater than or equal to the feature level defined by param
    GameVersionLess       = 13, // Whether the current game version is less than the feature level defined by param
    SessionTypeEqual      = 14, // Whether the session type is equal to the type defined by param
    SessionTypeNotEqual   = 15, // Whether the session type is not equal to the type defined by param
    GameModeEqual         = 16, // Whether the game mode is equal to the mode defined by param
    GameModeNotEqual      = 17, // Whether the game mode is not equal to the mode defined by param
    HudModeEqual          = 18 // Whether the hud mode is equal to the mode defined by param
}