//! All the functionality related to widgets resides in this module.
use std::time::Duration;

use druid::widget::{Align, Button, Flex, Label, LensWrap, Padding, Slider, Switch};
use druid::{
    BoxConstraints, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UnitPoint,
    UpdateCtx, WidgetExt,
};
use druid::{Env, TimerToken, Widget};
use once_cell::sync::Lazy;

use crate::settings;
use crate::settings::Settings;
use crate::state::TomataState;
use crate::tomata;
use crate::tomata::{Period, HOUR_S, MINUTE_S, SECOND_S};

// [`Duration::new`] is not yet `const` so instead we use `Lazy` initialized
// static variable.
static TICK_INTERVAL: Lazy<Duration> = Lazy::new(|| Duration::from_secs(1));

pub struct TomataApp {
    timer_id: TimerToken,
    widget_tree: Box<dyn Widget<TomataState>>,
}

/// Main widget that holds the widget tree of all the elements that
/// build the application.
impl TomataApp {
    pub fn new() -> TomataApp {
        TomataApp {
            timer_id: TimerToken::INVALID,
            widget_tree: Box::new(make_main_window_widget_tree()),
        }
    }
}

impl Widget<TomataState> for TomataApp {
    fn event(
        &mut self,
        ctx: &mut EventCtx<'_, '_>,
        event: &Event,
        data: &mut TomataState,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                // Sets up te timer which fires the [`Event::Timer`] event
                // after specified amount of time. This mechanism is
                // used to count elapsed time.
                self.timer_id = ctx.request_timer(*TICK_INTERVAL);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    if !data.is_stopwatch_paused() {
                        data.increase_elapsed_time(*TICK_INTERVAL);
                    }
                    if data.is_period_finished() {
                        data.cycle_to_next_period();
                    }
                    // Timer must be requested each time seperately.
                    self.timer_id = ctx.request_timer(*TICK_INTERVAL);
                }
            }
            _ => {}
        }
        self.widget_tree.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx<'_, '_>,
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
        ctx: &mut UpdateCtx<'_, '_>,
        old_data: &TomataState,
        data: &TomataState,
        env: &Env,
    ) {
        self.widget_tree.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx<'_, '_>,
        bc: &BoxConstraints,
        data: &TomataState,
        env: &Env,
    ) -> Size {
        self.widget_tree.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &TomataState, env: &Env) {
        self.widget_tree.paint(ctx, data, env);
    }
}

fn make_main_window_widget_tree() -> impl Widget<TomataState> {
    let remaining_time_label = Label::new(|data: &TomataState, _env: &_| {
        tomata::duration_to_string(&data.calculate_remaining_time())
    })
    .with_text_size(52.0);

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

    Flex::column()
        .with_child(Align::centered(remaining_time_label))
        .with_child(Padding::new(
            1.0,
            Align::centered(
                Flex::row()
                    .with_child(start_button)
                    .with_child(pause_button)
                    .with_child(reset_button)
                    .with_child(work_period_button)
                    .with_child(short_break_period_button)
                    .with_child(long_break_period_button),
            ),
        ))
        .with_spacer(10.0)
        .with_flex_child(make_settings_wdiget_tree(), 1.0)
}

fn make_settings_wdiget_tree() -> impl Widget<TomataState> {
    Padding::new(
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
            .with_child(make_beep_volume_adjustment_row())
            .with_spacer(3.0)
            .with_child(make_save_row())
            .with_spacer(3.0),
    )
}

fn make_period_adjustment_row(period: Period) -> impl Widget<TomataState> {
    Flex::row()
        .with_child(make_period_name_label(period))
        .with_flex_child(
            Align::right(
                Flex::row()
                    .with_child(make_period_value_label(period))
                    .with_child(make_period_adjustment_buttons(period)),
            ),
            1.0,
        )
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
    Flex::row().with_child(description_label).with_flex_child(
        Align::right(
            Flex::row()
                .with_child(value_label)
                .with_child(make_short_breaks_adjustment_buttons()),
        ),
        1.0,
    )
}

fn make_short_breaks_number_before_long_break() -> impl Widget<TomataState> {
    let label: Label<usize> = Label::new(|data: &usize, _env: &_| format!("{}", *data));
    let label = LensWrap::new(label, Settings::short_breaks_number);
    LensWrap::new(label, TomataState::settings)
}

fn make_short_breaks_adjustment_buttons() -> impl Widget<TomataState> {
    let plus_button = Button::new("+").on_click(move |_ctx, data: &mut Settings, _env| {
        data.increase_short_breaks_number(1);
    });
    let minus_button = Button::new("\u{2212}").on_click(move |_ctx, data: &mut Settings, _env| {
        data.decrease_short_breaks_number(1);
    });
    Flex::row()
        .with_child(LensWrap::new(plus_button, TomataState::settings))
        .with_child(LensWrap::new(minus_button, TomataState::settings))
}

fn make_long_break_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Include long breaks:");
    let switch = Switch::new();
    let switch = LensWrap::new(switch, Settings::long_breaks_are_included);
    let switch = LensWrap::new(switch, TomataState::settings);
    Flex::row()
        .with_child(description_label)
        .with_flex_child(Align::right(switch), 1.0)
}

fn make_next_period_starts_automatically_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Start next period automatically:");
    let switch = Switch::new();
    let switch = LensWrap::new(switch, Settings::next_period_starts_automatically);
    let switch = LensWrap::new(switch, TomataState::settings);
    Flex::row()
        .with_child(description_label)
        .with_flex_child(Align::right(switch), 1.0)
}

fn make_system_notifications_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Use system notifications:");
    let switch = Switch::new();
    let switch = LensWrap::new(switch, Settings::system_notifications_are_enabled);
    let switch = LensWrap::new(switch, TomataState::settings);
    Flex::row()
        .with_child(description_label)
        .with_flex_child(Align::right(switch), 1.0)
}

fn make_period_finishing_sound_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Use beeping sound when period is ending:");
    let switch = Switch::new();
    let switch = LensWrap::new(switch, Settings::period_ending_sound_is_enabled);
    let switch = LensWrap::new(switch, TomataState::settings);
    Flex::row()
        .with_child(description_label)
        .with_flex_child(Align::right(switch), 1.0)
}

fn make_beep_volume_adjustment_row() -> impl Widget<TomataState> {
    let description_label = Label::new("Beep volume:");
    let slider = Slider::new().with_range(0.0, 1.0);
    let slider = LensWrap::new(slider, Settings::beep_volume);
    let slider = LensWrap::new(slider, TomataState::settings);
    let beep_button = Button::new("try").on_click(move |_ctx, data: &mut TomataState, _env| {
        data.beep();
    });
    Flex::row().with_child(description_label).with_flex_child(
        Align::right(Flex::row().with_child(beep_button).with_child(slider)),
        1.0,
    )
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
