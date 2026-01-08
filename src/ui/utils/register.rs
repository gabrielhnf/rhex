use ratatui::{buffer::Buffer, layout::Rect};
use taffy::{TaffyTree, prelude::{TaffyMaxContent, auto, length}};

use crate::ui::windows::Window;

pub struct WindowRegister {
    windows: Vec<Box<dyn Window>>,
    area: Rect,
    areas: Vec<Rect>,
    tree: TaffyTree,
    active_window: usize,
}

impl WindowRegister {
    pub fn new() -> Self {
        Self { 
        windows: vec![],
        area: Rect::default(),
        areas: vec![],
        tree: TaffyTree::new(),
        active_window: 0,
        }
    }

    pub fn initialize(&mut self, area: Rect){
        self.area = area;
    }

    pub fn calculate(&mut self){
        self.areas.clear();
        let root_node = self.tree.new_leaf(
            taffy::Style {
                size: taffy::Size { width: length(self.area.width), height: length(self.area.height) },
                ..Default::default()
            }
        ).unwrap();

        for _i in 0..self.windows.len() {
            let leaf = self.tree.new_leaf(
                taffy::Style {
                    size: taffy::Size { width: auto(), height: auto() },
                    flex_grow: 1.0,
                    ..Default::default()
                }
            ).unwrap();

            self.tree.add_child(root_node, leaf).unwrap();
        }

        self.tree.compute_layout(root_node, taffy::Size::MAX_CONTENT).unwrap();

        for node in self.tree.children(root_node).unwrap() {
            let area = Rect { 
                x: self.tree.layout(node).unwrap().location.x as u16,
                y: self.tree.layout(node).unwrap().location.y as u16,
                width: self.tree.layout(node).unwrap().size.width as u16,
                height: self.tree.layout(node).unwrap().size.height as u16,
            };

            self.areas.push(area);
        }
    }

    pub fn render(&mut self, buf: &mut Buffer){
        assert_eq!(self.areas.len(), self.windows.len());
        for i in 0..self.windows.len(){
            self.windows[i].render(self.areas[i], buf);
        }
    }

    pub fn register<T: Window + 'static>(&mut self, window: T){
        self.windows.push(Box::new(window));
    }

    pub fn switch(&mut self){
        self.active_window = (self.active_window + 1) % self.windows.len();
    }

    pub fn get_active(&mut self) -> &mut dyn Window {
        self.windows[self.active_window].as_mut()
    }
}

impl<'a> IntoIterator for &'a mut WindowRegister {
    type Item = &'a mut dyn Window;
    type IntoIter = std::iter::Map<std::slice::IterMut<'a, Box<dyn Window>>, fn(&'a mut Box<dyn Window>) -> &'a mut dyn Window>;

    fn into_iter(self) -> Self::IntoIter {
        self.windows.iter_mut().map(|b| b.as_mut())
    }
}
