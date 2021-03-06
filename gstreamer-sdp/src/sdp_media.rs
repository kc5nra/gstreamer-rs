// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
// //
// // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// // option. This file may not be copied, modified, or distributed
// // except according to those terms.

use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops;
use std::ptr;

use glib::translate::*;
use gst;

use gst_sdp_sys;

use sdp_attribute::SDPAttribute;
use sdp_bandwidth::SDPBandwidth;
use sdp_connection::SDPConnection;
use sdp_key::SDPKey;

glib_wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SDPMedia(Boxed<gst_sdp_sys::GstSDPMedia>);

    match fn {
        copy => |ptr| {
            let mut copy = ptr::null_mut();
            gst_sdp_sys::gst_sdp_media_copy(ptr as *const gst_sdp_sys::GstSDPMedia, &mut copy);
            copy
        },
        free => |ptr| {
            gst_sdp_sys::gst_sdp_media_free(ptr);
        },
    }
}

impl SDPMedia {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut media = ptr::null_mut();
            gst_sdp_sys::gst_sdp_media_new(&mut media);
            from_glib_full(media)
        }
    }
}

impl Default for SDPMedia {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for SDPMedia {}
unsafe impl Sync for SDPMedia {}

impl ops::Deref for SDPMedia {
    type Target = SDPMediaRef;

    fn deref(&self) -> &SDPMediaRef {
        unsafe { &*(self.to_glib_none().0 as *const SDPMediaRef) }
    }
}

impl ops::DerefMut for SDPMedia {
    fn deref_mut(&mut self) -> &mut SDPMediaRef {
        unsafe { &mut *(self.to_glib_none_mut().0 as *mut SDPMediaRef) }
    }
}

impl fmt::Debug for SDPMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <SDPMediaRef as fmt::Debug>::fmt(&*self, f)
    }
}

impl fmt::Display for SDPMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <SDPMediaRef as fmt::Display>::fmt(&*self, f)
    }
}

#[repr(C)]
pub struct SDPMediaRef(gst_sdp_sys::GstSDPMedia);

impl fmt::Debug for SDPMediaRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::cell::RefCell;

        struct DebugIter<I>(RefCell<I>);
        impl<I: Iterator> fmt::Debug for DebugIter<I>
        where
            I::Item: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_list().entries(&mut *self.0.borrow_mut()).finish()
            }
        }

        f.debug_struct("SDPMedia")
            .field("formats", &DebugIter(RefCell::new(self.formats())))
            .field("connections", &DebugIter(RefCell::new(self.connections())))
            .field("bandwidths", &DebugIter(RefCell::new(self.bandwidths())))
            .field("attributes", &DebugIter(RefCell::new(self.attributes())))
            .field("information", &self.get_information())
            .field("key", &self.get_key())
            .field("media", &self.get_media())
            .field("port", &self.get_port())
            .field("num-ports", &self.get_num_ports())
            .field("proto", &self.get_proto())
            .finish()
    }
}

impl fmt::Display for SDPMediaRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_text() {
            Ok(text) => f.write_str(text.as_str()),
            Err(_) => Err(fmt::Error),
        }
    }
}

unsafe impl Send for SDPMediaRef {}
unsafe impl Sync for SDPMediaRef {}

impl SDPMediaRef {
    pub fn add_attribute(&mut self, key: &str, value: Option<&str>) {
        let value = value.to_glib_none();
        unsafe {
            gst_sdp_sys::gst_sdp_media_add_attribute(&mut self.0, key.to_glib_none().0, value.0)
        };
    }

    pub fn add_bandwidth(&mut self, bwtype: &str, bandwidth: u32) {
        unsafe {
            gst_sdp_sys::gst_sdp_media_add_bandwidth(
                &mut self.0,
                bwtype.to_glib_none().0,
                bandwidth,
            )
        };
    }

    pub fn add_connection(
        &mut self,
        nettype: &str,
        addrtype: &str,
        address: &str,
        ttl: u32,
        addr_number: u32,
    ) {
        unsafe {
            gst_sdp_sys::gst_sdp_media_add_connection(
                &mut self.0,
                nettype.to_glib_none().0,
                addrtype.to_glib_none().0,
                address.to_glib_none().0,
                ttl,
                addr_number,
            )
        };
    }

    pub fn add_format(&mut self, format: &str) {
        unsafe { gst_sdp_sys::gst_sdp_media_add_format(&mut self.0, format.to_glib_none().0) };
    }

    pub fn as_text(&self) -> Result<String, glib::error::BoolError> {
        unsafe {
            match from_glib_full(gst_sdp_sys::gst_sdp_media_as_text(&self.0)) {
                Some(s) => Ok(s),
                None => Err(glib_bool_error!(
                    "Failed to convert the contents of media to a text string"
                )),
            }
        }
    }

    pub fn attributes(&self) -> AttributesIter {
        AttributesIter::new(self)
    }

    pub fn formats(&self) -> FormatsIter {
        FormatsIter::new(self)
    }

    pub fn bandwidths(&self) -> BandwidthsIter {
        BandwidthsIter::new(self)
    }

    pub fn connections(&self) -> ConnectionsIter {
        ConnectionsIter::new(self)
    }

    pub fn attributes_len(&self) -> u32 {
        unsafe { gst_sdp_sys::gst_sdp_media_attributes_len(&self.0) }
    }

    pub fn attributes_to_caps(&self, caps: &mut gst::CapsRef) -> Result<(), ()> {
        let result =
            unsafe { gst_sdp_sys::gst_sdp_media_attributes_to_caps(&self.0, caps.as_mut_ptr()) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn bandwidths_len(&self) -> u32 {
        unsafe { gst_sdp_sys::gst_sdp_media_bandwidths_len(&self.0) }
    }

    pub fn connections_len(&self) -> u32 {
        unsafe { gst_sdp_sys::gst_sdp_media_connections_len(&self.0) }
    }

    pub fn formats_len(&self) -> u32 {
        unsafe { gst_sdp_sys::gst_sdp_media_formats_len(&self.0) }
    }

    pub fn get_attribute(&self, idx: u32) -> Option<&SDPAttribute> {
        if idx >= self.attributes_len() {
            return None;
        }

        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_attribute(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPAttribute))
            }
        }
    }

    pub fn get_attribute_val(&self, key: &str) -> Option<&str> {
        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_attribute_val(&self.0, key.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                let result = CStr::from_ptr(ptr).to_str();
                match result {
                    Ok(attr) => Some(attr),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn get_attribute_val_n(&self, key: &str, nth: u32) -> Option<&str> {
        unsafe {
            let ptr =
                gst_sdp_sys::gst_sdp_media_get_attribute_val_n(&self.0, key.to_glib_none().0, nth);
            if ptr.is_null() {
                None
            } else {
                let result = CStr::from_ptr(ptr).to_str();
                match result {
                    Ok(attr) => Some(attr),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn get_bandwidth(&self, idx: u32) -> Option<&SDPBandwidth> {
        if idx >= self.bandwidths_len() {
            return None;
        }

        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_bandwidth(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPBandwidth))
            }
        }
    }

    pub fn get_caps_from_media(&self, pt: i32) -> Option<gst::Caps> {
        unsafe { from_glib_full(gst_sdp_sys::gst_sdp_media_get_caps_from_media(&self.0, pt)) }
    }

    pub fn get_connection(&self, idx: u32) -> Option<&SDPConnection> {
        if idx >= self.connections_len() {
            return None;
        }

        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_connection(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPConnection))
            }
        }
    }

    pub fn get_format(&self, idx: u32) -> Option<&str> {
        if idx >= self.formats_len() {
            return None;
        }

        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_format(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                let result = CStr::from_ptr(ptr).to_str();
                match result {
                    Ok(attr) => Some(attr),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn get_information(&self) -> Option<&str> {
        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_information(&self.0);
            if ptr.is_null() {
                None
            } else {
                let result = CStr::from_ptr(ptr).to_str();
                match result {
                    Ok(attr) => Some(attr),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn get_key(&self) -> Option<&SDPKey> {
        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_key(&self.0);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPKey))
            }
        }
    }

    pub fn get_media(&self) -> Option<&str> {
        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_media(&self.0);
            if ptr.is_null() {
                None
            } else {
                let result = CStr::from_ptr(ptr).to_str();
                match result {
                    Ok(attr) => Some(attr),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn get_num_ports(&self) -> u32 {
        unsafe { gst_sdp_sys::gst_sdp_media_get_num_ports(&self.0) }
    }

    pub fn get_port(&self) -> u32 {
        unsafe { gst_sdp_sys::gst_sdp_media_get_port(&self.0) }
    }

    pub fn get_proto(&self) -> Option<&str> {
        unsafe {
            let ptr = gst_sdp_sys::gst_sdp_media_get_proto(&self.0);
            if ptr.is_null() {
                None
            } else {
                let result = CStr::from_ptr(ptr).to_str();
                match result {
                    Ok(attr) => Some(attr),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn insert_attribute(&mut self, idx: Option<u32>, attr: SDPAttribute) -> Result<(), ()> {
        if let Some(idx) = idx {
            if idx >= self.attributes_len() {
                return Err(());
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let mut attr = mem::ManuallyDrop::new(attr);
        let result =
            unsafe { gst_sdp_sys::gst_sdp_media_insert_attribute(&mut self.0, idx, &mut attr.0) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn insert_bandwidth(&mut self, idx: Option<u32>, bw: SDPBandwidth) -> Result<(), ()> {
        if let Some(idx) = idx {
            if idx >= self.bandwidths_len() {
                return Err(());
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let mut bw = mem::ManuallyDrop::new(bw);
        let result =
            unsafe { gst_sdp_sys::gst_sdp_media_insert_bandwidth(&mut self.0, idx, &mut bw.0) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn insert_connection(&mut self, idx: Option<u32>, conn: SDPConnection) -> Result<(), ()> {
        if let Some(idx) = idx {
            if idx >= self.connections_len() {
                return Err(());
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let mut conn = mem::ManuallyDrop::new(conn);
        let result =
            unsafe { gst_sdp_sys::gst_sdp_media_insert_connection(&mut self.0, idx, &mut conn.0) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn insert_format(&mut self, idx: Option<u32>, format: &str) -> Result<(), ()> {
        if let Some(idx) = idx {
            if idx >= self.formats_len() {
                return Err(());
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let result = unsafe {
            gst_sdp_sys::gst_sdp_media_insert_format(&mut self.0, idx, format.to_glib_none().0)
        };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn remove_attribute(&mut self, idx: u32) -> Result<(), ()> {
        if idx >= self.attributes_len() {
            return Err(());
        }

        let result = unsafe { gst_sdp_sys::gst_sdp_media_remove_attribute(&mut self.0, idx) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn remove_bandwidth(&mut self, idx: u32) -> Result<(), ()> {
        if idx >= self.bandwidths_len() {
            return Err(());
        }

        let result = unsafe { gst_sdp_sys::gst_sdp_media_remove_bandwidth(&mut self.0, idx) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn remove_connection(&mut self, idx: u32) -> Result<(), ()> {
        if idx >= self.connections_len() {
            return Err(());
        }

        let result = unsafe { gst_sdp_sys::gst_sdp_media_remove_connection(&mut self.0, idx) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn remove_format(&mut self, idx: u32) -> Result<(), ()> {
        if idx >= self.formats_len() {
            return Err(());
        }

        let result = unsafe { gst_sdp_sys::gst_sdp_media_remove_format(&mut self.0, idx) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn replace_attribute(&mut self, idx: u32, attr: SDPAttribute) -> Result<(), ()> {
        if idx >= self.attributes_len() {
            return Err(());
        }

        let mut attr = mem::ManuallyDrop::new(attr);
        let result =
            unsafe { gst_sdp_sys::gst_sdp_media_replace_attribute(&mut self.0, idx, &mut attr.0) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn replace_bandwidth(&mut self, idx: u32, bw: SDPBandwidth) -> Result<(), ()> {
        if idx >= self.bandwidths_len() {
            return Err(());
        }

        let mut bw = mem::ManuallyDrop::new(bw);
        let result =
            unsafe { gst_sdp_sys::gst_sdp_media_replace_bandwidth(&mut self.0, idx, &mut bw.0) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn replace_connection(&mut self, idx: u32, conn: SDPConnection) -> Result<(), ()> {
        if idx >= self.connections_len() {
            return Err(());
        }

        let mut conn = mem::ManuallyDrop::new(conn);
        let result =
            unsafe { gst_sdp_sys::gst_sdp_media_replace_connection(&mut self.0, idx, &mut conn.0) };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn replace_format(&mut self, idx: u32, format: &str) -> Result<(), ()> {
        if idx >= self.formats_len() {
            return Err(());
        }

        let result = unsafe {
            gst_sdp_sys::gst_sdp_media_replace_format(&mut self.0, idx, format.to_glib_none().0)
        };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn set_information(&mut self, information: &str) {
        unsafe {
            gst_sdp_sys::gst_sdp_media_set_information(&mut self.0, information.to_glib_none().0)
        };
    }

    pub fn set_key(&mut self, type_: &str, data: &str) {
        unsafe {
            gst_sdp_sys::gst_sdp_media_set_key(
                &mut self.0,
                type_.to_glib_none().0,
                data.to_glib_none().0,
            )
        };
    }

    pub fn set_media(&mut self, med: &str) {
        unsafe { gst_sdp_sys::gst_sdp_media_set_media(&mut self.0, med.to_glib_none().0) };
    }

    pub fn set_port_info(&mut self, port: u32, num_ports: u32) {
        unsafe { gst_sdp_sys::gst_sdp_media_set_port_info(&mut self.0, port, num_ports) };
    }

    pub fn set_proto(&mut self, proto: &str) {
        unsafe { gst_sdp_sys::gst_sdp_media_set_proto(&mut self.0, proto.to_glib_none().0) };
    }

    pub fn set_media_from_caps(caps: &gst::Caps, media: &mut SDPMedia) -> Result<(), ()> {
        assert_initialized_main_thread!();
        let result = unsafe {
            gst_sdp_sys::gst_sdp_media_set_media_from_caps(
                caps.to_glib_none().0,
                media.to_glib_none_mut().0,
            )
        };
        match result {
            gst_sdp_sys::GST_SDP_OK => Ok(()),
            _ => Err(()),
        }
    }
}

impl Borrow<SDPMediaRef> for SDPMedia {
    fn borrow(&self) -> &SDPMediaRef {
        &*self
    }
}

impl BorrowMut<SDPMediaRef> for SDPMedia {
    fn borrow_mut(&mut self) -> &mut SDPMediaRef {
        &mut *self
    }
}

impl ToOwned for SDPMediaRef {
    type Owned = SDPMedia;

    fn to_owned(&self) -> SDPMedia {
        unsafe {
            let mut ptr = ptr::null_mut();
            gst_sdp_sys::gst_sdp_media_copy(&self.0, &mut ptr);
            from_glib_full(ptr)
        }
    }
}

macro_rules! define_iter(
    ($name:ident, $typ:ty, $get_item:expr, $get_len:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        media: &'a SDPMediaRef,
        idx: u32,
        len: u32,
    }

    impl<'a> $name<'a> {
        fn new(media: &'a SDPMediaRef) -> $name<'a> {
            skip_assert_initialized!();
            let len = $get_len(media);

            $name {
                media,
                idx: 0,
                len,
            }
        }
    }

    impl<'a> Iterator for $name<'a> {
        type Item = $typ;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.len {
                return None;
            }

            let item = $get_item(self.media, self.idx)?;
            self.idx += 1;
            Some(item)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            if self.idx == self.len {
                return (0, Some(0))
            }

            let remaining = (self.len - self.idx) as usize;

            (remaining, Some(remaining))
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.len {
                return None;
            }

            self.len -= 1;

            $get_item(self.media, self.len)
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    }
);

define_iter!(
    BandwidthsIter,
    &'a SDPBandwidth,
    |media: &'a SDPMediaRef, idx| media.get_bandwidth(idx),
    |media: &SDPMediaRef| media.bandwidths_len()
);
define_iter!(
    FormatsIter,
    &'a str,
    |media: &'a SDPMediaRef, idx| media.get_format(idx),
    |media: &SDPMediaRef| media.formats_len()
);
define_iter!(
    ConnectionsIter,
    &'a SDPConnection,
    |media: &'a SDPMediaRef, idx| media.get_connection(idx),
    |media: &SDPMediaRef| media.connections_len()
);
define_iter!(
    AttributesIter,
    &'a SDPAttribute,
    |media: &'a SDPMediaRef, idx| media.get_attribute(idx),
    |media: &SDPMediaRef| media.attributes_len()
);

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        gst::init().unwrap();
    }

    #[test]
    fn debug_impl() {
        init();

        let sdp = SDPMedia::new();
        assert!(!format!("{:?}", sdp).is_empty());
    }
}
