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
        push("dankmeme.globed2/test");
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
        push("dankmeme.globed2/counter-change");
        push("dankmeme.globed2/player-join");
        push("dankmeme.globed2/player-leave");
        push("dankmeme.globed2/display-data-refreshed");

        push("dankmeme.globed2/scripting.spawn-group");
        push("dankmeme.globed2/scripting.set-item");
        push("dankmeme.globed2/scripting.request-script-logs");
        push("dankmeme.globed2/scripting.move-group");
        push("dankmeme.globed2/scripting.move-group-absolute");
        push("dankmeme.globed2/scripting.follow-player");
        push("dankmeme.globed2/scripting.follow-rotation");

        push("dankmeme.globed2/2p.link-request");
        push("dankmeme.globed2/2p.unlink");

        push("dankmeme.globed2/switcheroo.full-state");
        push("dankmeme.globed2/switcheroo.switch");
    }

    Ok(())
}
