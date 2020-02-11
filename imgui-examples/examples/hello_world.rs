use imgui::*;
use std::path;

mod support;

struct FileExplorer {
    pub path: Vec<String>,

    into: bool,
    sel_buf: ImString,
}

impl ToString for FileExplorer {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.path.iter().for_each(|elem| {
            s.push_str(elem);
            s.push('/')
        });

        s
    }
}

impl FileExplorer {
    pub fn new(path: String) -> FileExplorer {
        FileExplorer {
            path: path.split("/")
                .filter(|str| { !str.trim().is_empty() })
                .map(|str| { str.to_string() })
                .collect(),

            into: false,
            sel_buf: ImString::with_capacity(32)
        }
    }

    pub fn draw(&mut self, ui: &Ui) {
        let path_str = self.to_string();
        let dir = path::Path::new(&path_str);
        let style = ui.clone_style();
        let mut new_path = None;
        let mut selected = None;

        Window::new(im_str!("file explorer"))
            .size([300.0, 500.0], Condition::FirstUseEver)
            .build(ui, || {
                ChildWindow::new("##FileExplorer.Path")
                    .horizontal_scrollbar(true)
                    .size([0.0, ui.text_line_height() + style.scrollbar_size + style.window_padding[1]])
                    .build(ui, || {
                        for (index, elem) in self.path.iter().enumerate() {
                            if index != 0 {
                                ui.same_line(0.0);
                            }

                            if !elem.trim().is_empty() {
                                if ui.button(&im_str!("{}", elem), [0.0, 0.0]) {
                                    new_path = Some(Vec::from(&self.path[0..index + 1]));
                                }
                            }
                        }

                        if self.into {
                            ui.set_scroll_x(ui.scroll_max_x());
                            self.into = false;
                        }
                    });

                ui.separator();

                ChildWindow::new("##FileExplorer.FileList")
                    .size([0.0, - (ui.text_line_height() + style.window_padding[1] * 2.0)])
                    .build(ui, || {
                        if let Ok(dirs) = dir.read_dir() {
                            for i in dirs {
                                if let Ok(i) = i {
                                    if let Some(filename) = i.file_name().to_str() {
                                        let buf = i.path();
                                        let mut name = filename.to_string();

                                        if buf.is_dir() {
                                            name.push('/');
                                        }

                                        if ui.button(&im_str!("{}", name), [0.0, 0.0]) {
                                            if buf.is_dir() {
                                                self.path.push(filename.to_string());
                                                self.into = true;
                                            } else {
                                                selected = Some(filename.to_string());
                                            }
                                        }
                                    }
                                } else {
                                    ui.text_colored([1.0, 0.0, 0.0, 1.0], "Error!");
                                }
                            }
                        }
                    });

                ui.separator();

                ui.input_text(im_str!(""), &mut self.sel_buf).build();
            });

        if let Some(path) = new_path {
            self.path = path;
        }

        if let Some(selected) = selected {
            self.sel_buf.clear();
            self.sel_buf.push_str(&selected);
        }
    }
}


fn main() {
    let mut button_clicked = false;
    let mut demo_opened = false;
    let mut fe = FileExplorer::new("E://Documents/Projects/".to_string());

    let system = support::init(file!());
    system.main_loop(|_, ui| {
        Window::new(im_str!("Hello world"))
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });

        Window::new(im_str!("Hoshino"))
            .size([640.0, 480.0], Condition::FirstUseEver)
            .build(ui, || {
                let clicked = ui.button(im_str!("Cute"), [0.0, 0.0]);

                if !button_clicked && clicked {
                    button_clicked = true;
                }

                ui.separator();

                if ui.collapsing_header(im_str!("Node")).build() {
                    ui.tree_node(im_str!("0")).label(im_str!("Leaf")).build(|| {
                        let style = ui.clone_style();
                        let sub_window_height = ui.text_line_height() + style.scrollbar_size + style.window_padding[1];
                        let window = ChildWindow::new(im_str!("##0"))
                            .size([0.0, sub_window_height])
                            .horizontal_scrollbar(true);

                        window.build(&ui, || {
                            for i in 0..100 {
                                if i != 0 || i == 99 {
                                    ui.same_line(0.0);
                                }

                                ui.button(&im_str!("Button{}", i), [0.0, 0.0]);
                            }
                        });
                    });
                }

                if button_clicked {
                    ui.text(im_str!("Hoshino is so cute!"));
                }
            });

        fe.draw(ui);

        ui.show_demo_window(&mut demo_opened);
    });
}
