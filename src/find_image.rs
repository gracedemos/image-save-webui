use leptos::*;
use std::rc::Rc;
use crate::utils;
use crate::utils::Image;
use solana_client_wasm::{
    solana_sdk::{pubkey::Pubkey, signature::Signable},
    WasmClient
};

#[component]
pub fn FindImage(set_pda: WriteSignal<Option<Pubkey>>, pubkey: ReadSignal<String>) -> impl IntoView {
    let (title, set_title) = create_signal(String::new());

    view! {
        <div class="flex flex-col bg-slate-400 p-4 rounded w-1/3 m-auto">
            <input type="text" placeholder="Title" class="p-4 mb-4 rounded" prop:value=title
            on:input=move |ev| set_title(event_target_value(&ev))/>
            <button class="bg-slate-600 p-4 rounded text-slate-200 hover:bg-slate-800"
            on:click=move |_| {
                    if title().ne(&String::from("")) {
                        let pda = get_pda(pubkey(), title());
                        set_pda(Some(pda));
                    }
                }>Find Image</button>
        </div>
    }
}

#[component]
pub fn Image(pda: ReadSignal<Option<Pubkey>>, set_pda: WriteSignal<Option<Pubkey>>, client: Rc<WasmClient>) -> impl IntoView {
    let pda = pda()
        .unwrap();
    let (data, set_data) = create_signal(Vec::<u8>::new());
    let future = async move {
        let temp_data = client.get_account_data(&pda)
            .await
            .unwrap();
        set_data(temp_data);
    };
    wasm_bindgen_futures::spawn_local(future);

    let image: Result<Image, Box<bincode::ErrorKind>> = bincode::deserialize(&data());

    if let Ok(img) = image {
        view! {
            <div class="flex flex-col bg-slate-400 p-4 rounded w-1/3 m-auto items-center">
                <img src={img.url}/>
                <button class="bg-slate-600 p-4 rounded text-slate-200 hover:bg-slate-800 w-full"
                on:click=move |_| set_pda(None)>Back</button>
            </div>
        }
    } else {
        view! {
            <div class="flex flex-col bg-slate-400 p-4 rounded w-1/3 m-auto items-center">
                <h1 class="p-4 mb-4">Image Not Found</h1>
                <button class="bg-slate-600 p-4 rounded text-slate-200 hover:bg-slate-800 w-full"
                on:click=move |_| set_pda(None)>Back</button>
            </div>
        }
    }
}

fn get_pda(pubkey: String, title: String) -> Pubkey {
    let key = Pubkey::try_from(pubkey.as_str())
        .unwrap();
    let program_id = Pubkey::try_from(utils::PROGRAM_ID)
        .unwrap();

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[key.as_ref(), title.as_bytes()],
        &program_id
    );

    pda
}
