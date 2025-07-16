use gtk::{gdk::MotionEvent, prelude::WidgetExt, Button, EventControllerMotion};
use libadwaita::{prelude::AnimationExt, Bin, CallbackAnimationTarget, Easing, TimedAnimation};

use crate::ui::components::bin_wrap_btn;

fn growable_anim_callback(param: f64, btn: Button, width: i32, height: i32) {
    
}

pub fn grow_hover_anim(widget: Button) -> TimedAnimation {
    TimedAnimation::builder()
        .widget(&widget)
        .easing(Easing::EaseInOutCubic)
        .target(&CallbackAnimationTarget::new(move |value| growable_anim_callback(value, widget.clone(), widget.width(), widget.height())))
        .duration(180) // 0.18s
        .value_from(1.0)
        .value_to(1.16)
        .alternate(true)
        .build()
}

pub fn set_pseudo_class_growable_bin(btn: Button) -> Bin {
    // Create animation manually because windows behaves weirdly
    #[cfg(target_os = "windows")] {
        let anim = grow_hover_anim(btn.clone());

        let ev = EventControllerMotion::new();
        ev.connect_enter(move |_, _, _| anim.play());
    }

    bin_wrap_btn(btn)
}
