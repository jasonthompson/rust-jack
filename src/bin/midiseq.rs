extern crate collections;
extern crate getopts;
extern crate jack;


use jack::{JackNframesT,JackClient};
use std::os;
use std::io::timer;
use std::time::duration::Duration;

fn print_usage() {
	  println!("usage: midiseq name nsamp [startindex note nsamp] ...... [startindex note nsamp]");
	  println!(" eg: jack_midiseq Sequencer 24000 0 60 8000 12000 63 8000");
	  println!(" will play a 1/2 sec loop (if srate is 48khz) with a c4 note at the start of the loop");
	  println!(" that lasts for 12000 samples, then a d4# that starts at 1/4 sec that lasts for 800 samples");
}

struct Note {
    freq: u8,
    start: JackNframesT,
    length: JackNframesT,
}

struct CallbackData {
    notes: Vec<Note>,
    loop_nsamp: JackNframesT,
    loop_index: JackNframesT,
    port: jack::JackPort,
}

fn process(nframes: JackNframesT, data:* mut CallbackData) -> int {
    let cbd = unsafe { &mut *data };
    let midi_buf = cbd.port.get_midi_buffer(nframes);
    midi_buf.clear_buffer();

    for i in range(0,nframes) {
        for note in cbd.notes.iter() {
            if note.start == cbd.loop_index {
                let event = midi_buf.reserve_event(i,3);
                event.write_data(0,0x90); // note on
                event.write_data(1,note.freq);
                event.write_data(2,64); // velocity
            }
            else if note.start + note.length == cbd.loop_index {
                let event = midi_buf.reserve_event(i,3);
                event.write_data(0,0x80); // note off
                event.write_data(1,note.freq);
                event.write_data(2,64); // velocity
            }
        }
        cbd.loop_index =
            if cbd.loop_index + 1 >= cbd.loop_nsamp {
                0
            } else {
                cbd.loop_index + 1
            }
    }
    0
}


fn get_nframes_arg(arg: &collections::string::String) -> JackNframesT {
    from_str::<JackNframesT>(arg.as_slice()).unwrap()
}

fn main() {
    let args: Vec<String> = os::args();
    if args.len() < 6 || (args.len()-3)%3 != 0 {
        print_usage();
        return;
    }

    let client = JackClient::open(args[1].as_slice(), jack::JackNullOption);
    let outport = client.register_port("out",jack::JACK_DEFAULT_MIDI_TYPE, jack::JackPortIsOutput, 0);

    let num_notes = (args.len()-3)/3;
    let mut notes = Vec::with_capacity(num_notes);

     for i in range(0,num_notes) {
         let start = get_nframes_arg(&args[3 + 3*i]);
         let freq = from_str::<u8>(args[4 + 3*i].as_slice()).unwrap();
         let length = get_nframes_arg(&args[5 + 3*i]);
         notes.push(Note {
             freq: freq,
             start: start,
             length: length,
         });
     }

    let mut cbdata = CallbackData {
        notes: notes,
        loop_nsamp: get_nframes_arg(&args[2]),
        loop_index: 0,
        port: outport,
    };

    client.set_process_callback(process,&mut cbdata);

    if !client.activate() {
        println!("can't activate")
    }

    loop {
        timer::sleep(Duration::minutes(1));
    }
}

