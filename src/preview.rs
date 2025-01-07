use std::{error::Error, fs};

use cairo::{Context, ImageSurface};

use crate::Rect;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long="inset", value_parser = Rect::from_args, default_value_t = Rect::default())]
    rect: Rect,

    #[clap()]
    src: String,

    #[clap()]
    dst: String,

    #[clap(long = "ticks")]
    ticks: Option<usize>,
}

pub fn execute(args: Args) -> Result<(), Box<dyn Error>> {
    let src = ImageSurface::create_from_png(&mut fs::File::open(&args.src)?)?;
    let ctx = Context::new(&src)?;
    let rect = Rect::from_xywh(0, 0, src.width() as i64, src.height() as i64).inset(&args.rect);
    if rect.is_empty() {
        return Err("empty rect".into());
    }

    let w = src.width() as f64;
    let h = src.height() as f64;

    ctx.save()?;
    ctx.new_path();
    ctx.move_to(0.0, 0.0);
    ctx.line_to(0.0, h);
    ctx.line_to(w, h);
    ctx.line_to(w, 0.0);
    ctx.line_to(0.0, 0.0);
    ctx.move_to(rect.left() as f64, rect.top() as f64);
    ctx.line_to(rect.right() as f64, rect.top() as f64);
    ctx.line_to(rect.right() as f64, rect.bottom() as f64);
    ctx.line_to(rect.left() as f64, rect.bottom() as f64);
    ctx.line_to(rect.left() as f64, rect.top() as f64);
    ctx.clip();

    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.4);
    ctx.rectangle(0.0, 0.0, src.width() as f64, src.height() as f64);
    ctx.fill()?;
    ctx.restore()?;

    ctx.save()?;
    ctx.new_path();
    if let Some(ticks) = args.ticks {
        let n = (w * 0.25 / ticks as f64) as usize;
        for i in 0..=n {
            let x = i as f64 * ticks as f64;
            ctx.move_to(x, 0.0);
            ctx.line_to(x, h);
            ctx.move_to(w - x, 0.0);
            ctx.line_to(w - x, h);
        }

        let n = (h * 0.25 / ticks as f64) as usize;
        for i in 0..=n {
            let y = i as f64 * ticks as f64;
            ctx.move_to(0.0, y);
            ctx.line_to(w, y);
            ctx.move_to(0.0, h - y);
            ctx.line_to(w, h - y);
        }
    }
    ctx.set_line_width(1.0);
    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.5);
    ctx.stroke()?;
    ctx.restore()?;

    src.write_to_png(&mut fs::File::create(&args.dst)?)?;
    Ok(())
}
