use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jstring};
use std::path::Path;
use anyhow::{Context, Result, anyhow};
use std::fs;

#[unsafe(no_mangle)]
pub extern "system" fn Java_tech_bananajuice_convertpixelart_RustInterop_convertFile<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input_path_j: JString<'local>,
    output_path_j: JString<'local>,
    timelapse: jboolean,
) -> jstring {
    let input_path_str: String = env.get_string(&input_path_j).unwrap().into();
    let output_path_str: String = env.get_string(&output_path_j).unwrap().into();

    let input_path = Path::new(&input_path_str);
    let output_path = Path::new(&output_path_str);
    let timelapse_bool = timelapse != 0;

    match convert_internal(input_path, output_path, timelapse_bool) {
        Ok(_) => {
            let output = env.new_string("Success").unwrap();
            output.into_raw()
        }
        Err(e) => {
            let error_msg = format!("Error: {}", e);
            let output = env.new_string(error_msg).unwrap();
            output.into_raw()
        }
    }
}

fn convert_internal(input_path: &Path, output_path: &Path, timelapse: bool) -> Result<()> {
    let doc = if input_path.is_file()
        && input_path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("psp"))
    {
        handle_psp_format(input_path, timelapse)?
    } else if input_path.is_file()
        && input_path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("psd"))
    {
        handle_psd_format(input_path)?
    } else if input_path.is_file()
        && input_path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| {
                ext.eq_ignore_ascii_case("ase") || ext.eq_ignore_ascii_case("aseprite")
            })
    {
        handle_aseprite_format(input_path)?
    } else if input_path.join("document.json").exists() {
        handle_modern_format(input_path)?
    } else if input_path.join("DocumentInfo.plist").exists() {
        handle_legacy_format(input_path)?
    } else {
        return Err(anyhow!(
            "No valid .psp, .psd, .ase, .aseprite file, or document.json/DocumentInfo.plist found in the given path"
        ));
    };

    let ext = output_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext == "ase" || ext == "aseprite" {
        let aseprite_file = aseprite_converter::convert(doc)?;

        let mut buffer = Vec::new();
        aseprite_file
            .write_to(&mut buffer)
            .map_err(|e| anyhow!("Failed to write Aseprite file: {}", e))?;
        fs::write(output_path, buffer)?;
    } else if ext == "png" {
        let img = doc.render();
        img.save(output_path)
            .context("Failed to write PNG file")?;
    } else {
        return Err(anyhow!(
            "Unsupported output format: '{}'. Supported formats are .ase, .aseprite, and .png",
            ext
        ));
    }

    Ok(())
}

fn handle_modern_format(pixaki_path: &Path) -> Result<pixel_art::Document> {
    let document_path = pixaki_path.join("document.json");
    let json_str = fs::read_to_string(document_path)?;
    let doc_v3: pixaki_v3::Document =
        serde_json::from_str(&json_str).context("Unable to parse document.json")?;

    pixaki_v3_converter::convert(doc_v3, pixaki_path)
}

fn handle_legacy_format(pixaki_path: &Path) -> Result<pixel_art::Document> {
    let plist_path = pixaki_path.join("DocumentInfo.plist");
    let doc_v2: pixaki_v2::Document =
        plist::from_file(plist_path).context("Failed to parse DocumentInfo.plist")?;

    pixaki_v2_converter::convert(doc_v2, pixaki_path)
}

fn handle_psp_format(psp_path: &Path, timelapse: bool) -> Result<pixel_art::Document> {
    let json_str = fs::read_to_string(psp_path)?;
    let doc_psp: pixel_studio_pro_v2::Document =
        serde_json::from_str(&json_str).context("Unable to parse .psp JSON document")?;

    pixel_studio_pro_v2_converter::convert(doc_psp, timelapse)
}

fn handle_psd_format(psd_path: &Path) -> Result<pixel_art::Document> {
    let bytes = fs::read(psd_path)?;
    psd_converter::convert(&bytes).context("Failed to parse .psd file")
}

fn handle_aseprite_format(ase_path: &Path) -> Result<pixel_art::Document> {
    let file = fs::File::open(ase_path)?;
    let aseprite_file =
        aseprite::AsepriteFile::from_reader(file).context("Failed to parse .aseprite file")?;
    aseprite_converter::reader::parse(aseprite_file)
}
