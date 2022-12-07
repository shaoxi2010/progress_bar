use std::path::Path;
use std::time::Duration;
use std::thread::sleep;

use linuxfb::FrameBuffer;
use anyhow::{Result, bail};
use bitmap::{DrawIo, WHITE, BitMap, BitMapResult, PixExt, Point, GREEN, RGB565, Painter, ARGB32};

const WIDTH: usize = 800;
const HEIGHT: usize = 480;

const PROGRESS_WIDTH:usize = 600;
const PROGRESS_HEIGHT:usize = 80;
const PROGRESS_BLOCK_SPARE:usize = 2;
const PROGRESS_BLOCKS:usize = 20;
const PROGRESS_BLOCK_HEIGHT:usize = PROGRESS_HEIGHT - 2 * PROGRESS_BLOCK_SPARE;
const PROGRESS_BLOCK_WIDTH:usize = PROGRESS_WIDTH / PROGRESS_BLOCKS - 2 * PROGRESS_BLOCK_SPARE;

fn draw_progress<T: PixExt + Copy + Default>(bitmap: BitMap<T>, val: usize, text: &str) -> BitMapResult<()> {
    use std::cmp::min;
    let textlen = 16 * text.len();
    bitmap.draw_text(((WIDTH - textlen) / 2,(HEIGHT - PROGRESS_HEIGHT) / 2 - 32 - PROGRESS_BLOCK_SPARE).into(), WHITE, text, 32)?;
    bitmap.fill_rectagle(((WIDTH - PROGRESS_WIDTH) / 2, (HEIGHT - PROGRESS_HEIGHT) / 2).into(), PROGRESS_WIDTH, PROGRESS_HEIGHT, WHITE)?; 
    let val = min(100, val);
    for x in 0..val / (100 / PROGRESS_BLOCKS) {
        let block_topleft:Point = ((WIDTH - PROGRESS_WIDTH) / 2 + PROGRESS_BLOCK_SPARE + PROGRESS_WIDTH / PROGRESS_BLOCKS * x,
         (HEIGHT - PROGRESS_HEIGHT) / 2 + PROGRESS_BLOCK_SPARE).into();
         bitmap.fill_rectagle(block_topleft, PROGRESS_BLOCK_WIDTH, PROGRESS_BLOCK_HEIGHT, GREEN)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut fb = FrameBuffer::new(Path::new("/dev/fb0"))?;
    let mut progress = 0;
    let (width, height) = fb.screen_size();
    let color_depth = fb.color_depth();
    while progress <= 1000 {
        if let Some(data) = fb.get_buff_data() {
            if color_depth == 16 {
                RGB565::painter(data, width, height, |bitmap| {
                    bitmap.clear()?;
                    draw_progress(bitmap, progress / 10, &format!("Progress:{}%", std::cmp::min(100, progress/10)))?;
                    Ok(())
                })?;
            } else if color_depth == 32 {
                ARGB32::painter(data, width, height, |bitmap| {
                    bitmap.clear()?;
                    draw_progress(bitmap, progress / 10, &format!("Progress:{}%", std::cmp::min(100, progress/10)))?;
                    Ok(())
                })?;
            } else {
                bail!("not support color")
            }
            fb.swap()?;
        } else {
            if color_depth == 16 {
                RGB565::painter(fb.get_disp_data(), width, height, |bitmap| {
                    bitmap.clear()?;
                    draw_progress(bitmap, progress / 10, &format!("Progress:{}%", std::cmp::min(100, progress/10)))?;
                    Ok(())
                })?;
            } else if color_depth == 32 {
                ARGB32::painter(fb.get_disp_data(), width, height, |bitmap| {
                    bitmap.clear()?;
                    draw_progress(bitmap, progress / 10, &format!("Progress:{}%", std::cmp::min(100, progress/10)))?;
                    Ok(())
                })?;
            } else {
                bail!("not support color")
            }
        }

        progress += 1;
        sleep(Duration::from_millis(100));
    }
    Ok(())
}
