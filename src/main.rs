use std::collections::VecDeque; 

use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

use calc;

static DARK_EXT: &'static str = include_str!("../res/calculator-dark.css");

#[cfg(feature = "light-theme")]
static LIGHT_EXT: &'static str = include_str!("../res/calculator-light.css");

#[cfg(not(feature = "light-theme"))]
fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(DARK_EXT)
        .build()
}

#[cfg(feature = "light-theme")]
fn get_theme() -> ThemeValue {
    ThemeValue::create()
        .extension_css(DARK_EXT)
        .extension_css(LIGHT_EXT)
        .build()
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Character(char),
    Operator(char),
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    actions:VecDeque<Action>,
}

impl MainViewState {
    fn action(&mut self, action: Action) {
        self.actions.push_back(action);
    }

    fn calculate(&mut self, ctx: &mut Context) {
        let result = match calc::eval(
            ctx.child("input")
                .get::<String16>("text")
                .to_string()
                .as_str(),
        ) {
            Ok(s) => s.to_string(),
            Err(e) => e.into(),
        };

        ctx.widget().set("text", String16::from(format!("{:.9}", result)));
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.actions.pop_front() {
            match action {
                Action::Character(character) => {
                    ctx.child("input").get_mut::<String16>("text").push(character);
                }
                Action::Operator(operator) => match operator {
                    'C' => {
                        ctx.widget().get_mut::<String16>("text").clear();
                        ctx.child("input").get_mut::<String16>("text").clear()
                    }
                    '=' => {
                        self.calculate(ctx);
                        ctx.child("input").get_mut::<String16>("text").clear()
                    }
                    _ => {}
                },
            }
        }
    }
}

fn get_button_selector(primary: bool) -> Selector {
    let selector = Selector::from("button");

    if primary {
        selector.class("primary")
    } else {
        selector
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
    Button::create()
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .selector(get_button_selector(primary))
        .on_click(move |states, _| -> bool {
            state(id, states).action(Action::Character(sight));
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::row(row))
        .build(ctx)
}

fn generate_operation_button(
    ctx: &mut BuildContext,
    id: Entity,
    sight: char,
    primary: bool,
    column: usize,
    row: usize,
) -> Entity {
    Button::create()
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .selector(get_button_selector(primary).class("square"))
        .on_click(move |states, _| -> bool {
            state(id, states).action(Action::Operator(sight));
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::row(row))
        .build(ctx)
}

widget!(MainView<MainViewState> {
    text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .width(212.0)
            .height(336.0)
            .text("")
            .child(
                Grid::create()
                    .rows(Rows::create().row(72.0).row("*").build())
                    .child(
                        Container::create()
                            .padding(8.0)
                            .selector(Selector::from("container").class("header"))
                            .attach(Grid::row(0))
                            .child(
                                Grid::create()
                                    .child(
                                        ScrollViewer::create()
                                            .scroll_viewer_mode(("custom", "disabled"))
                                            .child(
                                                TextBlock::create()
                                                    .width(0.0)
                                                    .height(14.0)
                                                    .text("")
                                                    .selector(
                                                        Selector::from("text-block").id("input"),
                                                    )
                                                    .vertical_alignment("start")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        TextBlock::create()
                                            .selector(Selector::from("text-block"))
                                            .text(id)
                                            .vertical_alignment("end")
                                            .horizontal_alignment("end")
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Container::create()
                            .selector(Selector::from("container").class("content"))
                            .padding(8.0)
                            .attach(Grid::row(1))
                            .child(
                                Grid::create()
                                    .columns(
                                        Columns::create()
                                            .column(48.0)
                                            .column(4.0)
                                            .column(48.0)
                                            .column(4.0)
                                            .column(48.0)
                                            .column(4.0)
                                            .column(48.0)
                                            .build(),
                                    )
                                    .rows(
                                        Rows::create()
                                            .row(48.0)
                                            .row(4.0)
                                            .row(48.0)
                                            .row(4.0)
                                            .row(48.0)
                                            .row(4.0)
                                            .row(48.0)
                                            .row(4.0)
                                            .row(48.0)
                                            .build(),
                                    )
                                    // row 0
                                    .child(generate_character_button(ctx, id, '(', false, 0, 0))
                                    .child(generate_character_button(ctx, id, ')', false, 2, 0))
                                    .child(generate_character_button(ctx, id, '^', false, 4, 0))
                                    .child(generate_character_button(ctx, id, '/', true, 6, 0))
                                    // row 2
                                    .child(generate_character_button(ctx, id, '7', false, 0, 2))
                                    .child(generate_character_button(ctx, id, '8', false, 2, 2))
                                    .child(generate_character_button(ctx, id, '9', false, 4, 2))
                                    .child(generate_character_button(ctx, id, '*', true, 6, 2))
                                    // row 4
                                    .child(generate_character_button(ctx, id, '4', false, 0, 4))
                                    .child(generate_character_button(ctx, id, '5', false, 2, 4))
                                    .child(generate_character_button(ctx, id, '6', false, 4, 4))
                                    .child(generate_character_button(ctx, id, '-', true, 6, 4))
                                    // row 6
                                    .child(generate_character_button(ctx, id, '1', false, 0, 6))
                                    .child(generate_character_button(ctx, id, '2', false, 2, 6))
                                    .child(generate_character_button(ctx, id, '3', false, 4, 6))
                                    .child(generate_character_button(ctx, id, '+', true, 6, 6))
                                    // row 8
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
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Calculator")
                .position((100.0, 100.0))
                .size(212.0, 336.0)
                .theme(get_theme())
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
