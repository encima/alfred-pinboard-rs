use super::*;

pub fn run(x: SubCommand) {
    info!("Starting in run");
    let mut print_config = false;
    let mut config: Config = Config::setup().unwrap_or_else(|err| {
        if !err.contains("authorization token") {
            ::show_error_alfred(&err);
            process::exit(1);
        }
        match &x {
            &SubCommand::Config {
                auth_token: Some(_),
                ..
            } => {
                let mut config = Config::new();
                match &x {
                    &SubCommand::Config {
                        ref display,
                        ref auth_token,
                        ref number_pins,
                        ref number_tags,
                        ref shared,
                        ref toread,
                        ref fuzzy,
                        ref tags_only,
                        ref auto_update,
                        ref suggest_tags,
                    } => {
                        print_config = *display;
                        config.auth_token = auth_token.as_ref().unwrap().clone();
                        number_pins.map(|val| config.pins_to_show = val);
                        number_tags.map(|val| config.tags_to_show = val);
                        shared.map(|val| config.private_new_pin = !val);
                        toread.map(|val| config.toread_new_pin = val);
                        fuzzy.map(|val| config.fuzzy_search = val);
                        tags_only.map(|val| config.tag_only_search = val);
                        auto_update.map(|val| config.auto_update_cache = val);
                        suggest_tags.map(|val| config.suggest_tags = val);
                        config.discover_dirs();
                    }
                    _ => (),
                }
                config
            }
            _ => {
                ::show_error_alfred(
                    "First-time config command should provide authorization token!",
                );
                process::exit(1);
            }
        }
    });

    match x {
        SubCommand::Config {
            display,
            auth_token,
            number_pins,
            number_tags,
            shared,
            toread,
            fuzzy,
            tags_only,
            auto_update,
            suggest_tags,
        } => {
            print_config = display;
            auth_token.map(|val| config.auth_token = val);
            number_pins.map(|val| config.pins_to_show = val);
            number_tags.map(|val| config.tags_to_show = val);
            shared.map(|val| config.private_new_pin = !val);
            toread.map(|val| config.toread_new_pin = val);
            fuzzy.map(|val| config.fuzzy_search = val);
            tags_only.map(|val| config.tag_only_search = val);
            auto_update.map(|val| config.auto_update_cache = val);
            suggest_tags.map(|val| config.suggest_tags = val);
        }
        _ => unreachable!(),
    }

    config.save().unwrap();

    if print_config {
        show_config(&config);
    }
}

fn show_config(config: &Config) {
    info!("Starting in show_config");
    // TODO: Add support for Alfred 2 by returning XML <09-02-18, Hamid> //
    // If Using Alfred Version >=3
    if config.is_alfred_v3() {
        use alfred::ItemBuilder;
        alfred::json::Builder::with_items(&[
            ItemBuilder::new("Only search tags")
                .subtitle(format!("{:?}", config.tag_only_search))
                .arg("pset tagonly")
                .icon_path("tagonly.png")
                .into_item(),
            ItemBuilder::new("Use fuzzy search")
                .subtitle(format!("{:?}", config.fuzzy_search))
                .arg("pset fuzzy")
                .icon_path("fuzzy.png")
                .into_item(),
            ItemBuilder::new("Automatically update cache")
                .subtitle(format!("{:?}", config.auto_update_cache))
                .arg("pset auto")
                .icon_path("auto_update_cache.png")
                .into_item(),
            ItemBuilder::new("Suggest popular tags for open browser tab")
                .subtitle(format!("{:?}", config.suggest_tags))
                .arg("pset suggest_tags")
                .icon_path("suggest.png")
                .into_item(),
            ItemBuilder::new("Mark new bookmarks as toread")
                .subtitle(format!("{:?}", config.toread_new_pin))
                .arg("pset toread")
                .icon_path("toread.png")
                .into_item(),
            ItemBuilder::new("Mark new bookmarks as private")
                .subtitle(format!("{:?}", config.private_new_pin))
                .arg("pset shared")
                .icon_path("private.png")
                .into_item(),
            ItemBuilder::new("Number of tags to show")
                .subtitle(format!("{:?}", config.tags_to_show))
                .arg("pset tags")
                .icon_path("no_of_tags.png")
                .into_item(),
            ItemBuilder::new("Number of bookmarks to show")
                .subtitle(format!("{:?}", config.pins_to_show))
                .arg("pset bookmarks")
                .icon_path("no_of_pins.png")
                .into_item(),
            ItemBuilder::new(format!("{:?}", config.update_time))
                .subtitle("Latest cache update")
                .icon_path("auto_update.png")
                .into_item(),
        ]).write(io::stdout())
            .unwrap();
    }
}
