//! All requests that can be send to the API.

use either::Either;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Request {
    pub message_id: String,
    #[serde(flatten)]
    pub ty: RequestType,
}

#[derive(Serialize)]
#[serde(tag = "request-type")]
pub(crate) enum RequestType {
    // --------------------------------
    // General
    // --------------------------------
    GetVersion,
    GetAuthRequired,
    Authenticate {
        /// Response to the auth challenge.
        auth: String,
    },
    #[serde(rename_all = "kebab-case")]
    SetFilenameFormatting {
        /// Filename formatting string to set.
        filename_formatting: String,
    },
    GetFilenameFormatting,
    GetStats,
    BroadcastCustomMessage {
        /// Identifier to be choosen by the client.
        realm: String,
        /// User-defined data.
        data: serde_json::Value,
    },
    GetVideoInfo,
    OpenProjector(Projector),
    // --------------------------------
    // Sources
    // --------------------------------
    GetSourcesList,
    GetSourceTypesList,
    #[serde(rename_all = "camelCase")]
    GetVolume {
        /// Source name.
        source: String,
        /// Output volume in decibels of attenuation instead of amplitude/mul.
        use_decibel: Option<bool>,
    },
    SetVolume(Volume),
    GetMute {
        /// Source name.
        source: String,
    },
    SetMute {
        /// Source name.
        source: String,
        /// Desired mute status.
        mute: bool,
    },
    ToggleMute {
        /// Source name.
        source: String,
    },
    #[serde(rename_all = "camelCase")]
    GetAudioActive {
        /// Source name.
        source_name: String,
    },
    #[serde(rename_all = "camelCase")]
    SetSourceName {
        /// Source name.
        source_name: String,
        /// New source name.
        new_name: String,
    },
    SetSyncOffset {
        /// Source name.
        source: String,
        /// The desired audio sync offset (in nanoseconds).
        offset: i64,
    },
    GetSyncOffset {
        /// Source name.
        source: String,
    },
    #[serde(rename_all = "camelCase")]
    GetSourceSettings {
        /// Source name.
        source_name: String,
        /// Type of the specified source. Useful for type-checking if you expect a specific settings
        /// schema.
        source_type: Option<String>,
    },
    SetSourceSettings(SourceSettings),
    GetTextGDIPlusProperties {
        /// Source name.
        source: String,
    },
    SetTextGDIPlusProperties(Box<TextGdiPlusProperties>),
    GetTextFreetype2Properties {
        /// Source name.
        source: String,
    },
    SetTextFreetype2Properties(TextFreetype2Properties),
    GetSpecialSources,
    #[serde(rename_all = "camelCase")]
    GetSourceFilters {
        /// Source name.
        source_name: String,
    },
    #[serde(rename_all = "camelCase")]
    GetSourceFilterInfo {
        /// Source name.
        source_name: String,
        /// Source filter name.
        filter_name: String,
    },
    AddFilterToSource(AddFilter),
    #[serde(rename_all = "camelCase")]
    RemoveFilterFromSource {
        /// Name of the source from which the specified filter is removed.
        source_name: String,
        /// Name of the filter to remove.
        filter_name: String,
    },
    ReorderSourceFilter(ReorderFilter),
    MoveSourceFilter(MoveFilter),
    SetSourceFilterSettings(SourceFilterSettings),
    SetSourceFilterVisibility(SourceFilterVisibility),
    #[serde(rename_all = "camelCase")]
    GetAudioMonitorType {
        /// Source name.
        source_name: String,
    },
    #[serde(rename_all = "camelCase")]
    SetAudioMonitorType {
        /// Source name.
        source_name: String,
        /// The monitor type to use. Options: `none`, `monitorOnly`, `monitorAndOutput`.
        monitor_type: String,
    },
    TakeSourceScreenshot(SourceScreenshot),
    // --------------------------------
    // Outputs
    // --------------------------------
    ListOutputs,
    #[serde(rename_all = "camelCase")]
    GetOutputInfo {
        /// Output name.
        output_name: String,
    },
    #[serde(rename_all = "camelCase")]
    StartOutput {
        /// Output name.
        output_name: String,
    },
    #[serde(rename_all = "camelCase")]
    StopOutput {
        /// Output name.
        output_name: String,
        /// Force stop (default: false).
        force: Option<bool>,
    },
    // --------------------------------
    // Profiles
    // --------------------------------
    #[serde(rename_all = "kebab-case")]
    SetCurrentProfile {
        /// Name of the desired profile.
        profile_name: String,
    },
    GetCurrentProfile,
    ListProfiles,
    // --------------------------------
    // Recording
    // --------------------------------
    StartStopRecording,
    StartRecording,
    StopRecording,
    PauseRecording,
    ResumeRecording,
    #[serde(rename_all = "kebab-case")]
    SetRecordingFolder {
        /// Path of the recording folder.
        rec_folder: String,
    },
    GetRecordingFolder,
    // --------------------------------
    // Replay Buffer
    // --------------------------------
    StartStopReplayBuffer,
    StartReplayBuffer,
    StopReplayBuffer,
    SaveReplayBuffer,
    // --------------------------------
    // Scene Collections
    // --------------------------------
    #[serde(rename_all = "kebab-case")]
    SetCurrentSceneCollection {
        /// Name of the desired scene collection.
        sc_name: String,
    },
    GetCurrentSceneCollection,
    ListSceneCollections,
    // --------------------------------
    // Scene Items
    // --------------------------------
    #[serde(rename_all = "kebab-case")]
    GetSceneItemProperties {
        /// Name of the scene the scene item belongs to. Defaults to the current scene.
        scene_name: Option<String>,
        /// Scene Item name (if this field is a string) or specification (if it is an object).
        #[serde(with = "either::serde_untagged")]
        item: Either<String, SceneItemSpecification>,
    },
    SetSceneItemProperties(SceneItemProperties),
    #[serde(rename_all = "kebab-case")]
    ResetSceneItem {
        /// Name of the scene the scene item belongs to. Defaults to the current scene.
        scene_name: Option<String>,
        /// Scene Item name (if this field is a string) or specification (if it is an object).
        #[serde(with = "either::serde_untagged")]
        item: Either<String, SceneItemSpecification>,
    },
    SetSceneItemRender(SceneItemRender),
    DeleteSceneItem {
        /// Name of the scene the scene item belongs to. Defaults to the current scene.
        scene: Option<String>,
        /// Scene item to delete.
        item: SceneItemSpecification, // TODO: fields are actually not optional
    },
    AddSceneItem(AddSceneItem),
    DuplicateSceneItem(DuplicateSceneItem),
    // --------------------------------
    // Scenes
    // --------------------------------
    #[serde(rename_all = "kebab-case")]
    SetCurrentScene {
        /// Name of the scene to switch to.
        scene_name: String,
    },
    GetCurrentScene,
    GetSceneList,
    #[serde(rename_all = "camelCase")]
    CreateScene {
        /// Name of the scene to create.
        scene_name: String,
    },
    ReorderSceneItems {
        /// Name of the scene to reorder (defaults to current).
        scene: Option<String>,
        /// Ordered list of objects with name and/or id specified. Id preferred due to uniqueness
        /// per scene.
        items: Vec<Scene>,
    },
    SetSceneTransitionOverride(SceneTransitionOverride),
    #[serde(rename_all = "camelCase")]
    RemoveSceneTransitionOverride {
        /// Name of the scene to remove the override from.
        scene_name: String,
    },
    #[serde(rename_all = "camelCase")]
    GetSceneTransitionOverride {
        /// Name of the scene to get the override for.
        scene_name: String,
    },
    // --------------------------------
    // Streaming
    // --------------------------------
    GetStreamingStatus,
    StartStopStreaming,
    StartStreaming {
        /// Special stream configuration. Please note: these won't be saved to OBS' configuration.
        stream: Option<Stream>,
    },
    StopStreaming,
    SetStreamSettings(SetStreamSettings),
    GetStreamSettings,
    SaveStreamSettings,
    SendCaptions {
        /// Captions text.
        text: String,
    },
    // --------------------------------
    // Studio Mode
    // --------------------------------
    GetStudioModeStatus,
    GetPreviewScene,
    #[serde(rename_all = "kebab-case")]
    SetPreviewScene {
        /// The name of the scene to preview.
        scene_name: String,
    },
    TransitionToProgram {
        /// Change the active transition before switching scenes. Defaults to the active transition.
        with_transition: Option<Transition>,
    },
    EnableStudioMode,
    DisableStudioMode,
    ToggleStudioMode,
    // --------------------------------
    // Transitions
    // --------------------------------
    GetTransitionList,
    GetCurrentTransition,
    #[serde(rename_all = "kebab-case")]
    SetCurrentTransition {
        /// The name of the transition.
        transition_name: String,
    },
    SetTransitionDuration {
        /// Desired duration of the transition (in milliseconds).
        duration: u64,
    },
    GetTransitionDuration,
}

/// Request information for [`open_projector`](crate::client::General::open_projector).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Projector {
    /// Type of projector: `Preview` (default), `Source`, `Scene`, `StudioProgram`, or `Multiview`
    /// (case insensitive).
    #[serde(rename = "type")]
    pub ty: Option<String>,
    /// Monitor to open the projector on. If -1 or omitted, opens a window.
    pub monitor: Option<i64>,
    /// Size and position of the projector window (only if monitor is -1). Encoded in Base64 using
    /// [Qt's geometry encoding](https://doc.qt.io/qt-5/qwidget.html#saveGeometry). Corresponds to
    /// OBS's saved projectors.
    pub geometry: Option<String>,
    /// Name of the source or scene to be displayed (ignored for other projector types).
    pub name: Option<String>,
}

/// Request information for [`set_volume`](crate::client::Sources::set_volume).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    /// Source name.
    pub source: String,
    /// Desired volume. Must be between `0.0` and `20.0` for mul, and under 26.0 for dB. OBS will
    /// interpret dB values under -100.0 as Inf. Note: The OBS volume sliders only reach a maximum
    /// of 1.0mul/0.0dB, however OBS actually supports larger values.
    pub volume: f64,
    /// Interperet `volume` data as decibels instead of amplitude/mul.
    pub use_decibel: Option<bool>,
}

/// Request information for [`set_source_settings`](crate::client::Sources::set_source_settings).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceSettings {
    /// Source name.
    pub source_name: String,
    /// Type of the specified source. Useful for type-checking to avoid settings a set of settings
    /// incompatible with the actual source's type.
    pub source_type: Option<String>,
    /// Source settings (varies between source types, may require some probing around).
    pub source_settings: serde_json::Value,
}

/// Request information for
/// [`set_text_gdi_plus_properties`](crate::client::Sources::set_text_gdi_plus_properties).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct TextGdiPlusProperties {
    /// Name of the source.
    pub source: String,
    /// Text Alignment ("left", "center", "right").
    pub align: Option<String>,
    /// Background color.
    pub bk_color: Option<u32>,
    /// Background opacity (0-100).
    pub bk_opacity: Option<u8>,
    /// Chat log.
    pub chatlog: Option<bool>,
    /// Chat log lines.
    pub chatlog_lines: Option<u64>,
    /// Text color.
    pub color: Option<u32>,
    /// Extents wrap.
    pub extents: Option<bool>,
    /// Extents cx.
    pub extents_cx: Option<i64>,
    /// Extents cy.
    pub extents_cy: Option<i64>,
    /// File path name.
    pub file: Option<String>,
    /// Read text from the specified file.
    pub read_from_file: Option<bool>,
    /// Holds data for the font. Ex:
    /// `"font": { "face": "Arial", "flags": 0, "size": 150, "style": "" }`.
    pub font: Option<Font>,
    /// Gradient enabled.
    pub gradient: Option<bool>,
    /// Gradient color.
    pub gradient_color: Option<u32>,
    /// Gradient direction.
    pub gradient_dir: Option<f32>,
    /// Gradient opacity (0-100).
    pub gradient_opacity: Option<u8>,
    /// Outline.
    pub outline: Option<bool>,
    /// Outline color.
    pub outline_color: Option<u32>,
    /// Outline size.
    pub outline_size: Option<u64>,
    /// Outline opacity (0-100).
    pub outline_opacity: Option<u8>,
    /// Text content to be displayed.
    pub text: Option<String>,
    /// Text vertical alignment ("top", "center", "bottom").
    pub valign: Option<String>,
    /// Vertical text enabled.
    pub vertical: Option<bool>,
    /// Visibility of the scene item.
    pub render: Option<bool>,
}

/// Request information for
/// [`set_text_freetype2_properties`](crate::client::Sources::set_text_freetype2_properties).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct TextFreetype2Properties {
    /// Source name.
    pub source: String,
    /// Gradient top color.
    pub color1: Option<u32>,
    /// Gradient bottom color.
    pub color2: Option<u32>,
    /// Custom width (0 to disable).
    pub custom_width: Option<u32>,
    /// Drop shadow.
    pub drop_shadow: Option<bool>,
    /// Holds data for the font. Ex:
    /// `"font": { "face": "Arial", "flags": 0, "size": 150, "style": "" }`.
    pub font: Option<Font>,
    /// Read text from the specified file.
    pub from_file: Option<bool>,
    /// Chat log.
    pub log_mode: Option<bool>,
    /// Outline.
    pub outline: Option<bool>,
    /// Text content to be displayed.
    pub text: Option<String>,
    /// File path.
    pub text_file: Option<String>,
    /// Word wrap.
    pub word_wrap: Option<bool>,
}

/// Request information for [`add_filter_to_source`](crate::client::Sources::add_filter_to_source).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFilter {
    /// Name of the source on which the filter is added.
    pub source_name: String,
    /// Name of the new filter.
    pub filter_name: String,
    /// Filter type.
    pub filter_type: String,
    /// Filter settings.
    pub filter_settings: serde_json::Value,
}

/// Request information for
/// [`reorder_source_filter`](crate::client::Sources::reorder_source_filter).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderFilter {
    /// Name of the source to which the filter belongs.
    pub source_name: String,
    /// Name of the filter to reorder.
    pub filter_name: String,
    /// Desired position of the filter in the chain.
    pub new_index: u32,
}

/// Request information for [`move_source_filter`](crate::client::Sources::move_source_filter).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveFilter {
    /// Name of the source to which the filter belongs.
    pub source_name: String,
    /// Name of the filter to reorder.
    pub filter_name: String,
    /// How to move the filter around in the source's filter chain. Either "up", "down", "top" or
    /// "bottom".
    pub movement_type: String,
}

/// Request information for
/// [`set_source_filter_settings`](crate::client::Sources::set_source_filter_settings).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFilterSettings {
    /// Name of the source to which the filter belongs.
    pub source_name: String,
    /// Name of the filter to reconfigure.
    pub filter_name: String,
    /// New settings. These will be merged to the current filter settings.
    pub filter_settings: serde_json::Value,
}

/// Request information for
/// [`set_source_filter_visibility`](crate::client::Sources::set_source_filter_visibility).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFilterVisibility {
    /// Source name.
    pub source_name: String,
    /// Source filter name.
    pub filter_name: String,
    /// New filter state.
    pub filter_enabled: bool,
}

/// Request information for
/// [`take_source_screenshot`](crate::client::Sources::take_source_screenshot).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceScreenshot {
    /// Source name. Note that, since scenes are also sources, you can also provide a scene name. If
    /// not provided, the currently active scene is used.
    pub source_name: Option<String>,
    /// Format of the Data URI encoded picture. Can be "png", "jpg", "jpeg" or "bmp" (or any other
    /// value supported by Qt's Image module).
    pub embed_picture_format: Option<String>,
    /// Full file path (file extension included) where the captured image is to be saved. Can be in
    /// a format different from [`embed_picture_format`](SourceScreenshot::embed_picture_format).
    /// Can be a relative path.
    pub save_to_file_path: Option<String>,
    /// Format to save the image file as (one of the values provided in the
    /// [`supported_image_export_formats`](crate::responses::Version::supported_image_export_formats)
    /// response field of [`get_version`](crate::client::General::get_version)). If not specified,
    /// tries to guess based on file extension.
    pub file_format: Option<String>,
    /// Compression ratio between -1 and 100 to write the image with. -1 is automatic, 1 is smallest
    /// file/most compression, 100 is largest file/least compression. Varies with image type.
    pub compress_quality: Option<i8>,
    /// Screenshot width. Defaults to the source's base width.
    pub width: Option<u32>,
    /// Screenshot height. Defaults to the source's base height.
    pub height: Option<u32>,
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SceneItemProperties {
    /// Name of the scene the source item belongs to. Defaults to the current scene.
    pub scene_name: Option<String>,
    /// Scene Item name (if this field is a string) or specification (if it is an object).
    #[serde(with = "either::serde_untagged")]
    pub item: Either<String, SceneItemSpecification>,
    /// Position of the scene item.
    pub position: Option<Position>,
    /// The new clockwise rotation of the item in degrees.
    pub rotation: Option<f64>,
    /// Scaling factor of the scene item.
    pub scale: Option<Scale>,
    /// Pixel cropping of the scene item before scaling.
    pub crop: Option<Crop>,
    /// The new visibility of the source. 'true' shows source, 'false' hides source.
    pub visible: Option<bool>,
    /// The new locked status of the source. 'true' keeps it in its current position, 'false' allows
    /// movement.
    pub locked: Option<bool>,
    /// Bounding box of the scene item.
    pub bounds: Option<Bounds>,
}

/// Request information for
/// [`set_scene_item_render`](crate::client::SceneItems::set_scene_item_render).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SceneItemRender {
    /// Name of the scene the scene item belongs to. Defaults to the currently active scene.
    pub scene_name: Option<String>,
    /// Scene Item name.
    pub source: String,
    /// true = shown ; false = hidden.
    pub render: bool,
}

/// Request information for [`add_scene_item`](crate::client::SceneItems::add_scene_item).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSceneItem {
    /// Name of the scene to create the scene item in.
    pub scene_name: String,
    /// Name of the source to be added.
    pub source_name: String,
    /// Whether to make the sceneitem visible on creation or not. Default `true`.
    pub set_visible: bool,
}

/// Request information for
/// [`duplicate_scene_item`](crate::client::SceneItems::duplicate_scene_item).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateSceneItem {
    /// Name of the scene to copy the item from. Defaults to the current scene.
    pub from_scene: Option<String>,
    /// Name of the scene to create the item in. Defaults to the current scene.
    pub to_scene: Option<String>,
    /// Scene Item to duplicate from the source scene.
    pub item: SceneItemSpecification, // TODO: fields are actually not optional
}

/// Request information for
/// [`set_scene_transition_override`](crate::client::Scenes::set_scene_transition_override).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneTransitionOverride {
    /// Name of the scene to switch to.
    pub scene_name: String,
    /// Name of the transition to use.
    pub transition_name: String,
    /// Duration in milliseconds of the transition if transition is not fixed. Defaults to the
    /// current duration specified in the UI if there is no current override and this value is not
    /// given.
    pub transition_duration: Option<i64>,
}

/// Request information for [`set_stream_settings`](crate::client::Streaming::set_stream_settings).
#[derive(Debug, Serialize)]
pub struct SetStreamSettings {
    /// The type of streaming service configuration, usually `rtmp_custom` or `rtmp_common`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The actual settings of the stream.
    pub settings: StreamSettings,
    /// Persist the settings to disk.
    pub save: bool,
}

// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------

/// Request information for
/// [`set_text_gdi_plus_properties`](crate::client::Sources::set_text_gdi_plus_properties) as part
/// of [`TextGdiPlusProperties`] and
/// [`set_text_freetype2_properties`](crate::client::Sources::set_text_freetype2_properties) as part
/// of [`TextFreetype2Properties`].
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Font {
    /// Font face.
    pub face: Option<String>,
    /// Font text styling flag. `Bold=1, Italic=2, Bold Italic=3, Underline=5, Strikeout=8`.
    pub flags: Option<u8>,
    /// Font text size.
    pub size: Option<u32>,
    /// Font Style (unknown function).
    pub style: Option<String>,
}

/// Request information for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties),
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`], [`reset_scene_item`](crate::client::SceneItems::reset_scene_item),
/// [`delete_scene_item`](crate::client::SceneItems::delete_scene_item) and
/// [`duplicate_scene_item`](crate::client::SceneItems::duplicate_scene_item) as part of
/// [`DuplicateSceneItem`].
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct SceneItemSpecification {
    /// Scene Item name.
    pub name: Option<String>,
    /// Scene Item ID.
    pub id: Option<i64>,
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`].
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Position {
    /// The new x position of the source.
    pub x: Option<f64>,
    /// The new y position of the source.
    pub y: Option<f64>,
    /// The new alignment of the source.
    pub alignment: Option<u8>,
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`].
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Scale {
    /// The new x scale of the item.
    pub x: Option<f64>,
    /// The new y scale of the item.
    pub y: Option<f64>,
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`].
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Crop {
    /// The new amount of pixels cropped off the top of the source before scaling.
    pub top: Option<i64>,
    /// The new amount of pixels cropped off the bottom of the source before scaling.
    pub bottom: Option<i64>,
    /// The new amount of pixels cropped off the left of the source before scaling.
    pub left: Option<i64>,
    /// The new amount of pixels cropped off the right of the source before scaling.
    pub right: Option<i64>,
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`].
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Bounds {
    /// The new bounds type of the source. Can be "OBS_BOUNDS_STRETCH", "OBS_BOUNDS_SCALE_INNER",
    /// "OBS_BOUNDS_SCALE_OUTER", "OBS_BOUNDS_SCALE_TO_WIDTH", "OBS_BOUNDS_SCALE_TO_HEIGHT",
    /// "OBS_BOUNDS_MAX_ONLY" or "OBS_BOUNDS_NONE".
    #[serde(rename = "type")]
    pub ty: Option<String>,
    /// The new alignment of the bounding box. (0-2, 4-6, 8-10).
    pub alignment: Option<u8>,
    /// The new width of the bounding box.
    pub x: Option<f64>,
    /// The new height of the bounding box.
    pub y: Option<f64>,
}

/// Request information for
/// [`reorder_scene_items`](crate::client::Scenes::reorder_scene_items) as part of
/// [`ReorderLineItems`](RequestType::ReorderSceneItems).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Scene {
    /// Id of a specific scene item. Unique on a scene by scene basis.
    id: Option<i64>,
    /// Name of a scene item. Sufficiently unique if no scene items share sources within the scene.
    name: Option<String>,
}

/// Request information for [`start_streaming`](crate::client::Streaming::start_streaming).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Stream {
    /// If specified ensures the type of stream matches the given type (usually 'rtmp_custom' or
    /// 'rtmp_common'). If the currently configured stream type does not match the given stream
    /// type, all settings must be specified in the `settings` object or an error will occur when
    /// starting the stream.
    #[serde(rename = "type")]
    ty: Option<String>,
    /// Adds the given object parameters as encoded query string parameters to the 'key' of the RTMP
    /// stream. Used to pass data to the RTMP service about the streaming. May be any String,
    /// Numeric, or Boolean field.
    metadata: Option<serde_json::Value>,
    /// Settings for the stream.
    settings: Option<StreamSettings>,
}

/// Request information for [`start_streaming`](crate::client::Streaming::start_streaming) as part
/// of [`Stream`] and [`set_stream_settings`](crate::client::Streaming::set_stream_settings) as part
/// of [`SetStreamSettings`].
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct StreamSettings {
    /// The publish URL.
    server: Option<String>,
    /// The publish key of the stream.
    key: Option<String>,
    /// Indicates whether authentication should be used when connecting to the streaming server.
    use_auth: Option<bool>,
    /// If authentication is enabled, the username for the streaming server. Ignored if
    /// [`use_auth`](Self::use_auth) is not set to `true`.
    username: Option<String>,
    /// If authentication is enabled, the password for the streaming server. Ignored if
    /// [`use_auth`](Self::use_auth) is not set to `true`.
    password: Option<String>,
}

/// Request information for
/// [`transition_to_program`](crate::client::StudioMode::transition_to_program).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Transition {
    /// Name of the transition.
    name: String,
    /// Transition duration (in milliseconds).
    duration: Option<u64>,
}
