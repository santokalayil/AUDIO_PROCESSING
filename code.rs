//  rustc code.rs && ./code


fn main() {
    println!("AUDIO PROCESSING UNDERSTANDING");
    println!("Frequency of Midi note (A5) number 81 ==> {}", frequency_of_pitch(81));
}

const MIDI_NOTE_NUMBER_OF_A4: u8 = 69;
const FREQUENCY_OF_MIDI_NOTE_A4: f32 = 440.;
const NOTES_PER_OCTAVE: u8 = 12;

// Mapping pitch to frequency
fn frequency_of_pitch(pitch: u8) -> f32  {
    2_f32.powf((pitch - MIDI_NOTE_NUMBER_OF_A4) as f32 / NOTES_PER_OCTAVE as f32) as f32 * FREQUENCY_OF_MIDI_NOTE_A4
}


