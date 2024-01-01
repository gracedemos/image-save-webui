use leptos::*;
use crate::utils;
use solana_client_wasm::{
    WasmClient,
    solana_sdk::pubkey::Pubkey,
    solana_sdk::transaction::Transaction,
    solana_sdk::instruction::Instruction
};

#[component]
pub fn AddImage(pubkey: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="flex flex-col bg-slate-400 p-4 rounded w-1/3 m-auto">
            <input type="text" placeholder="URL" class="p-4 rounded mb-4"/>
            <input type="text" placeholder="Title" class="p-4 rounded mb-4"/>
            <button class="bg-slate-600 p-4 rounded text-slate-200 hover:bg-slate-800">Add Image</button>
        </div>
    }
}
