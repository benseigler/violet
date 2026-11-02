use crate::{
    SampleProcessor, SourceInterpreter, audio_input::AudioInput,
    audio_input_socket::AudioInputSocket, audio_output::AudioOutput,
    audio_output_socket::AudioOutputSocket, interpretation_storage::InterpretationStorage,
    spatial_input::SpatialInput, spatial_input_socket::SpatialInputSocket,
};

/**
Renders spatial audio scenes.

A renderer is constructed using a `RendererBuilder`.
*/
pub struct Renderer<Interpreter, Processor, AudioIn, SpatialIn, AudioOut>
where
    AudioIn: AudioInput,
    SpatialIn: SpatialInput<Scalar = AudioIn::Sample>,
    AudioOut: AudioOutput,
    Interpreter: SourceInterpreter<SpatialIn>,
    Processor: SampleProcessor<AudioIn, AudioOut>,
{
    pub(crate) interpreter: Interpreter,
    pub(crate) processor: Processor,
    pub(crate) audio_input: AudioInputSocket<AudioIn>,
    pub(crate) spatial_input: SpatialInputSocket<SpatialIn>,
    pub(crate) audio_output: AudioOutputSocket<AudioOut>,
    pub(crate) interpretation_storage: InterpretationStorage,
}

impl<Interpreter, Processor, AudioIn, SpatialIn, AudioOut>
    Renderer<Interpreter, Processor, AudioIn, SpatialIn, AudioOut>
where
    AudioIn: AudioInput,
    SpatialIn: SpatialInput<Scalar = AudioIn::Sample>,
    AudioOut: AudioOutput,
    Interpreter: SourceInterpreter<SpatialIn>,
    Processor: SampleProcessor<AudioIn, AudioOut>,
{
    fn interpret_sources(&mut self) {
        self.interpreter
            .interpret_sources(&self.spatial_input, &mut self.interpretation_storage);
    }
    fn process_samples(&mut self) {
        self.processor.process_samples(
            &self.interpretation_storage,
            &self.audio_input,
            &mut self.audio_output,
        );
    }
    /// Updates the frame number stored in the input and output sockets.
    fn set_frames(&mut self, frames: usize) {
        self.audio_input.set_frames(frames);
        self.spatial_input.set_frames(frames);
        self.interpretation_storage.set_frames(frames);
        self.audio_output.set_frames(frames);
    }
    /**
    Tries to render a single frame, returning `None` if that frame was not
    available.
    */
    pub fn render_frame(&mut self) -> Option<()> {
        self.render_frames(1)
            .and_then(|frames| (frames == 1).then_some(()))
    }
    /**
    Tries to render the given amount of frames and returns the amount of
    frames actually rendered.

    If more frames are requested than there are available, this method will
    only render the available frames.
    */
    pub fn render_frames(&mut self, frames: usize) -> Option<usize> {
        let frames = frames.min(self.frames_available()?);
        if frames == 0 {
            return Some(0);
        }
        self.render_frames_unchecked(frames);
        Some(frames)
    }
    /**
    Renders the given amount of frames without checking if that amount of
    frames are available for rendering.

    Rendering more frames than are available can have wildly unpredictable
    results depending on the renderer's inputs and outputs.
    */
    fn render_frames_unchecked(&mut self, frames: usize) {
        self.set_frames(frames);
        self.prepare_chunk(frames);
        self.interpret_sources();
        self.process_samples();
        self.advance(frames);
    }
    /**
    Renders the frames that are available for rendering, returning `None`
    if no frames were available.
    */
    pub fn render_available_frames(&mut self) -> Option<usize> {
        let frames = self.frames_available()?;
        if frames == 0 {
            return Some(0);
        }
        self.render_frames_unchecked(frames);
        Some(frames)
    }
    /// Returns the number of audio sources in the rendering scene.
    pub fn source_count(&self) -> usize {
        self.audio_input().channel_count()
    }
    /**
    Returns the amount of frames the connectors have available for rendering.
    Returns `None` if a connector has no frames available and will not have any
    more in the future.
    */
    fn connector_frames_available(&self) -> Option<usize> {
        let audio_in = self.audio_input().frames_available()?;
        let spatial_in = self.spatial_input().frames_available()?;
        let audio_out = self.audio_output().frames_available()?;
        let max_frames = self.interpretation_storage.max_frames();
        Some(audio_in.min(spatial_in).min(audio_out).min(max_frames))
    }
    /**
    Returns the amount of frames available for rendering. Returns `None` if
    this renderer has finished rendering and will not have any more frames
    available.
    */
    pub fn frames_available(&self) -> Option<usize> {
        let available = self.connector_frames_available()?;
        let sample_rate = self.audio_input().sample_rate();
        Some(
            self.sample_processor()
                .acceptable_frames(sample_rate, available)
                .min(available),
        )
    }
    fn prepare_chunk(&mut self, frames: usize) {
        self.audio_input_mut().prepare_chunk(frames);
        self.spatial_input_mut().prepare_chunk(frames);
        self.audio_output_mut().prepare_chunk(frames);
    }
    /// Advances all connectors.
    fn advance(&mut self, frames: usize) {
        self.audio_input_mut().advance(frames);
        self.spatial_input_mut().advance(frames);
        self.audio_output_mut().advance(frames);
    }
    /// Returns the channel count of the audio output/sample processor.
    pub fn output_channels(&self) -> usize {
        self.audio_output().channel_count()
    }

    /// Returns an immutable reference to the audio input.
    pub fn audio_input(&self) -> &AudioIn {
        self.audio_input.input()
    }
    /// Returns a mutable reference to the audio input.
    pub fn audio_input_mut(&mut self) -> &mut AudioIn {
        self.audio_input.input_mut()
    }
    /// Returns an immutable reference to the sample processor.
    pub fn sample_processor(&self) -> &Processor {
        &self.processor
    }
    /// Returns a mutable reference to the sample processor.
    pub fn sample_processor_mut(&mut self) -> &mut Processor {
        &mut self.processor
    }
    /// Returns an immutable reference to the source interpreter.
    pub fn interpreter(&self) -> &Interpreter {
        &self.interpreter
    }
    /// Returns a mutable reference to the source interpreter.
    pub fn interpreter_mut(&mut self) -> &mut Interpreter {
        &mut self.interpreter
    }
    /// Returns an immutable reference to the spatial input.
    pub fn spatial_input(&self) -> &SpatialIn {
        self.spatial_input.input()
    }
    /// Returns a mutable reference to the spatial input.
    pub fn spatial_input_mut(&mut self) -> &mut SpatialIn {
        self.spatial_input.input_mut()
    }
    /// Returns an immutable reference to the audio output.
    pub fn audio_output(&self) -> &AudioOut {
        self.audio_output.output()
    }
    /// Returns a mutable reference to the audio output.
    pub fn audio_output_mut(&mut self) -> &mut AudioOut {
        self.audio_output.output_mut()
    }
}
