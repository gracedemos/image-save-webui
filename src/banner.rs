use leptos::*;
use crate::utils;

#[component]
pub fn Banner(set_tab: WriteSignal<i32>, pubkey: ReadSignal<String>, set_pubkey: WriteSignal<String>) -> impl IntoView {
    view! {
        <div class="flex bg-slate-400 p-4 justify-between items-center w-full">
            <h1 class="text-2xl flex-1">Image Save</h1>
            <div class="justify-center flex">
                <button class="bg-slate-600 p-4 mr-4 text-slate-200 rounded hover:bg-slate-800"
                on:click=move |_| set_tab(0)>Find Image</button>
                <button class="bg-slate-600 p-4 text-slate-200 rounded hover:bg-slate-800"
                on:click=move |_| set_tab(1)>Add Image</button>
            </div>
            <div class="flex-1 flex flex-row-reverse">
                <button class="bg-slate-600 p-4 text-slate-200 rounded hover:bg-slate-800"
                on:click=move |_| {
                        let future = async move {
                            let key: String = utils::connect()
                                .await
                                .try_into()
                                .unwrap();
                            set_pubkey(key);
                        };
                        wasm_bindgen_futures::spawn_local(future);
                    }>
                    {
                        move || if pubkey().eq(&String::from("")) {
                            String::from("Connect to Wallet")
                        } else {
                            pubkey()
                        }
                    }
                </button>
            </div>
        </div>
    }
}
