// Copyright 2019 Wojciech Jakóbczyk
//
// Author: Wojciech Jakóbczyk (jakobczyk.woj@gmail.com)
//
// This file is part of Kawa Synth.
//
// Kawa Synth is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Kawa Synth is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Kawa Synth.  If not, see <https://www.gnu.org/licenses/>.

use super::framework::*;
use super::*;

use crate::elements_handlers::*;
use crate::{InputDeviceId, PanelId, APP};

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

fn setup_knobs<'a>() -> Vec<Knob<'a>> {
    vec![
        Knob::new(
            Point::new(KNOB_POS_X[0], KNOB_POS_Y),
            "Lvl",
            InputDeviceId::Knob1 as InputId,
            create_knob_handler(Param::ExcBowLevel),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[1], KNOB_POS_Y),
            "Tmbr",
            InputDeviceId::Knob2 as InputId,
            create_knob_handler(Param::ExcBowTimbre),
        ),
        Knob::new(
            Point::new(KNOB_POS_X[2], KNOB_POS_Y),
            "Cntr",
            InputDeviceId::Knob3 as InputId,
            create_knob_handler(Param::ExcEnvShape),
        ),
    ]
}

pub fn setup_exciter_buttons<'a>(active: i8) -> Vec<Button<'a>> {
    vec![
        Button::new(
            Point::new(BUTTON_POS_X[0], BUTTON_POS_Y),
            if active == 0 { "*Bow" } else { " Bow" },
            InputDeviceId::Button1 as InputId,
            Box::new(|_value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelBow);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[1], BUTTON_POS_Y),
            if active == 1 { "*Blw" } else { " Blw" },
            InputDeviceId::Button2 as InputId,
            Box::new(|_value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelBlow);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[2], BUTTON_POS_Y),
            if active == 2 { "*Str" } else { " Str" },
            InputDeviceId::Button3 as InputId,
            Box::new(|_value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelStrike);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[3], BUTTON_POS_Y),
            "Res",
            InputDeviceId::Button4 as InputId,
            Box::new(|_value: bool| {
                unsafe {
                    (*APP).change_panel(&mut *APP, PanelId::PanelRes);
                }
                true
            }),
        ),
        Button::new(
            Point::new(BUTTON_POS_X[4], BUTTON_POS_Y),
            "Sys",
            InputDeviceId::Button5 as InputId,
            Box::new(|_value: bool| true),
        ),
    ]
}

pub fn setup<'a>() -> (Vec<Button<'a>>, Vec<Knob<'a>>) {
    (setup_exciter_buttons(0), setup_knobs())
}
