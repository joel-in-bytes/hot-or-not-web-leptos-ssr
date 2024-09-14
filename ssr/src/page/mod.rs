pub mod about_us;
pub mod account_transfer;
pub mod airdrop;
pub mod err;
pub mod faq;
#[cfg(any(feature = "oauth-ssr", feature = "oauth-hydrate"))]
pub mod google_redirect;
pub mod leaderboard;
pub mod logout;
pub mod menu;
pub mod notifs;
pub mod post_view;
pub mod privacy;
pub mod profile;
pub mod refer_earn;
pub mod root;
pub mod settings;
pub mod terms;
pub mod token;
pub mod upload;
pub mod view_profile_redirect;
pub mod wallet;
