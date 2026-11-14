// SPDX-License-Identifier: GPL-3.0-only

use smithay::{
    desktop::space::SpaceElement, reexports::wayland_server::DisplayHandle, utils::Rectangle,
    wayland::seat::WaylandFocus,
};
use tracing::error;

use crate::{
    utils::prelude::*,
    wayland::protocols::{
        qtea_toplevel_management::{
            QteaToplevelManagementHandler, QteaToplevelManagementState,
            delegate_qtea_toplevel_management,
        },
        toplevel_info::ToplevelInfoHandler,
    },
};

impl QteaToplevelManagementHandler for State {
    fn toplevel_management_state(&mut self) -> &mut QteaToplevelManagementState {
        &mut self.common.qtea_toplevel_management_state
    }

    fn set_geometry(
        &mut self,
        _dh: &DisplayHandle,
        window: &<Self as ToplevelInfoHandler>::Window,
        geo: Rectangle<i32, Global>,
    ) {
        let mut shell = self.common.shell.write();
        let Some(mapped) = shell.element_for_surface(window).cloned() else {
            error!("could not find element for window: {}", window.0.id());
            return;
        };
        let Some(surface) = window.wl_surface() else {
            error!("could not find surface for window: {}", window.0.id());
            return;
        };
        let Some(output) = shell.visible_output_for_surface(&surface).cloned() else {
            error!("could not find output for window: {}", window.0.id());
            return;
        };
        let Some(space) = shell.space_for_mut(&mapped) else {
            error!("could not find space for window: {}", window.0.id());
            return;
        };

        // width/height changes are applied immediately
        // but for x/y we have to map it
        // also width/height don't get applied if we do it before the mapping??
        space
            .floating_layer
            .map(mapped.clone(), Some(geo.loc.to_local(&output)));
        mapped.set_geometry(geo);
    }
}

delegate_qtea_toplevel_management!(State);
