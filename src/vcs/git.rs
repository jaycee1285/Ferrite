//! Compatibility VCS layer for the pruned Ferrite fork.
//!
//! The original app used `git2` for branch display and file status badges.
//! This fork disables those features but keeps a small API-compatible shim so
//! the rest of the app can be simplified incrementally.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Status marker for a single file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GitFileStatus {
    #[default]
    Clean,
    Modified,
    Staged,
    StagedModified,
    Untracked,
    Ignored,
    Deleted,
    Renamed,
    Conflict,
}

impl GitFileStatus {
    pub fn label(&self) -> &'static str {
        ""
    }

    pub fn icon(&self) -> &'static str {
        ""
    }

    pub fn is_visible(&self) -> bool {
        false
    }
}

/// Disabled Git service shim.
#[derive(Debug, Default)]
pub struct GitService;

impl GitService {
    pub fn new() -> Self {
        Self
    }

    pub fn open(&mut self, _path: &Path) -> Result<bool, std::io::Error> {
        Ok(false)
    }

    pub fn close(&mut self) {}

    pub fn is_open(&self) -> bool {
        false
    }

    pub fn repo_root(&self) -> Option<&Path> {
        None
    }

    pub fn current_branch(&self) -> Option<String> {
        None
    }

    pub fn refresh_status(&mut self) {}

    pub fn file_status(&mut self, _path: &Path) -> GitFileStatus {
        GitFileStatus::Clean
    }

    pub fn get_all_statuses(&mut self) -> HashMap<PathBuf, GitFileStatus> {
        HashMap::new()
    }

    pub fn directory_status(&mut self, _dir_path: &Path) -> GitFileStatus {
        GitFileStatus::Clean
    }
}

const GIT_REFRESH_INTERVAL: Duration = Duration::from_secs(10);
const GIT_DEBOUNCE_DURATION: Duration = Duration::from_millis(500);

/// Retained only to avoid a broader app-state refactor in the same pass.
#[derive(Debug)]
pub struct GitAutoRefresh {
    last_refresh: Option<Instant>,
    last_request: Option<Instant>,
    pending_refresh: bool,
    was_focused: bool,
}

impl Default for GitAutoRefresh {
    fn default() -> Self {
        Self::new()
    }
}

impl GitAutoRefresh {
    pub fn new() -> Self {
        Self {
            last_refresh: None,
            last_request: None,
            pending_refresh: false,
            was_focused: true,
        }
    }

    pub fn request_refresh(&mut self) {
        self.last_request = Some(Instant::now());
        self.pending_refresh = true;
    }

    pub fn update_focus(&mut self, is_focused: bool) -> bool {
        let focus_gained = is_focused && !self.was_focused;
        self.was_focused = is_focused;
        if focus_gained {
            self.request_refresh();
        }
        focus_gained
    }

    pub fn should_periodic_refresh(&self) -> bool {
        match self.last_refresh {
            Some(last) => last.elapsed() >= GIT_REFRESH_INTERVAL,
            None => true,
        }
    }

    pub fn should_execute_refresh(&self) -> bool {
        if !self.pending_refresh {
            return false;
        }

        match self.last_request {
            Some(request_time) => request_time.elapsed() >= GIT_DEBOUNCE_DURATION,
            None => false,
        }
    }

    pub fn mark_refreshed(&mut self) {
        self.last_refresh = Some(Instant::now());
        self.pending_refresh = false;
    }

    pub fn tick(&mut self, workspace_open: bool) -> bool {
        if !workspace_open {
            return false;
        }

        if self.should_execute_refresh() {
            return true;
        }

        if self.should_periodic_refresh() {
            self.pending_refresh = true;
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_file_status_visibility() {
        assert_eq!(GitFileStatus::Clean.label(), "");
        assert_eq!(GitFileStatus::Clean.icon(), "");
        assert!(!GitFileStatus::Clean.is_visible());
    }

    #[test]
    fn test_git_service_is_disabled() {
        let mut service = GitService::new();
        assert!(!service.is_open());
        assert!(service.current_branch().is_none());
        assert!(service.repo_root().is_none());
        assert!(!service.open(Path::new(".")).unwrap());
        assert!(service.get_all_statuses().is_empty());
        assert_eq!(service.file_status(Path::new(".")), GitFileStatus::Clean);
    }
}
