use crate::{SkyTexMessage, SkydefsIndex};
use crate::id24json::{ID24JsonData};

struct SkydefsEditor {
    skydefs_index: SkydefsIndex,
}

enum SkydefsMessage {
    UpdateSkyTexProp(SkyTexMessage),
    UpdateSkyTexPropFG(SkyTexMessage),
    ChangeSkyType(crate::id24json::skydefs::SkyType),
    ChangeFireSpeed(f32),
    NewSky,
    NewFlatmapping,
    DeleteSky(usize),
    DeleteFlatmapping(usize),
    SelectSky(Option<usize>),
    SelectFlatmapping(Option<usize>),
}

impl SkydefsEditor {
    fn view(&self, json: &ID24JsonData) -> cosmic::Element<SkydefsMessage> {
        cosmic::widget::text("Skydefs").into()
    }
    fn update(&mut self, json: &mut ID24JsonData, message: SkydefsMessage) -> cosmic::Task<cosmic::Action<SkydefsMessage>> {
        match message {
            SkydefsMessage::NewSky => todo!(),
            SkydefsMessage::NewFlatmapping => unimplemented!(),
            _ => todo!()
        }
    }
}