#[macro_use] extern crate jack;

use jack::*;
use std::io::timer::*;
use std::io::println;
use std::time::duration::Duration;
use std::ptr;
use std::num::Float;

struct CallbackData {
  client: JackClient,
  port: JackPort,
  synth: Synthdata,
}

struct Synthdata {
  phase: f32,
}

fn main() {
    // so now... on start up, get a file, mmap a file 
    // loop is copy bytes

    //phase = (phase + (1 / (sr / 440))) 
    fn calc_point(synth: &mut Synthdata, sample_rate: u32) -> f32 { 
      synth.phase = synth.phase + (1f32 / (sample_rate as f32 / 440f32)) ;
      0.8f32 * synth.phase.to_radians().sin()
    }

    unsafe fn sine(nframes: JackNframesT, data:&mut CallbackData) { 
      let buf:*mut f32 = (*data).port.get_buffer(nframes);
      
      for i in range(0,nframes) {
        std::ptr::write(buf.offset(i as isize),calc_point(&mut data.synth, (*data).client.sample_rate()));
      }
    }

    unsafe fn silence(nframes: JackNframesT, data:&mut CallbackData) {
      let buf:*mut f32 = (*data).port.get_buffer(nframes);
      std::ptr::set_memory(buf,0,nframes as usize);
    } 

    fn callback(frames: JackNframesT, data: *mut CallbackData) -> isize {
      unsafe {
        sine(frames, &mut *data);
      }
      0
    };

    let client = JackClient::open("kuma", JackNullOption);
    let port = client.register_port("out", JACK_DEFAULT_AUDIO_TYPE, JackPortIsOutput, 0);
    let synth = Synthdata{ phase: 0f32 };

    let mut cbd = CallbackData {
      port: port,
      client: client,
      synth: synth,
    };

    if (client.set_process_callback(callback, &mut cbd) & client.activate()) { 
        sleep(Duration::seconds(20));
        client.close();
    } else {
      println("error!!");
    }
}
