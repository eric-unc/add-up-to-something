use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use adder_codec_rs::adder_codec_core::{SourceCamera, TimeMode};
use adder_codec_rs::adder_codec_core::codec::EncoderType;
use adder_codec_rs::transcoder::source::framed::Framed;
use adder_codec_rs::transcoder::source::video::{Source, VideoBuilder};
use rayon::current_num_threads;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: this video is boring as fuck
    let out = BufWriter::new(File::create("Apartment quiet.adder")?);

    let mut framed_src = Framed::new(
        // the filename, color input, and scale
        "Apartment quiet.mp4".to_string(), true, 0.5)?
        .frame_start(0)?
        .write_out(SourceCamera::FramedU8, TimeMode::DeltaT, EncoderType::Raw, /*EncoderOptions::default(), */out)?
        .contrast_thresholds(10, 10)
        .show_display(true)
        .auto_time_parameters(255, 255 * 30, None)?;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(current_num_threads())
        .build()
        .unwrap();

    loop {
        match framed_src.consume(1, &pool) {
            Ok(_) => {} // Returns Vec<Vec<Event>>, but we're just writing the events out in this example
            Err(e) => {
                eprintln!("Err: {e:?}");
                break;
            }
        }

        // todo?
    }

    framed_src.get_video_mut().end_write_stream().unwrap();

    println!("Finished!");

    Ok(())
}
