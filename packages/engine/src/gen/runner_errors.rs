#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::cast_sign_loss,
    clippy::empty_enum,
    clippy::used_underscore_binding,
    clippy::redundant_static_lifetimes,
    clippy::redundant_field_names,
    clippy::unused_imports,
    unused_imports
)]
// automatically generated by the FlatBuffers compiler, do not modify

use super::runner_error::*;
use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum RunnerErrorsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RunnerErrors<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RunnerErrors<'a> {
    type Inner = RunnerErrors<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> RunnerErrors<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        RunnerErrors { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RunnerErrorsArgs<'args>,
    ) -> flatbuffers::WIPOffset<RunnerErrors<'bldr>> {
        let mut builder = RunnerErrorsBuilder::new(_fbb);
        if let Some(x) = args.inner {
            builder.add_inner(x);
        }
        builder.finish()
    }

    pub const VT_INNER: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn inner(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<RunnerError<'a>>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<RunnerError>>,
            >>(RunnerErrors::VT_INNER, None)
            .unwrap()
    }
}

impl flatbuffers::Verifiable for RunnerErrors<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<RunnerError>>,
            >>(&"inner", Self::VT_INNER, true)?
            .finish();
        Ok(())
    }
}
pub struct RunnerErrorsArgs<'a> {
    pub inner: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<RunnerError<'a>>>,
        >,
    >,
}
impl<'a> Default for RunnerErrorsArgs<'a> {
    #[inline]
    fn default() -> Self {
        RunnerErrorsArgs {
            inner: None, // required field
        }
    }
}
pub struct RunnerErrorsBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RunnerErrorsBuilder<'a, 'b> {
    #[inline]
    pub fn add_inner(
        &mut self,
        inner: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<RunnerError<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(RunnerErrors::VT_INNER, inner);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RunnerErrorsBuilder<'a, 'b> {
        let start = _fbb.start_table();
        RunnerErrorsBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<RunnerErrors<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_.required(o, RunnerErrors::VT_INNER, "inner");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for RunnerErrors<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("RunnerErrors");
        ds.field("inner", &self.inner());
        ds.finish()
    }
}
#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_runner_errors<'a>(buf: &'a [u8]) -> RunnerErrors<'a> {
    unsafe { flatbuffers::root_unchecked::<RunnerErrors<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_runner_errors<'a>(buf: &'a [u8]) -> RunnerErrors<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<RunnerErrors<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `RunnerErrors`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_errors_unchecked`.
pub fn root_as_runner_errors(buf: &[u8]) -> Result<RunnerErrors, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<RunnerErrors>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `RunnerErrors` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_runner_errors_unchecked`.
pub fn size_prefixed_root_as_runner_errors(
    buf: &[u8],
) -> Result<RunnerErrors, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<RunnerErrors>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `RunnerErrors` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_errors_unchecked`.
pub fn root_as_runner_errors_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<RunnerErrors<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<RunnerErrors<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `RunnerErrors` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_errors_unchecked`.
pub fn size_prefixed_root_as_runner_errors_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<RunnerErrors<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<RunnerErrors<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a RunnerErrors and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `RunnerErrors`.
pub unsafe fn root_as_runner_errors_unchecked(buf: &[u8]) -> RunnerErrors {
    flatbuffers::root_unchecked::<RunnerErrors>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed RunnerErrors and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `RunnerErrors`.
pub unsafe fn size_prefixed_root_as_runner_errors_unchecked(buf: &[u8]) -> RunnerErrors {
    flatbuffers::size_prefixed_root_unchecked::<RunnerErrors>(buf)
}
#[inline]
pub fn finish_runner_errors_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RunnerErrors<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_runner_errors_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RunnerErrors<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
