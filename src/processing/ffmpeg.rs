use std::collections::hash_map::DefaultHasher;
use std::convert::Infallible;
use std::fs::{read, read_dir, remove_dir_all, remove_file, write};
use std::hash::{Hash, Hasher};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;

use anyhow::Context;
use image::{load_from_memory, DynamicImage};
use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use super::filetype::{get_sig, Type};
use crate::core::error::FluxError;
use crate::core::media_container::DecodeLimits;
use crate::util::owned_child::IntoOwnedChild;
use crate::util::tmpfile::{TmpFile, TmpFolder};
use crate::util::{hash_buffer, pad_left};

pub fn run_ffmpeg_command(commands: &[&str], pre_commands: &[&str], input: &[u8]) -> Result<Vec<u8>, FluxError> {
    let cpus = num_cpus::get().to_string();

    let rand_string = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();

    let mut args = Vec::from(["-y", "-hide_banner", "-loglevel", "error"]);
    args.extend_from_slice(pre_commands);
    let mut body_hasher = DefaultHasher::new();
    input.hash(&mut body_hasher);

    let hex = format!("{:x}", body_hasher.finish());
    let out_file = TmpFile::new(format!("{}{}", hex, rand_string));
    let in_file = TmpFile::new(format!("{}_", out_file.filename()));
    in_file.write(input).context("Failed to write input file")?;

    if input.len() > 0 {
        args.extend_from_slice(&["-i", in_file.path(), "-threads", &cpus]);
    }

    args.extend_from_slice(commands);
    args.push(out_file.path());

    let command = Command::new("ffmpeg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(&args)
        .spawn()?
        .into_owned_child();

    let exit = command.wait_with_output()?;
    let code = exit.status;

    if !code.success() {
        return Err(FluxError::ScriptError(
            String::from_utf8_lossy(&exit.stderr).to_string(),
        ));
    }

    let new_buffer = read(out_file.path())?;
    Ok(new_buffer)
}

pub fn ah_shit(input: Vec<u8>) -> Result<Vec<u8>, FluxError> {
    run_ffmpeg_command(
        &[
            "-i",
            "./assets/video/ahshit.mp4",
            "-filter_complex",
            "[0]scale=1280:720,setdar=16/9,zoompan=z='zoom+0.001':x='if(gte(zoom,1.5),x,x-1)':y='y':d=76[zoom];[1]colorkey=0x00FF00:similarity=0.45:blend=0.0[shit];[zoom][shit]overlay",
            "-shortest",
            "-f",
            "mp4",
        ],
        &[],
        &input,
    )
}

pub fn april_fools(input: Vec<u8>) -> Result<Vec<u8>, FluxError> {
    run_ffmpeg_command(
        &[
            "-r",
            "30",
            "-i",
            "./assets/video/april.mp4",
            "-filter_complex",
            "[0]scale=480:480,setdar=1[img];[1]setdar=1[vid];[img][vid]concat",
            "-movflags",
            "faststart",
            "-pix_fmt",
            "yuv420p",
            "-vsync",
            "2",
            "-f",
            "mp4",
        ],
        &["-t", "1", "-r", "1"],
        &input,
    )
}

pub fn drip(input: &[u8]) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "./assets/audio/drip.mp3",
            "-t",
            "20",
            "-vf",
            "pad=ceil(iw/2)*2:ceil(ih/2)*2",
            "-movflags",
            "faststart",
            "-pix_fmt",
            "yuv420p",
            "-f",
            "mp4",
        ],
        {
            if sig == Type::Gif {
                &["-ignore_loop", "0"]
            } else {
                &["-loop", "1"]
            }
        },
        input,
    )
}

pub fn femurbreaker(input: &[u8]) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "./assets/audio/femurbreaker.mp3",
            "-t",
            "15",
            "-vf",
            "pad=ceil(iw/2)*2:ceil(ih/2)*2,vignette=PI/4",
            "-movflags",
            "faststart",
            "-pix_fmt",
            "yuv420p",
            "-f",
            "mp4",
        ],
        {
            if sig == Type::Gif {
                &["-ignore_loop", "0"]
            } else {
                &["-loop", "1"]
            }
        },
        input,
    )
}

pub fn siren(input: &[u8]) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "./assets/audio/siren.mp3",
            "-t",
            "22",
            "-vf",
            "fade=t=in:st=0:d=1,eq=gamma_r=2,pad=ceil(iw/2)*2:ceil(ih/2)*2,vignette=PI/4",
            "-movflags",
            "faststart",
            "-pix_fmt",
            "yuv420p",
            "-f",
            "mp4",
        ],
        {
            if sig == Type::Gif {
                &["-ignore_loop", "0"]
            } else {
                &["-loop", "1"]
            }
        },
        &input,
    )
}

pub fn sweden(input: &[u8]) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "./assets/audio/sweden.mp3",
            "-t",
            "22",
            "-vf",
            "fade=t=in:st=0:d=12,pad=ceil(iw/2)*2:ceil(ih/2)*2",
            "-movflags",
            "faststart",
            "-pix_fmt",
            "yuv420p",
            "-f",
            "mp4",
        ],
        {
            if sig == Type::Gif {
                &["-ignore_loop", "0"]
            } else {
                &["-loop", "1"]
            }
        },
        &input,
    )
}

pub fn terraria(input: &[u8]) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "./assets/audio/terraria.mp3",
            "-t",
            "15",
            "-profile:v",
            "baseline",
            "-level",
            "3.0",
            "-crf",
            "20",
            "-pix_fmt",
            "yuv420p",
            "-movflags",
            "+faststart",
            "-vf",
            "fade=in:0:100,pad=ceil(iw/2)*2:ceil(ih/2)*2",
            "-f",
            "mp4",
        ],
        {
            if sig == Type::Gif {
                &["-ignore_loop", "0"]
            } else {
                &["-loop", "1"]
            }
        },
        &input,
    )
}

pub fn video_to_dynamic_images(input: &[u8], limits: &DecodeLimits) -> Result<Vec<DynamicImage>, FluxError> {
    let time_limit = limits
        .video_time_limit
        .unwrap_or(Duration::from_secs(45))
        .as_secs()
        .to_string();
    let fps_limit = format!("fps={}", limits.frame_rate_limit.unwrap_or(20));

    let cpus = num_cpus::get().to_string();

    let folder_name = hash_buffer(input);
    std::fs::create_dir(format!("/tmp/{}", folder_name))?;

    let in_path = format!("/tmp/{}/input", folder_name);
    write(&in_path, input)?;

    let out_path = format!("/tmp/{}/out%05d.bmp", folder_name);

    let mut args = Vec::from(["-y", "-hide_banner", "-loglevel", "error"]);
    args.extend_from_slice(&["-t", &time_limit, "-i", &in_path, "-threads", &cpus]);
    args.extend_from_slice(&["-vf", &fps_limit]);
    args.push(&out_path);

    let command = Command::new("ffmpeg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(&args)
        .spawn()?
        .into_owned_child();

    let exit = command.wait_with_output()?;
    let code = exit.status;

    if !code.success() {
        return Err(FluxError::ScriptError(
            String::from_utf8_lossy(&exit.stderr).to_string(),
        ));
    }

    let new_buffers = read_dir(format!("/tmp/{}", folder_name))?;
    let new_buffers_count = read_dir(format!("/tmp/{}", folder_name))?.count();
    let mut images: Vec<DynamicImage> = vec![];
    let mut new_paths_sorted: Vec<String> = vec![String::new(); new_buffers_count];

    for buf in new_buffers {
        let buf = buf.unwrap();
        let path = buf.path().to_string_lossy().to_string();
        if path.ends_with("bmp") {
            let filename = buf.file_name().to_string_lossy().to_string();
            let num = filename[3..8].to_string().parse::<usize>().unwrap();
            new_paths_sorted[num] = path;
        }
    }

    for path in new_paths_sorted {
        if !path.is_empty() {
            let file = read(path)?;
            images.push(DynamicImage::ImageRgba8(load_from_memory(&file)?.into_rgba8()));
        }
    }

    remove_dir_all(format!("/tmp/{}", folder_name))?;

    Ok(images)
}

pub fn split_video(input: &[u8], limits: DecodeLimits) -> Result<(Vec<DynamicImage>, Vec<u8>), FluxError> {
    let boxed = Box::<[u8]>::from(input);
    let arced = Arc::<[u8]>::from(boxed);
    let arced_clone = arced.clone();

    let audio_task = spawn(move || run_ffmpeg_command(&["-f", "mp3"], &[], &arced));
    let video_task = spawn(move || video_to_dynamic_images(&arced_clone, &limits));

    let imgs = video_task.join().unwrap()?;
    let audio = audio_task
        .join()
        .unwrap()
        .or::<Infallible>(Ok(Vec::<u8>::new()))
        .unwrap();

    Ok((imgs, audio))
}

pub fn create_video_from_split(
    dyn_images: Vec<DynamicImage>,
    audio: &[u8],
    limits: &DecodeLimits,
) -> Result<Vec<u8>, FluxError> {
    let cpus = num_cpus::get().to_string();

    let folder_name = hash_buffer(&[1, 2, 3, 4]);
    let files = format!("{}/*.bmp", folder_name);

    std::fs::create_dir(format!("{}", folder_name))?;
    // drop = delete folder
    let _tmpfolder = TmpFolder::new(&folder_name);

    for image in dyn_images.iter().enumerate() {
        image.1.save_with_format(
            format!("{}/{}.bmp", folder_name, pad_left(image.0.to_string(), 5, '0')),
            image::ImageFormat::Bmp,
        )?;
    }

    let out_path = format!("{}/output.mp4", folder_name);

    let mut args = Vec::from(["-y", "-hide_banner", "-loglevel", "error"]);
    let fps = limits.frame_rate_limit.unwrap_or(20).to_string();
    args.extend_from_slice(&[
        "-framerate",
        &fps,
        "-pattern_type",
        "glob",
        "-i",
        &files,
        "-threads",
        &cpus,
    ]);
    args.extend_from_slice(&["-c:v", "libx264", "-pix_fmt", "yuv420p"]);
    args.push(&out_path);

    let command = Command::new("ffmpeg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(&args)
        .spawn()?
        .into_owned_child();

    let exit = command.wait_with_output()?;
    let code = exit.status;

    if !code.success() {
        return Err(FluxError::ScriptError(
            String::from_utf8_lossy(&exit.stderr).to_string(),
        ));
    }

    let silent_video = read(out_path)?;

    if !audio.is_empty() {
        let mut body_hasher = DefaultHasher::new();
        audio.hash(&mut body_hasher);

        let hex = format!("{:x}", body_hasher.finish());
        let in_path = format!("/tmp/{}__", hex);

        write(&in_path, audio)?;

        let video = run_ffmpeg_command(
            &[
                "-i",
                &in_path,
                "-c",
                "copy",
                "-map",
                "0:v:0",
                "-map",
                "1:a:0",
                "-shortest",
                "-c:a",
                "aac",
                "-b:a",
                "128k",
                "-f",
                "mp4",
            ],
            &[],
            &silent_video,
        )?;

        remove_file(&in_path)?;

        Ok(video)
    } else {
        Ok(silent_video)
    }
}

pub fn get_video_dimensions(input: &[u8]) -> Result<(usize, usize), FluxError> {
    let mut body_hasher = DefaultHasher::new();
    input.hash(&mut body_hasher);

    let hex = format!("{:x}", body_hasher.finish());
    let in_path = format!("/tmp/{}", hex);

    write(&in_path, input)?;

    let args = Vec::from([
        "-hide_banner",
        "-loglevel",
        "error",
        "-select_streams",
        "v:0",
        "-show_entries",
        "stream=width,height",
        "-of",
        "csv=s=x:p=0",
        &in_path,
    ]);

    let command = Command::new("ffprobe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&args)
        .spawn()?
        .into_owned_child();

    let output = command.wait_with_output()?;
    let dimensions = String::from_utf8_lossy(&output.stdout).to_string();

    let parts = dimensions
        .split('x')
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    remove_file(in_path)?;
    Ok((parts[0], parts[1]))
}

pub fn get_video_length(input: &[u8]) -> Result<Duration, FluxError> {
    let mut body_hasher = DefaultHasher::new();
    input.hash(&mut body_hasher);

    let hex = format!("{:x}", body_hasher.finish());
    let in_path = format!("/tmp/{}.mp4", hex);

    write(&in_path, input)?;

    let args = Vec::from([
        "-show_entries",
        "format=duration",
        "-v",
        "quiet",
        "-of",
        "csv=p=0",
        &in_path,
    ]);

    let command = Command::new("ffprobe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&args)
        .spawn()?
        .into_owned_child();

    let output = command.wait_with_output()?;
    remove_file(in_path)?;

    let time = String::from_utf8_lossy(&output.stdout).to_string();
    let duration = time
        .trim()
        .parse::<f32>()
        .map(|o| Duration::from_millis((o * 1000.0) as u64))
        .map_err(|_| FluxError::ScriptError("Could not parse video length from ffprobe output".to_string()))?;

    Ok(duration)
}

pub fn get_video_frame_count(input: &[u8]) -> Result<usize, FluxError> {
    let mut body_hasher = DefaultHasher::new();
    input.hash(&mut body_hasher);

    let hex = format!("{:x}", body_hasher.finish());
    let in_path = format!("/tmp/{}", hex);

    write(&in_path, input)?;

    let args = Vec::from([
        "-hide_banner",
        "-loglevel",
        "error",
        "-select_streams",
        "v:0",
        "-count_packets",
        "-show_entries",
        "stream=nb_read_packets",
        "-of",
        "csv=p=0",
        &in_path,
    ]);

    let command = Command::new("ffprobe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&args)
        .spawn()?
        .into_owned_child();

    let output = command.wait_with_output()?;
    let count = String::from_utf8_lossy(&output.stdout).to_string();

    let count = count
        .trim()
        .parse::<usize>()
        .map_err(|_| FluxError::ScriptError("Could not parse frame count from ffprobe output".to_string()))?;

    remove_file(in_path)?;
    Ok(count)
}

pub fn get_video_first_frame(input: &[u8]) -> Result<Vec<u8>, FluxError> {
    let frame = run_ffmpeg_command(&["-vf", "select=eq(n\\,0)", "-vframes", "1", "-f", "apng"], &[], input)?;

    Ok(frame)
}

pub fn get_video_fps(input: &[u8]) -> Result<f64, FluxError> {
    let mut body_hasher = DefaultHasher::new();
    input.hash(&mut body_hasher);

    let hex = format!("{:x}", body_hasher.finish());
    let file = TmpFile::new(hex);

    file.write(input)?;

    let args = Vec::from([
        "-hide_banner",
        "-loglevel",
        "error",
        "-select_streams",
        "v:0",
        "-show_entries",
        "stream=r_frame_rate",
        "-of",
        "csv=p=0",
        file.path(),
    ]);

    let command = Command::new("ffprobe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&args)
        .spawn()?
        .into_owned_child();

    let output = command.wait_with_output()?;
    let fps = String::from_utf8_lossy(&output.stdout).to_string();

    let fps = if fps.contains("/") {
        let parts = fps.trim().split_once('/').unwrap();
        let numer = parts
            .0
            .parse::<f64>()
            .context(format!("Failed to parse fps numer {}", parts.0))?;
        let denom = parts
            .1
            .parse::<f64>()
            .context(format!("Failed to parse fps denom {}", parts.1))?;

        numer / denom
    } else {
        fps.trim().parse::<f64>().context("Failed to parse fps")?
    };

    Ok(fps)
}

pub mod ffmpeg_operations {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use anyhow::Context;
    use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

    use crate::util::tmpfile::TmpFile;

    use super::*;

    pub fn vf_manipulate(input: &[u8], vf: &str) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(&["-vf", vf, "-f", "mp4"], &[], input)
    }

    pub fn filter_manipulate_fmt(input: &[u8], vf: &str, fmt: &str) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(&["-filter_complex", vf, "-f", fmt], &[], input)
    }

    pub fn flop_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(&["-vf", "hflip", "-c:a", "copy", "-f", "mp4"], &[], input)
    }

    pub fn flip_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(&["-vf", "vflip", "-c:a", "copy", "-f", "mp4"], &[], input)
    }

    /// MUST take even values
    pub fn resize_video(input: &[u8], w: usize, h: usize) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(&["-s", &format!("{w}x{h}"), "-c:a", "copy", "-f", "mp4"], &[], input)
    }

    pub fn grayscale_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        vf_manipulate(input, "hue=s=0")
    }

    pub fn invert_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        vf_manipulate(input, "negate")
    }

    pub fn pixelize_video(input: &[u8], w: u64, h: u64) -> Result<Vec<u8>, FluxError> {
        filter_manipulate_fmt(input, &format!("pixelize=w={w}:h={h}"), "mp4")
    }

    pub fn bitcrush_audio(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        filter_manipulate_fmt(input, "acrusher=bits=2:mode=log:aa=1:samples=128", "mp3")
    }

    pub fn vibrato_audio(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        if !input.is_empty() {
            filter_manipulate_fmt(input, "vibrato", "mp3")
        } else {
            Ok(vec![])
        }
    }

    pub fn reverse_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(&["-vf", "reverse", "-af", "areverse", "-f", "mp4"], &[], input)
    }

    pub fn rotate_video(input: &[u8], degrees: usize) -> Result<Vec<u8>, FluxError> {
        match degrees {
            0 | 90 | 180 | 270 => run_ffmpeg_command(
                &[
                    "-map_metadata",
                    "0",
                    "-metadata:s:v",
                    &format!("rotate={}", degrees),
                    "-codec",
                    "copy",
                    "-f",
                    "mp4",
                ],
                &[],
                input,
            ),
            _ => run_ffmpeg_command(
                &["-vf", &format!("rotate={}", (degrees as f64).to_radians()), "-f", "mp4"],
                &[],
                input,
            ),
        }
    }

    pub fn caption_video(input: &[u8], text_image: DynamicImage, bottom: bool) -> Result<Vec<u8>, FluxError> {
        let text_image_height = text_image.height();

        let png_path = format!("/tmp/{}_caption_png", hash_buffer(input));
        text_image.save_with_format(&png_path, image::ImageFormat::Png)?;

        let output = if bottom {
            let d = get_video_dimensions(input)?.1;

            run_ffmpeg_command(
                &[
                    "-i",
                    &png_path,
                    "-filter_complex",
                    &format!("[0] pad=w=iw:h={text_image_height}+ih:x=0:y=0:color=black,overlay=0:{d}"),
                    "-f",
                    "mp4",
                ],
                &[],
                input,
            )?
        } else {
            run_ffmpeg_command(
                &[
                    "-i",
                    &png_path,
                    "-filter_complex",
                    &format!(
                        "[0] pad=w=iw:h={0}+ih:x=0:y={0}:color=black,overlay=0:0",
                        text_image_height
                    ),
                    "-f",
                    "mp4",
                ],
                &[],
                input,
            )?
        };

        remove_file(&png_path)?;

        Ok(output)
    }

    pub fn crop_video(input: &[u8], x: usize, y: usize, width: usize, height: usize) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(
            &[
                "-filter:v",
                &format!("crop={}:{}:{}:{}", width, height, x, y),
                "-f",
                "mp4",
            ],
            &[],
            input,
        )
    }

    pub fn speed_video(input: &[u8], speed: f64) -> Result<Vec<u8>, FluxError> {
        let speed = speed.clamp(0.5, 100.0);
        run_ffmpeg_command(
            &[
                "-filter:v",
                &format!("setpts={:.2}*PTS", 1.0 / speed),
                "-filter:a",
                &format!("atempo={:.2}", speed),
                "-f",
                "mp4",
            ],
            &[],
            input,
        )
    }

    pub fn rainbow_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        let frame_count = get_video_frame_count(input)?;
        vf_manipulate(input, &format!("hue=h=n*360/{}", frame_count))
    }

    pub fn spin_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        let frame_count = get_video_frame_count(input)?;
        vf_manipulate(
            input,
            &format!("rotate=a=\\(n*360/{}\\)*\\(3.141529/180\\)", frame_count),
        )
    }

    pub fn video_to_gif(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        let dimensions = get_video_dimensions(input)?;
        run_ffmpeg_command(
            &[
                "-vf",
                &format!(
                    "{}split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
                    if dimensions.0 > 384 {
                        "scale=384:-1:flags=lanczos,"
                    } else {
                        ""
                    }
                ),
                "-loop",
                "0",
                "-f",
                "gif",
            ],
            &[],
            input,
        )
    }

    pub fn gloop_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        let reversed = reverse_video(input)?;
        let hashed_video = hash_buffer(input);
        let hashed_reversed = hash_buffer(&reversed);

        let forwards_in = format!("/tmp/{}_gloop_forwards_in", hashed_video);
        let backwards_in = format!("/tmp/{}_gloop_backwards_in", hashed_reversed);

        fs::write(&forwards_in, input)?;
        fs::write(&backwards_in, &reversed)?;

        let input_txt = format!("file '{}'\nfile '{}'", forwards_in, backwards_in);
        let input_txt_path = format!("/tmp/{}_gloop_input.txt", hashed_video);
        fs::write(input_txt_path.clone(), &input_txt)?;

        let out_path = format!("/tmp/{}_gloop_out.mp4", hashed_video);

        let args = Vec::from([
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            &input_txt_path,
            "-c",
            "copy",
            &out_path,
        ]);

        let c = Command::new("ffmpeg")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(&args)
            .spawn()?
            .into_owned_child();

        c.wait_with_output()?;

        let data = fs::read(&out_path)?;

        remove_file(&forwards_in)?;
        remove_file(&backwards_in)?;
        remove_file(&input_txt_path)?;
        remove_file(&out_path)?;

        Ok(data)
    }

    pub fn audio_to_pcm(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(
            &["-b:a", "44100", "-ac", "1", "-c:a", "pcm_s16le", "-f", "s16le"],
            &["-t", "4"],
            input,
        )
        .map_err(|_| FluxError::CorruptInput("Failed to process audio".to_owned()))
    }

    pub fn slice_video(input: &[u8], start: &str, time: &str) -> Result<Vec<u8>, FluxError> {
        run_ffmpeg_command(
            &["-ss", start, "-t", time, "-f", "mp4"],
            &["-hide_banner", "-loglevel", "error", "-nostats"],
            input,
        )
    }

    pub fn scramble_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        let mut len_remaining = get_video_length(input)
            .context("Failed to get video length")?
            .as_millis();
        let mut next_start = 0;
        let mut slice_sections = Vec::new();

        while len_remaining > 0 {
            let next_chunk_len = thread_rng().gen_range(1..500).clamp(0, len_remaining) as u128;
            len_remaining -= next_chunk_len;
            next_start += next_chunk_len;

            let chunk_len_secs = next_chunk_len / 1000;
            let chunk_len_millis_rem = next_chunk_len % 1000;
            let chunk_len_fmt = format!(
                "{}.{}",
                chunk_len_secs,
                pad_left(chunk_len_millis_rem.to_string(), 3, '0')
            );

            let next_start_secs = next_start / 1000;
            let next_start_millis_rem = next_start % 1000;
            let next_start_fmt = format!(
                "{}.{}",
                next_start_secs,
                pad_left(next_start_millis_rem.to_string(), 3, '0')
            );
            slice_sections.push((chunk_len_fmt, next_start_fmt));
        }

        let mut files = vec![TmpFile::new(""); slice_sections.len()];
        files.par_iter_mut().enumerate().try_for_each(|(i, f)| {
            let (chunk_len_fmt, next_start_fmt) = slice_sections.get(i).unwrap();

            let slice = slice_video(input, next_start_fmt, chunk_len_fmt)?;

            let hashed_slice = hash_buffer(&slice);
            let file = TmpFile::new(format!("{hashed_slice}-{i}.mp4"));
            file.write(&slice)?;

            *f = file;
            Ok::<(), FluxError>(())
        })?;

        files.shuffle(&mut thread_rng());

        let file_list = files
            .iter()
            .map(|x| format!("file {}", x.path()))
            .collect::<Vec<_>>()
            .join("\n");

        let list_name = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let list_file = TmpFile::new(format!("{list_name}.txt"));
        list_file.write(&file_list)?;

        run_ffmpeg_command(
            &["-i", list_file.path(), "-f", "mp4"],
            &["-f", "concat", "-hide_banner", "-loglevel", "error", "-safe", "0"],
            &[],
        )
    }
}
