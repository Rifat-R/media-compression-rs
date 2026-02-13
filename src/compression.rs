use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};

pub enum CompressionFormat {
    Mp4,
    Mkv,
    Webp,
}
pub fn compress_media(input: &str, output: &str, format: CompressionFormat) -> Result<(), Error> {
    let mut cmd = Command::new("ffmpeg");

    cmd.arg("-i").arg(input);
    cmd.arg("-y");

    match format {
        CompressionFormat::Mp4 => {
            cmd.args(&[
                "-c:v", "libx264", "-crf", "28", "-preset", "fast", "-c:a", "aac", "-b:a", "128k",
            ]);
        }
        CompressionFormat::Mkv => {
            cmd.args(&["-c:v", "libx265", "-crf", "28", "-c:a", "copy"]);
        }
        CompressionFormat::Webp => {
            cmd.args(&[
                "-c:v",
                "libwebp",
                "-filter:v",
                "fps=15,scale=640:-1",
                "-lossless",
                "0",
                "-compression_level",
                "6",
                "-q:v",
                "50",
                "-loop",
                "0",
            ]);
        }
    }

    cmd.arg(output);

    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::inherit());

    let status = cmd.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "FFmpeg command failed"))
    }
}
