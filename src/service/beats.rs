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
        .args(vec![
            "-i",
            source.as_str(),
            "-ac",
            "1",
            destination.as_str(),
        ])
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
    let reader = WavReader::open(wav_file)?;
    let format = reader.spec();
    let duration = reader.duration();
    let period = 1.0 / format.sample_rate as Smpl;
    info!("wav spec: {:?}, duration: {}", format, reader.duration());

    let mut tempo = Tempo::new(OnsetMode::SpecDiff, BUF_SIZE, HOP_SIZE, format.sample_rate)?;
    let mut offsets = vec![];
    let mut time = 0.0;
    let mut offset = 0;

    let samples = reader
        .into_samples()
        .map(|sample| sample.map(|sample: i16| sample as Smpl * I16_TO_SMPL))
        .collect::<Result<Vec<_>, _>>()?;

    for block in samples.chunks(HOP_SIZE) {
        if block.len() == HOP_SIZE {
            let result = tempo.do_result(block)?;
            if result > 0.0 {
                offsets.push(time);
            }
        }
        offset += block.len();
        time = offset as Smpl * period;
    }

    let elapsed = start.elapsed();
    info!("detected {} beats in {:?}", offsets.len(), elapsed);

    Ok(Beats {
        offsets,
        length: duration as f32 / format.sample_rate as f32,
    })
}

#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use crate::service::{beats::detect_beats, directories::Directories};

    #[traced_test]
    #[test]
    fn test_detect_beats() {
        let file = "./samples/xtal.opus";
        let dirs = Directories::new().unwrap();
        let beats = detect_beats(file, &dirs).unwrap();
        beats.offsets.iter().for_each(|offset| {
            assert!(*offset <= beats.length);
        })
    }
}
