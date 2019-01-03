use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hubcaps::{Credentials, Github, Result};
use hubcaps::rate_limit::*;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        std::env::var("GITHUB_TOKEN").ok().map(Credentials::Token),
    );
    let status: RateLimitStatus = tokio::runtime::Runtime::new()?.block_on(github.rate_limit().get())?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let print_resource = |name: &str, res: RateLimitResourceStatus| {
        let reset = Duration::from_secs(u64::from(res.reset) - now);
        println!("{}: {}/{} reset in {}s", name, res.remaining, res.limit, reset.as_secs());
    };
    print_resource("core", status.resources.core);
    print_resource("search", status.resources.search);
    print_resource("graphql", status.resources.graphql);
    Ok(())
}
