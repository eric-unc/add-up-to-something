use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use adder_codec_rs::adder_codec_core::{SourceCamera, TimeMode};
use adder_codec_rs::adder_codec_core::codec::EncoderType;
use adder_codec_rs::adder_codec_core::codec::EncoderOptions;
use adder_codec_rs::transcoder::source::framed::Framed;
use adder_codec_rs::transcoder::source::video::{Source, SourceError, VideoBuilder};
use rayon::current_num_threads;

fn main() -> Result<(), Box<dyn Error>> {
  let out = BufWriter::new(
    File::create("out.adder")?);

  let mut framed_src = Framed::new(
    // the filename, color input, and scale
    "in.mp4".to_string(), false, 0.25)?
    // the specific frame to start transcoding from
    .frame_start(0)?;

  let plane = framed_src.get_video_ref().state.plane;

  framed_src = framed_src
    .write_out(SourceCamera::FramedU8,
      TimeMode::DeltaT,
      EncoderType::Raw,
      EncoderOptions::default(plane),
      out)?
    .crf(5) // 0 best encoder quality, 9 worst=
    .show_display(true) // show live view
    .auto_time_parameters(255, 255 * 30, None)?;

  let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(current_num_threads())
    .build()
    .unwrap();

  loop {
    // intake one frame
    match framed_src.consume(1, &pool) {
      Ok(events_in_frame) => {
        for events_in_row in events_in_frame {
          for event in events_in_row {
            if event.coord.x >= 20
              && event.coord.x <= 80
              && event.coord.y == 50 {
              println!("{:?}", event);
            }
          }
        }
      }
      // end of stream
      Err(SourceError::NoData) => break,
      Err(e) => {
        eprintln!("Err: {e:?}");
        break;
      }
    }
  }

  framed_src.get_video_mut()
    .end_write_stream().unwrap();

  println!("Finished!");

  Ok(())
}
