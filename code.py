# Mapping pitch to frequency

MIDI_NOTE_NUMBER_OF_A4 = 69
FREQUENCY_OF_MIDI_NOTE_A4 = 440  # Hertz (measured in)  440hz is the frequency of A4 which is the reference point in music
NOTES_PER_OCTAVE = 12


def frequency_of_pitch(pitch):  # pitch is midi note number for specific pitch like D3 etc.
    global MIDI_NOTE_NUMBER_OF_A4, FREQUENCY_OF_MIDI_NOTE_A4, NOTES_PER_OCTAVE
    return 2 ** ((pitch - MIDI_NOTE_NUMBER_OF_A4) / NOTES_PER_OCTAVE) * FREQUENCY_OF_MIDI_NOTE_A4 

frequency = lambda midi_note_number: 2 ** ((midi_note_number - 69) / 12) * 440 

frequency(81)


if __name__ == "__main__":
    print(f"Frequency of Midi note (A5) number 81 ==> {frequency_of_pitch(pitch=81)}")
    print(f"Lambda function usage: Frequency of Midi note (A5) number 81 ==> {frequency(midi_note_number=81)}")
