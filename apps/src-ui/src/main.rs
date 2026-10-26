
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div style="text-align: center; padding: 40px;">
            <h1>"Hello World!"</h1>
            <p>"Tauri + Leptosで作成したデスクトップアプリ"</p>
            <button on:click=|_| {
                println!("ボタンが押されました");
            }>
                "クリック！"
            </button>
        </div>
    }
}
