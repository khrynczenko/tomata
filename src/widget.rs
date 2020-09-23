use std::time::Duration;

use druid::widget::{Button, Flex, Label};
use druid::{
    BoxConstraints, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx,
};
use druid::{Env, TimerToken, Widget};

use crate::state::TomataState;
use crate::tomata;
use crate::tomata::SECOND_S;

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
