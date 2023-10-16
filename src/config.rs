use std::collections::HashMap;

fn add_to_config(map: &mut HashMap<String, String>, binds: bool, config_file1: &str, config_file2: &str) {
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
            let key: &str = &line[(quote_idxs[0] + 1)..quote_idxs[1]];
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
        "con_enable",
        "sound_device_override",
        "ui_mainmenu_bkgnd_movie_1016BB11$9",
        "snd_menumusic_volume$4",
        "cachedvalue_count_partybrowser",
        "cl_promoted_settings_acknowledged",
        "csgo_map_preview_scale",
    ];
    for command in ignored_commands {
        config.remove(command);
    }
}

fn config_section(config: &mut HashMap<String, String>, name: &str, commands: Vec<&str>) -> String {
    let mut ret = format!("// {}\n", name);
    for command in commands {
        let val = config.get(command).unwrap();
        let new_line = format!("{} \"{}\"\n", command, val);
        ret.push_str(&new_line);
        config.remove(command);
    }
    ret.push('\n');
    ret
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
                "{}{}\n",
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
            "cl_crosshair_friendly_warning",
            "cl_crosshair_outlinethickness",
            "cl_crosshair_recoil",
            "cl_crosshair_sniper_show_normal_inaccuracy",
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

pub fn config_str(custom_machine_convars: String, custom_user_convars: String, custom_binds: String) -> String {
    let mut default_config = HashMap::new();
    let mut custom_config = HashMap::new();
    let mut binds = HashMap::new();

    add_to_config(
        &mut default_config,
        false,
        include_str!("../defaults/cs2_machine_convars.vcfg"),
        include_str!("../defaults/cs2_user_convars_0_slot0.vcfg")
    );

    add_to_config(
        &mut custom_config,
        false,
        &custom_machine_convars,
        &custom_user_convars
    );
    filter_config(&mut custom_config);

    add_to_config(
        &mut binds,
        true,
        include_str!("../defaults/user_keys_default.vcfg"),
        &custom_binds
    );

    let mut out = String::from("con_enable \"1\"\n\n");
    out.push_str(&write_sections(&mut custom_config));

    let mut binds_vec = Vec::new();
    for (key, value) in binds {
        if value == "<unbound>" { continue; }
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
        }
        settings_vec.push(format!("{} \"{}\"\n", key, value));
    }
    settings_vec.sort();
    out.push_str("// SETTINGS\n");
    for setting in settings_vec {
        out.push_str(&setting);
    }
    out.push_str("\n\necho \"--- autoexec loaded ---\"");
    out
    // TODO Special Keybinds
}
