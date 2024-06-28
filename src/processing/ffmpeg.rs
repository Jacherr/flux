use std::collections::hash_map::DefaultHasher;
use std::convert::Infallible;
use std::fs::{read, read_dir, remove_dir_all, remove_file, write};
use std::hash::{Hash, Hasher};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread::spawn;

use image::{load_from_memory, DynamicImage};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use super::filetype::{get_sig, Type};
use crate::core::error::FluxError;
use crate::util::owned_child::IntoOwnedChild;
use crate::util::{hash_buffer, pad_left};

pub fn run_ffmpeg_command(commands: &[&str], pre_commands: &[&str], input: &[u8]) -> Result<Vec<u8>, FluxError> {
    let cpus = num_cpus::get().to_string();

    let rand_string = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();

    let mut body_hasher = DefaultHasher::new();
    input.hash(&mut body_hasher);

    let hex = format!("{:x}", body_hasher.finish());
    let out_path = format!("/tmp/{}{}", hex, rand_string);
    let in_path = format!("{}_", out_path);

    write(&in_path, input)?;

    let mut args = Vec::from(["-y", "-hide_banner", "-loglevel", "error"]);
    args.extend_from_slice(pre_commands);
    args.extend_from_slice(&["-i", &in_path, "-threads", &cpus]);
    args.extend_from_slice(commands);
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

    let new_buffer = read(&out_path)?;
    remove_file(out_path)?;
    remove_file(in_path)?;
    Ok(new_buffer)
}

pub fn ah_shit(input: Vec<u8>) -> Result<Vec<u8>, FluxError> {
    run_ffmpeg_command(
        &[
            "-i",
            "../subproc/assets/video/ahshit.mp4",
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
            "../subproc/assets/video/april.mp4",
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

pub fn drip(input: Vec<u8>) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "../subproc/assets/audio/drip.mp3",
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
        &input,
    )
}

pub fn femurbreaker(input: Vec<u8>) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "../subproc/assets/audio/femurbreaker.mp3",
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
        &input,
    )
}

pub fn siren(input: Vec<u8>) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "../subproc/assets/audio/siren.mp3",
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

pub fn sweden(input: Vec<u8>) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "../subproc/assets/audio/sweden.mp3",
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

pub fn terraria(input: Vec<u8>) -> Result<Vec<u8>, FluxError> {
    let sig = get_sig(&input).ok_or(FluxError::UnsupportedFiletype)?;

    run_ffmpeg_command(
        &[
            "-i",
            "../subproc/assets/audio/terraria.mp3",
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

pub fn video_to_dynamic_images(input: &[u8]) -> Result<Vec<DynamicImage>, FluxError> {
    let cpus = num_cpus::get().to_string();

    let folder_name = hash_buffer(input);
    std::fs::create_dir(format!("/tmp/{}", folder_name))?;

    let in_path = format!("/tmp/{}/input", folder_name);
    write(&in_path, input)?;

    let out_path = format!("/tmp/{}/out%05d.bmp", folder_name);

    let mut args = Vec::from(["-y", "-hide_banner", "-loglevel", "error"]);
    args.extend_from_slice(&["-t", "45", "-i", &in_path, "-threads", &cpus]);
    args.extend_from_slice(&["-vf", "fps=20"]);
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
            images.push(load_from_memory(&file)?);
        }
    }

    remove_dir_all(format!("/tmp/{}", folder_name))?;

    Ok(images)
}

pub fn split_video(input: &[u8]) -> Result<(Vec<DynamicImage>, Vec<u8>), FluxError> {
    let boxed = Box::<[u8]>::from(input);
    let arced = Arc::<[u8]>::from(boxed);
    let arced_clone = arced.clone();

    let audio_task = spawn(move || run_ffmpeg_command(&["-f", "mp3"], &[], &arced));
    let video_task = spawn(move || video_to_dynamic_images(&arced_clone));

    let imgs = video_task.join().unwrap()?;
    let audio = audio_task
        .join()
        .unwrap()
        .or::<Infallible>(Ok(Vec::<u8>::new()))
        .unwrap();

    Ok((imgs, audio))
}

pub fn create_video_from_split(dyn_images: Vec<DynamicImage>, audio: &[u8]) -> Result<Vec<u8>, FluxError> {
    let cpus = num_cpus::get().to_string();

    let folder = format!("/tmp/{}", hash_buffer(&[1, 2, 3, 4]));
    let files = format!("{}/*.bmp", folder);

    std::fs::create_dir(format!("{}", folder))?;

    for image in dyn_images.iter().enumerate() {
        image.1.save_with_format(
            format!("{}/{}.bmp", folder, pad_left(image.0.to_string(), 5, '0')),
            image::ImageFormat::Bmp,
        )?;
    }

    let out_path = format!("{}/output.mp4", folder);

    let mut args = Vec::from(["-y", "-hide_banner", "-loglevel", "error"]);
    args.extend_from_slice(&[
        "-framerate",
        "20",
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
    println!("{}", count.trim());
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

pub mod ffmpeg_operations {
    use std::fs;

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

    pub fn grayscale_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        vf_manipulate(input, "hue=s=0")
    }

    pub fn invert_video(input: &[u8]) -> Result<Vec<u8>, FluxError> {
        vf_manipulate(input, "negate")
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

    pub fn caption_video(input: &[u8], text_image: DynamicImage) -> Result<Vec<u8>, FluxError> {
        let text_image_height = text_image.height();

        let png_path = format!("/tmp/{}_caption_png", hash_buffer(input));
        text_image.save_with_format(&png_path, image::ImageFormat::Png)?;

        let output = run_ffmpeg_command(
            &[
                "-i",
                &png_path,
                "-filter_complex",
                &format!(
                    "[0]pad=w=iw:h={0}+ih:x=0:y={0}:color=black,overlay=0:0",
                    text_image_height
                ),
                "-f",
                "mp4",
            ],
            &[],
            input,
        )?;

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
}
