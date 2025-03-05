use anyhow::Result;
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use jihaz_primal::pathbuf_to_string::PathBufToString;
use resvg::tiny_skia;
use std::{fmt::Display, fs, io::BufWriter, path::Path};
use tauri_icns::{IconFamily, Image};
use usvg::{fontdb, Tree};

use crate::{message::DeferredTaskMessage, message_receiver::MessageReceiver, sleep_for};

/// Default dimension values for the icon
pub const ICONS_DIMENTIONS: [u32; 6] = [16, 32, 64, 128, 256, 512];

pub const SLEEP: u64 = 0;

/// Produces the .icns file from the given SVG file, and saves the result in the target path.
/// 
/// The dimensions are for the generated PNG images within this icns file.
/// 
/// It also provides a list of progress messages which can be helpful for apps that need to show them.
pub fn produce_icns_file(
    original_svg_path: impl AsRef<Path>,
    target_icns_file_path: impl AsRef<Path>,
    dimentions: &[u32],
    progress_msg_receiver: Option<&dyn MessageReceiver<DeferredTaskMessage, IconTaskMessage>>,
) -> Result<()> {

    let svg_tree = read_svg_tree(original_svg_path.as_ref())?;
    sleep_for(SLEEP);
    progress_msg_receiver.map(|r| r.send(IconTaskMessage::began_production(
        original_svg_path.as_ref().to_string().unwrap(), 
    )));

    let mut icon_family = IconFamily::new();

    for rect_dimention in dimentions {
    let png = produce_png(
            *rect_dimention, 
            &svg_tree, 
            None, 
            progress_msg_receiver
        )?;
        let image = Image::read_png(&*png)?;
        icon_family.add_icon(&image)?;
    }

    let file = BufWriter::new(fs::File::create(target_icns_file_path.as_ref())?);
    icon_family.write(file)?;

    sleep_for(SLEEP);
    progress_msg_receiver.map(|r| r.send(IconTaskMessage::written_icons_file(
        target_icns_file_path.as_ref().to_string().unwrap(), 
        "ICNS".to_string()
    )));

    Ok(())
}


/// Produces the .ico file from the given SVG file, and saves the result in the target path.
/// 
/// The dimensions are for the generated PNG images within this .ico file.
pub fn produce_ico_file(
    original_svg_path: impl AsRef<Path>,
    target_ico_file_path: impl AsRef<Path>,
    dimentions: &[u32],
    progress_msg_receiver: Option<&dyn MessageReceiver<DeferredTaskMessage, IconTaskMessage>>,
) -> Result<()> {
    let svg_tree = read_svg_tree(original_svg_path.as_ref())?;
    sleep_for(SLEEP);
    progress_msg_receiver.map(|r| r.send(IconTaskMessage::began_production(
        original_svg_path.as_ref().to_string().unwrap(), 
    )));

    let mut icondir = IconDir::new(ResourceType::Icon);
    for rect_dimention in dimentions {
        let png= produce_png(
            *rect_dimention, 
            &svg_tree, 
            None, 
            progress_msg_receiver
        )?;
        let image = IconImage::read_png(&*png)?;
        let encoded_image = IconDirEntry::encode(&image)?;
        icondir.add_entry(encoded_image);
    }

    let out_file = fs::File::create(target_ico_file_path.as_ref())?;
    icondir.write(out_file)?;

    sleep_for(SLEEP);
    progress_msg_receiver.map(|r| r.send(IconTaskMessage::written_icons_file(
        target_ico_file_path.as_ref().to_string().unwrap(), 
        "ICNS".to_string()
    )));

    Ok(())
}

pub fn read_svg_tree(original_svg_path: impl AsRef<Path>) -> Result<Tree> {
    let mut opt = usvg::Options::default();
    // Get the file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(original_svg_path.as_ref())?
        .parent()
        .map(|p| p.to_path_buf());
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();

    let svg_data = std::fs::read(original_svg_path)?;
    Ok(usvg::Tree::from_data(&svg_data, &opt, &fontdb)?)
}

/// Produce a PNG file data from an SVG file handler
pub fn produce_png(
    frame_side_length: u32, 
    svg_tree: &Tree, 
    target_icns_file_path: Option<&Path>,
    progress_msg_receiver: Option<&dyn MessageReceiver<DeferredTaskMessage, IconTaskMessage>>,
) -> Result<Vec<u8>> {

    let pixmap_size = svg_tree.size().to_int_size();
    assert_eq!(pixmap_size.width(), pixmap_size.height());

    sleep_for(SLEEP);
    progress_msg_receiver.map(|r| r.send(IconTaskMessage::encoding_png(
        frame_side_length as usize, 
        pixmap_size.width() as usize
    )));

    let mut pixmap = tiny_skia::Pixmap::new(frame_side_length, frame_side_length).unwrap();
    
    let scale = frame_side_length as f32 / pixmap_size.width() as f32;
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    
    resvg::render(&svg_tree, transform, &mut pixmap.as_mut());

    if let Some(target_icns_file_path) = target_icns_file_path {
        let mut png_save_path = target_icns_file_path.parent().unwrap().to_path_buf();
        png_save_path.push(format!("{}x{}.png", frame_side_length, frame_side_length));
        let _ = pixmap.save_png(&png_save_path);

        sleep_for(SLEEP);
        progress_msg_receiver.map(|r| r.send(IconTaskMessage::written_png(
            frame_side_length as usize, 
            png_save_path.to_string().unwrap()
        )));

    }
    Ok(pixmap.encode_png()?)
}

#[derive(Debug, Clone)]
pub enum IconTaskMessage {

    // - - - Progress - - -

    BeganProducingIcons {
        source_svg_path: String,
    },
    WrittenIconsFile {
        icons_file_path: String,
        icons_file_kind: String
    },
    EncodingPNG {
        png_dimension: usize,
        source_svg_size: usize,
    },
    WrittenPNG {
        png_dimension: usize,
        png_path: String,
    },
    FinishedProducingIcons {
        target_icons_file_path: String,
    }

    // - - - Errors - - -

}

impl IconTaskMessage {
    pub fn began_production(
        source_svg_path: String,
    ) -> IconTaskMessage {
        IconTaskMessage::BeganProducingIcons { source_svg_path }
    }

    pub fn written_icons_file(
        icons_file_path: String,
        icons_file_kind: String,
    ) -> IconTaskMessage {
        IconTaskMessage::WrittenIconsFile { icons_file_path, icons_file_kind } 
    }

    pub fn encoding_png(
        png_dimension: usize,
        source_svg_size: usize,
    ) -> IconTaskMessage {
        IconTaskMessage::EncodingPNG { png_dimension, source_svg_size }
    }

    pub fn written_png(
        png_dimension: usize,
        png_path: String,
    ) -> IconTaskMessage {
        IconTaskMessage::WrittenPNG { png_dimension, png_path }
    }

    pub fn finished(
        target_icons_file_path: String,
    ) -> IconTaskMessage {
        IconTaskMessage::FinishedProducingIcons { target_icons_file_path }
    }

    pub fn set(
        &mut self, 
        new_progress_message: IconTaskMessage,
    ) {
        *self = new_progress_message;
    }
}

impl Display for IconTaskMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {

            IconTaskMessage::BeganProducingIcons {
                source_svg_path
            } => {
                f.write_fmt(format_args!("Began producing icons from SVG file {source_svg_path}."))
            }

            IconTaskMessage::WrittenIconsFile { icons_file_path, icons_file_kind } => {
                f.write_fmt(format_args!("Written {icons_file_kind} icons file at {icons_file_path}"))
            }

            IconTaskMessage::EncodingPNG { png_dimension, source_svg_size } => {
                f.write_fmt(format_args!("Encoding {png_dimension}x{png_dimension}.png, from SVG of size {source_svg_size}x{source_svg_size}"))
            }

            IconTaskMessage::WrittenPNG { png_dimension, png_path } => {
                f.write_fmt(format_args!("Saving PNG file {png_dimension}x{png_dimension}.png, to path {png_path}"))
            }

            IconTaskMessage::FinishedProducingIcons {
                target_icons_file_path,
            } => {
                f.write_fmt(format_args!("Finished producing icons to file: {target_icons_file_path}"))
            }
        }
    }
}