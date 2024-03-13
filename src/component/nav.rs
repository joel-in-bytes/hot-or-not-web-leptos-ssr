use super::nav_icons::*;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
fn NavIcon(
    idx: usize,
    #[prop(into)] href: MaybeSignal<String>,
    #[prop(into)] icon: icondata_core::Icon,
    #[prop(optional)] filled_icon: Option<icondata_core::Icon>,
    cur_selected: Memo<usize>,
) -> impl IntoView {
    view! {
        <a href=href class="flex items-center justify-center">
            <Show
                when=move || cur_selected() == idx
                fallback=move || view! { <Icon icon=icon class="text-white h-6 w-6"/> }
            >
                <Icon
                    icon=filled_icon.unwrap_or(icon)
                    class="text-primary-600 aspect-square h-6 w-6"
                />
                <div class="absolute bottom-0 bg-primary-600 py-1 w-6 blur-md"></div>
            </Show>
        </a>
    }
}

#[component]
fn TrophyIcon(idx: usize, cur_selected: Memo<usize>) -> impl IntoView {
    view! {
        <a href="/leaderboard" class="flex items-center justify-center">
            <Show
                when=move || cur_selected() == idx
                fallback=move || {
                    view! { <Icon icon=TrophySymbol class="text-white fill-none h-6 w-6"/> }
                }
            >

                <Icon
                    icon=TrophySymbolFilled
                    class="text-primary-600 fill-none aspect-square h-6 w-6"
                />
                <div class="absolute bottom-0 bg-primary-600 py-1 w-6 blur-md"></div>
            </Show>
        </a>
    }
}

#[component]
fn UploadIcon(idx: usize, cur_selected: Memo<usize>) -> impl IntoView {
    view! {
        <a href="/upload" class="flex items-center justify-center rounded-fullt text-white">
            <Show
                when=move || cur_selected() == idx
                fallback=move || {
                    view! {
                        <Icon
                            icon=icondata::AiPlusOutlined
                            class="rounded-full bg-primary-500 h-10 w-10 p-2"
                        />
                    }
                }
            >

                <Icon
                    icon=icondata::AiPlusOutlined
                    class="bg-primary-600 rounded-full aspect-square h-10 w-10 p-2"
                />
                <div class="absolute bottom-0 bg-primary-600 py-1 w-10 blur-md"></div>
            </Show>
        </a>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let cur_location = use_location();
    let home_path = create_rw_signal("/".to_string());
    let cur_selected = create_memo(move |_| {
        let path = cur_location.pathname.get();
        match path.as_str() {
            "/" => 0,
            "/leaderboard" => 1,
            "/upload" => 2,
            "/wallet" | "/transactions" => 3,
            "/menu" => 4,
            s if s.starts_with("/hot-or-not") => {
                home_path.set(path);
                0
            }
            _ => 4,
        }
    });
    let bg_color = move || {
        if cur_selected() == 0 {
            "bg-transparent"
        } else {
            "bg-black"
        }
    };

    view! {
        <div class=move || {
            format!(
                "flex flex-row justify-between px-6 py-2 w-full {} fixed left-0 bottom-0 z-50",
                bg_color(),
            )
        }>
            <NavIcon
                idx=0
                href=home_path
                icon=HomeSymbol
                filled_icon=HomeSymbolFilled
                cur_selected=cur_selected
            />
            <TrophyIcon idx=1 cur_selected/>
            <UploadIcon idx=2 cur_selected/>
            <NavIcon
                idx=3
                href="/wallet"
                icon=WalletSymbol
                filled_icon=WalletSymbolFilled
                cur_selected=cur_selected
            />
            <NavIcon idx=4 href="/menu" icon=MenuSymbol cur_selected=cur_selected/>
        </div>
    }
}
