// SPDX-License-Identifier: GPL-3.0-only

use smithay::{
    input::SeatHandler,
    reexports::wayland_server::{
        Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, backend::GlobalId,
    },
    utils::Rectangle,
};

pub use cosmic_protocols::toplevel_management::v1::server::zcosmic_toplevel_manager_v1::ZcosmicToplelevelManagementCapabilitiesV1 as ManagementCapabilities;
use qtea_cosmic_protocols::toplevel_management::v1::server::zqtea_cosmic_toplevel_manager_v1::{
    self, ZqteaCosmicToplevelManagerV1,
};

use crate::{utils::geometry::Global, wayland::protocols::toplevel_management::ManagementWindow};

use super::toplevel_info::{ToplevelInfoHandler, window_from_handle};

#[derive(Debug)]
pub struct QteaToplevelManagementState {
    instances: Vec<ZqteaCosmicToplevelManagerV1>,
    global: GlobalId,
}

#[allow(unused_variables)]
pub trait QteaToplevelManagementHandler: ToplevelInfoHandler + SeatHandler
where
    <Self as ToplevelInfoHandler>::Window: ManagementWindow,
{
    fn toplevel_management_state(&mut self) -> &mut QteaToplevelManagementState;

    fn set_geometry(
        &mut self,
        dh: &DisplayHandle,
        window: &<Self as ToplevelInfoHandler>::Window,
        geo: Rectangle<i32, Global>,
    );
}

pub struct QteaToplevelManagerGlobalData;

impl QteaToplevelManagementState {
    pub fn new<D>(dh: &DisplayHandle) -> QteaToplevelManagementState
    where
        D: GlobalDispatch<ZqteaCosmicToplevelManagerV1, QteaToplevelManagerGlobalData>
            + Dispatch<ZqteaCosmicToplevelManagerV1, ()>
            + QteaToplevelManagementHandler
            + 'static,
        <D as ToplevelInfoHandler>::Window: ManagementWindow,
    {
        let global = dh
            .create_global::<D, ZqteaCosmicToplevelManagerV1, _>(1, QteaToplevelManagerGlobalData);
        QteaToplevelManagementState {
            instances: Vec::new(),
            global,
        }
    }

    pub fn global_id(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> GlobalDispatch<ZqteaCosmicToplevelManagerV1, QteaToplevelManagerGlobalData, D>
    for QteaToplevelManagementState
where
    D: GlobalDispatch<ZqteaCosmicToplevelManagerV1, QteaToplevelManagerGlobalData>
        + Dispatch<ZqteaCosmicToplevelManagerV1, ()>
        + QteaToplevelManagementHandler
        + 'static,
    <D as ToplevelInfoHandler>::Window: ManagementWindow,
{
    fn bind(
        state: &mut D,
        _dh: &DisplayHandle,
        _client: &Client,
        resource: New<ZqteaCosmicToplevelManagerV1>,
        _global_data: &QteaToplevelManagerGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        let instance = data_init.init(resource, ());
        state.toplevel_management_state().instances.push(instance);
    }
}

impl<D> Dispatch<ZqteaCosmicToplevelManagerV1, (), D> for QteaToplevelManagementState
where
    D: GlobalDispatch<ZqteaCosmicToplevelManagerV1, QteaToplevelManagerGlobalData>
        + Dispatch<ZqteaCosmicToplevelManagerV1, ()>
        + QteaToplevelManagementHandler
        + 'static,
    <D as ToplevelInfoHandler>::Window: ManagementWindow,
{
    fn request(
        state: &mut D,
        _client: &Client,
        _obj: &ZqteaCosmicToplevelManagerV1,
        request: zqtea_cosmic_toplevel_manager_v1::Request,
        _data: &(),
        dh: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            zqtea_cosmic_toplevel_manager_v1::Request::SetGeometry {
                toplevel,
                x,
                y,
                width,
                height,
            } => {
                let window = window_from_handle(toplevel).unwrap();
                let rectangle = Rectangle::new((x, y).into(), (width, height).into());
                state.set_geometry(dh, &window, rectangle);
            }
            _ => unreachable!(),
        }
    }
}

macro_rules! delegate_qtea_toplevel_management {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            qtea_cosmic_protocols::toplevel_management::v1::server::zqtea_cosmic_toplevel_manager_v1::ZqteaCosmicToplevelManagerV1: $crate::wayland::protocols::qtea_toplevel_management::QteaToplevelManagerGlobalData
        ] => $crate::wayland::protocols::qtea_toplevel_management::QteaToplevelManagementState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            qtea_cosmic_protocols::toplevel_management::v1::server::zqtea_cosmic_toplevel_manager_v1::ZqteaCosmicToplevelManagerV1: ()
        ] => $crate::wayland::protocols::qtea_toplevel_management::QteaToplevelManagementState);
    };
}
pub(crate) use delegate_qtea_toplevel_management;
