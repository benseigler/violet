use crate::{
    audio_input::AudioInput, audio_input_socket::AudioInputSocket, audio_output::AudioOutput,
    audio_output_socket::AudioOutputSocket, interpretation_storage::InterpretationStorage,
    renderer::Renderer, sample_processor::SampleProcessor, source_interpreter::SourceInterpreter,
    spatial_input::SpatialInput, spatial_input_socket::SpatialInputSocket,
};

/// Builds a renderer.
#[derive(Debug)]
pub struct RendererBuilder<Interpreter, Processor, AudioIn, SpatialIn, AudioOut> {
    interpreter: Option<Interpreter>,
    processor: Option<Processor>,
    audio_input: Option<AudioIn>,
    spatial_input: Option<SpatialIn>,
    audio_output: Option<AudioOut>,
    max_block_size: Option<usize>,
}
impl<Interpreter, Processor, AudioIn, SpatialIn, AudioOut> Default
    for RendererBuilder<Interpreter, Processor, AudioIn, SpatialIn, AudioOut>
{
    fn default() -> Self {
        Self {
            interpreter: None,
            processor: None,
            audio_input: None,
            spatial_input: None,
            audio_output: None,
            max_block_size: None,
        }
    }
}

impl<Interpreter, Processor, AudioIn, SpatialIn, AudioOut>
    RendererBuilder<Interpreter, Processor, AudioIn, SpatialIn, AudioOut>
where
    AudioIn: AudioInput,
    SpatialIn: SpatialInput<Scalar = AudioIn::Sample>,
    AudioOut: AudioOutput,
    Interpreter: SourceInterpreter<SpatialIn>,
    Processor: SampleProcessor<AudioIn, AudioOut>,
{
    /// Creates a new `RendererBuilder`.
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_max_block_size(&mut self, max_block_size: usize) {
        self.max_block_size = Some(max_block_size)
    }
    /**
    Sets the spatial input of the renderer.

    The source count of the spatial input should equal the channel count of
    the audio input.
    */
    pub fn set_spatial_input(mut self, spatial_input: SpatialIn) -> Self {
        self.spatial_input = Some(spatial_input);
        self
    }
    /**
    Sets the audio input of the renderer.

    The channel count of the audio input should equal the source count of
    the spatial input.
    */
    pub fn set_audio_input(mut self, audio_input: AudioIn) -> Self {
        self.audio_input = Some(audio_input);
        self
    }
    /**
    Sets the audio output of the renderer.

    The channel count of the audio output should equal the channel count
    of the sample processor.
    */
    pub fn set_audio_output(mut self, audio_output: AudioOut) -> Self {
        self.audio_output = Some(audio_output);
        self
    }
    /// Sets the source interpreter of the renderer.
    pub fn set_source_interpreter(mut self, interpreter: Interpreter) -> Self {
        self.interpreter = Some(interpreter);
        self
    }
    /**
    Sets the sample processor of the renderer.

    The channel count of the sample processor should equal the channel
    count of the audio output.
    */
    pub fn set_sample_processor(mut self, processor: Processor) -> Self {
        self.processor = Some(processor);
        self
    }
    pub fn max_frames_available(&self) -> Result<usize, RendererBuildError> {
        let audio_input = self.audio_input.as_ref().unwrap().max_frames_available();
        let spatial_input = self.spatial_input.as_ref().unwrap().max_frames_available();
        let audio_output = self.audio_output.as_ref().unwrap().max_frames_available();
        println!("Audio Input: {audio_input}");
        println!("Spatial Input: {spatial_input}");
        println!("Audio Output: {audio_output}");
        let mut max = audio_input.min(spatial_input.min(audio_output));
        if let Some(max_block_size) = self.max_block_size {
            max = max.min(max_block_size);
        }

        let sample_rate = self.audio_input.as_ref().unwrap().sample_rate();
        (max >= self.processor.as_ref().unwrap().minimum_frames(sample_rate))
            .ok_or(RendererBuildError::InsufficientMaxFramesAvailable)?;

        Ok(max)
    }
    /**
    Attempts to build the renderer, returning a `RendererBuildError` if
    the renderer fails to build.
    */
    pub fn build(
        self,
    ) -> Result<Renderer<Interpreter, Processor, AudioIn, SpatialIn, AudioOut>, RendererBuildError>
    {
        self.check_any_missing()?;

        self.inputs_compatible()?;
        self.output_compatible()?;
        self.interpretation_types_compatible()?;

        let max_frames_available = self.max_frames_available()?;

        let interpreter = self.interpreter.unwrap();
        let processor = self.processor.unwrap();
        let audio_input = self.audio_input.unwrap();
        let spatial_input = self.spatial_input.unwrap();
        let audio_output = self.audio_output.unwrap();

        let count = audio_input.channel_count();

        let interpretation_storage =
            InterpretationStorage::new(&processor, count, max_frames_available);
        Ok(Renderer {
            interpreter,
            processor,
            audio_input: AudioInputSocket::new(audio_input),
            spatial_input: SpatialInputSocket::new(spatial_input),
            audio_output: AudioOutputSocket::new(audio_output),
            interpretation_storage,
        })
    }
    fn inputs_compatible(&self) -> Result<(), RendererBuildError> {
        use RendererBuildError::*;

        let audio = self.audio_input.as_ref().unwrap();
        let spatial = self.spatial_input.as_ref().unwrap();
        let audio_channels = audio.channel_count();
        let spatial_channels = spatial.channel_count();
        if audio_channels != spatial_channels {
            return Err(InputChannelMismatch);
        }
        let audio_sample_rate = audio.sample_rate();
        let spatial_sample_rate = spatial.sample_rate();
        if audio_sample_rate != spatial_sample_rate {
            return Err(InputSampleRateMismatch);
        }
        Ok(())
    }
    fn output_compatible(&self) -> Result<(), RendererBuildError> {
        use RendererBuildError::*;

        let processor_samples = self.processor.as_ref().unwrap().output_channels();
        let output_channels = self.audio_output.as_ref().unwrap().channel_count();
        if processor_samples != output_channels {
            return Err(OutputChannelMismatch);
        }

        // These unwraps should be fine since we check if these are `Some`
        // before this.
        let input_sample_rate = self.audio_input.as_ref().unwrap().sample_rate();
        let output_sample_rate = self.audio_output.as_ref().unwrap().sample_rate();

        if input_sample_rate != output_sample_rate {
            return Err(OutputSampleRateMismatch);
        }
        Ok(())
    }
    fn interpretation_types_compatible(&self) -> Result<(), RendererBuildError> {
        use RendererBuildError::*;

        // Check layout
        let interpreter = self.interpreter.as_ref().unwrap();
        let processor = self.processor.as_ref().unwrap();
        let i_layout = interpreter.interpretation_layout();
        let p_layout = processor.interpretation_layout();
        if i_layout != p_layout {
            return Err(InterpretationTypeMismatch);
        }
        // Check length
        let i_length = interpreter.interpretation_length();
        let p_length = processor.interpretation_length();
        if i_length != p_length {
            return Err(InterpretationLengthMismatch);
        }
        // Check initialization equality
        let mut i_bytes = vec![0u8; i_layout.size()];
        let mut p_bytes = vec![0u8; i_layout.size()];
        unsafe { interpreter.initialize_interpretation(&mut i_bytes) };
        unsafe { processor.initialize_interpretation(&mut p_bytes) };
        for (b1, b2) in i_bytes.iter().zip(p_bytes.iter()) {
            (b1 == b2).ok_or(InterpretationInitMismatch)?;
        }
        Ok(())
    }
    fn check_any_missing(&self) -> Result<(), RendererBuildError> {
        use RendererBuildError::*;
        self.interpreter.as_ref().ok_or(MissingInterpreter)?;
        self.processor.as_ref().ok_or(MissingProcessor)?;
        self.audio_input.as_ref().ok_or(MissingAudioInput)?;
        self.spatial_input.as_ref().ok_or(MissingAudioOutput)?;
        self.audio_output.as_ref().ok_or(MissingSpatialInput)?;
        Ok(())
    }
}

// TODO: Validate audio input and processor delay length compatibility
// and error when invalid.
#[derive(Debug, Clone, Copy)]
pub enum RendererBuildError {
    MissingInterpreter,
    MissingProcessor,
    MissingAudioInput,
    MissingSpatialInput,
    MissingAudioOutput,
    InputSampleRateMismatch,
    OutputSampleRateMismatch,
    InputChannelMismatch,
    OutputChannelMismatch,
    InterpretationTypeMismatch,
    InterpretationLengthMismatch,
    InterpretationInitMismatch,
    InsufficientMaxFramesAvailable,
}
