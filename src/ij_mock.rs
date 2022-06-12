
//! the module for intellij-rust
//! intellij-rust doesn't have support for proc macros.
//! so I rewrite minimum html! macro parser with simple macro.
//! Thanks to this, you can get syntax highlights in {}!

use yew::Html;

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! html {
    (@root [$($tag: tt)*] (<$tag0: ident $($rest: tt)*)) => {
        html! { @check_tag $tag0 }
        html! { @attrs [$tag0 $($tag)*] ($($rest)*) }
    };
    (@root [$($tag: tt)*] (<> $($rest: tt)*)) => {
        html! { @body [@ $($tag)+] ($($rest)*) }
    };

    (@attrs [$($tag: tt)+] (> $($rest: tt)*)) => {
        html! { @body [$($tag)+] ($($rest)*) }
    };
    (@attrs [$($tag: tt)+] ($($attr_name: ident)-+ $($rest: tt)*)) => {
        html! { @attr_pre_eq [$($tag)+] [$($attr_name)-+] ($($rest)*) }
    };
    (@attrs [$($tag: tt)+] ({$attr_name: ident} $($rest: tt)*)) => {
        let _ = $attr_name;
        html! { @attrs [$($tag)+] ($($rest)*) }
    };
    (@attrs [$tag0: ident $($tag: tt)+] (/>$($rest: tt)*)) => {
        html! { @body [$($tag)+] ($($rest)*) }
    };

    (@attr_pre_eq [$($tag: tt)+] [$($attr_name: ident)-+] (= $($rest: tt)*)) => {
        html! { @attr_value [$($tag)+] [$($attr_name)-+] ($($rest)*) }
    };
    (@attr_pre_eq [$($tag: tt)+] [$($attr_name: ident)-+] ($($rest: tt)*)) => {
        compile_error!("no = on attribute")
    };

    (@attr_value [$($tag: tt)+] [$($attr_name: ident)-+] ({$($expr: tt)*} $($rest: tt)*)) => {
        let _ = html!(@attr_type $($attr_name)-+ $($expr)*); // TODO: type check
        html! { @attrs [$($tag)+] ($($rest)*) }
    };
    (@attr_value [$($tag: tt)+] [$($attr_name: ident)-+] ($literal: literal $($rest: tt)*)) => {
        let _ = html!(@attr_type $($attr_name)-+ $literal); // TODO: type check
        html! { @attrs [$($tag)+] ($($rest)*) }
    };

    (@body [$($tag: tt)+] ({ for $($expr: tt)*} $($rest: tt)*)) => {
        $crate::ij_mock::iter_expected($($expr)*); // TODO: type check
        html! { @body [$($tag)+] ($($rest)*) }
    };
    (@body [$($tag: tt)+] ({$($expr: tt)*} $($rest: tt)*)) => {
        let _ = ($($expr)*); // TODO: type check
        html! { @body [$($tag)+] ($($rest)*) }
    };
    (@body [$($tag: tt)*] (<$tag0: ident $($rest: tt)*)) => {
        html! { @check_tag $tag0 }
        html! { @attrs [$tag0 $($tag)*] ($($rest)*) }
    };
    (@body [$tag0: ident $($tag: tt)+] (</$tag1: ident> $($rest: tt)*)) => {
        html! { @body [$($tag)+] ($($rest)*) }
    };
    (@body [$tag0: ident] (</$tag1: ident> $($rest: tt)*)) => {
        $crate::ij_mock::html()
    };

    (@check_tag $tag: ident) => {
        {
            use $crate::ij_mock::tags::*;
            fn _unused() -> $tag {
                todo!()
            }
        }
    };

    (@attr_type onabort $($expr: tt)*) => {html!(@attr_type @event onabort $($expr)*)};
    (@attr_type onauxclick $($expr: tt)*) => {html!(@attr_type @event onauxclick $($expr)*)};
    (@attr_type onblur $($expr: tt)*) => {html!(@attr_type @event onblur $($expr)*)};
    (@attr_type oncancel $($expr: tt)*) => {html!(@attr_type @event oncancel $($expr)*)};
    (@attr_type oncanplay $($expr: tt)*) => {html!(@attr_type @event oncanplay $($expr)*)};
    (@attr_type oncanplaythrough $($expr: tt)*) => {html!(@attr_type @event oncanplaythrough $($expr)*)};
    (@attr_type onchange $($expr: tt)*) => {html!(@attr_type @event onchange $($expr)*)};
    (@attr_type onclick $($expr: tt)*) => {html!(@attr_type @event onclick $($expr)*)};
    (@attr_type onclose $($expr: tt)*) => {html!(@attr_type @event onclose $($expr)*)};
    (@attr_type oncontextmenu $($expr: tt)*) => {html!(@attr_type @event oncontextmenu $($expr)*)};
    (@attr_type oncuechange $($expr: tt)*) => {html!(@attr_type @event oncuechange $($expr)*)};
    (@attr_type ondblclick $($expr: tt)*) => {html!(@attr_type @event ondblclick $($expr)*)};
    (@attr_type ondrag $($expr: tt)*) => {html!(@attr_type @event ondrag $($expr)*)};
    (@attr_type ondragend $($expr: tt)*) => {html!(@attr_type @event ondragend $($expr)*)};
    (@attr_type ondragenter $($expr: tt)*) => {html!(@attr_type @event ondragenter $($expr)*)};
    (@attr_type ondragexit $($expr: tt)*) => {html!(@attr_type @event ondragexit $($expr)*)};
    (@attr_type ondragleave $($expr: tt)*) => {html!(@attr_type @event ondragleave $($expr)*)};
    (@attr_type ondragover $($expr: tt)*) => {html!(@attr_type @event ondragover $($expr)*)};
    (@attr_type ondragstart $($expr: tt)*) => {html!(@attr_type @event ondragstart $($expr)*)};
    (@attr_type ondrop $($expr: tt)*) => {html!(@attr_type @event ondrop $($expr)*)};
    (@attr_type ondurationchange $($expr: tt)*) => {html!(@attr_type @event ondurationchange $($expr)*)};
    (@attr_type onemptied $($expr: tt)*) => {html!(@attr_type @event onemptied $($expr)*)};
    (@attr_type onended $($expr: tt)*) => {html!(@attr_type @event onended $($expr)*)};
    (@attr_type onerror $($expr: tt)*) => {html!(@attr_type @event onerror $($expr)*)};
    (@attr_type onfocus $($expr: tt)*) => {html!(@attr_type @event onfocus $($expr)*)};
    (@attr_type onfocusin $($expr: tt)*) => {html!(@attr_type @event onfocusin $($expr)*)};
    (@attr_type onfocusout $($expr: tt)*) => {html!(@attr_type @event onfocusout $($expr)*)};
    (@attr_type onformdata $($expr: tt)*) => {html!(@attr_type @event onformdata $($expr)*)};
    (@attr_type oninput $($expr: tt)*) => {html!(@attr_type @event oninput $($expr)*)};
    (@attr_type oninvalid $($expr: tt)*) => {html!(@attr_type @event oninvalid $($expr)*)};
    (@attr_type onkeydown $($expr: tt)*) => {html!(@attr_type @event onkeydown $($expr)*)};
    (@attr_type onkeypress $($expr: tt)*) => {html!(@attr_type @event onkeypress $($expr)*)};
    (@attr_type onkeyup $($expr: tt)*) => {html!(@attr_type @event onkeyup $($expr)*)};
    (@attr_type onload $($expr: tt)*) => {html!(@attr_type @event onload $($expr)*)};
    (@attr_type onloadeddata $($expr: tt)*) => {html!(@attr_type @event onloadeddata $($expr)*)};
    (@attr_type onloadedmetadata $($expr: tt)*) => {html!(@attr_type @event onloadedmetadata $($expr)*)};
    (@attr_type onloadstart $($expr: tt)*) => {html!(@attr_type @event onloadstart $($expr)*)};
    (@attr_type onmousedown $($expr: tt)*) => {html!(@attr_type @event onmousedown $($expr)*)};
    (@attr_type onmouseenter $($expr: tt)*) => {html!(@attr_type @event onmouseenter $($expr)*)};
    (@attr_type onmouseleave $($expr: tt)*) => {html!(@attr_type @event onmouseleave $($expr)*)};
    (@attr_type onmousemove $($expr: tt)*) => {html!(@attr_type @event onmousemove $($expr)*)};
    (@attr_type onmouseout $($expr: tt)*) => {html!(@attr_type @event onmouseout $($expr)*)};
    (@attr_type onmouseover $($expr: tt)*) => {html!(@attr_type @event onmouseover $($expr)*)};
    (@attr_type onmouseup $($expr: tt)*) => {html!(@attr_type @event onmouseup $($expr)*)};
    (@attr_type onpause $($expr: tt)*) => {html!(@attr_type @event onpause $($expr)*)};
    (@attr_type onplay $($expr: tt)*) => {html!(@attr_type @event onplay $($expr)*)};
    (@attr_type onplaying $($expr: tt)*) => {html!(@attr_type @event onplaying $($expr)*)};
    (@attr_type onprogress $($expr: tt)*) => {html!(@attr_type @event onprogress $($expr)*)};
    (@attr_type onratechange $($expr: tt)*) => {html!(@attr_type @event onratechange $($expr)*)};
    (@attr_type onreset $($expr: tt)*) => {html!(@attr_type @event onreset $($expr)*)};
    (@attr_type onresize $($expr: tt)*) => {html!(@attr_type @event onresize $($expr)*)};
    (@attr_type onscroll $($expr: tt)*) => {html!(@attr_type @event onscroll $($expr)*)};
    (@attr_type onsecuritypolicyviolation $($expr: tt)*) => {html!(@attr_type @event onsecuritypolicyviolation $($expr)*)};
    (@attr_type onseeked $($expr: tt)*) => {html!(@attr_type @event onseeked $($expr)*)};
    (@attr_type onseeking $($expr: tt)*) => {html!(@attr_type @event onseeking $($expr)*)};
    (@attr_type onselect $($expr: tt)*) => {html!(@attr_type @event onselect $($expr)*)};
    (@attr_type onslotchange $($expr: tt)*) => {html!(@attr_type @event onslotchange $($expr)*)};
    (@attr_type onstalled $($expr: tt)*) => {html!(@attr_type @event onstalled $($expr)*)};
    (@attr_type onsubmit $($expr: tt)*) => {html!(@attr_type @event onsubmit $($expr)*)};
    (@attr_type onsuspend $($expr: tt)*) => {html!(@attr_type @event onsuspend $($expr)*)};
    (@attr_type ontimeupdate $($expr: tt)*) => {html!(@attr_type @event ontimeupdate $($expr)*)};
    (@attr_type ontoggle $($expr: tt)*) => {html!(@attr_type @event ontoggle $($expr)*)};
    (@attr_type onvolumechange $($expr: tt)*) => {html!(@attr_type @event onvolumechange $($expr)*)};
    (@attr_type onwaiting $($expr: tt)*) => {html!(@attr_type @event onwaiting $($expr)*)};
    (@attr_type onwheel $($expr: tt)*) => {html!(@attr_type @event onwheel $($expr)*)};
    (@attr_type oncopy $($expr: tt)*) => {html!(@attr_type @event oncopy $($expr)*)};
    (@attr_type oncut $($expr: tt)*) => {html!(@attr_type @event oncut $($expr)*)};
    (@attr_type onpaste $($expr: tt)*) => {html!(@attr_type @event onpaste $($expr)*)};
    (@attr_type onanimationcancel $($expr: tt)*) => {html!(@attr_type @event onanimationcancel $($expr)*)};
    (@attr_type onanimationend $($expr: tt)*) => {html!(@attr_type @event onanimationend $($expr)*)};
    (@attr_type onanimationiteration $($expr: tt)*) => {html!(@attr_type @event onanimationiteration $($expr)*)};
    (@attr_type onanimationstart $($expr: tt)*) => {html!(@attr_type @event onanimationstart $($expr)*)};
    (@attr_type ongotpointercapture $($expr: tt)*) => {html!(@attr_type @event ongotpointercapture $($expr)*)};
    (@attr_type onloadend $($expr: tt)*) => {html!(@attr_type @event onloadend $($expr)*)};
    (@attr_type onlostpointercapture $($expr: tt)*) => {html!(@attr_type @event onlostpointercapture $($expr)*)};
    (@attr_type onpointercancel $($expr: tt)*) => {html!(@attr_type @event onpointercancel $($expr)*)};
    (@attr_type onpointerdown $($expr: tt)*) => {html!(@attr_type @event onpointerdown $($expr)*)};
    (@attr_type onpointerenter $($expr: tt)*) => {html!(@attr_type @event onpointerenter $($expr)*)};
    (@attr_type onpointerleave $($expr: tt)*) => {html!(@attr_type @event onpointerleave $($expr)*)};
    (@attr_type onpointerlockchange $($expr: tt)*) => {html!(@attr_type @event onpointerlockchange $($expr)*)};
    (@attr_type onpointerlockerror $($expr: tt)*) => {html!(@attr_type @event onpointerlockerror $($expr)*)};
    (@attr_type onpointermove $($expr: tt)*) => {html!(@attr_type @event onpointermove $($expr)*)};
    (@attr_type onpointerout $($expr: tt)*) => {html!(@attr_type @event onpointerout $($expr)*)};
    (@attr_type onpointerover $($expr: tt)*) => {html!(@attr_type @event onpointerover $($expr)*)};
    (@attr_type onpointerup $($expr: tt)*) => {html!(@attr_type @event onpointerup $($expr)*)};
    (@attr_type onselectionchange $($expr: tt)*) => {html!(@attr_type @event onselectionchange $($expr)*)};
    (@attr_type onselectstart $($expr: tt)*) => {html!(@attr_type @event onselectstart $($expr)*)};
    (@attr_type onshow $($expr: tt)*) => {html!(@attr_type @event onshow $($expr)*)};
    (@attr_type ontouchcancel $($expr: tt)*) => {html!(@attr_type @event ontouchcancel $($expr)*)};
    (@attr_type ontouchend $($expr: tt)*) => {html!(@attr_type @event ontouchend $($expr)*)};
    (@attr_type ontouchmove $($expr: tt)*) => {html!(@attr_type @event ontouchmove $($expr)*)};
    (@attr_type ontouchstart $($expr: tt)*) => {html!(@attr_type @event ontouchstart $($expr)*)};
    (@attr_type ontransitioncancel $($expr: tt)*) => {html!(@attr_type @event ontransitioncancel $($expr)*)};
    (@attr_type ontransitionend $($expr: tt)*) => {html!(@attr_type @event ontransitionend $($expr)*)};
    (@attr_type ontransitionrun $($expr: tt)*) => {html!(@attr_type @event ontransitionrun $($expr)*)};
    (@attr_type ontransitionstart $($expr: tt)*) => {html!(@attr_type @event ontransitionstart $($expr)*)};
    (@attr_type @event $ident: ident $($expr: tt)*) => {::yew::html::$ident::Wrapper::__macro_new($($expr)*)};
    (@attr_type $ident: ident $($expr: tt)*) => { ($($expr)*) };

    (@$ident: ident $($rest: tt)*) => {
        compile_error!(concat!("no internal rule matched: ", stringify!($ident), ": ", stringify!($($rest)*)))
    };

    ($($rest: tt)*) => {
        {
            html! { @root [] ($($rest)*) }
        }
    };
}

#[allow(non_camel_case_types, dead_code)]
pub mod tags {
    macro_rules! structs {
        ($name: ident $($rest: ident)*) => {
            pub struct $name {}
            structs!{$($rest)*}
        };
        () => {};
    }
    structs! {
        a abbr address area article aside audio
        b base bdi bdo blockquote body br button
        canvas caption cite code col colgroup
        data datalist dd del details dfn dialog div dl dt
        em embed fieldset figcaption figure footer form
        h1 h2 h3 h4 h5 h6 head header hgroup hr html i
        iframe img input ins kbd label legend li link
        main map mark math menu meta meter nav noscript
        object ol optgroup option output p picture pre progress
        q rp rt ruby s samp script section select slot small source
        span strong style sub summary sup svg table tbody td template
        textarea tfoot th thead time title tr track u ul var video wbr
    }
}

pub fn html() -> Html {
    unreachable!()
}
pub fn iter_expected<T>(_iter: impl IntoIterator<Item = T>) {}
