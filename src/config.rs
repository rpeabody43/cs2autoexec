use std::collections::HashMap;

macro_rules! strip_key {
    ($key: expr) => {
        match $key.find('$') {
            Some (idx) => &$key[..idx],
            None => $key
        }
    }
}

fn add_to_config(
    map: &mut HashMap<String, String>,
    binds: bool,
    config_file1: &str,
    config_file2: &str,
) {
    for config_file in [config_file1, config_file2] {
        let lines = config_file.split('\n');
        for line in lines {
            let quote_idxs = line
                .bytes()
                .enumerate()
                .filter(|(_, c)| *c == b'"')
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            if quote_idxs.len() < 4 {
                continue;
            }
            let key: &str = strip_key!(&line[(quote_idxs[0] + 1)..quote_idxs[1]]);
            let val: &str = &line[(quote_idxs[2] + 1)..quote_idxs[3]];
            match binds {
                true => map.insert(format!("bind \"{}\"", key), val.to_string()),
                false => map.insert(key.to_string(), val.to_string()),
            };
        }
    }
}

fn filter_config(config: &mut HashMap<String, String>) {
    let ignored_commands = vec![
        "cachedvalue_count_partybrowser",
        "cachedvalue_count_teammates",
        "cl_embedded_stream_audio_volume",
        "cl_promoted_settings_acknowledged",
        "cl_redemption_reset_timestamp",
        "con_enable",
        "csgo_map_preview_scale",
        "lobby_default_privacy_bits2",
        "panorama_console_position_and_size",
        "player_nevershow_communityservermessage",
        "player_teamplayedlast",
        "r_show_build_info",
        "sv_specspeed",
        "ui_news_last_read_link",
        "ui_playsettings_maps_listen_competitive",
        "ui_playsettings_maps_listen_deathmatch",
        "ui_playsettings_maps_official_gungameprogressive",
        "ui_playsettings_mode_official_v20",
        "voice_threshold",
    ];
    for command in ignored_commands {
        config.remove(command);
    }
}

macro_rules! section_body {
    ($config: expr, [$command: literal]) => {
        {
            let val = $config.get($command).unwrap();
            let line = format!("{} \"{}\"\n", $command, val);
            $config.remove($command);
            line
        }
    };
    ($config: expr, [$command: literal, $($x: literal),+]) => {
        {
            format!(
                "{}{}",
                section_body!($config, [$command]),
                section_body!($config, [$($x),+])
            )
        }
    }
}

macro_rules! write_section {
    ($config: expr, $name: literal, [$($command: literal),+]) => {
        {
            format!(
                "// {}\n{}\n",
                $name,
                section_body!($config, [$($command),+])
            )
        }
    }
}

fn write_sections(config: &mut HashMap<String, String>) -> String {
    let viewmodel = write_section!(
        config,
        "VIEWMODEL",
        [
            "viewmodel_presetpos",
            "viewmodel_fov",
            "viewmodel_offset_x",
            "viewmodel_offset_y",
            "viewmodel_offset_z"
        ]
    );
    let crosshair = write_section!(
        config,
        "CROSSHAIR",
        [
            "cl_crosshair_drawoutline",
            "cl_crosshair_dynamic_maxdist_splitratio",
            "cl_crosshair_dynamic_splitalpha_innermod",
            "cl_crosshair_dynamic_splitalpha_outermod",
            "cl_crosshair_dynamic_splitdist",
            "cl_crosshair_outlinethickness",
            "cl_crosshair_recoil",
            "cl_crosshair_sniper_width",
            "cl_crosshair_t",
            "cl_crosshairalpha",
            "cl_crosshaircolor",
            "cl_crosshaircolor_b",
            "cl_crosshaircolor_g",
            "cl_crosshaircolor_r",
            "cl_crosshairdot",
            "cl_crosshairgap",
            "cl_crosshairgap_useweaponvalue",
            "cl_crosshairsize",
            "cl_crosshairstyle",
            "cl_crosshairthickness",
            "cl_crosshairusealpha",
            "cl_fixedcrosshairgap"
        ]
    );
    format!("{}{}", viewmodel, crosshair)
}

pub fn config_str(
    custom_machine_convars: String,
    custom_user_convars: String,
    custom_binds: String,
) -> String {
    let mut default_config = HashMap::new();
    let mut custom_config = HashMap::new();
    let mut binds = HashMap::new();

    add_to_config(
        &mut default_config,
        false,
        include_str!("../defaults/cs2_machine_convars.vcfg"),
        include_str!("../defaults/cs2_user_convars_0_slot0.vcfg"),
    );

    add_to_config(
        &mut custom_config,
        false,
        &custom_machine_convars,
        &custom_user_convars,
    );
    filter_config(&mut custom_config);

    add_to_config(
        &mut binds,
        true,
        include_str!("../defaults/user_keys_default.vcfg"),
        &custom_binds,
    );

    let mut out = String::from("con_enable \"1\"\n\n");
    out.push_str(&write_sections(&mut custom_config));

    let mut binds_vec = Vec::new();
    for (key, value) in binds {
        if value == "<unbound>" {
            continue;
        }
        binds_vec.push(format!("{} \"{}\"\n", key, value));
    }
    binds_vec.sort();
    out.push_str("// BINDS\nunbindall\n");
    for bind in binds_vec {
        out.push_str(&bind);
    }
    out.push('\n');

    let mut settings_vec = Vec::new();
    for (key, value) in custom_config {
        if let Some(default_val) = default_config.get(&key) {
            if &value == default_val {
                continue;
            }
            // The devs add new config options all the time, I'm choosing
            // not to include the ones newer than the default config I have
            settings_vec.push(format!("{} \"{}\"\n", key, value));
        }
    }
    settings_vec.sort();
    out.push_str("// SETTINGS\n");
    for setting in settings_vec {
        out.push_str(&setting);
    }
    out.push_str("\necho \"--- autoexec loaded ---\"\n// Generated at https://rpeabs.net/cs2autoexec/");
    out
}
