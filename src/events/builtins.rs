use super::*;

pub(super) const CENTRAL_BUILTINS_MAX: u32 = 1;
pub(super) const GAME_BUILTINS_MAX: u32 = 1;

pub(super) fn build_central_builtins(
    version: u32,
    out: &mut Vec<Arc<str>>,
    cache: &EventStringCache,
) -> Result<(), EventDictionaryBuildError> {
    if version > CENTRAL_BUILTINS_MAX {
        return Err(EventDictionaryBuildError::UnsupportedBuiltinsVersion(
            version,
        ));
    }

    let mut push = |s: &str| out.push(cache.get(s));

    if version >= 1 {
        push("globed/test");
    }

    Ok(())
}

pub(super) fn build_game_builtins(
    version: u32,
    out: &mut Vec<Arc<str>>,
    cache: &EventStringCache,
) -> Result<(), EventDictionaryBuildError> {
    if version > GAME_BUILTINS_MAX {
        return Err(EventDictionaryBuildError::UnsupportedBuiltinsVersion(
            version,
        ));
    }

    let mut push = |s: &str| out.push(cache.get(s));

    if version >= 1 {
        push("globed/counter-change");
        push("globed/display-data-refreshed");

        push("globed/scripting.custom");
        push("globed/scripting.spawn-group");
        push("globed/scripting.set-item");
        push("globed/scripting.request-script-logs");
        push("globed/scripting.move-group");
        push("globed/scripting.follow-player");
        push("globed/scripting.follow-rotation");
        push("globed/scripting.follow-absolute");

        push("globed/2p.link");
        push("globed/2p.unlink");

        push("globed/switcheroo.full-state");
        push("globed/switcheroo.switch");
    }

    Ok(())
}
