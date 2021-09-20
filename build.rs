use anyhow::*;
use fs_extra::{copy_items, dir::CopyOptions};
use glob::glob;
use std::{
    env,
    fs::{read_to_string, write},
    path::PathBuf,
};

use naga::{
    front::glsl::Options,
    valid::{Capabilities, ValidationFlags},
};

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: naga::ShaderStage,
}

impl ShaderData {
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let extension = src_path
            .extension()
            .context("File has no extension")?
            .to_str()
            .context("Extension cannot be converted to &str")?;
        let kind = match extension {
            "vert" => naga::ShaderStage::Vertex,
            "frag" => naga::ShaderStage::Fragment,
            "comp" => naga::ShaderStage::Compute,
            _ => bail!("Unsupported shader: {}", src_path.display()),
        };

        let src = read_to_string(src_path.clone())?;
        let spv_path = src_path.with_extension(format!("{}.spv", extension));

        Ok(Self {
            src,
            src_path,
            spv_path,
            kind,
        })
    }
}

fn main() -> Result<()> {
    // This tells cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=res/*");

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("res/");
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    // Collect all shaders recursively within /src/
    let mut shader_paths = [
        glob("./src/**/*.vert")?,
        glob("./src/**/*.frag")?,
        glob("./src/**/*.comp")?,
    ];

    // This could be parallelized
    let shaders = shader_paths
        .iter_mut()
        .flatten()
        .map(|glob_result| ShaderData::load(glob_result?))
        .collect::<Vec<Result<_>>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    // This can't be parallelized. The [shaderc::Compiler] is not
    // thread safe. Also, it creates a lot of resources. You could
    // spawn multiple processes to handle this, but it would probably
    // be better just to only compile shaders that have been changed
    // recently.
    for shader in shaders {
        // This tells cargo to rerun this script if something in /src/ changes.
        println!(
            "cargo:rerun-if-changed={}",
            shader.src_path.as_os_str().to_str().unwrap()
        );

        let mut parser = naga::front::glsl::Parser::default();
        let mut validator =
            naga::valid::Validator::new(ValidationFlags::all(), Capabilities::all());

        let mut options = Options::from(shader.kind);

        let mut module = parser.parse(&options, &shader.src).unwrap();
        let mut module_info = validator.validate(&module).unwrap();

        let spv =
            naga::back::spv::write_vec(&module, &module_info, &naga::back::spv::Options::default())
                .unwrap();

        let bytes = spv
            .iter()
            .fold(Vec::with_capacity(spv.len() * 4), |mut v, w| {
                v.extend_from_slice(&w.to_le_bytes());
                v
            });

        write(shader.spv_path, bytes.as_slice())?;
    }

    Ok(())
}
