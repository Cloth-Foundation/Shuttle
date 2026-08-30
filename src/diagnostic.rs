use std::fmt;
use std::ops::Range;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

pub fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|left, right| {
        let left_path = left.path.as_deref().map(display_path).unwrap_or_default();
        let right_path = right.path.as_deref().map(display_path).unwrap_or_default();
        left_path
            .cmp(&right_path)
            .then_with(|| {
                left.position
                    .map(|position| position.line)
                    .cmp(&right.position.map(|position| position.line))
            })
            .then_with(|| {
                left.position
                    .map(|position| position.column)
                    .cmp(&right.position.map(|position| position.column))
            })
            .then_with(|| left.message.cmp(&right.message))
    });
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    path: Option<PathBuf>,
    position: Option<SourcePosition>,
    message: String,
}

impl Diagnostic {
    #[must_use]
    pub fn global(message: impl Into<String>) -> Self {
        Self {
            path: None,
            position: None,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn file(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            path: Some(path.into()),
            position: None,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn at_span(
        path: impl Into<PathBuf>,
        source: &str,
        span: Range<usize>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: Some(path.into()),
            position: Some(position_for_offset(source, span.start)),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn at_position(
        path: impl Into<PathBuf>,
        position: SourcePosition,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: Some(path.into()),
            position: Some(position),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn position(&self) -> Option<SourcePosition> {
        self.position
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.path, self.position) {
            (Some(path), Some(position)) => write!(
                formatter,
                "{}:{}:{}: error: {}",
                display_path(path),
                position.line,
                position.column,
                self.message
            ),
            (Some(path), None) => {
                write!(formatter, "{}: error: {}", display_path(path), self.message)
            }
            (None, _) => write!(formatter, "shuttle: error: {}", self.message),
        }
    }
}

#[must_use]
pub fn position_for_offset(source: &str, byte_offset: usize) -> SourcePosition {
    let mut offset = byte_offset.min(source.len());
    while !source.is_char_boundary(offset) {
        offset -= 1;
    }
    let prefix = &source[..offset];
    let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
    let column = prefix[line_start..].chars().count() + 1;
    SourcePosition { line, column }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::{SourcePosition, position_for_offset};

    #[test]
    fn computes_unicode_source_positions() {
        let source = "first\nnaïve = true\n";
        let offset = source.find("true").expect("test input contains true");
        assert_eq!(
            position_for_offset(source, offset),
            SourcePosition { line: 2, column: 9 }
        );
        assert_eq!(
            position_for_offset("é", 1),
            SourcePosition { line: 1, column: 1 }
        );
    }
}
