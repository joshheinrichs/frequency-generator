# frequency-generator

A simple command line tool for generating vorbis audio files with specific
frequencies. Type `frequency-generator -h` for full usage information. No input
validation is performed at the moment, so don't be surprised if things don't
work properly if you disrespect the 
[Nyquist-Shannon sampling theorem](https://en.wikipedia.org/wiki/Nyquist%E2%80%93Shannon_sampling_theorem).

## Example


```
$ frequency-generator 1000 --length 5
frequency: 1000Hz
length: 5s
amplitude: 0.75
sample rate: 48000
output: output.ogg
```
