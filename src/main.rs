mod config;
mod special_binds;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{EventTarget, File, HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

const FILENAMES: &[&str] = &[
    "cs2_machine_convars.vcfg",
    "cs2_user_convars_0_slot0.vcfg",
    "cs2_user_keys_0_slot0.vcfg",
];

#[derive(Default, Clone, PartialEq, Eq, Store)]
struct State {
    machine_convars: Option<File>,
    user_convars: Option<File>,
    binds: Option<File>,
    files_valid: [bool; 3],
    autoexec: Option<String>,
}

#[derive(Clone, PartialEq, Properties)]
struct FileInputProps {
    name: String,
    idx: usize,
}

#[function_component]
fn FileInput(FileInputProps { name, idx }: &FileInputProps) -> Html {
    let (state, dispatch) = use_store::<State>();
    let id = format!("file-input-{}", idx);

    let current_file = match idx {
        0 => &state.machine_convars,
        1 => &state.user_convars,
        2 => &state.binds,
        _ => {
            panic!("index greater than 2 not allowed")
        }
    };

    let target_filename = FILENAMES[*idx];
    let style_class = if let Some(file) = current_file {
        if file.name() == target_filename {
            dispatch.reduce_mut(|state| state.files_valid[*idx] = true);
            "file-valid"
        } else {
            "file-different"
        }
    } else {
        ""
    };

    // Loads file into appropriate place depending on element clicked
    let callback = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Event should have a target");
        let elm: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();
        let file = elm.files().unwrap().get(0).unwrap();
        match elm.id().as_str() {
            "file-input-0" => {
                dispatch.reduce_mut(|state| state.machine_convars = Some(file));
            }
            "file-input-1" => {
                dispatch.reduce_mut(|state| state.user_convars = Some(file));
            }
            "file-input-2" => {
                dispatch.reduce_mut(|state| state.binds = Some(file));
            }
            _ => {}
        }
    });

    html! {
        <label for={id.clone()} class={classes!("file-upload", style_class)}>
            { name }
            <input onchange={callback}
                id={id.clone()}
                type="file"
                accept=".vcfg"
            />
        </label>
    }
}

macro_rules! load_file {
    ($file_opt: expr) => {{
        let promise = &$file_opt.clone().unwrap().text();
        match JsFuture::from(promise.clone()).await {
            Ok(jsval) => jsval.as_string().unwrap(),
            Err(_jserr) => {
                panic!("Something weird happened");
            }
        }
    }};
}

async fn load_all_files(state: &mut State) {
    let mut valid = true;
    for f in [&state.machine_convars, &state.user_convars, &state.binds] {
        if f.is_none() {
            valid = false;
        }
    }

    if !valid {
        return;
    }
    state.autoexec = Some(config::config_str(
        load_file!(state.machine_convars),
        load_file!(state.user_convars),
        load_file!(state.binds),
    ));
}

fn download_file(text: &str) {
    let filename = "autoexec.cfg";

    let blob_val = js_sys::Array::from_iter(std::iter::once(wasm_bindgen::JsValue::from_str(text)));

    let mut blob_props = web_sys::BlobPropertyBag::new();
    blob_props.type_("text/cfg");
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_val, &blob_props).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let element = document
        .create_element("a")
        .unwrap()
        .unchecked_into::<HtmlElement>();
    element.set_attribute("href", &url).unwrap();
    element.set_attribute("download", filename).unwrap();

    element.click();
}

#[function_component]
fn App() -> Html {
    let (state, dispatch) = use_store::<State>();

    let clear_files = dispatch.reduce_mut_callback(|state| {
        state.machine_convars = None;
        state.user_convars = None;
        state.binds = None;
        for idx in 0..=2 {
            state.files_valid[idx] = false;
        }
        state.autoexec = None;
    });

    let autoexec_gen: Callback<MouseEvent> =
        dispatch.reduce_mut_future_callback(|state| Box::pin(load_all_files(state)));

    let textbox_update = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("E should have a target");
        let text_val = target.unchecked_into::<HtmlInputElement>().value();
        let value_opt = if text_val.is_empty() {
            None
        } else {
            Some(text_val)
        };
        dispatch.reduce_mut(|state| state.autoexec = value_opt);
    });

    let textbox_value = match &state.autoexec {
        Some(txt) => txt.clone(),
        None => "".to_string(),
    };

    let download_callback = Callback::from(move |_e| {
        if let Some(text) = &state.autoexec {
            download_file(text);
        }
    });

    html! {
        <>
        <div class="title">
            <h1>{ "Counter-Strike 2 Autoexec Builder" }</h1>
        </div>
        <div class="container">
            <div class="panel">
                <h2> {"Initial Setup"} </h2>
                <p>
                    { "Upload the following 3 files from " }
                    <br /><samp> {"[STEAM INSTALL PATH]/userdata/[YOUR STEAM ID]/730/local/cfg"}</samp>
                </p>
                <div class="file-upload-flex"> {
                        FILENAMES.iter().enumerate().map(|(idx, name)| {
                            html! {
                                <FileInput name={*name} idx={idx} />
                            }
                        }).collect::<Html>()

                }</div>
                <div class="buttons-container">
                    <button class="file-manage-button" onclick={clear_files}>{ "Clear Files" }</button>
                    <button class="file-manage-button" onclick={autoexec_gen}>{ "Generate Autoexec" }</button>
                </div>
                <p>
                    { "Custom aliases (i.e. +jumpthrow) are not saved in your config and will not be placed in the autoexec" }
                </p>
                <button id="download-button" onclick={download_callback} class="file-manage-button">{ "Download autoexec.cfg" }</button>
            </div>
            <div class="autoexec-box">
                <textarea
                    autocomplete="off" autocorrect="off" autocapitalize="off" spellcheck="false"
                    rows={23} value={textbox_value}
                    onchange={textbox_update}
                />
            </div>
            <special_binds::SpecialBindsContainer />
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
