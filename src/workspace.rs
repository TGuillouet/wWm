use windows_sys::Win32::Graphics::Gdi::HMONITOR;

use crate::{
    monitor::MonitorResolution,
    tree::{Node, TilingDirection},
    windows::{TilingMode, Window},
};

type WindowType = Box<Node<Window>>;

pub struct Workspace {
    pub monitor_handle: HMONITOR,
    monitor_resolution: MonitorResolution,
    pub windows: WindowType,

    current_window_index: usize,
}
impl Workspace {
    pub fn new(hmonitor: HMONITOR, resolution: MonitorResolution) -> Self {
        let tree = Box::new(Node::new(Window::new("()", 1), TilingDirection::Vertical));
        Self {
            monitor_handle: hmonitor,
            monitor_resolution: resolution,
            windows: tree,
            current_window_index: 0,
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.insert(
            window,
            self.current_window_index,
            TilingDirection::Horizontal,
        );
    }

    pub fn remove_window(window: &mut WindowType, window_handle: isize) {
        // Find the window to remove
        let has_window_to_remove = window
            .childrens
            .iter()
            .any(|child| child.value.hwnd == window_handle);

        if !has_window_to_remove {
            for children in window.childrens.iter_mut() {
                Workspace::remove_window(children, window_handle);
            }
            return;
        }

        // Get the parent and retain only the windows that do not have the handle passed in parameter
        window
            .childrens
            .retain(|child| child.value.hwnd != window_handle);
    }

    pub fn arrange_windows(&self) {
        self.arrange_recursive(
            &self.windows,
            self.monitor_resolution.rect.left,
            self.monitor_resolution.rect.top,
            self.monitor_resolution.width,
            self.monitor_resolution.height,
        )
    }

    pub fn set_current_next(&mut self) {
        if self.windows.childrens.is_empty() {
            return;
        }

        // TODO: Allow to insert the next opened window after the one we are at (Do not need to advance if we are at the end of the tree)
        self.current_window_index += 1;

        if self.current_window_index >= self.windows.childrens.len() {
            self.current_window_index = 0;
        }
    }

    pub fn set_current_previous(&mut self) {
        if self.windows.childrens.is_empty() {
            return;
        }

        if self.current_window_index == 0 {
            self.current_window_index = self.windows.childrens.len();
        }

        // TODO: Allow to insert the next opened window before the one we are at (Do not need to advance if we are at the end of the tree)
        self.current_window_index -= 1;
    }

    fn arrange_recursive(
        &self,
        current_node: &WindowType,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let borrowed_node = current_node;
        if borrowed_node.is_leaf() {
            return;
        }

        let mut child_x = x;
        let mut child_y = y;

        let managed_childrens: Vec<&WindowType> = borrowed_node
            .childrens
            .iter()
            .filter(|item| item.clone().value.mode == TilingMode::Managed)
            .collect();
        let width_ratio = width / managed_childrens.len() as i32;
        let height_ratio = height / managed_childrens.len() as i32;

        for children in borrowed_node.childrens.iter() {
            let borrowed_children = children;
            match borrowed_children.value.mode {
                TilingMode::Managed => {
                    let child_width = if borrowed_children.direction == TilingDirection::Horizontal
                    {
                        width_ratio
                    } else {
                        width
                    };
                    let child_height = if borrowed_children.direction == TilingDirection::Vertical {
                        height_ratio
                    } else {
                        height
                    };
                    if borrowed_children.is_leaf() {
                        let new_width = child_width;
                        let new_height = child_height;
                        let new_x = child_x;
                        let new_y = child_y;

                        borrowed_children
                            .value
                            .set_window_pos(new_x, new_y, new_width, new_height);
                    } else {
                        self.arrange_recursive(
                            children,
                            child_x,
                            child_y,
                            child_width,
                            child_height,
                        )
                    }

                    match borrowed_children.direction {
                        TilingDirection::Vertical => child_y += child_height,
                        TilingDirection::Horizontal => child_x += child_width,
                    }
                }
                TilingMode::Monocle => {
                    if borrowed_children.is_leaf() {
                        borrowed_children.value.set_window_pos(
                            self.monitor_resolution.rect.left,
                            self.monitor_resolution.rect.top,
                            self.monitor_resolution.width,
                            self.monitor_resolution.height,
                        );
                        borrowed_children.value.put_on_top();
                    } else {
                        let child_width =
                            if borrowed_children.direction == TilingDirection::Horizontal {
                                width_ratio
                            } else {
                                width
                            };
                        let child_height =
                            if borrowed_children.direction == TilingDirection::Vertical {
                                height_ratio
                            } else {
                                height
                            };
                        self.arrange_recursive(
                            children,
                            child_x,
                            child_y,
                            child_width,
                            child_height,
                        )
                    }
                }
            }
        }
    }
}
