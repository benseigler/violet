use xpans_render::host::{InterpretSources, InterpretationInfo};

use crate::{
    interpretation_storage::InterpretationStorage, spatial_input_socket::SpatialInputSocket,
};

/// Evaluates an audio source's properties into values used by a sample processor.
pub trait SourceInterpreter<In>
where
    Self: InterpretSources<SpatialInputSocket<In>, InterpretationStorage> + InterpretationInfo,
{
}
impl<T, In> SourceInterpreter<In> for T where
    T: ?Sized
        + InterpretSources<SpatialInputSocket<In>, InterpretationStorage>
        + InterpretationInfo
{
}
