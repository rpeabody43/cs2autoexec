mod config;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{EventTarget, File, HtmlInputElement};
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

    let test_str = match &state.autoexec {
        Some(txt) => txt.clone(),
        None => "".to_string(),
    };

    html! {
        <>
        <div class="title">
            <h1>{ "Counter-Strike 2 Autoexec Builder" }</h1>
        </div>
        <div class="container">
            <div class="panel">
                <p>
                    { "Navigate to " }
                    <samp> {"[STEAM INSTALL PATH]/userdata/[YOUR STEAM ID]/730/local/cfg"}</samp>
                    {" (or wherever your steam install is) and upload the following 3 files:"}
                </p>
                <div class="file-upload-container"> {
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
            </div>
            <div class="autoexec-box">
                <textarea
                    autocomplete="off" autocorrect="off" autocapitalize="off" spellcheck="false"
                    rows={23} value={test_str}
                />
            </div>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
