#[derive(Debug)]
pub enum WmAction {
    Workspace(WorkspaceAction),
    Close { hwnd: isize },
}

#[derive(Debug)]
pub enum WorkspaceAction {
    NextAsCurrent,
    PreviousAsCurrent,
    // SetCurrentWindow { hwnd: isize },
}
