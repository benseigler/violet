use crate::audio_output::AudioOutput;

/// Holds the audio output and additional state used to write samples to it.
pub struct AudioOutputSocket<AudioOut>
where
    AudioOut: AudioOutput,
{
    output: AudioOut,
    frames: usize,
}

impl<AudioOut> AudioOutputSocket<AudioOut>
where
    AudioOut: AudioOutput,
{
    pub(crate) fn new(output: AudioOut) -> Self {
        Self { output, frames: 0 }
    }
    pub(crate) fn output(&self) -> &AudioOut {
        &self.output
    }
    pub(crate) fn output_mut(&mut self) -> &mut AudioOut {
        &mut self.output
    }
    pub(crate) fn set_frames(&mut self, frames: usize) {
        self.frames = frames;
    }
}
impl<AudioOut> xpans_render::host::Output<AudioOut::Sample> for AudioOutputSocket<AudioOut>
where
    AudioOut: AudioOutput,
{
    fn frames(&self) -> usize {
        self.frames
    }
    fn channels(&self) -> usize {
        self.output().channel_count()
    }
    fn set_channel(&mut self, frame: usize, channel: usize, sample: AudioOut::Sample) {
        self.output.set_sample(channel, frame, sample);
    }
}
