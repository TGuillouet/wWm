use crate::windows::TilingMode;

pub enum WmAction {
    Workspace(WorkspaceAction),
    Close { hwnd: isize },
}

pub enum WorkspaceAction {
    NextAsCurrent,
    PreviousAsCurrent,
    // SetCurrentWindow { hwnd: isize },
    ToggleMode(TilingMode),
    PutCurrentWindowInWorkspace { workspace_index: usize },
}
