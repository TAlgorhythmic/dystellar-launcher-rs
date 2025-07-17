#[cfg(target_os = "windows")]
use gtk::{glib::object::ObjectExt, graphene::Point, gsk::Transform, prelude::GestureExt, EventControllerMotion, GestureClick};
#[cfg(target_os = "windows")]
use libadwaita::{prelude::AnimationExt, CallbackAnimationTarget, Easing, TimedAnimation};

use gtk::{prelude::WidgetExt, Align, Button};
use libadwaita::Bin;

use crate::ui::components::bin_wrap_btn;


#[cfg(target_os = "windows")]
fn growable_anim_callback(param: f64, btn: &Button) {
    let allocation = btn.allocation();
    let width = allocation.width();
    let height = allocation.height();

    let newwidth = (width as f64 * param).floor() - 2.0;
    let newheight = (height as f64 * param).floor() - 2.0;

    let transform = Transform::new()
        .scale(param as f32, param as f32)
        .translate(&Point::new(width as f32 / 2.0, height as f32 / 2.0))
        .translate(&Point::new(-newwidth as f32 / 2.0, -newheight as f32 / 2.0));

    btn.allocate(width, height, btn.allocated_baseline(), Some(transform));
}

#[cfg(target_os = "windows")]
fn clickable_anim_callback(param: f64, btn: &Button) {
    btn.set_opacity(param);
}

#[cfg(target_os = "windows")]
fn grow_hover_anim(btn: Button) -> TimedAnimation {
    TimedAnimation::builder()
        .widget(&btn)
        .easing(Easing::EaseInOutCubic)
        .target(&CallbackAnimationTarget::new(move |value| growable_anim_callback(value, &btn)))
        .duration(180) // 0.18s
        .value_from(1.0)
        .value_to(1.16)
        .alternate(false)
        .build()
}

#[cfg(target_os = "windows")]
fn brightness_click_animation(btn: Button) -> TimedAnimation {
    TimedAnimation::builder()
        .widget(&btn)
        .easing(Easing::EaseInOutCubic)
        .target(&CallbackAnimationTarget::new(move |value| clickable_anim_callback(value, &btn)))
        .duration(45) // 0.18s
        .value_from(1.0)
        .value_to(0.6)
        .alternate(false)
        .build()
}

#[cfg(target_os = "windows")]
fn shrink_click_animation(btn: Button) -> TimedAnimation {
    TimedAnimation::builder()
        .widget(&btn)
        .easing(Easing::EaseInOutCubic)
        .target(&CallbackAnimationTarget::new(move |value| growable_anim_callback(value, &btn)))
        .duration(45) // 0.18s
        .value_from(1.0)
        .value_to(0.96)
        .alternate(false)
        .build()
}

pub fn add_clickable_growable_animation_btn(btn: Button) -> Bin {
    #[cfg(target_os = "windows")] {
        let anim_grow = grow_hover_anim(btn.clone());
        let anim_grow_cl = anim_grow.clone();
        let anim_grow_cl2 = anim_grow.clone();
        let anim_grow_cl3 = anim_grow.clone();

        let ev_grow = EventControllerMotion::new();
        ev_grow.connect_enter(move |_, _, _| {
            anim_grow.set_reverse(false);
            anim_grow.set_duration(180);
            anim_grow.play();
        });
        ev_grow.connect_leave(move |_| {
            anim_grow_cl.set_reverse(true);
            anim_grow_cl.set_duration(180);
            anim_grow_cl.play();
        });

        let anim_click = brightness_click_animation(btn.clone());
        let anim_click_cl = anim_click.clone();

        let ev_click = GestureClick::new();
        ev_click.connect_pressed(move |ges, _, _, _| {
            ges.set_state(gtk::EventSequenceState::Claimed);

            anim_click.set_reverse(false);
            anim_click.set_duration(45);
            anim_click.play();
            anim_grow_cl2.set_reverse(true);
            anim_grow_cl2.set_duration(45);
            anim_grow_cl2.play();
        });

        let ev_grow_cl = ev_grow.clone();
        ev_click.connect_released(move |ges, _, _, _| {
            ges.set_state(gtk::EventSequenceState::Claimed);

            anim_click_cl.set_reverse(true);
            anim_click_cl.set_duration(180);
            anim_click_cl.play();

            anim_grow_cl3.set_duration(180);
            if ev_grow_cl.contains_pointer() {
                anim_grow_cl3.set_reverse(false);
                anim_grow_cl3.play();
            }
        });
        ev_click.connect_cancel(|ges, _| {
            ges.set_state(gtk::EventSequenceState::Denied);
        });

        btn.add_controller(ev_grow);
        btn.add_controller(ev_click);
    }
    #[cfg(not(target_os = "windows"))] {
        btn.add_css_class("growable");
        btn.add_css_class("clickable");
    }

    bin_wrap_btn(btn)
}

pub fn add_clickable_animation_btn(btn: Button) -> Bin {
    #[cfg(target_os = "windows")] {
        let shrink_click = shrink_click_animation(btn.clone());
        let shrink_click_cl = shrink_click.clone();
        let anim_click = brightness_click_animation(btn.clone());
        let anim_click_cl = anim_click.clone();

        let ev_click = GestureClick::new();
        ev_click.connect_pressed(move |ges, _, _, _| {
            ges.set_state(gtk::EventSequenceState::Claimed);

            anim_click.set_reverse(false);
            anim_click.set_duration(45);
            anim_click.play();
            shrink_click.set_reverse(false);
            shrink_click.set_duration(45);
            shrink_click.play();
        });

        ev_click.connect_released(move |ges, _, _, _| {
            ges.set_state(gtk::EventSequenceState::Claimed);

            anim_click_cl.set_reverse(true);
            anim_click_cl.set_duration(180);
            anim_click_cl.play();

            shrink_click_cl.set_duration(180);
            shrink_click_cl.set_reverse(true);
            shrink_click_cl.play();
        });
        ev_click.connect_cancel(|ges, _| {
            ges.set_state(gtk::EventSequenceState::Denied);
        });

        btn.add_controller(ev_click);
    }
    #[cfg(not(target_os = "windows"))] {
        btn.add_css_class("clickable");
    }

    bin_wrap_btn(btn)
}

pub fn add_growable_animation_btn(btn: Button) -> Bin {
    #[cfg(target_os = "windows")] {
        let anim = grow_hover_anim(btn.clone());
        let anim_cl = anim.clone();

        let ev = EventControllerMotion::new();
        ev.connect_enter(move |_, _, _| {
            anim.set_reverse(false);
            anim.play();
        });
        ev.connect_leave(move |_| {
            anim_cl.set_reverse(true);
            anim_cl.play();
        });

        btn.add_controller(ev);
    }
    #[cfg(not(target_os = "windows"))] {
        btn.add_css_class("growable");
    }

    bin_wrap_btn(btn)
}
