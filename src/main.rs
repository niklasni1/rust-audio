#[macro_use] extern crate jack;

use jack::*;
use std::io::timer::*;
use std::io::println;
use std::time::duration::Duration;

struct CallbackData {
  client: JackClient,
  port: JackPort,
}

fn main() {
    // so now... on start up, get a file, mmap a file 
    // loop is copy bytes

    unsafe fn silence(nframes: JackNframesT, data:&mut CallbackData) {
      let buf:*mut f32 = (*data).port.get_buffer(nframes);
      std::ptr::set_memory(buf,0,nframes as usize);
    } 

    fn callback(frames: JackNframesT, data: *mut CallbackData) -> isize {
      unsafe {
        silence(frames, &mut *data);
      }
      0
    };

    let client = JackClient::open("kuma", JackNullOption);
    let port = client.register_port("out", JACK_DEFAULT_AUDIO_TYPE, JackPortIsOutput, 0);

    let mut cbd = CallbackData {
      port: port,
      client: client,
    };

    if (client.set_process_callback(callback, &mut cbd) & client.activate()) { 
        sleep(Duration::seconds(5));
        client.close();
    } else {
      println("error!!");
    }
}
