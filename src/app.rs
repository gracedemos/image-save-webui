use leptos::*;

use crate::banner::Banner;
use crate::find_image::{FindImage, Image};
use crate::add_image::AddImage;
use crate::utils;
use std::rc::Rc;
use solana_client_wasm::{
    WasmClient,
    solana_sdk::pubkey::Pubkey
};

#[component]
pub fn App() -> impl IntoView {
    let client = Rc::new(WasmClient::new(utils::RPC_URL));
    let (tab, set_tab) = create_signal(0);
    let (pubkey, set_pubkey) = create_signal(String::new());
    let (pda, set_pda) = create_signal::<Option<Pubkey>>(None);

    let future = async move {
        let key: String = utils::trustedConnect()
            .await
            .try_into()
            .unwrap();
        set_pubkey(key);
    };
    wasm_bindgen_futures::spawn_local(future);

    view! {
        <div class="h-screen bg-slate-800 flex flex-col items-center">
            <Banner set_tab=set_tab pubkey=pubkey set_pubkey=set_pubkey/>
            {
                move || if tab() == 0 {
                    if let None = pda() {
                        view! {<FindImage set_pda=set_pda pubkey=pubkey/>}
                    } else {
                        view! {<Image pda=pda set_pda=set_pda client=client.clone()/>}
                    }
                } else {
                    view! {<AddImage pubkey=pubkey client=client.clone()/>}
                }
            }
        </div>
    }
}

