//! # Colorop
//!
//! A color transform operation applied on a plane.
//!
//! They form fixed chains through the NEXT property of each colorop.
//! The COLOR_PIPELINE property of eligible planes can be set to the starting
//! colorop of a chain.
//!
//! [`ClientCapability::PlaneColorPipeline`] must be set for these properties to
//! be available.

use crate::control;
use drm_ffi as ffi;

/// A handle to a specific Colorop
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Handle(control::RawResourceHandle);

// Safety: Handle is repr(transparent) over NonZeroU32
unsafe impl bytemuck::ZeroableInOption for Handle {}
unsafe impl bytemuck::PodInOption for Handle {}

impl From<Handle> for control::RawResourceHandle {
    fn from(handle: Handle) -> Self {
        handle.0
    }
}

impl From<Handle> for u32 {
    fn from(handle: Handle) -> Self {
        handle.0.into()
    }
}

impl From<control::RawResourceHandle> for Handle {
    fn from(handle: control::RawResourceHandle) -> Self {
        Handle(handle)
    }
}

impl control::ResourceHandle for Handle {
    const FFI_TYPE: u32 = ffi::DRM_MODE_OBJECT_COLOROP;
}

impl std::fmt::Debug for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("colorop::Handle").field(&self.0).finish()
    }
}
