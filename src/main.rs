use std::process;
use anyhow::{Context, Result};
use regex::Regex;
type Error = Box<dyn std::error::Error>;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    exec("yolo --version").context("please install `pip install -U ultralytics`")?;
    
    let model = "/Users/es/inference/models/n/weights/human_fallen1009.pt";

    let source = "/Users/es/inference/images";

    let command = format!("yolo detect predict model={} source='{}' save=False", model, source);
    let output = exec(&command).context("failed to run yolo detect predict")?;
    
    // 提取 Fall 和空格前面的数字
    extract_fall_info(&output);


    Ok(())
}

fn exec(command: &str) -> Result<String> {
    let args = command.split_ascii_whitespace().collect::<Vec<_>>();
    let (cmd, args) = args.split_first().unwrap();
    let output = std::process::Command::new(cmd)
        .args(args)
        .output()
        .with_context(|| format!("failed to run `{}`", command))?;
    if !output.status.success() {
        anyhow::bail!("failed to run `{}`", command);
    }
    let stdout = String::from_utf8(output.stdout)
        .with_context(|| format!("failed to run `{}`", command))?;
    Ok(stdout.trim().to_string())
}

fn extract_fall_info(output: &str) {
    let re_fall = Regex::new(r"(\d+)\s+Falls?").unwrap();
    let re_total = Regex::new(r"image \d+/(\d+)").unwrap();
    
    // 两个人是 Fall(s), 它是出现在一帧图片中的
    let mut fall_count = 0;
    for cap in re_fall.captures_iter(output) {
        println!("frame Detected Fall(s): {}", &cap[1]);
        //fall_count += cap[1].parse::<i32>().unwrap_or(0);
        fall_count += 1;
    }

    if let Some(cap) = re_total.captures(output) {
        let total_images = cap[1].parse::<i32>().unwrap_or(1); // 防止除以零
        let fall_ratio = fall_count as f32 / total_images as f32;
        println!("Detected Fall(s): {} times", fall_count);
        println!("Total images: {}", total_images);
        println!("Fall ratio: {:.2}%", fall_ratio * 100.0);
    }
}
