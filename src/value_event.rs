/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */

use gateway_addon_rust::{
    event::Event,
    event_description::{EventDescription, NoData},
};

pub struct ValueEvent {}

impl ValueEvent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Event for ValueEvent {
    type Data = NoData;

    fn name(&self) -> String {
        "value_event".to_owned()
    }

    fn description(&self) -> EventDescription<Self::Data> {
        EventDescription::default()
            .title("Property value updated event")
            .description("Raised whenever the value of random_property is updated")
    }
}
