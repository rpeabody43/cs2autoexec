mod config;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, File, HtmlInputElement};
use yew::prelude::*;
use yewdux::prelude::*;

const FILENAMES: &[&str] = &[
    "cs2_user_convars_0_slot0.vcfg",
    "cs2_machine_convars.vcfg",
    "cs2_user_keys_0_slot0.vcfg",
];

#[derive(Default, Clone, PartialEq, Eq, Store)]
struct State {
    user_convars: Option<File>,
    machine_convars: Option<File>,
    binds: Option<File>,
}

#[derive(Clone, PartialEq, Properties)]
struct FileInputProps {
    name: String,
    id: String,
}

#[function_component]
fn FileInput(FileInputProps { name, id }: &FileInputProps) -> Html {
    let (_, dispatch) = use_store::<State>();

    // Loads file into appropriate place depending on element clicked
    let callback = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Event should have a target");
        let elm: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();
        let file = elm.files().unwrap().get(0).unwrap();
        match elm.id().as_str() {
            "file-input-0" => {
                dispatch.reduce_mut(|state| state.user_convars = Some(file));
            }
            "file-input-1" => {
                dispatch.reduce_mut(|state| state.machine_convars = Some(file));
            }
            "file-input-2" => {
                dispatch.reduce_mut(|state| state.binds = Some(file));
            }
            _ => {}
        }
    });

    html! {
        <label for={id.clone()} class="file-upload">
            { name }
            <input onchange={callback}
                id={id.clone()}
                type="file"
                accept=".vcfg"
            />
        </label>
    }
}

#[function_component]
fn App() -> Html {
    let (state, _) = use_store::<State>();

    let names = vec![
        match &state.user_convars {
            Some(file) => file.name(),
            None => String::from("None"),
        },
        match &state.machine_convars {
            Some(file) => file.name(),
            None => String::from("None"),
        },
        match &state.binds {
            Some(file) => file.name(),
            None => String::from("None"),
        },
    ];
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
                                let id = format!("file-input-{}", idx);
                                html! {
                                    <FileInput name={*name} id={id} />
                                }
                            }).collect::<Html>()

                    }</div>
                    { "Loaded: " }
                    <div> {
                        names.iter().enumerate().map(|(idx, name)| {
                            html! {
                                <div> {
                                    format!("{}: {}", idx, name)
                                }</div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                </div>
                <div class="autoexec-box"></div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
