extern crate orbtk;
use orbtk::*;

extern crate calc;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

static DARK_THEME_EXTENSION: &'static str = include_str!("dark-theme-extension.css");
static LIGHT_THEME_EXTENSION: &'static str = include_str!("light-theme-extension.css");

#[derive(Default)]
struct MainViewState {
    result: RefCell<String>,
    updated: Cell<bool>,
}

impl MainViewState {
    fn clear(&self) {
        self.result.borrow_mut().clear();
        self.updated.set(true);
    }

    fn eval(&self) {
        let result = match calc::eval(&*self.result.borrow()) {
            Ok(s) => s.to_string(),
            Err(e) => e.into(),
        };

        (*self.result.borrow_mut()) = result;
        self.updated.set(true);
    }
    fn input(&self, sight: &str) {
        let result = self.result.borrow().clone();
        (*self.result.borrow_mut()) = format!("{}{}", result, sight);
        self.updated.set(true);
    }
}

impl State for MainViewState {
    fn update(&self, widget: &mut WidgetContainer) {
        if let Ok(label) = widget.borrow_mut_property::<Label>() {
            if self.updated.get() {
                label.0 = self.result.borrow().clone();
            } else {
                *self.result.borrow_mut() = label.0.clone();
            }

            self.updated.set(false);
        }
    }
}

fn generate_button(state: &Rc<MainViewState>, sight: &str) -> Option<Rc<Widget>> {
    let sight = String::from(sight);
    let state = state.clone();

    Some(Rc::new(Button {
        label: Property::new(Label(sight.clone())),
        event_handlers: vec![Rc::new(MouseEventHandler {
            on_click: Some(Rc::new(
                move |_pos: Point, _widget: &mut WidgetContainer| -> bool {
                    state.input(&String::from(sight.clone()));
                    true
                },
            )),
            ..Default::default()
        })],
        selector: (Property::new(Selector::new(Some(String::from("calculatorbutton"))))),
        ..Default::default()
    }))
}

struct MainView {
    state: Rc<MainViewState>,
    result: Property<Label>,
}

impl Widget for MainView {
    fn template(&self) -> Template {
        let eval_state = self.state.clone();
        let clear_state = self.state.clone();

        Template::Single(Rc::new(Column {
            children: vec![
                Rc::new(Container {
                    child: Some(Rc::new(TextBox {
                        label: self.result.clone(),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                Rc::new(Row {
                    children: vec![
                        Rc::new(Container {
                            child: generate_button(&self.state, "("),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, ")"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "^"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "/"),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                Rc::new(Row {
                    children: vec![
                        Rc::new(Container {
                            child: generate_button(&self.state, "7"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "8"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "9"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "*"),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                Rc::new(Row {
                    children: vec![
                        Rc::new(Container {
                            child: generate_button(&self.state, "4"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "5"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "6"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "-"),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                Rc::new(Row {
                    children: vec![
                        Rc::new(Container {
                            child: generate_button(&self.state, "1"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "2"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "3"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "+"),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                Rc::new(Row {
                    children: vec![
                        Rc::new(Container {
                            child: generate_button(&self.state, "0"),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: generate_button(&self.state, "."),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: Some(Rc::new(Button {
                                label: Property::new(Label(String::from("C"))),
                                event_handlers: vec![Rc::new(MouseEventHandler {
                                    on_click: Some(Rc::new(
                                        move |_pos: Point, _widget: &mut WidgetContainer| -> bool {
                                            clear_state.clear();
                                            true
                                        },
                                    )),
                                    ..Default::default()
                                })],
                                selector: (Property::new(Selector::new(Some(String::from(
                                    "calculatorbutton",
                                ))))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: Some(Rc::new(Button {
                                label: Property::new(Label(String::from("="))),
                                event_handlers: vec![Rc::new(MouseEventHandler {
                                    on_click: Some(Rc::new(
                                        move |_pos: Point, _widget: &mut WidgetContainer| -> bool {
                                            eval_state.eval();
                                            true
                                        },
                                    )),
                                    ..Default::default()
                                })],
                                selector: (Property::new(Selector::new(Some(String::from(
                                    "calculatorbutton",
                                ))))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.result.build()]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }
}

fn main() {
    let mut application = Application::new();

    let theme = format!("{}{}", DARK_THEME_EXTENSION, DEFAULT_THEME_CSS);
    //let theme = format!("{}{}", LIGHT_THEME_EXTENSION, LIGHT_THEME_CSS);
    
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 176, 256))
        .with_title("Calculator")
        .with_theme(Theme::parse(&theme))
        .with_root(MainView {
            state: Rc::new(MainViewState::default()),
            result: Property::new(Label(String::from(""))),
        })
        .with_debug_flag(false)
        .build();
    application.run();
}
