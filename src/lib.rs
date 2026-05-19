////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod cli;
pub mod custom;
pub mod util;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub const IDENTITY: &str = r#"
Mbombo, also called Bumba, is the creator god in the religion and mythology of the Kuba people of Central Africa in the area that is now known as Democratic Republic of the Congo

In the Mbombo creation myth, Mbombo was a giant in form and white in color. The myth describes the creation of the universe from nothing

Role: Mbombo is considered a creator god in Bushongo mythology
Creation story: According to legend, in the beginning, there was only darkness and water, and Mbombo was the only being. He was a giant, pale god who eventually felt pain in his stomach and vomited up the sun, moon, stars, and then the Earth itself, including animals and people
Symbolism: His creation of the world through vomiting is unique and symbolic, often interpreted as a metaphor for creative force through suffering or sacrifice
Assistants: After creation, Mbombo delegated tasks to his sons and some of the first humans and animals to help finish shaping the world
"#;

const HELP: &str = r"Command line file forger

Examples:
  mb --files header.html footer.html --out output/page.html
  mb --files config.tmpl --out config.json --replace VERSION=1.0.0 API_URL=https://api.example.com
  mb --in templates/ --files base.tmpl nav.tmpl --out build/index.html --replace {{YEAR}}=2026:token {{AUTHOR}}=Daniel:token
  mb --files script.js --out dist/script.min.js --replace console.log=:line
  mb --files README.md --out README.md --replace v0.0.0=v1.2.3";

////////////////////////////////////////////////////////////////////////////////////////////////////
