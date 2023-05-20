use std::{process::Command, time::Instant};

use crate::{util::commandline_error, Result as AppResult};
use aubio::{OnsetMode, Smpl, Tempo};
use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::eyre;
use hound::WavReader;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::directories::Directories;

const BUF_SIZE: usize = 512;
const HOP_SIZE: usize = 256;
const I16_TO_SMPL: Smpl = 1.0 / (1 << 16) as Smpl;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Beats {
    pub offsets: Vec<f32>,
    pub length: f32,
}

fn convert_to_wav(
    source: impl AsRef<Utf8Path>,
    directories: &Directories,
) -> AppResult<Utf8PathBuf> {
    let source = source.as_ref();
    let file_stem = source
        .file_stem()
        .ok_or(eyre!("input file must have a filename"))?;
    let destination = directories.music_dir().join(format!("{file_stem}.wav"));
    info!(
        "converting file at {} to WAV, destination = {}",
        source, destination
    );
    if destination.is_file() {
        info!("wav file already exists, returning early");
        return Ok(destination);
    }

    let output = Command::new("ffmpeg")
        .args(vec!["-i", source.as_str(), destination.as_str()])
        .output()?;
    if !output.status.success() {
        commandline_error(output)
    } else {
        Ok(destination)
    }
}

pub fn detect_beats(file: impl AsRef<Utf8Path>, directories: &Directories) -> AppResult<Beats> {
    let start = Instant::now();
    let file = file.as_ref();
    let wav_file = convert_to_wav(file, directories)?;
    let mut reader = WavReader::open(wav_file)?;
    let format = reader.spec();
    info!("wav spec: {:?}, duration: {}", format, reader.duration());

    let mut samples = reader.samples();
    let mut tempo = Tempo::new(OnsetMode::SpecFlux, BUF_SIZE, HOP_SIZE, format.sample_rate)?;
    let mut offsets = vec![];
    loop {
        let block = samples
            .by_ref()
            .map(|sample| sample.map(|sample: i16| sample as Smpl * I16_TO_SMPL))
            .take(HOP_SIZE)
            .collect::<Result<Vec<Smpl>, _>>()?;

        if block.len() == HOP_SIZE {
            let result = tempo.do_result(block.as_slice().as_ref())?;
            if result > 0.0 {
                offsets.push(tempo.get_last_s());
            }
        }

        if block.len() < HOP_SIZE {
            break;
        }
    }
    let elapsed = start.elapsed();
    info!("detected {} beats in {:?}", offsets.len(), elapsed);

    Ok(Beats {
        offsets,
        length: reader.duration() as f32 / format.sample_rate as f32,
    })
}
