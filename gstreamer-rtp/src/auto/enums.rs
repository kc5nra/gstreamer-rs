// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;
use gst_rtp_sys;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RTCPFBType {
    FbTypeInvalid,
    RtpfbTypeNack,
    RtpfbTypeTmmbr,
    RtpfbTypeTmmbn,
    RtpfbTypeRtcpSrReq,
    RtpfbTypeTwcc,
    PsfbTypeSli,
    PsfbTypeTstn,
    PsfbTypeVbcn,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTCPFBType {
    type GlibType = gst_rtp_sys::GstRTCPFBType;

    fn to_glib(&self) -> gst_rtp_sys::GstRTCPFBType {
        match *self {
            RTCPFBType::FbTypeInvalid => gst_rtp_sys::GST_RTCP_FB_TYPE_INVALID,
            RTCPFBType::RtpfbTypeNack => gst_rtp_sys::GST_RTCP_RTPFB_TYPE_NACK,
            RTCPFBType::RtpfbTypeTmmbr => gst_rtp_sys::GST_RTCP_RTPFB_TYPE_TMMBR,
            RTCPFBType::RtpfbTypeTmmbn => gst_rtp_sys::GST_RTCP_RTPFB_TYPE_TMMBN,
            RTCPFBType::RtpfbTypeRtcpSrReq => gst_rtp_sys::GST_RTCP_RTPFB_TYPE_RTCP_SR_REQ,
            RTCPFBType::RtpfbTypeTwcc => gst_rtp_sys::GST_RTCP_RTPFB_TYPE_TWCC,
            RTCPFBType::PsfbTypeSli => gst_rtp_sys::GST_RTCP_PSFB_TYPE_SLI,
            RTCPFBType::PsfbTypeTstn => gst_rtp_sys::GST_RTCP_PSFB_TYPE_TSTN,
            RTCPFBType::PsfbTypeVbcn => gst_rtp_sys::GST_RTCP_PSFB_TYPE_VBCN,
            RTCPFBType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_rtp_sys::GstRTCPFBType> for RTCPFBType {
    fn from_glib(value: gst_rtp_sys::GstRTCPFBType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTCPFBType::FbTypeInvalid,
            1 => RTCPFBType::RtpfbTypeNack,
            3 => RTCPFBType::RtpfbTypeTmmbr,
            4 => RTCPFBType::RtpfbTypeTmmbn,
            5 => RTCPFBType::RtpfbTypeRtcpSrReq,
            15 => RTCPFBType::RtpfbTypeTwcc,
            2 => RTCPFBType::PsfbTypeSli,
            6 => RTCPFBType::PsfbTypeTstn,
            7 => RTCPFBType::PsfbTypeVbcn,
            value => RTCPFBType::__Unknown(value),
        }
    }
}

impl StaticType for RTCPFBType {
    fn static_type() -> Type {
        unsafe { from_glib(gst_rtp_sys::gst_rtcpfb_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTCPFBType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTCPFBType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RTCPFBType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RTCPSDESType {
    Invalid,
    End,
    Cname,
    Name,
    Email,
    Phone,
    Loc,
    Tool,
    Note,
    Priv,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTCPSDESType {
    type GlibType = gst_rtp_sys::GstRTCPSDESType;

    fn to_glib(&self) -> gst_rtp_sys::GstRTCPSDESType {
        match *self {
            RTCPSDESType::Invalid => gst_rtp_sys::GST_RTCP_SDES_INVALID,
            RTCPSDESType::End => gst_rtp_sys::GST_RTCP_SDES_END,
            RTCPSDESType::Cname => gst_rtp_sys::GST_RTCP_SDES_CNAME,
            RTCPSDESType::Name => gst_rtp_sys::GST_RTCP_SDES_NAME,
            RTCPSDESType::Email => gst_rtp_sys::GST_RTCP_SDES_EMAIL,
            RTCPSDESType::Phone => gst_rtp_sys::GST_RTCP_SDES_PHONE,
            RTCPSDESType::Loc => gst_rtp_sys::GST_RTCP_SDES_LOC,
            RTCPSDESType::Tool => gst_rtp_sys::GST_RTCP_SDES_TOOL,
            RTCPSDESType::Note => gst_rtp_sys::GST_RTCP_SDES_NOTE,
            RTCPSDESType::Priv => gst_rtp_sys::GST_RTCP_SDES_PRIV,
            RTCPSDESType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_rtp_sys::GstRTCPSDESType> for RTCPSDESType {
    fn from_glib(value: gst_rtp_sys::GstRTCPSDESType) -> Self {
        skip_assert_initialized!();
        match value {
            -1 => RTCPSDESType::Invalid,
            0 => RTCPSDESType::End,
            1 => RTCPSDESType::Cname,
            2 => RTCPSDESType::Name,
            3 => RTCPSDESType::Email,
            4 => RTCPSDESType::Phone,
            5 => RTCPSDESType::Loc,
            6 => RTCPSDESType::Tool,
            7 => RTCPSDESType::Note,
            8 => RTCPSDESType::Priv,
            value => RTCPSDESType::__Unknown(value),
        }
    }
}

impl StaticType for RTCPSDESType {
    fn static_type() -> Type {
        unsafe { from_glib(gst_rtp_sys::gst_rtcpsdes_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTCPSDESType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTCPSDESType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RTCPSDESType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RTCPType {
    Invalid,
    Sr,
    Rr,
    Sdes,
    Bye,
    App,
    Rtpfb,
    Psfb,
    Xr,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTCPType {
    type GlibType = gst_rtp_sys::GstRTCPType;

    fn to_glib(&self) -> gst_rtp_sys::GstRTCPType {
        match *self {
            RTCPType::Invalid => gst_rtp_sys::GST_RTCP_TYPE_INVALID,
            RTCPType::Sr => gst_rtp_sys::GST_RTCP_TYPE_SR,
            RTCPType::Rr => gst_rtp_sys::GST_RTCP_TYPE_RR,
            RTCPType::Sdes => gst_rtp_sys::GST_RTCP_TYPE_SDES,
            RTCPType::Bye => gst_rtp_sys::GST_RTCP_TYPE_BYE,
            RTCPType::App => gst_rtp_sys::GST_RTCP_TYPE_APP,
            RTCPType::Rtpfb => gst_rtp_sys::GST_RTCP_TYPE_RTPFB,
            RTCPType::Psfb => gst_rtp_sys::GST_RTCP_TYPE_PSFB,
            RTCPType::Xr => gst_rtp_sys::GST_RTCP_TYPE_XR,
            RTCPType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_rtp_sys::GstRTCPType> for RTCPType {
    fn from_glib(value: gst_rtp_sys::GstRTCPType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTCPType::Invalid,
            200 => RTCPType::Sr,
            201 => RTCPType::Rr,
            202 => RTCPType::Sdes,
            203 => RTCPType::Bye,
            204 => RTCPType::App,
            205 => RTCPType::Rtpfb,
            206 => RTCPType::Psfb,
            207 => RTCPType::Xr,
            value => RTCPType::__Unknown(value),
        }
    }
}

impl StaticType for RTCPType {
    fn static_type() -> Type {
        unsafe { from_glib(gst_rtp_sys::gst_rtcp_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTCPType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTCPType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RTCPType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RTCPXRType {
    Invalid,
    Lrle,
    Drle,
    Prt,
    Rrt,
    Dlrr,
    Ssumm,
    VoipMetrics,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for RTCPXRType {
    type GlibType = gst_rtp_sys::GstRTCPXRType;

    fn to_glib(&self) -> gst_rtp_sys::GstRTCPXRType {
        match *self {
            RTCPXRType::Invalid => gst_rtp_sys::GST_RTCP_XR_TYPE_INVALID,
            RTCPXRType::Lrle => gst_rtp_sys::GST_RTCP_XR_TYPE_LRLE,
            RTCPXRType::Drle => gst_rtp_sys::GST_RTCP_XR_TYPE_DRLE,
            RTCPXRType::Prt => gst_rtp_sys::GST_RTCP_XR_TYPE_PRT,
            RTCPXRType::Rrt => gst_rtp_sys::GST_RTCP_XR_TYPE_RRT,
            RTCPXRType::Dlrr => gst_rtp_sys::GST_RTCP_XR_TYPE_DLRR,
            RTCPXRType::Ssumm => gst_rtp_sys::GST_RTCP_XR_TYPE_SSUMM,
            RTCPXRType::VoipMetrics => gst_rtp_sys::GST_RTCP_XR_TYPE_VOIP_METRICS,
            RTCPXRType::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gst_rtp_sys::GstRTCPXRType> for RTCPXRType {
    fn from_glib(value: gst_rtp_sys::GstRTCPXRType) -> Self {
        skip_assert_initialized!();
        match value {
            -1 => RTCPXRType::Invalid,
            1 => RTCPXRType::Lrle,
            2 => RTCPXRType::Drle,
            3 => RTCPXRType::Prt,
            4 => RTCPXRType::Rrt,
            5 => RTCPXRType::Dlrr,
            6 => RTCPXRType::Ssumm,
            7 => RTCPXRType::VoipMetrics,
            value => RTCPXRType::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl StaticType for RTCPXRType {
    fn static_type() -> Type {
        unsafe { from_glib(gst_rtp_sys::gst_rtcpxr_type_get_type()) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl<'a> FromValueOptional<'a> for RTCPXRType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl<'a> FromValue<'a> for RTCPXRType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl SetValue for RTCPXRType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RTPPayload {
    Pcmu,
    _1016,
    G721,
    Gsm,
    G723,
    Dvi48000,
    Dvi416000,
    Lpc,
    Pcma,
    G722,
    L16Stereo,
    L16Mono,
    Qcelp,
    Cn,
    Mpa,
    G728,
    Dvi411025,
    Dvi422050,
    G729,
    Cellb,
    Jpeg,
    Nv,
    H261,
    Mpv,
    Mp2t,
    H263,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTPPayload {
    type GlibType = gst_rtp_sys::GstRTPPayload;

    fn to_glib(&self) -> gst_rtp_sys::GstRTPPayload {
        match *self {
            RTPPayload::Pcmu => gst_rtp_sys::GST_RTP_PAYLOAD_PCMU,
            RTPPayload::_1016 => gst_rtp_sys::GST_RTP_PAYLOAD_1016,
            RTPPayload::G721 => gst_rtp_sys::GST_RTP_PAYLOAD_G721,
            RTPPayload::Gsm => gst_rtp_sys::GST_RTP_PAYLOAD_GSM,
            RTPPayload::G723 => gst_rtp_sys::GST_RTP_PAYLOAD_G723,
            RTPPayload::Dvi48000 => gst_rtp_sys::GST_RTP_PAYLOAD_DVI4_8000,
            RTPPayload::Dvi416000 => gst_rtp_sys::GST_RTP_PAYLOAD_DVI4_16000,
            RTPPayload::Lpc => gst_rtp_sys::GST_RTP_PAYLOAD_LPC,
            RTPPayload::Pcma => gst_rtp_sys::GST_RTP_PAYLOAD_PCMA,
            RTPPayload::G722 => gst_rtp_sys::GST_RTP_PAYLOAD_G722,
            RTPPayload::L16Stereo => gst_rtp_sys::GST_RTP_PAYLOAD_L16_STEREO,
            RTPPayload::L16Mono => gst_rtp_sys::GST_RTP_PAYLOAD_L16_MONO,
            RTPPayload::Qcelp => gst_rtp_sys::GST_RTP_PAYLOAD_QCELP,
            RTPPayload::Cn => gst_rtp_sys::GST_RTP_PAYLOAD_CN,
            RTPPayload::Mpa => gst_rtp_sys::GST_RTP_PAYLOAD_MPA,
            RTPPayload::G728 => gst_rtp_sys::GST_RTP_PAYLOAD_G728,
            RTPPayload::Dvi411025 => gst_rtp_sys::GST_RTP_PAYLOAD_DVI4_11025,
            RTPPayload::Dvi422050 => gst_rtp_sys::GST_RTP_PAYLOAD_DVI4_22050,
            RTPPayload::G729 => gst_rtp_sys::GST_RTP_PAYLOAD_G729,
            RTPPayload::Cellb => gst_rtp_sys::GST_RTP_PAYLOAD_CELLB,
            RTPPayload::Jpeg => gst_rtp_sys::GST_RTP_PAYLOAD_JPEG,
            RTPPayload::Nv => gst_rtp_sys::GST_RTP_PAYLOAD_NV,
            RTPPayload::H261 => gst_rtp_sys::GST_RTP_PAYLOAD_H261,
            RTPPayload::Mpv => gst_rtp_sys::GST_RTP_PAYLOAD_MPV,
            RTPPayload::Mp2t => gst_rtp_sys::GST_RTP_PAYLOAD_MP2T,
            RTPPayload::H263 => gst_rtp_sys::GST_RTP_PAYLOAD_H263,
            RTPPayload::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_rtp_sys::GstRTPPayload> for RTPPayload {
    fn from_glib(value: gst_rtp_sys::GstRTPPayload) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTPPayload::Pcmu,
            1 => RTPPayload::_1016,
            2 => RTPPayload::G721,
            3 => RTPPayload::Gsm,
            4 => RTPPayload::G723,
            5 => RTPPayload::Dvi48000,
            6 => RTPPayload::Dvi416000,
            7 => RTPPayload::Lpc,
            8 => RTPPayload::Pcma,
            9 => RTPPayload::G722,
            10 => RTPPayload::L16Stereo,
            11 => RTPPayload::L16Mono,
            12 => RTPPayload::Qcelp,
            13 => RTPPayload::Cn,
            14 => RTPPayload::Mpa,
            15 => RTPPayload::G728,
            16 => RTPPayload::Dvi411025,
            17 => RTPPayload::Dvi422050,
            18 => RTPPayload::G729,
            25 => RTPPayload::Cellb,
            26 => RTPPayload::Jpeg,
            28 => RTPPayload::Nv,
            31 => RTPPayload::H261,
            32 => RTPPayload::Mpv,
            33 => RTPPayload::Mp2t,
            34 => RTPPayload::H263,
            value => RTPPayload::__Unknown(value),
        }
    }
}

impl StaticType for RTPPayload {
    fn static_type() -> Type {
        unsafe { from_glib(gst_rtp_sys::gst_rtp_payload_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTPPayload {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTPPayload {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RTPPayload {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RTPProfile {
    Unknown,
    Avp,
    Savp,
    Avpf,
    Savpf,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for RTPProfile {
    type GlibType = gst_rtp_sys::GstRTPProfile;

    fn to_glib(&self) -> gst_rtp_sys::GstRTPProfile {
        match *self {
            RTPProfile::Unknown => gst_rtp_sys::GST_RTP_PROFILE_UNKNOWN,
            RTPProfile::Avp => gst_rtp_sys::GST_RTP_PROFILE_AVP,
            RTPProfile::Savp => gst_rtp_sys::GST_RTP_PROFILE_SAVP,
            RTPProfile::Avpf => gst_rtp_sys::GST_RTP_PROFILE_AVPF,
            RTPProfile::Savpf => gst_rtp_sys::GST_RTP_PROFILE_SAVPF,
            RTPProfile::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<gst_rtp_sys::GstRTPProfile> for RTPProfile {
    fn from_glib(value: gst_rtp_sys::GstRTPProfile) -> Self {
        skip_assert_initialized!();
        match value {
            0 => RTPProfile::Unknown,
            1 => RTPProfile::Avp,
            2 => RTPProfile::Savp,
            3 => RTPProfile::Avpf,
            4 => RTPProfile::Savpf,
            value => RTPProfile::__Unknown(value),
        }
    }
}

impl StaticType for RTPProfile {
    fn static_type() -> Type {
        unsafe { from_glib(gst_rtp_sys::gst_rtp_profile_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTPProfile {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTPProfile {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RTPProfile {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
