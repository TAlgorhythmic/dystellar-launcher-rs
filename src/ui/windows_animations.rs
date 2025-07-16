use gtk::{gdk::MotionEvent, glib::property::PropertyGet, gsk::Transform, prelude::WidgetExt, Button, EventControllerMotion};
use libadwaita::{prelude::AnimationExt, Bin, CallbackAnimationTarget, Easing, TimedAnimation};

use crate::ui::components::bin_wrap_btn;

fn growable_anim_callback(param: f64, btn: &Button) {
    let (_, width, _, _) = btn.measure(gtk::Orientation::Horizontal, -1);
    let (_, height, _, _) = btn.measure(gtk::Orientation::Vertical, -1);
    let transform = Transform::new().scale(param as f32, param as f32);

    btn.allocate(width, height, btn.allocated_baseline(), Some(transform));
}

fn grow_hover_anim(widget: Button) -> TimedAnimation {
    TimedAnimation::builder()
        .widget(&widget)
        .easing(Easing::EaseInOutCubic)
        .target(&CallbackAnimationTarget::new(move |value| growable_anim_callback(value, &widget)))
        .duration(180) // 0.18s
        .value_from(1.0)
        .value_to(1.16)
        .alternate(false)
        .build()
}

pub fn windows_add_growable_animation_bin(btn: Button) -> Bin {
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
    bin_wrap_btn(btn)
}
