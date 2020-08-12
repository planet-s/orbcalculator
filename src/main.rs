use std::collections::VecDeque;

use orbtk::{
    prelude::*,
    shell::prelude::Key,
    theme::{COLORS_RON, FONTS_RON},
    theming::config::ThemeConfig,
};

#[cfg(not(feature = "light"))]
use orbtk::theme::DARK_THEME_RON;

#[cfg(feature = "light")]
use orbtk::theme::LIGHT_THEME_RON;

use calc;

// --- KEYS --

pub static STYLE_CONTENT: &'static str = "content_area";
pub static STYLE_HEADER: &'static str = "header_area";
pub static STYLE_BUTTON: &'static str = "button_calculator";
pub static STYLE_BUTTON_PRIMARY: &'static str = "button_calculator_primary";
pub static STYLE_INPUT: &'static str = "input";
pub static STYLE_RESULT: &'static str = "result";

static ID_INPUT: &'static str = "input";

// --- KEYS --

// --- THEME --

#[cfg(not(feature = "light"))]
static DARK_EXT: &'static str = include_str!("../assets/calculator_dark.ron");

#[cfg(not(feature = "light"))]
fn theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(DARK_THEME_RON)
            .extend(ThemeConfig::from(DARK_EXT))
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}

#[cfg(feature = "light")]
static LIGHT_EXT: &'static str = include_str!("../assets/calculator_light.ron");

#[cfg(feature = "light")]
fn theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(LIGHT_THEME_RON)
            .extend(ThemeConfig::from(LIGHT_EXT))
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}

// --- THEME --

#[derive(Debug, Copy, Clone)]
enum Action {
    Character(char),
    Operator(char),
}

#[derive(Default, AsAny)]
pub struct MainState {
    actions: VecDeque<Action>,
    input: Entity,
}

impl MainState {
    fn action(&mut self, action: Action) {
        self.actions.push_back(action);
    }

    fn key_input(&mut self, key: Key) {}

    fn calculate(&mut self, ctx: &mut Context) {
        let result = match calc::eval(
            ctx.get_widget(self.input)
                .get::<String16>("text")
                .to_string()
                .as_str(),
        ) {
            Ok(s) => s.to_string(),
            Err(e) => e.into(),
        };

        ctx.widget()
            .set("text", String16::from(format!("{:.9}", result)));
    }
}

impl State for MainState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.input = ctx
            .entity_of_child(ID_INPUT)
            .expect("MainState.init: input child could not be found.");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.actions.pop_front() {
            match action {
                Action::Character(character) => {
                    ctx.get_widget(self.input)
                        .get_mut::<String16>("text")
                        .push(character);
                }
                Action::Operator(operator) => match operator {
                    'C' => {
                        ctx.widget().get_mut::<String16>("text").clear();
                        ctx.get_widget(self.input)
                            .get_mut::<String16>("text")
                            .clear()
                    }
                    '=' => {
                        self.calculate(ctx);
                        ctx.get_widget(self.input)
                            .get_mut::<String16>("text")
                            .clear()
                    }
                    _ => {}
                },
            }
        }
    }
}

fn generate_character_button(
    ctx: &mut BuildContext,
    id: Entity,
    sight: char,
    primary: bool,
    column: usize,
    row: usize,
) -> Entity {
    let style = if primary {
        STYLE_BUTTON_PRIMARY
    } else {
        STYLE_BUTTON
    };

    let button = Button::new()
        .style(style)
        .min_size(48, 48)
        .text(sight.to_string())
        .on_click(move |states, _| -> bool {
            state(id, states).action(Action::Character(sight));
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::row(row));

    button.build(ctx)
}

fn generate_operation_button(
    ctx: &mut BuildContext,
    id: Entity,
    sight: char,
    primary: bool,
    column: usize,
    row: usize,
) -> Entity {
    let style = if primary {
        STYLE_BUTTON_PRIMARY
    } else {
        STYLE_BUTTON
    };

    let button = Button::new()
        .style(style)
        .min_size(48, 48)
        .text(sight.to_string())
        .on_click(move |states, _| -> bool {
            state(id, states).action(Action::Operator(sight));
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::row(row));

    button.build(ctx)
}

widget!(MainView<MainState> : KeyDownHandler {
    text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .text("")
            .child(
                Grid::new()
                    .rows(Rows::new().add(72).add("*"))
                    // header
                    .child(
                        Container::new()
                            .style(STYLE_HEADER)
                            .attach(Grid::row(0))
                            .padding(8)
                            .child(
                                Grid::new()
                                    .child(
                                        ScrollViewer::new()
                                            .mode(("custom", "disabled"))
                                            .child(
                                                TextBlock::new()
                                                    .id(ID_INPUT)
                                                    .style(STYLE_INPUT)
                                                    .width(0)
                                                    .height(14)
                                                    .text("")
                                                    .v_align("start")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        TextBlock::new()
                                            .style(STYLE_RESULT)
                                            .text(id)
                                            .v_align("end")
                                            .h_align("end")
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    // content
                    .child(
                        Container::new()
                            .style(STYLE_CONTENT)
                            .padding(8)
                            .attach(Grid::row(1))
                            .child(
                                Grid::new()
                                    .columns(
                                        Columns::new()
                                            .add(48)
                                            .add(4)
                                            .add(48)
                                            .add(4)
                                            .add(48)
                                            .add(4)
                                            .add(48),
                                    )
                                    .rows(
                                        Rows::new()
                                            .add(48)
                                            .add(4)
                                            .add(48)
                                            .add(4)
                                            .add(48)
                                            .add(4)
                                            .add(48)
                                            .add(4)
                                            .add(48),
                                    )
                                    // add 0
                                    .child(generate_character_button(ctx, id, '(', false, 0, 0))
                                    .child(generate_character_button(ctx, id, ')', false, 2, 0))
                                    .child(generate_character_button(ctx, id, '^', false, 4, 0))
                                    .child(generate_character_button(ctx, id, '/', true, 6, 0))
                                    // add 2
                                    .child(generate_character_button(ctx, id, '7', false, 0, 2))
                                    .child(generate_character_button(ctx, id, '8', false, 2, 2))
                                    .child(generate_character_button(ctx, id, '9', false, 4, 2))
                                    .child(generate_character_button(ctx, id, '*', true, 6, 2))
                                    // add 4
                                    .child(generate_character_button(ctx, id, '4', false, 0, 4))
                                    .child(generate_character_button(ctx, id, '5', false, 2, 4))
                                    .child(generate_character_button(ctx, id, '6', false, 4, 4))
                                    .child(generate_character_button(ctx, id, '-', true, 6, 4))
                                    // add 6
                                    .child(generate_character_button(ctx, id, '1', false, 0, 6))
                                    .child(generate_character_button(ctx, id, '2', false, 2, 6))
                                    .child(generate_character_button(ctx, id, '3', false, 4, 6))
                                    .child(generate_character_button(ctx, id, '+', true, 6, 6))
                                    // add 8
                                    .child(generate_character_button(ctx, id, '0', false, 0, 8))
                                    .child(generate_character_button(ctx, id, '.', false, 2, 8))
                                    .child(generate_operation_button(ctx, id, 'C', false, 4, 8))
                                    .child(generate_operation_button(ctx, id, '=', true, 6, 8))
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_key_down(move |states, e| {
                states.get_mut::<MainState>(id).key_input(e.key);
                true
            })
    }
}

fn main() {
    Application::from_name("OrbCalculator")
        .theme(theme())
        .window(|ctx| {
            Window::new()
                .title("OrbCalculator")
                .position((100, 100))
                .size(220, 344)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

// helper to request MainState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainState {
    states.get_mut(id)
}
