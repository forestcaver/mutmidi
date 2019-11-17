use super::framework::*;
use super::*;

use crate::elements_handlers::*;
use crate::InputDeviceId;

use heapless::consts::U8;
use heapless::Vec;

fn setup_knobs<'a>() -> Vec<Knob<'a>, U8> {
    let mut knobs = Vec::<_, U8>::new();

    knobs
        .push(Knob::new(
            Point::new(KNOB_POS_X[0], KNOB_POS_Y),
            "Lvl",
            InputDeviceId::Knob1 as InputId,
            create_knob_handler(Param::ExcStrikeLevel),
        ))
        .unwrap();
    knobs
        .push(Knob::new(
            Point::new(KNOB_POS_X[1], KNOB_POS_Y),
            "Tmbr",
            InputDeviceId::Knob2 as InputId,
            create_knob_handler(Param::ExcStrikeTimbre),
        ))
        .unwrap();
    knobs
        .push(Knob::new(
            Point::new(KNOB_POS_X[3], KNOB_POS_Y),
            "Mllt",
            InputDeviceId::Knob4 as InputId,
            create_knob_handler(Param::ExcStrikeMeta),
        ))
        .unwrap();

    knobs
}

pub fn setup<'a>() -> (Vec<Button<'a>, U8>, Vec<Knob<'a>, U8>) {
    (super::panel_bow::setup_exciter_buttons(2), setup_knobs())
}
