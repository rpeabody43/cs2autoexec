use yew::prelude::*;

fn from_cfg_string(cfg_str: &str) -> Html {
    let lines = cfg_str.split('\n');
    html! {
        <p class="cfg">
            {
                lines
                .map(|line| {
                    html! {
                        <>
                            { line }
                            <br />
                        </>
                    }
                })
                .collect::<Html>()
            }
        </p>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct SimpleBindProps {
    title: &'static str,
    description: &'static str,
    pub children: Html,
}

#[function_component]
fn SimpleBind(
    SimpleBindProps {
        title,
        description,
        children,
    }: &SimpleBindProps,
) -> Html {
    html! {
        <div class="special-bind">
            <h3> { title } </h3>
            <div class="bind-content">
                <p>
                    { description }
                </p>
                { children.clone() }
            </div>
        </div>
    }
}

#[function_component]
pub fn SpecialBindsContainer() -> Html {
    html! {
        <div class="panel">
            <h2> { "Useful Binds" } </h2>
            <div id="binds">
                <SimpleBind
                    title="Radar Toggle"
                    description="Zoom the radar in and out with a keybind"
                >
                    { from_cfg_string("bind \"[KEY]\" \"toggle cl_radar_scale 1.0 0.4\"") }
                </SimpleBind>
                <SimpleBind
                    title="Clear Nades (requires sv_cheats 1)"
                    description="For practice servers: clear smokes, mollys, etc. with a single keybind"
                >
                    {from_cfg_string(
                        r#"
                        alias "killsmoke" "ent_fire smokegrenade_projectile kill"
                        alias "killmolly" "ent_fire molotov_projectile kill"
                        alias "killflash" "ent_fire flashbang_projectile kill"
                        alias "killhe" "ent_fire hegrenade_projectile kill"
                        alias "killdecoy" "ent_fire decoy_projectile kill"
                        alias "mute" "stopsound"
                        bind "[KEY]" "killsmoke; killmolly; killflash; killhe; killdecoy; mute"
                        "#
                    )}
                </SimpleBind>
                <SimpleBind
                    title="Buy Binds"
                    description="Instantly buy a weapon or utility with a keybind."
                >
                    <p>{"Run `buy` in console without arguments for a list of weapon/utility names."}</p>
                    { from_cfg_string("bind \"[KEY]\" \"buy [WEAPON]\"") }
                </SimpleBind>
            </div>
            <p id="special-keys-ref"><a target="_blank" rel="noopener noreferrer" href="https://developer.valvesoftware.com/wiki/Bind#Special_Keys">
                { "List of Key Names" }
            </a></p>
        </div>
    }
}
