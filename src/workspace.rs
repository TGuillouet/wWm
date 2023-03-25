use std::cell::RefCell;

use windows_sys::Win32::Graphics::Gdi::HMONITOR;

use crate::{
    monitor::MonitorResolution,
    tree::{Node, TilingDirection},
    windows::{TilingMode, Window},
};

type WindowType = RefCell<Box<Node<Window>>>;

pub struct Workspace {
    pub monitor_handle: HMONITOR,
    monitor_resolution: MonitorResolution,
    pub windows: WindowType,
    current_window: Option<WindowType>,
}
impl Workspace {
    pub fn new(hmonitor: HMONITOR, resolution: MonitorResolution) -> Self {
        let tree = RefCell::new(Box::new(Node::new(
            Window::new("()", 1),
            TilingDirection::Vertical,
        )));
        let tree2 = RefCell::new(Box::new(Node::new(
            Window::new("()", 1),
            TilingDirection::Vertical,
        )));
        Self {
            monitor_handle: hmonitor,
            monitor_resolution: resolution,
            windows: tree,
            current_window: Some(tree2),
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows
            .borrow_mut()
            .childrens
            .push(RefCell::new(Box::new(Node::new(
                window.clone(),
                TilingDirection::Horizontal,
            ))));

        self.current_window
            .as_ref()
            .unwrap()
            .borrow_mut()
            .childrens
            .push(RefCell::new(Box::new(Node::new(
                window,
                TilingDirection::Horizontal,
            ))));
    }

    pub fn remove_window(window: &mut RefCell<Box<Node<Window>>>, window_handle: isize) {
        // Find the window to remove
        let has_window_to_remove = window
            .borrow_mut()
            .childrens
            .iter()
            .any(|child| child.borrow().value.hwnd == window_handle);

        if !has_window_to_remove {
            for children in window.borrow_mut().childrens.iter_mut() {
                Workspace::remove_window(children, window_handle);
            }
            return;
        }

        // Get the parent and retain only the windows that do not have the handle passed in parameter
        window
            .borrow_mut()
            .childrens
            .retain(|child| child.borrow().value.hwnd != window_handle);
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
        if let Some(current_window) = self.current_window.take() {
            if current_window.borrow().childrens.is_empty() {
                return;
            }
            println!("{:?}", current_window.borrow().value.title);
            self.current_window = Some(current_window.borrow().clone().childrens[1].clone());
            println!(
                "{:?}",
                self.current_window.as_ref().unwrap().borrow().value.title
            );
        }
    }

    pub fn set_current_previous(&self) {}

    fn arrange_recursive(
        &self,
        current_node: &RefCell<Box<Node<Window>>>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        if current_node.borrow().is_leaf() {
            return;
        }

        let mut child_x = x;
        let mut child_y = y;

        let borrowed_node = current_node.borrow();
        let managed_childrens: Vec<&RefCell<Box<Node<Window>>>> = borrowed_node
            .childrens
            .iter()
            .filter(|item| item.clone().borrow().value.mode == TilingMode::Managed)
            .collect();
        let width_ratio = width / managed_childrens.len() as i32;
        let height_ratio = height / managed_childrens.len() as i32;

        for children in borrowed_node.childrens.iter() {
            let borrowed_children = children.borrow();
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
