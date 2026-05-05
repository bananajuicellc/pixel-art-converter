use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Document {
    pub version: u32,
    pub id: String,
    pub name: String,
    pub source: Option<String>,
    pub width: u16,
    pub height: u16,
    #[serde(rename = "Type")]
    pub doc_type: u32,
    pub clips: Vec<Clip>,
    pub background: bool,
    pub background_color: Option<Color>,
    pub tile_mode: bool,
    pub tile_fade: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Clip {
    pub id: String,
    pub name: String,
    pub frames: Vec<Frame>,
    pub layer_types: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Frame {
    pub id: String,
    pub delay: f64,
    pub layers: Vec<Layer>,
    pub layer_groups: Vec<LayerGroup>,
    pub active_layer_index: Option<u32>,
    #[serde(rename = "_reference")]
    pub reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub opacity: f64,
    pub transparency: f64,
    pub hidden: bool,
    pub linked: bool,
    pub outline: u32,
    pub lock: u32,
    pub sx: i32,
    pub sy: i32,
    pub version: u32,
    #[serde(rename = "_historyJson")]
    pub history_json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct LayerGroup {
    pub id: String,
    pub name: String,
    pub index: u32,
    pub hidden: bool,
    pub collapsed: bool,
    pub layers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct History {
    pub actions: Vec<Action>,
    pub index: u32,
    #[serde(rename = "_source")]
    pub source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    pub tool: u32,
    pub color_indexes: Option<Vec<u32>>,
    pub positions: String,
    pub colors: String,
    pub meta: Option<String>,
    pub invalid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psp_v2_deserialization() {
        let json_data = r#"{
            "Version": 2,
            "Id": "abc",
            "Name": "test",
            "Width": 32,
            "Height": 32,
            "Type": 0,
            "Clips": [
                {
                    "Id": "clip1",
                    "Name": "Clip 1",
                    "Frames": [
                        {
                            "Id": "frame1",
                            "Delay": 0.5,
                            "Layers": [
                                {
                                    "Id": "layer1",
                                    "Name": "Layer 1",
                                    "Opacity": 0.8,
                                    "Transparency": -1.0,
                                    "Hidden": false,
                                    "Linked": false,
                                    "Outline": 0,
                                    "Lock": 0,
                                    "Sx": 0,
                                    "Sy": 0,
                                    "Version": 1,
                                    "_historyJson": "{\"Actions\":[],\"Index\":0}"
                                }
                            ],
                            "LayerGroups": []
                        }
                    ],
                    "LayerTypes": [0]
                }
            ],
            "Background": false,
            "TileMode": false,
            "TileFade": 0
        }"#;

        let doc: Document = serde_json::from_str(json_data).unwrap();
        assert_eq!(doc.width, 32);
        assert_eq!(doc.clips[0].frames[0].layers[0].opacity, 0.8);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Pen = 0,
    Pipette = 1,
    Eraser = 2,
    Fill = 3,
    MoveCamera = 4,
    GenericTool = 5,
    Clear = 6,
    Copy = 7,
    Cut = 8,
    Paste = 9,
    Move = 10,
    MirrorByX = 11,
    MirrorByY = 12,
    FlipByX = 13,
    FlipByY = 14,
    RotateLeft = 15,
    RotateRight = 16,
    DotPen = 17,
    ReplaceColor = 18,
    EraserPen = 19,
    PasteImage = 20,
    RotateRect = 21,
    DitheringPen = 22,
    MagicWand = 23,
    ColorAdjustment = 24,
    Brush = 25,
    PixelSelect = 26,
    Lasso = 27,
    Cursor = 28,
    OutlineTool = 29,
}

impl TryFrom<u32> for Tool {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tool::Pen),
            1 => Ok(Tool::Pipette),
            2 => Ok(Tool::Eraser),
            3 => Ok(Tool::Fill),
            4 => Ok(Tool::MoveCamera),
            5 => Ok(Tool::GenericTool),
            6 => Ok(Tool::Clear),
            7 => Ok(Tool::Copy),
            8 => Ok(Tool::Cut),
            9 => Ok(Tool::Paste),
            10 => Ok(Tool::Move),
            11 => Ok(Tool::MirrorByX),
            12 => Ok(Tool::MirrorByY),
            13 => Ok(Tool::FlipByX),
            14 => Ok(Tool::FlipByY),
            15 => Ok(Tool::RotateLeft),
            16 => Ok(Tool::RotateRight),
            17 => Ok(Tool::DotPen),
            18 => Ok(Tool::ReplaceColor),
            19 => Ok(Tool::EraserPen),
            20 => Ok(Tool::PasteImage),
            21 => Ok(Tool::RotateRect),
            22 => Ok(Tool::DitheringPen),
            23 => Ok(Tool::MagicWand),
            24 => Ok(Tool::ColorAdjustment),
            25 => Ok(Tool::Brush),
            26 => Ok(Tool::PixelSelect),
            27 => Ok(Tool::Lasso),
            28 => Ok(Tool::Cursor),
            29 => Ok(Tool::OutlineTool),
            _ => Err(()),
        }
    }
}
