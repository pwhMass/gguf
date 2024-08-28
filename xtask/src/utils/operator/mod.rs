﻿mod cast;
mod merge;
mod set_meta;
mod sort;

use super::{compile_patterns, Content, DataPromise};
use ggus::{GGmlType, GGufMetaDataValueType, GENERAL_ARCHITECTURE};
use regex::Regex;
use std::collections::HashMap;

pub(crate) enum Operator {
    FilterMetaKey(Regex),
    FilterTensorName(Regex),
    Cast(GGmlType),
    MergeLinear(bool),
    SetMeta(HashMap<String, (GGufMetaDataValueType, Vec<u8>)>),
    SortTensors,
}

impl Operator {
    #[inline]
    pub fn filter_meta_key(p: impl AsRef<str>) -> Self {
        Self::FilterMetaKey(compile_patterns(p.as_ref()))
    }

    #[inline]
    pub fn filter_tensor_name(p: impl AsRef<str>) -> Self {
        Self::FilterTensorName(compile_patterns(p.as_ref()))
    }
}

impl Content<'_> {
    pub fn apply(&mut self, op: Operator) {
        use Operator::*;
        match op {
            FilterMetaKey(r) => self.meta_kvs.retain(|k, _| r.is_match(k)),
            FilterTensorName(r) => self.tensors.retain(|k, _| r.is_match(k)),
            Cast(ty) => self.cast(ty),
            MergeLinear(ty) => self.merge_linear(ty),
            SetMeta(map) => self.set_meta(map),
            SortTensors => self.sort_tensors(),
        }
    }

    fn assert_llama(&self) {
        match self
            .meta_kvs
            .get(GENERAL_ARCHITECTURE)
            .expect("missing architecture")
            .value_reader()
            .read_general_architecture_val()
        {
            Ok("llama") => {}
            Ok(arch) => todo!("unsupported architecture: {arch}"),
            Err(e) => panic!("failed to read architecture: {e:?}"),
        }
    }
}
