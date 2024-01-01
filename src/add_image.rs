use leptos::*;
use crate::utils;
use crate::utils::Image;
use std::rc::Rc;
use solana_client_wasm::{
    WasmClient,
    solana_sdk::pubkey::Pubkey,
    solana_sdk::transaction::Transaction,
    solana_sdk::instruction::Instruction,
    solana_sdk::instruction::AccountMeta,
    solana_sdk::system_program
};

#[component]
pub fn AddImage(pubkey: ReadSignal<String>, client: Rc<WasmClient>) -> impl IntoView {
    let (url, set_url) = create_signal(String::new());
    let (title, set_title) = create_signal(String::new());

    view! {
        <div class="flex flex-col bg-slate-400 p-4 rounded w-1/3 m-auto">
            <input type="text" placeholder="URL" class="p-4 rounded mb-4" prop:value=url
            on:input=move |ev| set_url(event_target_value(&ev))/>
            <input type="text" placeholder="Title" class="p-4 rounded mb-4" prop:value=title
            on:input=move |ev| set_title(event_target_value(&ev))/>
            <button class="bg-slate-600 p-4 rounded text-slate-200 hover:bg-slate-800"
            on:click=move |_| submit_image(pubkey, url(), title())>Add Image</button>
        </div>
    }
}

fn submit_image(signer: ReadSignal<String>, url: String, title: String) {
    let signer = Pubkey::try_from(signer().as_str())
        .unwrap();
    let image = Image {url, title};
    let program_id = Pubkey::try_from(utils::PROGRAM_ID)
        .unwrap();

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[signer.as_ref(), image.title.as_bytes()],
        &program_id
    );

    let instruction = Instruction::new_with_bincode(
        signer,
        &image,
        vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(pda, false),
            AccountMeta::new(system_program::id(), false)
        ]
    );

    let transaction = Transaction::new_with_payer(&[instruction], Some(&signer));
    let future = async move {
        utils::sendTransaction(transaction)
            .await; 
    };
    wasm_bindgen_futures::spawn_local(future);
}
