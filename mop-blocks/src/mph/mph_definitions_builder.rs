use crate::mph::MphDefinitions;
use alloc::{string::String, vec::Vec};

/// Definitions Builder for MPH
///
/// # Types
///
/// * `C`: Constraints Storage
/// * `O`: Objectives Storage
/// * `SD`: Solution Domain
#[derive(Clone, Debug)]
pub struct MphDefinitionsBuilder<C, O, SD> {
  pub(crate) hard_cstrs: Vec<C>,
  pub(crate) name: Option<String>,
  pub(crate) objs: Vec<O>,
  pub(crate) solution_domain: Option<SD>,
}

impl<C, O, SD> MphDefinitionsBuilder<C, O, SD> {
  pub fn build(self) -> MphDefinitions<C, O, SD> {
    MphDefinitions {
      hard_cstrs: self.hard_cstrs,
      name: self.name.unwrap(),
      objs: self.objs,
      solution_domain: self.solution_domain.unwrap(),
    }
  }

  pub fn hard_cstrs(mut self, hard_cstrs: Vec<C>) -> Self {
    self.hard_cstrs = hard_cstrs;
    self
  }

  pub fn name<IN>(mut self, name: IN) -> Self
  where
    IN: Into<String>,
  {
    self.name = Some(name.into());
    self
  }

  pub fn objs(mut self, objs: Vec<O>) -> Self {
    self.objs = objs;
    self
  }

  pub fn push_hard_cstr(mut self, hard_cstr: C) -> Self {
    self.hard_cstrs.push(hard_cstr);
    self
  }

  pub fn push_obj(mut self, obj: O) -> Self {
    self.objs.push(obj);
    self
  }

  pub fn solution_domain(mut self, solution_domain: SD) -> Self {
    self.solution_domain = Some(solution_domain);
    self
  }
}

impl<C, O, SD> Default for MphDefinitionsBuilder<C, O, SD> {
  fn default() -> Self {
    Self { hard_cstrs: Vec::new(), objs: Vec::new(), name: None, solution_domain: None }
  }
}
