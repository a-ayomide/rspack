use std::io;

use thiserror::Error;

#[derive(Debug)]
/// ## Warning
/// For a [TraceableError], the path is required.
/// Because if the source code is missing when you construct a [TraceableError], we could read it from file system later
/// when convert it into [crate::Diagnostic], but the reverse will not working.
pub struct TraceableError {
  pub path: String,
  pub start: usize,
  pub end: usize,
  pub error_message: String,
  pub title: String,
  pub source: Option<String>,
  pub kind: DiagnosticKind,
}

impl TraceableError {
  pub fn from_path(
    path: String,
    start: usize,
    end: usize,
    title: String,
    error_message: String,
  ) -> Self {
    Self {
      path,
      start,
      end,
      error_message,
      source: None,
      title,
      kind: DiagnosticKind::Internal,
    }
  }
  pub fn with_source(mut self, source: String) -> Self {
    self.source = Some(source);
    self
  }

  pub fn with_kind(mut self, kind: DiagnosticKind) -> Self {
    self.kind = kind;
    self
  }
}

#[derive(Error, Debug)]
pub enum Error {
  #[error("{0}")]
  InternalError(String),

  #[error("")]
  TraceableError(TraceableError),

  #[error("")]
  Io {
    #[from]
    source: io::Error,
  },
  #[error("")]
  Anyhow {
    #[from]
    source: anyhow::Error,
  },
  #[error("")]
  BatchErrors(Vec<Error>),
}

impl Error {
  pub fn kind(&self) -> DiagnosticKind {
    match self {
      Error::InternalError(_) => DiagnosticKind::Internal,
      Error::TraceableError(_) => todo!(),
      Error::Io { .. } => todo!(),
      Error::Anyhow { .. } => todo!(),
      Error::BatchErrors(_) => todo!(),
    }
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum DiagnosticKind {
  JavaScript,
  Typescript,
  Jsx,
  Tsx,
  Scss,
  Css,
  #[default]
  Internal,
  Io,
  Json,
  Html,
}

/// About the manually implementation,
/// dispaly string should be snake, for consistency.
impl std::fmt::Display for DiagnosticKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DiagnosticKind::JavaScript => write!(f, "javascript"),
      DiagnosticKind::Typescript => write!(f, "typescript"),
      DiagnosticKind::Jsx => write!(f, "jsx"),
      DiagnosticKind::Tsx => write!(f, "tsx"),
      DiagnosticKind::Scss => write!(f, "scss"),
      DiagnosticKind::Css => write!(f, "css"),
      DiagnosticKind::Internal => write!(f, "internal"),
      DiagnosticKind::Io => write!(f, "io"),
      DiagnosticKind::Json => write!(f, "json"),
      DiagnosticKind::Html => write!(f, "html"),
    }
  }
}
