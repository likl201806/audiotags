// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

#![allow(
    non_camel_case_types,
    unused,
    non_snake_case,
    clippy::needless_return,
    clippy::redundant_closure_call,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::unused_unit,
    clippy::double_parens,
    clippy::let_and_return,
    clippy::too_many_arguments,
    clippy::match_single_binding,
    clippy::clone_on_copy,
    clippy::let_unit_value,
    clippy::deref_addrof,
    clippy::explicit_auto_deref,
    clippy::borrow_deref_ref,
    clippy::needless_borrow
)]

// Section: imports

use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::{transform_result_dco, Lifetimeable, Lockable};
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate!(
    default_stream_sink_codec = SseCodec,
    default_rust_opaque = RustOpaqueMoi,
    default_rust_auto_opaque = RustAutoOpaqueMoi,
);
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.11.1";
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = -1546393296;

// Section: executor

flutter_rust_bridge::frb_generated_default_handler!();

// Section: wire_funcs

fn wire__crate__api__picture__picture_new_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "picture_new",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_picture_type =
                <crate::api::picture::PictureType>::sse_decode(&mut deserializer);
            let api_mime_type =
                <Option<crate::api::picture::MimeType>>::sse_decode(&mut deserializer);
            let api_bytes = <Vec<u8>>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Result::<_, ()>::Ok(crate::api::picture::Picture::new(
                        api_picture_type,
                        api_mime_type,
                        api_bytes,
                    ))?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__api__read_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "read",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_path = <String>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, crate::api::error::AudioTagsError>((move || {
                    let output_ok = crate::api::api::read(api_path)?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__tag__tag_default_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "tag_default",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Result::<_, ()>::Ok(crate::api::tag::Tag::default())?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__tag__tag_is_empty_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "tag_is_empty",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_that = <crate::api::tag::Tag>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, ()>((move || {
                    let output_ok = Result::<_, ()>::Ok(crate::api::tag::Tag::is_empty(&api_that))?;
                    Ok(output_ok)
                })())
            }
        },
    )
}
fn wire__crate__api__api__write_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "write",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let message = unsafe {
                flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let mut deserializer =
                flutter_rust_bridge::for_generated::SseDeserializer::new(message);
            let api_path = <String>::sse_decode(&mut deserializer);
            let api_data = <crate::api::tag::Tag>::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse::<_, crate::api::error::AudioTagsError>((move || {
                    let output_ok = crate::api::api::write(api_path, api_data)?;
                    Ok(output_ok)
                })())
            }
        },
    )
}

// Section: dart2rust

impl SseDecode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return String::from_utf8(inner).unwrap();
    }
}

impl SseDecode for crate::api::error::AudioTagsError {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut tag_ = <i32>::sse_decode(deserializer);
        match tag_ {
            0 => {
                return crate::api::error::AudioTagsError::InvalidPath;
            }
            1 => {
                return crate::api::error::AudioTagsError::NoTags;
            }
            2 => {
                let mut var_message = <String>::sse_decode(deserializer);
                return crate::api::error::AudioTagsError::OpenFile {
                    message: var_message,
                };
            }
            3 => {
                let mut var_message = <String>::sse_decode(deserializer);
                return crate::api::error::AudioTagsError::Write {
                    message: var_message,
                };
            }
            _ => {
                unimplemented!("");
            }
        }
    }
}

impl SseDecode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap() != 0
    }
}

impl SseDecode for f32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_f32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for Vec<crate::api::picture::Picture> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<crate::api::picture::Picture>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<u8>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for crate::api::picture::MimeType {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => crate::api::picture::MimeType::Png,
            1 => crate::api::picture::MimeType::Jpeg,
            2 => crate::api::picture::MimeType::Tiff,
            3 => crate::api::picture::MimeType::Bmp,
            4 => crate::api::picture::MimeType::Gif,
            _ => unreachable!("Invalid variant for MimeType: {}", inner),
        };
    }
}

impl SseDecode for Option<String> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<String>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<f32> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<f32>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<crate::api::picture::MimeType> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<crate::api::picture::MimeType>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for Option<u32> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        if (<bool>::sse_decode(deserializer)) {
            return Some(<u32>::sse_decode(deserializer));
        } else {
            return None;
        }
    }
}

impl SseDecode for crate::api::picture::Picture {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_pictureType = <crate::api::picture::PictureType>::sse_decode(deserializer);
        let mut var_mimeType = <Option<crate::api::picture::MimeType>>::sse_decode(deserializer);
        let mut var_bytes = <Vec<u8>>::sse_decode(deserializer);
        return crate::api::picture::Picture {
            picture_type: var_pictureType,
            mime_type: var_mimeType,
            bytes: var_bytes,
        };
    }
}

impl SseDecode for crate::api::picture::PictureType {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => crate::api::picture::PictureType::Other,
            1 => crate::api::picture::PictureType::Icon,
            2 => crate::api::picture::PictureType::OtherIcon,
            3 => crate::api::picture::PictureType::CoverFront,
            4 => crate::api::picture::PictureType::CoverBack,
            5 => crate::api::picture::PictureType::Leaflet,
            6 => crate::api::picture::PictureType::Media,
            7 => crate::api::picture::PictureType::LeadArtist,
            8 => crate::api::picture::PictureType::Artist,
            9 => crate::api::picture::PictureType::Conductor,
            10 => crate::api::picture::PictureType::Band,
            11 => crate::api::picture::PictureType::Composer,
            12 => crate::api::picture::PictureType::Lyricist,
            13 => crate::api::picture::PictureType::RecordingLocation,
            14 => crate::api::picture::PictureType::DuringRecording,
            15 => crate::api::picture::PictureType::DuringPerformance,
            16 => crate::api::picture::PictureType::ScreenCapture,
            17 => crate::api::picture::PictureType::BrightFish,
            18 => crate::api::picture::PictureType::Illustration,
            19 => crate::api::picture::PictureType::BandLogo,
            20 => crate::api::picture::PictureType::PublisherLogo,
            _ => unreachable!("Invalid variant for PictureType: {}", inner),
        };
    }
}

impl SseDecode for crate::api::tag::Tag {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_title = <Option<String>>::sse_decode(deserializer);
        let mut var_trackArtist = <Option<String>>::sse_decode(deserializer);
        let mut var_album = <Option<String>>::sse_decode(deserializer);
        let mut var_albumArtist = <Option<String>>::sse_decode(deserializer);
        let mut var_year = <Option<u32>>::sse_decode(deserializer);
        let mut var_genre = <Option<String>>::sse_decode(deserializer);
        let mut var_trackNumber = <Option<u32>>::sse_decode(deserializer);
        let mut var_trackTotal = <Option<u32>>::sse_decode(deserializer);
        let mut var_discNumber = <Option<u32>>::sse_decode(deserializer);
        let mut var_discTotal = <Option<u32>>::sse_decode(deserializer);
        let mut var_lyrics = <Option<String>>::sse_decode(deserializer);
        let mut var_duration = <Option<u32>>::sse_decode(deserializer);
        let mut var_pictures = <Vec<crate::api::picture::Picture>>::sse_decode(deserializer);
        let mut var_bpm = <Option<f32>>::sse_decode(deserializer);
        return crate::api::tag::Tag {
            title: var_title,
            track_artist: var_trackArtist,
            album: var_album,
            album_artist: var_albumArtist,
            year: var_year,
            genre: var_genre,
            track_number: var_trackNumber,
            track_total: var_trackTotal,
            disc_number: var_discNumber,
            disc_total: var_discTotal,
            lyrics: var_lyrics,
            duration: var_duration,
            pictures: var_pictures,
            bpm: var_bpm,
        };
    }
}

impl SseDecode for u32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap()
    }
}

impl SseDecode for () {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {}
}

fn pde_ffi_dispatcher_primary_impl(
    func_id: i32,
    port: flutter_rust_bridge::for_generated::MessagePort,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        1 => wire__crate__api__picture__picture_new_impl(port, ptr, rust_vec_len, data_len),
        2 => wire__crate__api__api__read_impl(port, ptr, rust_vec_len, data_len),
        3 => wire__crate__api__tag__tag_default_impl(port, ptr, rust_vec_len, data_len),
        4 => wire__crate__api__tag__tag_is_empty_impl(port, ptr, rust_vec_len, data_len),
        5 => wire__crate__api__api__write_impl(port, ptr, rust_vec_len, data_len),
        _ => unreachable!(),
    }
}

fn pde_ffi_dispatcher_sync_impl(
    func_id: i32,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        _ => unreachable!(),
    }
}

// Section: rust2dart

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::error::AudioTagsError {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self {
            crate::api::error::AudioTagsError::InvalidPath => [0.into_dart()].into_dart(),
            crate::api::error::AudioTagsError::NoTags => [1.into_dart()].into_dart(),
            crate::api::error::AudioTagsError::OpenFile { message } => {
                [2.into_dart(), message.into_into_dart().into_dart()].into_dart()
            }
            crate::api::error::AudioTagsError::Write { message } => {
                [3.into_dart(), message.into_into_dart().into_dart()].into_dart()
            }
            _ => {
                unimplemented!("");
            }
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::api::error::AudioTagsError
{
}
impl flutter_rust_bridge::IntoIntoDart<crate::api::error::AudioTagsError>
    for crate::api::error::AudioTagsError
{
    fn into_into_dart(self) -> crate::api::error::AudioTagsError {
        self
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::picture::MimeType {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self {
            Self::Png => 0.into_dart(),
            Self::Jpeg => 1.into_dart(),
            Self::Tiff => 2.into_dart(),
            Self::Bmp => 3.into_dart(),
            Self::Gif => 4.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for crate::api::picture::MimeType {}
impl flutter_rust_bridge::IntoIntoDart<crate::api::picture::MimeType>
    for crate::api::picture::MimeType
{
    fn into_into_dart(self) -> crate::api::picture::MimeType {
        self
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::picture::Picture {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.picture_type.into_into_dart().into_dart(),
            self.mime_type.into_into_dart().into_dart(),
            self.bytes.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for crate::api::picture::Picture {}
impl flutter_rust_bridge::IntoIntoDart<crate::api::picture::Picture>
    for crate::api::picture::Picture
{
    fn into_into_dart(self) -> crate::api::picture::Picture {
        self
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::picture::PictureType {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self {
            Self::Other => 0.into_dart(),
            Self::Icon => 1.into_dart(),
            Self::OtherIcon => 2.into_dart(),
            Self::CoverFront => 3.into_dart(),
            Self::CoverBack => 4.into_dart(),
            Self::Leaflet => 5.into_dart(),
            Self::Media => 6.into_dart(),
            Self::LeadArtist => 7.into_dart(),
            Self::Artist => 8.into_dart(),
            Self::Conductor => 9.into_dart(),
            Self::Band => 10.into_dart(),
            Self::Composer => 11.into_dart(),
            Self::Lyricist => 12.into_dart(),
            Self::RecordingLocation => 13.into_dart(),
            Self::DuringRecording => 14.into_dart(),
            Self::DuringPerformance => 15.into_dart(),
            Self::ScreenCapture => 16.into_dart(),
            Self::BrightFish => 17.into_dart(),
            Self::Illustration => 18.into_dart(),
            Self::BandLogo => 19.into_dart(),
            Self::PublisherLogo => 20.into_dart(),
            _ => unreachable!(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for crate::api::picture::PictureType
{
}
impl flutter_rust_bridge::IntoIntoDart<crate::api::picture::PictureType>
    for crate::api::picture::PictureType
{
    fn into_into_dart(self) -> crate::api::picture::PictureType {
        self
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for crate::api::tag::Tag {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.title.into_into_dart().into_dart(),
            self.track_artist.into_into_dart().into_dart(),
            self.album.into_into_dart().into_dart(),
            self.album_artist.into_into_dart().into_dart(),
            self.year.into_into_dart().into_dart(),
            self.genre.into_into_dart().into_dart(),
            self.track_number.into_into_dart().into_dart(),
            self.track_total.into_into_dart().into_dart(),
            self.disc_number.into_into_dart().into_dart(),
            self.disc_total.into_into_dart().into_dart(),
            self.lyrics.into_into_dart().into_dart(),
            self.duration.into_into_dart().into_dart(),
            self.pictures.into_into_dart().into_dart(),
            self.bpm.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for crate::api::tag::Tag {}
impl flutter_rust_bridge::IntoIntoDart<crate::api::tag::Tag> for crate::api::tag::Tag {
    fn into_into_dart(self) -> crate::api::tag::Tag {
        self
    }
}

impl SseEncode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
    }
}

impl SseEncode for crate::api::error::AudioTagsError {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        match self {
            crate::api::error::AudioTagsError::InvalidPath => {
                <i32>::sse_encode(0, serializer);
            }
            crate::api::error::AudioTagsError::NoTags => {
                <i32>::sse_encode(1, serializer);
            }
            crate::api::error::AudioTagsError::OpenFile { message } => {
                <i32>::sse_encode(2, serializer);
                <String>::sse_encode(message, serializer);
            }
            crate::api::error::AudioTagsError::Write { message } => {
                <i32>::sse_encode(3, serializer);
                <String>::sse_encode(message, serializer);
            }
            _ => {
                unimplemented!("");
            }
        }
    }
}

impl SseEncode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self as _).unwrap();
    }
}

impl SseEncode for f32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_f32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for Vec<crate::api::picture::Picture> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <crate::api::picture::Picture>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <u8>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for crate::api::picture::MimeType {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                crate::api::picture::MimeType::Png => 0,
                crate::api::picture::MimeType::Jpeg => 1,
                crate::api::picture::MimeType::Tiff => 2,
                crate::api::picture::MimeType::Bmp => 3,
                crate::api::picture::MimeType::Gif => 4,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for Option<String> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <String>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<f32> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <f32>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<crate::api::picture::MimeType> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <crate::api::picture::MimeType>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for Option<u32> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <bool>::sse_encode(self.is_some(), serializer);
        if let Some(value) = self {
            <u32>::sse_encode(value, serializer);
        }
    }
}

impl SseEncode for crate::api::picture::Picture {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <crate::api::picture::PictureType>::sse_encode(self.picture_type, serializer);
        <Option<crate::api::picture::MimeType>>::sse_encode(self.mime_type, serializer);
        <Vec<u8>>::sse_encode(self.bytes, serializer);
    }
}

impl SseEncode for crate::api::picture::PictureType {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                crate::api::picture::PictureType::Other => 0,
                crate::api::picture::PictureType::Icon => 1,
                crate::api::picture::PictureType::OtherIcon => 2,
                crate::api::picture::PictureType::CoverFront => 3,
                crate::api::picture::PictureType::CoverBack => 4,
                crate::api::picture::PictureType::Leaflet => 5,
                crate::api::picture::PictureType::Media => 6,
                crate::api::picture::PictureType::LeadArtist => 7,
                crate::api::picture::PictureType::Artist => 8,
                crate::api::picture::PictureType::Conductor => 9,
                crate::api::picture::PictureType::Band => 10,
                crate::api::picture::PictureType::Composer => 11,
                crate::api::picture::PictureType::Lyricist => 12,
                crate::api::picture::PictureType::RecordingLocation => 13,
                crate::api::picture::PictureType::DuringRecording => 14,
                crate::api::picture::PictureType::DuringPerformance => 15,
                crate::api::picture::PictureType::ScreenCapture => 16,
                crate::api::picture::PictureType::BrightFish => 17,
                crate::api::picture::PictureType::Illustration => 18,
                crate::api::picture::PictureType::BandLogo => 19,
                crate::api::picture::PictureType::PublisherLogo => 20,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for crate::api::tag::Tag {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Option<String>>::sse_encode(self.title, serializer);
        <Option<String>>::sse_encode(self.track_artist, serializer);
        <Option<String>>::sse_encode(self.album, serializer);
        <Option<String>>::sse_encode(self.album_artist, serializer);
        <Option<u32>>::sse_encode(self.year, serializer);
        <Option<String>>::sse_encode(self.genre, serializer);
        <Option<u32>>::sse_encode(self.track_number, serializer);
        <Option<u32>>::sse_encode(self.track_total, serializer);
        <Option<u32>>::sse_encode(self.disc_number, serializer);
        <Option<u32>>::sse_encode(self.disc_total, serializer);
        <Option<String>>::sse_encode(self.lyrics, serializer);
        <Option<u32>>::sse_encode(self.duration, serializer);
        <Vec<crate::api::picture::Picture>>::sse_encode(self.pictures, serializer);
        <Option<f32>>::sse_encode(self.bpm, serializer);
    }
}

impl SseEncode for u32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self).unwrap();
    }
}

impl SseEncode for () {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {}
}

#[cfg(not(target_family = "wasm"))]
mod io {
    // This file is automatically generated, so please do not edit it.
    // @generated by `flutter_rust_bridge`@ 2.11.1.

    // Section: imports

    use super::*;
    use flutter_rust_bridge::for_generated::byteorder::{
        NativeEndian, ReadBytesExt, WriteBytesExt,
    };
    use flutter_rust_bridge::for_generated::{transform_result_dco, Lifetimeable, Lockable};
    use flutter_rust_bridge::{Handler, IntoIntoDart};

    // Section: boilerplate

    flutter_rust_bridge::frb_generated_boilerplate_io!();
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
mod web {
    // This file is automatically generated, so please do not edit it.
    // @generated by `flutter_rust_bridge`@ 2.11.1.

    // Section: imports

    use super::*;
    use flutter_rust_bridge::for_generated::byteorder::{
        NativeEndian, ReadBytesExt, WriteBytesExt,
    };
    use flutter_rust_bridge::for_generated::wasm_bindgen;
    use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
    use flutter_rust_bridge::for_generated::{transform_result_dco, Lifetimeable, Lockable};
    use flutter_rust_bridge::{Handler, IntoIntoDart};

    // Section: boilerplate

    flutter_rust_bridge::frb_generated_boilerplate_web!();
}
#[cfg(target_family = "wasm")]
pub use web::*;
