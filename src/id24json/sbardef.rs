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
    // TODO: these can all be left out, so make sure to represent this somehow
    canvas: Canvas,
    graphic: Graphic,
    animation: Animation,
    face: Face,
    facebackground: FaceBG,
    number: Number,
    percent: Percent
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
    None = 0,
    // TODO: name the conditions better
    WeaponOwned = 1,
    WeaponSelected = 2,
    WeaponNotSelected = 3,
    CurrWeaponValidAmmo = 4,
    MatchesCurrWeaponAmmo = 5,
    idk6 = 6,
    idk7 = 7,
    idk8 = 8,
    idk9 = 9,
    idk10 = 10,
    idk11 = 11,
    idk12 = 12,
    idk13 = 13,
    idk14 = 14,
    idk15 = 15,
    idk16 = 16,
    idk17 = 17,
    idk18 = 18,
}