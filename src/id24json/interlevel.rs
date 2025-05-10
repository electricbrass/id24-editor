#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Layer {
    anims: Vec<Anim>,
    conditions: Option<Vec<Condition>> // TODO: is length 0 allowed?
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Anim {
    x: u16,
    y: u16,
    frames: Vec<Frame>,
    conditions: Option<Vec<Condition>> // TODO: is length 0 allowed?
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Frame {
    image: String,
    #[serde(rename = "type")]
    frame_type: FrameType,
    duration: f32,
    maxduration: f32,
}

impl serde::Serialize for FrameType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_u16(self.to_u16())
    }
}

impl<'a> serde::Deserialize<'a> for FrameType {
    fn deserialize<D>(deserializer: D) -> Result<FrameType, D::Error> where D: serde::Deserializer<'a> {
        let value = u16::deserialize(deserializer)?;
        FrameType::from_u16(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, PartialEq, Debug)]
struct FrameType {
    random_offset: bool,
    duration: Duration
}

#[derive(Clone, PartialEq, Debug)]
enum Duration {
    None,
    Infinite,
    Fixed,
    Random
}

impl FrameType {
    fn from_u16(value: u16) -> Result<Self, &'static str> {
        let duration = match value & 0b111 {
            0b000 => Duration::None,
            0b001 => Duration::Infinite,
            0b010 => Duration::Fixed,
            0b100 => Duration::Random,
            _ => return Err("Multiple durations specified."),
        };
        let random_offset = (value & 0b1_0000_0000_0000) != 0;

        Ok(Self {
            random_offset,
            duration,
        })
    }

    fn to_u16(&self) -> u16 {
        let duration_bits = match self.duration {
            Duration::None     => 0b000,
            Duration::Infinite => 0b001,
            Duration::Fixed    => 0b010,
            Duration::Random   => 0b100,
        };
        let offset_bit = if self.random_offset { 0b1_0000_0000_0000 } else { 0 };

        duration_bits | offset_bit
    }
}


#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct Condition {
    condition: ConditionType,
    param: u8
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone, PartialEq, Debug)]
#[repr(u8)]
enum ConditionType {
    None             = 0,
    CurrMapGreater   = 1, // Current map number is greater than the param value
    CurrMapEqual     = 2, // Current map number is equal to the param value
    MapVisited       = 3, // The map number corresponding to the param value has been visited
    CurrMapNotSecret = 4, // The current map is not a secret map
    AnySecretVisited = 5, // Any secret map has been visited
    OnFinishedScreen = 6, // The current screen is the "finished" screen
    OnEnteringScreen = 7, // The current screen is the “entering” screen
}