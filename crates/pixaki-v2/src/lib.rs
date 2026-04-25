use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Document {
    pub size: Size,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Symbol {
    pub name: String,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Frame {
    pub duration: u32,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Layer {
    pub alpha: f64,
    pub image_filename: String,
    pub visible: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2_deserialization() {
        let plist_data = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                <key>Size</key>
                <dict>
                    <key>Width</key>
                    <real>64</real>
                    <key>Height</key>
                    <real>64</real>
                </dict>
                <key>Symbols</key>
                <array>
                    <dict>
                        <key>Name</key>
                        <string>Symbol 1</string>
                        <key>Frames</key>
                        <array>
                            <dict>
                                <key>Duration</key>
                                <integer>100</integer>
                                <key>Layers</key>
                                <array>
                                    <dict>
                                        <key>Alpha</key>
                                        <real>1</real>
                                        <key>ImageFilename</key>
                                        <string>layer1.png</string>
                                        <key>Visible</key>
                                        <true/>
                                    </dict>
                                </array>
                            </dict>
                        </array>
                    </dict>
                </array>
            </dict>
            </plist>
        "#;
        let doc: Document = plist::from_bytes(plist_data.as_bytes()).unwrap();
        assert_eq!(doc.size.width, 64.0);
        assert_eq!(doc.symbols[0].name, "Symbol 1");
    }
}
