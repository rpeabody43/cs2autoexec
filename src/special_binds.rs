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
                    title="Jumpthrow"
                    description="Binds a key to jump and release a grenade at the same time. Unnecessary in CS2 but nice to have."
                >
                    {from_cfg_string(
                        r#"alias "+jumpaction" "+jump;"
                        alias "+throwaction" "-attack; -attack2"
                        alias "-jumpaction" "-jump"
                        bind "[KEY]" "+jumpaction;+throwaction;""#
                    )}
                </SimpleBind>
                <SimpleBind
                    title="Runthrow"
                    description="Binds a key to run, jump and release a grenade at the same time."
                >
                    {from_cfg_string(
                        r#"alias "+throwaction" "-attack; -attack2"
                        alias "+runthrow" "+forward;+jump;"
                        alias "-runthrow" "-jump;-forward"
                        bind "[KEY]" "+runthrow;+throwaction""#
                    )}
                </SimpleBind>
                <SimpleBind
                    title="Crouch Jump"
                    description="Binds a key to crouch and jump at once, letting you easily jump on some boxes you can't otherwise."
                >
                    {from_cfg_string(
                        r#"alias "+hjump" "+jump; +duck"
                        alias "-hjump" "-jump; -duck"
                        bind "[KEY]" "+hjump""#
                    )}
                </SimpleBind>
                <SimpleBind
                    title="Drop Bomb"
                    description="Allows dropping the bomb with a single key."
                >
                    {from_cfg_string(
                        r#"alias "+bomb" "slot3; slot5"
                        alias "-bomb" "drop; lastinv"
                        bind "[KEY]" "+bomb""#
                    )}
                </SimpleBind>
                <SimpleBind
                    title="Buy Binds"
                    description="Instantly buy a weapon or utility with a keybind. See image below for weapon names and their associated slot in CS2."
                >
                    { from_cfg_string("bind \"[KEY]\" \"buy [WEAPON]\"") }
                    <img src="img/buybindcodes.jpg" />
                </SimpleBind>
            </div>
            <p id="special-keys-ref"><a target="_blank" rel="noopener noreferrer" href="https://developer.valvesoftware.com/wiki/Bind#Special_Keys">
                { "List of Key Names" }
            </a></p>
        </div>
    }
}
