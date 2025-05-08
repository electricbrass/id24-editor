#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Executable {
    #[serde(rename = "doom1.9")]
    Doom1_9,
    LimitRemoving,
    Bugfixed,
    #[serde(rename = "boom2.02")]
    Boom2_02,
    CompLevel9,
    MBF,
    MBF21,
    MBF21EX,
    ID24
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Registered,
    Retail,
    Commercial
}