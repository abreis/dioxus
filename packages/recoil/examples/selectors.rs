use dioxus_core::prelude::*;
use recoil::*;

const A: Atom<i32> = |_| 0;
const B: Atom<i32> = |_| 0;
const C: Selector<i32> = |api| api.get(&A) + api.get(&B);

static App: FC<()> = |ctx, _| {
    use_init_recoil_root(ctx);
    rsx! { in ctx,
        div {
            Banner {}
            BtnA {}
            BtnB {}
        }
    }
};

static Banner: FC<()> = |ctx, _| {
    let count = use_recoil_value(ctx, &C);
    ctx.render(rsx! { h1 { "Count: {count}" } })
};

static BtnA: FC<()> = |ctx, _| {
    let (a, set) = use_recoil_state(ctx, &A);
    rsx! { in ctx,
        div { "a"
            button { "+", onclick: move |_| set(a + 1) }
            button { "-", onclick: move |_| set(a - 1) }
        }
    }
};

static BtnB: FC<()> = |ctx, _| {
    let (b, set) = use_recoil_state(ctx, &B);
    rsx! { in ctx,
        div { "b"
            button { "+", onclick: move |_| set(b + 1) }
            button { "-", onclick: move |_| set(b - 1) }
        }
    }
};

fn main() {
    wasm_bindgen_futures::spawn_local(dioxus_web::WebsysRenderer::start(App))
}