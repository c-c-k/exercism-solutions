const SOUND_MAP: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let sounds = SOUND_MAP
        .iter()
        .filter(|(divisor, _)| n.is_multiple_of(*divisor))
        .map(|(_, sound)| *sound)
        .collect::<String>();

    if sounds.is_empty() {
        n.to_string()
    } else {
        sounds
    }
}
