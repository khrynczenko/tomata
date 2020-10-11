use std::time::Duration;

use druid::widget::{Align, Button, Flex, Label, Padding, Switch};
use druid::{
    BoxConstraints, Event, EventCtx, LayoutCtx, LensWrap, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UnitPoint, UpdateCtx, WidgetExt,
};
use druid::{Env, Selector, TimerToken, Widget, WindowDesc};

use crate::settings;
use crate::settings::Settings;
use crate::state::TomataState;
use crate::tomata;
use crate::tomata::{Period, APPLICATION_NAME, HOUR_S, MINUTE_S, SECOND_S, WINDOW_SIZE_PX};

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
        TomataApp {
            timer_id: TimerToken::INVALID,
            widget_tree: Box::new(make_main_window_widget_tree()),
        }
    }
}

impl Widget<TomataState> for TomataApp {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut TomataState, env: &Env) {
        match event {
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(make_tick_interval());
            }
            Event::Command(command) => {
                let settings_selector: Selector<TomataState> = Selector::new("Settings");
                let about_selector: Selector<TomataState> = Selector::new("About");
                if command.is(about_selector) {
                    ctx.new_window(make_about_window());
                } else if command.is(settings_selector) {
                    ctx.new_window(make_settings_window());
                }
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    if !data.is_stopwatch_paused() {
                        data.increase_elapsed_time(make_tick_interval());
                    }
                    if data.is_period_finished() {
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

fn make_main_window_widget_tree() -> impl Widget<TomataState> {
    let remaining_time_label = Label::new(|data: &TomataState, _env: &_| {
        tomata::duration_to_string(&data.calculate_remaining_time())
    })
    .with_text_size(32.0);

    let start_button =
        Button::new("Start").on_click(|_ctx, data: &mut TomataState, _env| data.start_stopwatch());

    let pause_button =
        Button::new("Pause").on_click(|_ctx, data: &mut TomataState, _env| data.pause_stopwatch());

    let reset_button =
        Button::new("Reset").on_click(|_ctx, data: &mut TomataState, _env| data.reset_stopwatch());

    let work_period_button = Button::new("Work")
        .on_click(|_ctx, data: &mut TomataState, _env| data.activate_period(Period::Work));

    let short_break_period_button = Button::new("Short")
        .on_click(|_ctx, data: &mut TomataState, _env| data.activate_period(Period::ShortBreak));

    let long_break_period_button = Button::new("Long")
        .on_click(|_ctx, data: &mut TomataState, _env| data.activate_period(Period::LongBreak));

    let widget_tree = Flex::column()
        .with_flex_child(Flex::row().with_flex_child(remaining_time_label, 1.0), 1.0)
        .with_flex_child(
            Flex::row()
                .with_flex_child(start_button, 1.0)
                .with_flex_child(pause_button, 1.0)
                .with_flex_child(reset_button, 1.0),
            1.0,
        )
        .with_flex_child(
            Flex::row()
                .with_flex_child(work_period_button, 1.0)
                .with_flex_child(short_break_period_button, 1.0)
                .with_flex_child(long_break_period_button, 1.0),
            1.0,
        );
    widget_tree
}

fn make_settings_window() -> WindowDesc<TomataState> {
    WindowDesc::new(make_settings_window_widget_tree)
        .title(APPLICATION_NAME)
        .window_size((420.0, 340.0))
        .resizable(false)
}

fn make_about_window() -> WindowDesc<TomataState> {
    WindowDesc::new(make_about_page)
        .title(APPLICATION_NAME)
        .window_size(WINDOW_SIZE_PX)
        .resizable(false)
}

fn make_settings_window_widget_tree() -> impl Widget<TomataState> {
    let tree = Padding::new(
        2.0,
        Flex::column()
            .with_child(make_period_adjustment_row(Period::Work))
            .with_spacer(3.0)
            .with_child(make_period_adjustment_row(Period::ShortBreak))
            .with_spacer(3.0)
            .with_child(make_period_adjustment_row(Period::LongBreak))
            .with_spacer(3.0)
            .with_child(make_short_breaks_number_adjustment_row())
            .with_spacer(3.0)
            .with_child(make_long_break_adjustment_row())
            .with_spacer(3.0)
            .with_child(make_next_period_starts_automatically_adjustment_row())
            .with_spacer(3.0)
            .with_child(make_system_notifications_adjustment_row())
            .with_spacer(3.0)
            .with_child(make_period_finishing_sound_adjustment_row())
            .with_spacer(3.0)
            .with_child(make_save_row())
            .with_spacer(3.0),
    );
    tree
}

fn make_period_adjustment_row(period: Period) -> impl Widget<TomataState> {
    let tree = Flex::row()
        .with_child(make_period_name_label(period))
        .with_flex_child(
            Flex::row()
                .with_child(Align::right(make_period_value_label(period)))
                .with_flex_child(Align::right(make_period_adjustment_buttons(period)), 1.0),
            1.0,
        );
    tree
}

fn make_period_name_label(period: Period) -> impl Widget<TomataState> {
    let text = match period {
        Period::Work => "Work interval: ",
        Period::ShortBreak => "Short break interval: ",
        Period::LongBreak => "Long break interval: ",
    };
    Label::new(text).padding(1.0).fix_width(170.0)
}

fn make_period_value_label(period: Period) -> impl Widget<TomataState> {
    let label = Label::new(move |data: &Settings, _env: &_| {
        tomata::duration_to_string(&data.convert_period_to_duration(period))
    });
    LensWrap::new(label, TomataState::settings)
}

fn make_period_adjustment_buttons(period: Period) -> impl Widget<TomataState> {
    let plus_one_hour_button = make_period_adjusting_button(Sign::Plus, Change::Hour, period);
    let minus_one_hour_button = make_period_adjusting_button(Sign::Minus, Change::Hour, period);
    let plus_one_minute_button = make_period_adjusting_button(Sign::Plus, Change::Minute, period);
    let minus_one_minute_button = make_period_adjusting_button(Sign::Minus, Change::Minute, period);
    let plus_one_second_button = make_period_adjusting_button(Sign::Plus, Change::Second, period);
    let minus_one_second_button = make_period_adjusting_button(Sign::Minus, Change::Second, period);
    Flex::row()
        .with_child(
            Flex::column()
                .with_child(plus_one_hour_button)
                .with_child(minus_one_hour_button)
                .fix_width(50.0),
        )
        .with_child(
            Flex::column()
                .with_child(plus_one_minute_button)
                .with_child(minus_one_minute_button)
                .fix_width(50.0),
        )
        .with_child(
            Flex::column()
                .with_child(plus_one_second_button)
                .with_child(minus_one_second_button)
                .fix_width(50.0),
        )
}

fn make_short_breaks_number_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Number of short breaks before long break:");
    let value_label = make_short_breaks_number_before_long_break();
    let tree = Flex::row().with_child(description_label).with_flex_child(
        Flex::row()
            .with_child(Align::right(value_label))
            .with_flex_child(Align::right(make_short_breaks_adjustment_buttons()), 1.0),
        1.0,
    );
    tree
}

fn make_short_breaks_number_before_long_break() -> impl Widget<TomataState> {
    let label: Label<usize> = Label::new(|data: &usize, _env: &_| format!("{}", *data));
    let label = LensWrap::new(label, Settings::short_breaks_number);
    LensWrap::new(label, TomataState::settings)
}

fn make_short_breaks_adjustment_buttons() -> impl Widget<TomataState> {
    let plus_button = Button::new("+")
        .on_click(move |_ctx, data: &mut Settings, _env| {
            data.increase_short_breaks_number(1);
        })
        .expand_width();
    let minus_button = Button::new("\u{2212}")
        .on_click(move |_ctx, data: &mut Settings, _env| {
            data.decrease_short_breaks_number(1);
        })
        .expand_width();
    Flex::row().with_child(
        Flex::column()
            .with_child(LensWrap::new(plus_button, TomataState::settings))
            .with_child(LensWrap::new(minus_button, TomataState::settings))
            .fix_width(50.0),
    )
}

fn make_long_break_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Include long breaks:");
    let switch = Switch::new();
    let switch = LensWrap::new(switch, Settings::long_breaks_are_included);
    let switch = LensWrap::new(switch, TomataState::settings);
    let row = Flex::row()
        .with_child(description_label)
        .with_flex_child(Align::right(switch), 1.0);
    row
}

fn make_next_period_starts_automatically_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Start next period automatically:");
    let switch = Switch::new();
    let switch = LensWrap::new(switch, Settings::next_period_starts_automatically);
    let switch = LensWrap::new(switch, TomataState::settings);
    let row = Flex::row()
        .with_child(description_label)
        .with_flex_child(Align::right(switch), 1.0);
    row
}

fn make_system_notifications_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Use system notifications:");
    let switch = Switch::new();
    let switch = LensWrap::new(switch, Settings::system_notifications_are_enabled);
    let switch = LensWrap::new(switch, TomataState::settings);
    let row = Flex::row()
        .with_child(description_label)
        .with_flex_child(Align::right(switch), 1.0);
    row
}

fn make_period_finishing_sound_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Use beeping sound when period is ending:");
    let switch = Switch::new();
    let switch = LensWrap::new(switch, Settings::period_ending_sound_is_enabled);
    let switch = LensWrap::new(switch, TomataState::settings);
    let row = Flex::row()
        .with_child(description_label)
        .with_flex_child(Align::right(switch), 1.0);
    row
}

fn make_save_row() -> impl Widget<TomataState> {
    let tree = Flex::row().with_child(Align::new(
        UnitPoint::RIGHT,
        Button::new("Save").on_click(|_ctx, data: &mut Settings, _env| {
            settings::save_settings_to_file(data, "settings.json").unwrap();
        }),
    ));
    LensWrap::new(tree, TomataState::settings)
}

fn make_about_page() -> impl Widget<TomataState> {
    let remaining_time_label = Label::new("About page!").with_text_size(32.0);
    remaining_time_label
}

enum Sign {
    Plus,
    Minus,
}

enum Change {
    Hour,
    Minute,
    Second,
}

fn make_period_adjusting_button(
    sign: Sign,
    change: Change,
    period: Period,
) -> impl Widget<TomataState> {
    let sign_char: char = match sign {
        Sign::Plus => '+',
        Sign::Minus => '\u{2212}',
    };
    let adjustment_method = match sign {
        Sign::Plus => Settings::increase_period_duration,
        Sign::Minus => Settings::decrease_period_duration,
    };
    let change_char: char = match change {
        Change::Hour => 'h',
        Change::Minute => 'm',
        Change::Second => 's',
    };
    let duration: Duration = match change {
        Change::Hour => Duration::from_secs(HOUR_S),
        Change::Minute => Duration::from_secs(MINUTE_S),
        Change::Second => Duration::from_secs(SECOND_S),
    };
    let button_text: String = [sign_char, '1', change_char].iter().collect();
    let button = Button::new(button_text)
        .on_click(move |_ctx, data: &mut Settings, _env| adjustment_method(data, period, duration))
        .expand_width();
    LensWrap::new(button, TomataState::settings)
}
