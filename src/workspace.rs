use windows_sys::Win32::Graphics::Gdi::HMONITOR;

use crate::{
    monitor::MonitorResolution,
    tree::{Node, TilingDirection},
    windows::Window,
};

#[derive(Debug)]
pub struct Workspace {
    pub monitor_handle: HMONITOR,
    monitor_resolution: MonitorResolution,
    windows: Node<Window>,
}
impl Workspace {
    pub fn new(hmonitor: HMONITOR, resolution: MonitorResolution) -> Self {
        Self {
            monitor_handle: hmonitor,
            monitor_resolution: resolution,
            windows: Node::new(
                Window {
                    title: "()".to_owned(),
                    hwnd: 1,
                },
                TilingDirection::Vertical,
            ),
        }
    }

    pub fn add_window(&mut self, window: Window) {
        println!(
            "Added window {} to monitor {}",
            &window.title, self.monitor_handle,
        );
        self.windows
            .childrens
            .push(Box::new(Node::new(window, TilingDirection::Horizontal)));
    }

    pub fn arrange_windows(&self) {
        self.arrange_recursive(
            &self.windows,
            0,
            0,
            self.monitor_resolution.width,
            self.monitor_resolution.height,
        )
    }

    fn arrange_recursive(
        &self,
        current_node: &Node<Window>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let mut child_x = x;
        let mut child_y = y;

        let width_ratio = width / current_node.childrens.len() as i32;
        let height_ratio = height / current_node.childrens.len() as i32;

        for children in current_node.childrens.iter() {
            let child_width = if children.direction == TilingDirection::Horizontal {
                width_ratio
            } else {
                width
            };
            let child_height = if children.direction == TilingDirection::Vertical {
                height_ratio
            } else {
                height
            };
            if children.is_leaf() {
                let new_width = child_width;
                let new_height = child_height;
                let new_x = child_x;
                let new_y = child_y;

                children
                    .value
                    .set_window_pos(new_x, new_y, new_width, new_height);
            } else {
                self.arrange_recursive(&children, child_x, child_y, child_width, child_height)
            }

            match children.direction {
                TilingDirection::Vertical => child_y += child_height,
                TilingDirection::Horizontal => child_x += child_width,
            }
        }
    }
}
