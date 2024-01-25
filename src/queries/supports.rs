use super::{Distrib, QueryResult};
use crate::{
    data::{
        browser_name::BrowserNameAtom,
        caniuse::{features::get_feature_stat, get_browser_stat, to_desktop_name, VersionDetail},
    },
    error::Error,
    Opts,
};

const Y: u8 = 1;
const A: u8 = 2;

pub(super) fn supports(name: &str, opts: &Opts) -> QueryResult {
    if let Some(feature) = get_feature_stat(name) {
        let distribs = feature
            .iter()
            .filter_map(|(name, versions)| {
                get_browser_stat(name, opts.mobile_to_desktop)
                    .map(|(name, stat)| (name, stat, versions))
            })
            .flat_map(|(name, browser_stat, versions)| {
                let desktop_name = opts
                    .mobile_to_desktop
                    .then_some(to_desktop_name(name))
                    .flatten();
                let check_desktop = desktop_name.is_some()
                    && browser_stat
                        .version_list
                        .iter()
                        .filter(|version| version.release_date.is_some())
                        .last()
                        .and_then(|latest_version| versions.get(&*latest_version.version))
                        .map(|flags| is_supported(*flags))
                        .unwrap_or_default();
                browser_stat
                    .version_list
                    .iter()
                    .filter_map(move |VersionDetail { version, .. }| {
                        versions
                            .get(&**version)
                            .or_else(|| match desktop_name {
                                Some(desktop_name) if check_desktop => feature
                                    .get(&BrowserNameAtom::from(desktop_name))
                                    .and_then(|versions| versions.get(&**version)),
                                _ => None,
                            })
                            .and_then(|flags| is_supported(*flags).then_some(version))
                    })
                    .map(move |version| Distrib::new(name, version))
            })
            .collect();
        Ok(distribs)
    } else {
        Err(Error::UnknownBrowserFeature(name.to_string()))
    }
}

fn is_supported(flags: u8) -> bool {
    flags & Y != 0 || flags & A != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        opts::Opts,
        test::{run_compare, should_failed},
    };
    use test_case::test_case;

    #[test_case("supports objectrtc"; "case 1")]
    #[test_case("supports    rtcpeerconnection"; "case 2")]
    #[test_case("supports        arrow-functions"; "case 3")]
    fn default_options(query: &str) {
        run_compare(query, &Opts::new());
    }

    #[test_case("supports filesystem"; "case 1")]
    #[test_case("supports  font-smooth"; "case 2")]
    fn mobile_to_desktop(query: &str) {
        run_compare(query, &Opts::new().mobile_to_desktop(true));
    }

    #[test]
    fn invalid() {
        assert_eq!(
            should_failed("supports xxxyyyzzz", &Opts::new()),
            Error::UnknownBrowserFeature(String::from("xxxyyyzzz"))
        );
    }
}
