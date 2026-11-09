
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use serde::Serialize;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        js_namespace = ["window","__TAURI__", "core"],
        js_name = invoke,
        catch
    )]
    async fn invoke_tauri(
        command: &str,
        args: JsValue,
    ) -> Result<JsValue, JsValue>;
}

#[derive(Serialize)]
struct GreetArgs {
    name: String,
}

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (system_info, set_system_info) = signal(String::new());

    let greet = move |_| {
        let name = name.get();
        let set_message = set_message;

        leptos::task::spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&GreetArgs { name });

            match args {
                Ok(args) => {
                    match invoke_tauri("greet", args).await {
                        Ok(result) => {
                            set_message.set(
                                result.as_string().unwrap_or_default()
                            );
                        }
                        Err(_) => {
                            set_message.set(
                                "Rust関数の呼び出しに失敗しました".into()
                            );
                        }
                    }
                }
                Err(_) => {
                    set_message.set(
                        "引数の変換に失敗しました".into()
                    );
                }
            }
        });
    };

    // システム情報取得機能
    let get_info = move |_| {
        leptos::task::spawn_local(async move {
            match invoke_tauri("get_system_info", JsValue::NULL).await {
                Ok(result) => {
                    set_system_info.set(
                        result.as_string().unwrap_or_default()
                    );
                }
                Err(_) => {
                    set_message.set(
                        "システム情報の取得に失敗しました".into()
                    );
                }
            }
        });
    };

    view! {
        <div style="text-align: center; padding: 40px;">
            <h1>"Tauri + Leptos"</h1>
            <p>"Rustバックエンドとの通信テスト"</p>

            <input
                type="text"
                placeholder="名前を入力"
                prop:value=move || name.get()
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                }
            />

            <button on:click=greet>
                "Rustの関数を実行"
            </button>

            <p>{move || message.get()}</p>

            <hr/>

            <h2>"windowsシステム情報"</h2>
            <button on:click=get_info>
                "システム情報を取得"
            </button>

            <pre style="white-space: pre-wrap; text-align: left; margin: 20px auto; max-width: 600px;">
                {move || system_info.get()}
            </pre>
        </div>
    }
}