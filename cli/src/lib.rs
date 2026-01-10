use clap::Parser;

/// InsploRay
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliConfig {

    /// Path to scene to render, ends with .obj
    #[arg(name = "file")]
    pub input_file_path: String,

    /// Samples per pixel
    #[arg(short, long, default_value_t = 64)]
    pub samples: u32,
    /// Seed value for sampler
    #[arg(long)]
    pub seed: Option<u32>,
    /// image width
    #[arg(short = 'W', long, default_value_t = 720)]
    pub width: u32,
    /// image height
    #[arg(short = 'H', long, default_value_t = 1280)]
    pub height: u32,
    /// Tile size
    #[arg(short, long, default_value_t = 32)]
    pub tile: u32,
    /// Maximum number of bounces
    #[arg(short, long, default_value_t = 5)]
    pub bounces: u32,

    /// Path to output file. Ending with .exr
    #[arg(short, long)]
    pub output: String,
}
