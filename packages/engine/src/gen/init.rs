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

use super::batch::*;
use super::metaversion::*;
use super::package_config::*;
use super::serialized::*;
use super::shared_context::*;
use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum InitOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Init<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Init<'a> {
    type Inner = Init<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> Init<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Init { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args InitArgs<'args>,
    ) -> flatbuffers::WIPOffset<Init<'bldr>> {
        let mut builder = InitBuilder::new(_fbb);
        if let Some(x) = args.package_config {
            builder.add_package_config(x);
        }
        if let Some(x) = args.shared_context {
            builder.add_shared_context(x);
        }
        builder.finish()
    }

    pub const VT_SHARED_CONTEXT: flatbuffers::VOffsetT = 4;
    pub const VT_PACKAGE_CONFIG: flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn shared_context(&self) -> Option<SharedContext<'a>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<SharedContext>>(Init::VT_SHARED_CONTEXT, None)
    }
    #[inline]
    pub fn package_config(&self) -> PackageConfig<'a> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<PackageConfig>>(Init::VT_PACKAGE_CONFIG, None)
            .unwrap()
    }
}

impl flatbuffers::Verifiable for Init<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<SharedContext>>(
                &"shared_context",
                Self::VT_SHARED_CONTEXT,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<PackageConfig>>(
                &"package_config",
                Self::VT_PACKAGE_CONFIG,
                true,
            )?
            .finish();
        Ok(())
    }
}
pub struct InitArgs<'a> {
    pub shared_context: Option<flatbuffers::WIPOffset<SharedContext<'a>>>,
    pub package_config: Option<flatbuffers::WIPOffset<PackageConfig<'a>>>,
}
impl<'a> Default for InitArgs<'a> {
    #[inline]
    fn default() -> Self {
        InitArgs {
            shared_context: None,
            package_config: None, // required field
        }
    }
}
pub struct InitBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> InitBuilder<'a, 'b> {
    #[inline]
    pub fn add_shared_context(
        &mut self,
        shared_context: flatbuffers::WIPOffset<SharedContext<'b>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<SharedContext>>(
                Init::VT_SHARED_CONTEXT,
                shared_context,
            );
    }
    #[inline]
    pub fn add_package_config(
        &mut self,
        package_config: flatbuffers::WIPOffset<PackageConfig<'b>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<PackageConfig>>(
                Init::VT_PACKAGE_CONFIG,
                package_config,
            );
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> InitBuilder<'a, 'b> {
        let start = _fbb.start_table();
        InitBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Init<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_
            .required(o, Init::VT_PACKAGE_CONFIG, "package_config");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for Init<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Init");
        ds.field("shared_context", &self.shared_context());
        ds.field("package_config", &self.package_config());
        ds.finish()
    }
}
