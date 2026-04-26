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
        push("dankmeme.globed2/test");
    }

    Ok(())
}
