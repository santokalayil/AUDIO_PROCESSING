//  rustc code.rs && ./code

// # Mapping pitch to frequency

// MIDI_NOTE_NUMBER_OF_A4 = 69
// FREQUENCY_OF_MIDI_NOTE_A4 = 440  # Hertz (measured in)  440hz is the frequency of A4 which is the reference point in music
// NOTES_PER_OCTAVE = 12

// def frequency_of_pitch(pitch):  # pitch is midi note number for specific pitch like D3 etc.
//     global MIDI_NOTE_NUMBER_OF_A4, FREQUENCY_OF_MIDI_NOTE_A4, NOTES_PER_OCTAVE
//     return 2 ** ((pitch - MIDI_NOTE_NUMBER_OF_A4) / NOTES_PER_OCTAVE) * FREQUENCY_OF_MIDI_NOTE_A4 



// frequency_of_pitch(pitch=81)  # 81 is the number of midi note A5 


fn main() {
    println!("AUDIO PROCESSING UNDERSTANDING");
    println!("Frequency of Midi note (A5) number 81 ==> {}", frequency_of_pitch(81));
}

const MIDI_NOTE_NUMBER_OF_A4: u8 = 69;
const FREQUENCY_OF_MIDI_NOTE_A4: f32 = 440.;
const NOTES_PER_OCTAVE: u8 = 12;

fn frequency_of_pitch(pitch: u8) -> f32  {
    2_f32.powf((pitch - MIDI_NOTE_NUMBER_OF_A4) as f32 / NOTES_PER_OCTAVE as f32) as f32 * FREQUENCY_OF_MIDI_NOTE_A4
}


