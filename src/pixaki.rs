use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub sprites: Vec<Sprite>,
}

#[derive(Debug, Deserialize)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    pub layers: Vec<Layer>,
    pub cels: Vec<Cel>,
    pub size: Size,
    pub duration: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub name: String,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    pub opacity: f64,
    #[serde(rename = "blendMode", default)]
    pub blend_mode: Option<String>,
    pub clips: Vec<Clip>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clip {
    pub item_identifier: String,
    pub range: Option<Range>,
}

#[derive(Debug, Deserialize)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cel {
    pub identifier: String,
    pub frame: Vec<Vec<f64>>,
}

// --- Older Format (v2) ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentV2 {
    pub size: SizeV2,
    pub symbols: Vec<SymbolV2>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SizeV2 {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SymbolV2 {
    pub name: String,
    pub frames: Vec<FrameV2>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FrameV2 {
    pub duration: u32,
    pub layers: Vec<LayerV2>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LayerV2 {
    pub alpha: f64,
    pub image_filename: String,
    pub visible: bool,
}
