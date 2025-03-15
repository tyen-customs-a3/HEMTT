use std::{collections::HashMap, ops::Range, rc::Rc, sync::RwLock};

use codespan_reporting::files;

use crate::WorkspacePath;

#[derive(Debug, Clone)]
pub struct WorkspaceFile {
    /// The source code of the file.
    source: Rc<str>,
    /// The starting byte indices in the source code.
    line_starts: Rc<[usize]>,
}

impl WorkspaceFile {
    fn line_start(&self, line_index: usize) -> Result<usize, files::Error> {
        use std::cmp::Ordering;

        // Handle empty file case
        if self.line_starts.is_empty() {
            if line_index == 0 {
                return Ok(0);
            }
            return Err(files::Error::LineTooLarge {
                given: line_index,
                max: 0,
            });
        }

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(*self
                .line_starts
                .get(line_index)
                .expect("failed despite previous check")),
            Ordering::Equal => Ok(self.source.len()),
            Ordering::Greater => Err(files::Error::LineTooLarge {
                given: line_index,
                max: self.line_starts.len() - 1,
            }),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct WorkspaceFiles {
    cache: RwLock<HashMap<WorkspacePath, WorkspaceFile>>,
}

impl WorkspaceFiles {
    /// Create a new `WorkspaceFiles` instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Get the file corresponding to the given id.
    fn get(&self, file_id: &WorkspacePath) -> WorkspaceFile {
        let mut cache = self
            .cache
            .write()
            .expect("failed to lock workspace files cache");
        cache
            .entry(file_id.clone())
            .or_insert_with(|| {
                let source = file_id.read_to_string().unwrap_or_default();
                let line_starts = if source.is_empty() {
                    vec![0]
                } else {
                    let mut starts = vec![0];
                    starts.extend(source.match_indices('\n').map(|(i, _)| i + 1));
                    starts
                };
                WorkspaceFile {
                    source: source.into(),
                    line_starts: line_starts.into(),
                }
            })
            .clone()
    }
}

impl<'files> files::Files<'files> for WorkspaceFiles {
    type FileId = &'files WorkspacePath;

    type Name = &'files str;

    type Source = Rc<str>;

    fn name(&'files self, id: Self::FileId) -> Result<Self::Name, files::Error> {
        Ok(id.data.path.as_str().trim_start_matches('/'))
    }

    fn source(&'files self, id: Self::FileId) -> Result<Self::Source, files::Error> {
        Ok(self.get(id).source)
    }

    fn line_index(&self, file_id: Self::FileId, byte_index: usize) -> Result<usize, files::Error> {
        self.get(file_id)
            .line_starts
            .binary_search(&byte_index)
            .map_or_else(|next_line| {
                // If next_line is 0, we're before the first line
                if next_line == 0 {
                    Ok(0)
                } else {
                    Ok(next_line - 1)
                }
            }, Ok)
    }

    fn line_range(
        &self,
        file_id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>, files::Error> {
        let file = self.get(file_id);
        let line_start = file.line_start(line_index)?;
        let next_line_start = file.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }
}
