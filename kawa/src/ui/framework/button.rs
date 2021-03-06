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

use super::*;
use alloc::boxed::Box;
use embedded_graphics::{fonts::Font6x12, prelude::*};

pub struct Button<'a> {
    pos: Point,
    caption: &'a str,
    input_id: InputId,
    highlight: bool,
    dirty: bool,
    handler: Box<dyn FnMut(bool) -> bool>,
}

impl<'a> core::fmt::Debug for Button<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Button({})", self.caption)
    }
}

impl<'a> Button<'a> {
    pub fn new(
        pos: Point,
        caption: &'a str,
        input_id: InputId,
        handler: Box<dyn FnMut(bool) -> bool>,
    ) -> Self {
        Button {
            pos,
            caption,
            input_id,
            highlight: false,
            dirty: true,
            handler,
        }
    }
}

impl Drawable for Button<'_> {
    fn render(&mut self, drawing: &mut impl Drawing<BinaryColor>) -> (Point, Size) {
        let render = Font6x12::render_str(&self.caption)
            .fill(Some(if self.highlight {
                BinaryColor::On
            } else {
                BinaryColor::Off
            }))
            .stroke(Some(if self.highlight {
                BinaryColor::Off
            } else {
                BinaryColor::On
            }))
            .translate(self.pos);
        drawing.draw(render);
        self.dirty = false;

        (self.pos, render.size())
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl<'a> InputConsumer for Button<'a> {
    fn input_reset(&mut self) {
        self.dirty = true;
    }

    fn input_update(&mut self, input_id: InputId, value: Value) {
        if let Value::Bool(value) = value {
            if input_id == self.input_id {
                if (self.handler)(value) {
                    self.dirty = true;
                }
            }
        }
    }
}
