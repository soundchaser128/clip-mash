use crate::{util::commandline_error, Result};
use camino::{Utf8Path, Utf8PathBuf};
use tokio::process::Command;

#[derive(Default)]
pub struct Ffmpeg {
    executable_path: Utf8PathBuf,
    inputs: Vec<String>,
    output: Option<String>,
    start: Option<String>,
    duration: Option<String>,
    video_codec: Option<String>,
    audio_codec: Option<String>,
    crf: Option<String>,
    preset: Option<String>,
    extra_args: Vec<String>,
    video_filter: Option<String>,
    working_directory: Option<Utf8PathBuf>,
    output_file: String,
}

impl Ffmpeg {
    pub fn new(executable: impl AsRef<Utf8Path>, output_file: String) -> Self {
        Ffmpeg {
            output_file,
            executable_path: executable.as_ref().to_owned(),
            ..Default::default()
        }
    }

    pub fn input(&mut self, input: impl Into<String>) -> &mut Self {
        self.inputs.push(input.into());
        self
    }

    pub fn working_directory(&mut self, working_directory: impl Into<Utf8PathBuf>) -> &mut Self {
        self.working_directory = Some(working_directory.into());
        self
    }

    pub fn start(&mut self, start: f64) -> &mut Self {
        self.start = Some(start.to_string());
        self
    }

    pub fn duration(&mut self, duration: f64) -> &mut Self {
        self.duration = Some(duration.to_string());
        self
    }

    pub async fn run(&self) -> Result<()> {
        let mut args = vec!["-hide-banner", "-loglevel", "warning"];

        if let Some(start) = &self.start {
            args.push("-ss");
            args.push(&start);
        }

        for input in &self.inputs {
            args.push("-i");
            args.push(&input);
        }

        args.push(&self.output_file);
        let mut command = Command::new(self.executable_path.as_str());
        command.args(args);

        if let Some(dir) = &self.working_directory {
            command.current_dir(dir);
        }

        let output = command.output().await?;

        if output.status.success() {
            Ok(())
        } else {
            commandline_error(output)
        }
    }
}
