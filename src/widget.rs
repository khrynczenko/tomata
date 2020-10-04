use std::time::Duration;

use druid::widget::{Align, Button, Flex, Label};
use druid::{
    BoxConstraints, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UnitPoint,
    UpdateCtx, WidgetExt,
};
use druid::{Env, Selector, TimerToken, Widget, WindowDesc};

use crate::state::TomataState;
use crate::tomata;
use crate::tomata::{Period, SECOND_S, WINDOW_SIZE_PX};

// const TICK_INTERVAL: Duration = Duration::new(ZERO_SECONDS, ONE_THOUSAND_NANOSECONDS);
// `Duration::new` is not a `const` yet so this function would suffice for now
fn make_tick_interval() -> Duration {
    Duration::from_secs(SECOND_S)
}

pub struct TomataApp {
    timer_id: TimerToken,
    widget_tree: Box<dyn Widget<TomataState>>,
}

impl TomataApp {
    pub fn new() -> TomataApp {
        let remaining_time_label = Label::new(|data: &TomataState, _env: &_| {
            tomata::duration_to_string(&data.calculate_remaining_time())
        })
        .with_text_size(32.0);

        let widget_tree = Flex::column()
            .with_flex_child(Flex::row().with_flex_child(remaining_time_label, 1.0), 1.0)
            .with_flex_child(
                Flex::row()
                    .with_flex_child(
                        Button::new("Start")
                            .on_click(|_ctx, data: &mut TomataState, _env| data.start()),
                        1.0,
                    )
                    .with_flex_child(
                        Button::new("Pause")
                            .on_click(|_ctx, data: &mut TomataState, _env| data.pause()),
                        1.0,
                    ),
                1.0,
            )
            .with_flex_child(
                Flex::row()
                    .with_flex_child(
                        Button::new("Work")
                            .on_click(|_ctx, data: &mut TomataState, _env| data.activate_work()),
                        1.0,
                    )
                    .with_flex_child(
                        Button::new("Short").on_click(|_ctx, data: &mut TomataState, _env| {
                            data.activate_short_break()
                        }),
                        1.0,
                    )
                    .with_flex_child(
                        Button::new("Long").on_click(|_ctx, data: &mut TomataState, _env| {
                            data.activate_long_break()
                        }),
                        1.0,
                    ),
                1.0,
            );
        TomataApp {
            timer_id: TimerToken::INVALID,
            widget_tree: Box::new(widget_tree),
        }
    }
}

impl Widget<TomataState> for TomataApp {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut TomataState, env: &Env) {
        match event {
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(make_tick_interval());
            }
            Event::Command(cmd) => {
                let settings_selector: Selector<TomataState> = Selector::new("Settings");
                let about_selector: Selector<TomataState> = Selector::new("About");
                if cmd.is(about_selector) {
                    let new_win = WindowDesc::new(make_about_page)
                        .window_size(WINDOW_SIZE_PX)
                        .resizable(false);
                    ctx.new_window(new_win);
                } else if cmd.is(settings_selector) {
                    let new_win = WindowDesc::new(make_settings_page)
                        .window_size((420.0, 225.0))
                        .resizable(false);
                    ctx.new_window(new_win);
                }
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    if !data.is_paused() {
                        data.increase_elapsed_time(make_tick_interval());
                    }
                    if data.is_finished() {
                        data.cycle_to_next_period();
                    }
                    self.timer_id = ctx.request_timer(make_tick_interval());
                }
            }
            _ => {}
        }
        self.widget_tree.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &TomataState,
        env: &Env,
    ) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.widget_tree.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &TomataState,
        data: &TomataState,
        env: &Env,
    ) {
        self.widget_tree.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &TomataState,
        env: &Env,
    ) -> Size {
        self.widget_tree.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &TomataState, env: &Env) {
        self.widget_tree.paint(ctx, data, env);
    }
}

fn make_settings_label(period: Period) -> impl Widget<TomataState> {
    let text = match period {
        Period::WorkPeriod => "Work interval: ",
        Period::ShortBreak => "Short break interval: ",
        Period::LongBreak => "Long break interval: ",
    };
    Label::new(text).padding(1.0).fix_width(170.0)
}

fn make_stepper_text(period: Period) -> impl Widget<TomataState> {
    Label::new(move |data: &TomataState, _env: &_| {
        tomata::duration_to_string(&data.get_settings().get_duration_for_period(period))
    })
}

fn make_settings_set_buttons(period: Period) -> impl Widget<TomataState> {
    Flex::row()
        .with_child(
            Flex::column()
                .with_child(
                    Button::new("+1h")
                        .on_click(move |_ctx, data: &mut TomataState, _env| {
                            data.increase_period_duration(period, Duration::from_secs(60 * 60))
                        })
                        .expand_width(),
                )
                .with_child(
                    Button::new("-1h")
                        .on_click(move |_ctx, data: &mut TomataState, _env| {
                            data.decrease_period_duration(period, Duration::from_secs(60 * 60))
                        })
                        .expand_width(),
                )
                .fix_width(50.0),
        )
        .with_child(
            Flex::column()
                .with_child(
                    Button::new("+1m")
                        .on_click(move |_ctx, data: &mut TomataState, _env| {
                            data.increase_period_duration(period, Duration::from_secs(60))
                        })
                        .expand_width(),
                )
                .with_child(
                    Button::new("-1m")
                        .on_click(move |_ctx, data: &mut TomataState, _env| {
                            data.decrease_period_duration(period, Duration::from_secs(60))
                        })
                        .expand_width(),
                )
                .fix_width(50.0),
        )
        .with_child(
            Flex::column()
                .with_child(
                    Button::new("+1s")
                        .on_click(move |_ctx, data: &mut TomataState, _env| {
                            data.increase_period_duration(period, Duration::from_secs(1))
                        })
                        .expand_width(),
                )
                .with_child(
                    Button::new("-1s")
                        .on_click(move |_ctx, data: &mut TomataState, _env| {
                            data.decrease_period_duration(period, Duration::from_secs(1))
                        })
                        .expand_width(),
                )
                .fix_width(50.0),
        )
}

fn make_settings_row(period: Period) -> impl Widget<TomataState> {
    let tree = Flex::row()
        .with_child(make_settings_label(period))
        .with_flex_child(
            Flex::row()
                .with_child(Align::right(make_stepper_text(period)))
                .with_flex_child(make_settings_set_buttons(period), 1.0),
            1.0,
        );
    tree
}

fn make_save_row() -> impl Widget<TomataState> {
    let tree = Flex::row().with_child(Align::new(
        UnitPoint::RIGHT,
        Button::new("Save").on_click(move |_ctx, data: &mut TomataState, _env| {
            data.serialize_settings("settings.json").unwrap()
        }),
    ));
    tree
}

fn make_about_page() -> impl Widget<TomataState> {
    let remaining_time_label = Label::new("About page!").with_text_size(32.0);
    remaining_time_label
}

fn make_settings_page() -> impl Widget<TomataState> {
    let tree = Flex::column()
        .with_child(make_settings_row(Period::WorkPeriod))
        .with_child(make_settings_row(Period::ShortBreak))
        .with_child(make_settings_row(Period::LongBreak))
        .with_child(make_save_row());
    tree
}
