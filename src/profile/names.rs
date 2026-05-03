//! Reddit-style username suggestions: `<adjective>-<noun>-<00..99>`.
//! No randomness crate; we seed a tiny xorshift PRNG from system time so
//! suggestions vary across launches but stay deterministic within a run.

use std::time::{SystemTime, UNIX_EPOCH};

const ADJECTIVES: &[&str] = &[
    "absurd", "ancient", "autumn", "bashful", "bitter", "breezy", "bumbling",
    "chunky", "clever", "cosmic", "cozy", "crispy", "dapper", "dizzy", "dotty",
    "dramatic", "drowsy", "dusty", "eager", "epic", "fancy", "feisty", "fierce",
    "fluffy", "foamy", "frantic", "frosty", "fuzzy", "gentle", "glossy",
    "gloomy", "goofy", "grumpy", "hidden", "humble", "ivory", "jagged",
    "jolly", "jumpy", "kindly", "lonely", "lucky", "mellow", "merry", "misty",
    "moody", "muddy", "nimble", "noble", "novel", "plucky", "polite",
    "prickly", "proud", "quirky", "rapid", "rare", "rascal", "rosy", "royal",
    "rustic", "salty", "sassy", "scrappy", "shabby", "shaggy", "sharp",
    "shiny", "silent", "silly", "sleepy", "smart", "snappy", "snug", "solemn",
    "sparkly", "spicy", "squishy", "stoic", "sturdy", "sunny", "swift",
    "tangled", "tender", "tidy", "tipsy", "tricky", "tropical", "vivid",
    "weary", "whimsical", "wiggly", "witty", "wobbly", "wooly", "zealous",
    "zesty",
];

const NOUNS: &[&str] = &[
    "abacus", "acorn", "anchor", "axolotl", "badger", "bagel", "banjo",
    "beacon", "beaver", "beetle", "biscuit", "bison", "blender", "bobcat",
    "bonbon", "bumblebee", "capybara", "cardamom", "caribou", "chinchilla",
    "cobbler", "comet", "compass", "dumpling", "eclipse", "ferret", "finch",
    "fjord", "flamingo", "fox", "gecko", "glacier", "goblin", "gondola",
    "gopher", "gremlin", "hedgehog", "hexagon", "hippo", "jackal",
    "jellyfish", "kazoo", "kettle", "koala", "lantern", "lemming", "lemur",
    "llama", "lobster", "lynx", "magnet", "manatee", "marmot", "meerkat",
    "mongoose", "narwhal", "ocelot", "octopus", "opossum", "otter", "owl",
    "panda", "parsnip", "pelican", "penguin", "pickle", "platypus",
    "polecat", "possum", "puffin", "quokka", "raccoon", "raven", "robin",
    "salmon", "sloth", "snail", "sparrow", "squid", "stoat", "tapir",
    "teapot", "tortoise", "tundra", "vulture", "walrus", "warbler", "weasel",
    "whale", "wombat", "yeti", "zebra",
];

pub struct Rng(u64);

impl Rng {
    pub fn from_clock() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0xDEADBEEF);
        // mix in process id for variance when nanos resolution is coarse
        let pid = std::process::id() as u64;
        Self(now ^ (pid.wrapping_mul(0x9E37_79B9_7F4A_7C15)) | 1)
    }
    pub fn next_u64(&mut self) -> u64 {
        // xorshift64
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    pub fn pick<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        &slice[(self.next_u64() as usize) % slice.len()]
    }
    pub fn range(&mut self, lo: u32, hi_exclusive: u32) -> u32 {
        debug_assert!(hi_exclusive > lo);
        lo + (self.next_u64() as u32) % (hi_exclusive - lo)
    }
}

/// One suggestion. ~50% include a numeric suffix for uniqueness.
pub fn suggest(rng: &mut Rng) -> String {
    let adj = rng.pick(ADJECTIVES);
    let noun = rng.pick(NOUNS);
    if rng.next_u64() & 1 == 0 {
        format!("{adj}-{noun}-{:02}", rng.range(0, 100))
    } else {
        format!("{adj}-{noun}")
    }
}

/// Generate `n` distinct suggestions. The pool is large enough that collisions
/// in small n are vanishingly rare, but we still de-dup defensively.
pub fn suggestions(n: usize, rng: &mut Rng) -> Vec<String> {
    let mut out = Vec::with_capacity(n);
    let mut seen = std::collections::HashSet::with_capacity(n);
    let mut attempts = 0;
    while out.len() < n && attempts < n * 8 {
        let s = suggest(rng);
        if seen.insert(s.clone()) {
            out.push(s);
        }
        attempts += 1;
    }
    out
}

pub fn pool_size() -> usize {
    ADJECTIVES.len() * NOUNS.len() * 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestion_is_well_formed() {
        let mut rng = Rng::from_clock();
        for _ in 0..50 {
            let s = suggest(&mut rng);
            // must be lowercase, alnum + hyphens, two or three segments
            assert!(s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'));
            let parts: Vec<&str> = s.split('-').collect();
            assert!(parts.len() == 2 || parts.len() == 3, "{s}");
            assert!(ADJECTIVES.contains(&parts[0]));
            assert!(NOUNS.contains(&parts[1]));
        }
    }

    #[test]
    fn batch_is_distinct() {
        let mut rng = Rng::from_clock();
        let v = suggestions(8, &mut rng);
        assert_eq!(v.len(), 8);
        let unique: std::collections::HashSet<_> = v.iter().collect();
        assert_eq!(unique.len(), 8);
    }

    #[test]
    fn pool_is_huge() {
        // sanity: multi-million options means low birthday-collision risk
        assert!(pool_size() > 500_000);
    }
}
