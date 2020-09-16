//! Convert kana to romaji
//!
//! # Examples
//! ```
//! use wana_kana::to_romaji::*;
//! use wana_kana::Options;
//! assert_eq!(to_romaji("ひらがな　カタカナ"), "hiragana katakana");
//! assert_eq!(to_romaji_with_opt("ひらがな　カタカナ", Options {upcase_katakana: true, ..Default::default() } ), "hiragana KATAKANA");
//! ```

use crate::options::Options;
use crate::utils::is_char_katakana::is_char_katakana;
use crate::utils::katakana_to_hiragana::*;

pub fn to_romaji(input: &str) -> String {
    to_romaji_with_opt(input, Options::default())
}

pub fn to_romaji_with_opt(orig: &str, options: Options) -> String {
    let kana = katakana_to_hiragana_with_opt(orig, true);
    let orig_chars = orig.chars().collect::<Vec<_>>();
    let chars = kana.chars().collect::<Vec<_>>();
    let mut ouput = String::with_capacity(orig.len());
    let len = chars.len();
    // Position in the string that is being evaluated
    let mut curr_pos = 0;

    while curr_pos != len {
        let result = TO_ROMAJI_NODE_TREE.get(&chars[curr_pos..]);
        //nothing found, pass through
        if result.1 == 0 {
            ouput.push(chars[curr_pos]);
            curr_pos += 1;
        } else {
            let convert_romaji_to_uppercase = {
                if orig_chars[curr_pos..curr_pos + result.1].iter().all(|c| is_char_katakana(*c)) {
                    options.upcase_katakana
                } else {
                    false
                }
            };

            if convert_romaji_to_uppercase {
                ouput.push_str(&result.0.to_uppercase());
            } else {
                ouput.push_str(result.0);
            }
            curr_pos += result.1;
        }
    }

    ouput
}

#[derive(Debug, Clone)]
pub struct Node {
    pub transitions: Option<Vec<(char, Node)>>,
    pub output: &'static str,
}

impl Node {
    pub fn get<'a>(&self, chars: &'a [char]) -> (&'static str, usize) {
        let mut i = 0;
        let mut curr_node = self;
        for char in chars.iter() {
            if let Some(trans_node) = curr_node.find_transition_node(*char) {
                curr_node = trans_node;
            } else {
                break;
            }
            i += 1;
        }
        (curr_node.output, i)
    }

    pub fn find_transition_node(&self, char: char) -> Option<&Node> {
        if let Some(t) = &self.transitions {
            t.binary_search_by_key(&char, |t| t.0).ok().map(|index| &t[index].1)
        } else {
            None
        }
    }

    fn sort(&mut self) {
        if let Some(transitions) = &mut self.transitions {
            transitions.sort_by_key(|el| el.0);
            for el in transitions {
                el.1.sort();
            }
        }
    }
}

lazy_static! {
    pub static ref TO_ROMAJI_NODE_TREE: Node = {
        let transitions = Some(vec![
            (
                '\u{3000}',
                Node {
                    transitions: None,
                    output: " ",
                },
            ),
            (
                '、',
                Node {
                    transitions: None,
                    output: ",",
                },
            ),
            (
                '。',
                Node {
                    transitions: None,
                    output: ".",
                },
            ),
            (
                '「',
                Node {
                    transitions: None,
                    output: "‘",
                },
            ),
            (
                '」',
                Node {
                    transitions: None,
                    output: "’",
                },
            ),
            (
                '『',
                Node {
                    transitions: None,
                    output: "“",
                },
            ),
            (
                '』',
                Node {
                    transitions: None,
                    output: "”",
                },
            ),
            (
                '〜',
                Node {
                    transitions: None,
                    output: "~",
                },
            ),
            (
                'ぁ',
                Node {
                    transitions: None,
                    output: "a",
                },
            ),
            (
                'あ',
                Node {
                    transitions: None,
                    output: "a",
                },
            ),
            (
                'ぃ',
                Node {
                    transitions: None,
                    output: "i",
                },
            ),
            (
                'い',
                Node {
                    transitions: None,
                    output: "i",
                },
            ),
            (
                'ぅ',
                Node {
                    transitions: None,
                    output: "u",
                },
            ),
            (
                'う',
                Node {
                    transitions: None,
                    output: "u",
                },
            ),
            (
                'ぇ',
                Node {
                    transitions: None,
                    output: "e",
                },
            ),
            (
                'え',
                Node {
                    transitions: None,
                    output: "e",
                },
            ),
            (
                'ぉ',
                Node {
                    transitions: None,
                    output: "o",
                },
            ),
            (
                'お',
                Node {
                    transitions: None,
                    output: "o",
                },
            ),
            (
                'か',
                Node {
                    transitions: None,
                    output: "ka",
                },
            ),
            (
                'が',
                Node {
                    transitions: None,
                    output: "ga",
                },
            ),
            (
                'き',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "kyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "kye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "kya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "kyu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "kyo",
                            },
                        ),
                    ]),
                    output: "ki",
                },
            ),
            (
                'ぎ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "gyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "gye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "gya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "gyu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "gyo",
                            },
                        ),
                    ]),
                    output: "gi",
                },
            ),
            (
                'く',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "kyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "kye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "kya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "kyu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "kyo",
                            },
                        ),
                    ]),
                    output: "ku",
                },
            ),
            (
                'ぐ',
                Node {
                    transitions: None,
                    output: "gu",
                },
            ),
            (
                'け',
                Node {
                    transitions: None,
                    output: "ke",
                },
            ),
            (
                'げ',
                Node {
                    transitions: None,
                    output: "ge",
                },
            ),
            (
                'こ',
                Node {
                    transitions: None,
                    output: "ko",
                },
            ),
            (
                'ご',
                Node {
                    transitions: None,
                    output: "go",
                },
            ),
            (
                'さ',
                Node {
                    transitions: None,
                    output: "sa",
                },
            ),
            (
                'ざ',
                Node {
                    transitions: None,
                    output: "za",
                },
            ),
            (
                'し',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "shyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "she",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "sha",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "shu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "sho",
                            },
                        ),
                    ]),
                    output: "shi",
                },
            ),
            (
                'じ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "jyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "je",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "ja",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "ju",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "jo",
                            },
                        ),
                    ]),
                    output: "ji",
                },
            ),
            (
                'す',
                Node {
                    transitions: None,
                    output: "su",
                },
            ),
            (
                'ず',
                Node {
                    transitions: None,
                    output: "zu",
                },
            ),
            (
                'せ',
                Node {
                    transitions: None,
                    output: "se",
                },
            ),
            (
                'ぜ',
                Node {
                    transitions: None,
                    output: "ze",
                },
            ),
            (
                'そ',
                Node {
                    transitions: None,
                    output: "so",
                },
            ),
            (
                'ぞ',
                Node {
                    transitions: None,
                    output: "zo",
                },
            ),
            (
                'た',
                Node {
                    transitions: None,
                    output: "ta",
                },
            ),
            (
                'だ',
                Node {
                    transitions: None,
                    output: "da",
                },
            ),
            (
                'ち',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "chyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "che",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "cha",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "chu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "cho",
                            },
                        ),
                    ]),
                    output: "chi",
                },
            ),
            (
                'ぢ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "jyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "je",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "ja",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "ju",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "jo",
                            },
                        ),
                    ]),
                    output: "ji",
                },
            ),
            (
                'っ',
                Node {
                    transitions: Some(vec![
                        (
                            '\u{3000}',
                            Node {
                                transitions: None,
                                output: " ",
                            },
                        ),
                        (
                            '、',
                            Node {
                                transitions: None,
                                output: ",",
                            },
                        ),
                        (
                            '。',
                            Node {
                                transitions: None,
                                output: ".",
                            },
                        ),
                        (
                            '「',
                            Node {
                                transitions: None,
                                output: "‘",
                            },
                        ),
                        (
                            '」',
                            Node {
                                transitions: None,
                                output: "’",
                            },
                        ),
                        (
                            '『',
                            Node {
                                transitions: None,
                                output: "“",
                            },
                        ),
                        (
                            '』',
                            Node {
                                transitions: None,
                                output: "”",
                            },
                        ),
                        (
                            '〜',
                            Node {
                                transitions: None,
                                output: "~",
                            },
                        ),
                        (
                            'ぁ',
                            Node {
                                transitions: None,
                                output: "a",
                            },
                        ),
                        (
                            'あ',
                            Node {
                                transitions: None,
                                output: "a",
                            },
                        ),
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "i",
                            },
                        ),
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "i",
                            },
                        ),
                        (
                            'ぅ',
                            Node {
                                transitions: None,
                                output: "u",
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "u",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "e",
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: "e",
                            },
                        ),
                        (
                            'ぉ',
                            Node {
                                transitions: None,
                                output: "o",
                            },
                        ),
                        (
                            'お',
                            Node {
                                transitions: None,
                                output: "o",
                            },
                        ),
                        (
                            'か',
                            Node {
                                transitions: None,
                                output: "kka",
                            },
                        ),
                        (
                            'が',
                            Node {
                                transitions: None,
                                output: "gga",
                            },
                        ),
                        (
                            'き',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "kkyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "kkye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "kkya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "kkyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "kkyo",
                                        },
                                    ),
                                ]),
                                output: "kki",
                            },
                        ),
                        (
                            'ぎ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "ggyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "ggye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ggya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ggyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "ggyo",
                                        },
                                    ),
                                ]),
                                output: "ggi",
                            },
                        ),
                        (
                            'く',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "kkyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "kkye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "kkya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "kkyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "kkyo",
                                        },
                                    ),
                                ]),
                                output: "kku",
                            },
                        ),
                        (
                            'ぐ',
                            Node {
                                transitions: None,
                                output: "ggu",
                            },
                        ),
                        (
                            'け',
                            Node {
                                transitions: None,
                                output: "kke",
                            },
                        ),
                        (
                            'げ',
                            Node {
                                transitions: None,
                                output: "gge",
                            },
                        ),
                        (
                            'こ',
                            Node {
                                transitions: None,
                                output: "kko",
                            },
                        ),
                        (
                            'ご',
                            Node {
                                transitions: None,
                                output: "ggo",
                            },
                        ),
                        (
                            'さ',
                            Node {
                                transitions: None,
                                output: "ssa",
                            },
                        ),
                        (
                            'ざ',
                            Node {
                                transitions: None,
                                output: "zza",
                            },
                        ),
                        (
                            'し',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "sshyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "sshe",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ssha",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "sshu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "ssho",
                                        },
                                    ),
                                ]),
                                output: "sshi",
                            },
                        ),
                        (
                            'じ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "jjyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "jje",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "jja",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "jju",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "jjo",
                                        },
                                    ),
                                ]),
                                output: "jji",
                            },
                        ),
                        (
                            'す',
                            Node {
                                transitions: None,
                                output: "ssu",
                            },
                        ),
                        (
                            'ず',
                            Node {
                                transitions: None,
                                output: "zzu",
                            },
                        ),
                        (
                            'せ',
                            Node {
                                transitions: None,
                                output: "sse",
                            },
                        ),
                        (
                            'ぜ',
                            Node {
                                transitions: None,
                                output: "zze",
                            },
                        ),
                        (
                            'そ',
                            Node {
                                transitions: None,
                                output: "sso",
                            },
                        ),
                        (
                            'ぞ',
                            Node {
                                transitions: None,
                                output: "zzo",
                            },
                        ),
                        (
                            'た',
                            Node {
                                transitions: None,
                                output: "tta",
                            },
                        ),
                        (
                            'だ',
                            Node {
                                transitions: None,
                                output: "dda",
                            },
                        ),
                        (
                            'ち',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "tchyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "tche",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "tcha",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "tchu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "tcho",
                                        },
                                    ),
                                ]),
                                output: "tchi",
                            },
                        ),
                        (
                            'ぢ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "jjyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "jje",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "jja",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "jju",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "jjo",
                                        },
                                    ),
                                ]),
                                output: "jji",
                            },
                        ),
                        (
                            'つ',
                            Node {
                                transitions: None,
                                output: "ttsu",
                            },
                        ),
                        (
                            'づ',
                            Node {
                                transitions: None,
                                output: "zzu",
                            },
                        ),
                        (
                            'て',
                            Node {
                                transitions: None,
                                output: "tte",
                            },
                        ),
                        (
                            'で',
                            Node {
                                transitions: None,
                                output: "dde",
                            },
                        ),
                        (
                            'と',
                            Node {
                                transitions: None,
                                output: "tto",
                            },
                        ),
                        (
                            'ど',
                            Node {
                                transitions: None,
                                output: "ddo",
                            },
                        ),
                        (
                            'な',
                            Node {
                                transitions: None,
                                output: "na",
                            },
                        ),
                        (
                            'に',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "nyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "nye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "nya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "nyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "nyo",
                                        },
                                    ),
                                ]),
                                output: "ni",
                            },
                        ),
                        (
                            'ぬ',
                            Node {
                                transitions: None,
                                output: "nu",
                            },
                        ),
                        (
                            'ね',
                            Node {
                                transitions: None,
                                output: "ne",
                            },
                        ),
                        (
                            'の',
                            Node {
                                transitions: None,
                                output: "no",
                            },
                        ),
                        (
                            'は',
                            Node {
                                transitions: None,
                                output: "hha",
                            },
                        ),
                        (
                            'ば',
                            Node {
                                transitions: None,
                                output: "bba",
                            },
                        ),
                        (
                            'ぱ',
                            Node {
                                transitions: None,
                                output: "ppa",
                            },
                        ),
                        (
                            'ひ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "hhyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "hhye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "hhya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "hhyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "hhyo",
                                        },
                                    ),
                                ]),
                                output: "hhi",
                            },
                        ),
                        (
                            'び',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "bbyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "bbye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "bbya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "bbyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "bbyo",
                                        },
                                    ),
                                ]),
                                output: "bbi",
                            },
                        ),
                        (
                            'ぴ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "ppyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "ppye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ppya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ppyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "ppyo",
                                        },
                                    ),
                                ]),
                                output: "ppi",
                            },
                        ),
                        (
                            'ふ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "ffyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "ffye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ffya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ffyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "ffyo",
                                        },
                                    ),
                                ]),
                                output: "ffu",
                            },
                        ),
                        (
                            'ぶ',
                            Node {
                                transitions: None,
                                output: "bbu",
                            },
                        ),
                        (
                            'ぷ',
                            Node {
                                transitions: None,
                                output: "ppu",
                            },
                        ),
                        (
                            'へ',
                            Node {
                                transitions: None,
                                output: "hhe",
                            },
                        ),
                        (
                            'べ',
                            Node {
                                transitions: None,
                                output: "bbe",
                            },
                        ),
                        (
                            'ぺ',
                            Node {
                                transitions: None,
                                output: "ppe",
                            },
                        ),
                        (
                            'ほ',
                            Node {
                                transitions: None,
                                output: "hho",
                            },
                        ),
                        (
                            'ぼ',
                            Node {
                                transitions: None,
                                output: "bbo",
                            },
                        ),
                        (
                            'ぽ',
                            Node {
                                transitions: None,
                                output: "ppo",
                            },
                        ),
                        (
                            'ま',
                            Node {
                                transitions: None,
                                output: "mma",
                            },
                        ),
                        (
                            'み',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "mmyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "mmye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "mmya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "mmyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "mmyo",
                                        },
                                    ),
                                ]),
                                output: "mmi",
                            },
                        ),
                        (
                            'む',
                            Node {
                                transitions: None,
                                output: "mmu",
                            },
                        ),
                        (
                            'め',
                            Node {
                                transitions: None,
                                output: "mme",
                            },
                        ),
                        (
                            'も',
                            Node {
                                transitions: None,
                                output: "mmo",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "ya",
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: "ya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "yu",
                            },
                        ),
                        (
                            'ゆ',
                            Node {
                                transitions: None,
                                output: "yu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "yo",
                            },
                        ),
                        (
                            'よ',
                            Node {
                                transitions: None,
                                output: "yo",
                            },
                        ),
                        (
                            'ら',
                            Node {
                                transitions: None,
                                output: "rra",
                            },
                        ),
                        (
                            'り',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "rryi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "rrye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "rrya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "rryu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "rryo",
                                        },
                                    ),
                                ]),
                                output: "rri",
                            },
                        ),
                        (
                            'る',
                            Node {
                                transitions: None,
                                output: "rru",
                            },
                        ),
                        (
                            'れ',
                            Node {
                                transitions: None,
                                output: "rre",
                            },
                        ),
                        (
                            'ろ',
                            Node {
                                transitions: None,
                                output: "rro",
                            },
                        ),
                        (
                            'わ',
                            Node {
                                transitions: None,
                                output: "wwa",
                            },
                        ),
                        (
                            'ゐ',
                            Node {
                                transitions: None,
                                output: "wwi",
                            },
                        ),
                        (
                            'ゑ',
                            Node {
                                transitions: None,
                                output: "wwe",
                            },
                        ),
                        (
                            'を',
                            Node {
                                transitions: None,
                                output: "wwo",
                            },
                        ),
                        (
                            'ん',
                            Node {
                                transitions: None,
                                output: "n",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ぃ',
                                        Node {
                                            transitions: None,
                                            output: "vvyi",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "vvye",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "vvya",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "vvyu",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "vvyo",
                                        },
                                    ),
                                ]),
                                output: "vvu",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "vva",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "vvi",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "vve",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "vvo",
                            },
                        ),
                        (
                            '・',
                            Node {
                                transitions: None,
                                output: "/",
                            },
                        ),
                        (
                            'ー',
                            Node {
                                transitions: None,
                                output: "-",
                            },
                        ),
                        (
                            '！',
                            Node {
                                transitions: None,
                                output: "!",
                            },
                        ),
                        (
                            '（',
                            Node {
                                transitions: None,
                                output: "(",
                            },
                        ),
                        (
                            '）',
                            Node {
                                transitions: None,
                                output: ")",
                            },
                        ),
                        (
                            '：',
                            Node {
                                transitions: None,
                                output: ":",
                            },
                        ),
                        (
                            '？',
                            Node {
                                transitions: None,
                                output: "?",
                            },
                        ),
                        (
                            '［',
                            Node {
                                transitions: None,
                                output: "[",
                            },
                        ),
                        (
                            '］',
                            Node {
                                transitions: None,
                                output: "]",
                            },
                        ),
                        (
                            '｛',
                            Node {
                                transitions: None,
                                output: "{",
                            },
                        ),
                        (
                            '｝',
                            Node {
                                transitions: None,
                                output: "}",
                            },
                        ),
                    ]),
                    output: "",
                },
            ),
            (
                'つ',
                Node {
                    transitions: None,
                    output: "tsu",
                },
            ),
            (
                'づ',
                Node {
                    transitions: None,
                    output: "zu",
                },
            ),
            (
                'て',
                Node {
                    transitions: None,
                    output: "te",
                },
            ),
            (
                'で',
                Node {
                    transitions: None,
                    output: "de",
                },
            ),
            (
                'と',
                Node {
                    transitions: None,
                    output: "to",
                },
            ),
            (
                'ど',
                Node {
                    transitions: None,
                    output: "do",
                },
            ),
            (
                'な',
                Node {
                    transitions: None,
                    output: "na",
                },
            ),
            (
                'に',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "nyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "nye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "nya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "nyu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "nyo",
                            },
                        ),
                    ]),
                    output: "ni",
                },
            ),
            (
                'ぬ',
                Node {
                    transitions: None,
                    output: "nu",
                },
            ),
            (
                'ね',
                Node {
                    transitions: None,
                    output: "ne",
                },
            ),
            (
                'の',
                Node {
                    transitions: None,
                    output: "no",
                },
            ),
            (
                'は',
                Node {
                    transitions: None,
                    output: "ha",
                },
            ),
            (
                'ば',
                Node {
                    transitions: None,
                    output: "ba",
                },
            ),
            (
                'ぱ',
                Node {
                    transitions: None,
                    output: "pa",
                },
            ),
            (
                'ひ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "hyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "hye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "hya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "hyu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "hyo",
                            },
                        ),
                    ]),
                    output: "hi",
                },
            ),
            (
                'び',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "byi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "bye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "bya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "byu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "byo",
                            },
                        ),
                    ]),
                    output: "bi",
                },
            ),
            (
                'ぴ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "pyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "pye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "pya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "pyu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "pyo",
                            },
                        ),
                    ]),
                    output: "pi",
                },
            ),
            (
                'ふ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "fyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "fye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "fya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "fyu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "fyo",
                            },
                        ),
                    ]),
                    output: "fu",
                },
            ),
            (
                'ぶ',
                Node {
                    transitions: None,
                    output: "bu",
                },
            ),
            (
                'ぷ',
                Node {
                    transitions: None,
                    output: "pu",
                },
            ),
            (
                'へ',
                Node {
                    transitions: None,
                    output: "he",
                },
            ),
            (
                'べ',
                Node {
                    transitions: None,
                    output: "be",
                },
            ),
            (
                'ぺ',
                Node {
                    transitions: None,
                    output: "pe",
                },
            ),
            (
                'ほ',
                Node {
                    transitions: None,
                    output: "ho",
                },
            ),
            (
                'ぼ',
                Node {
                    transitions: None,
                    output: "bo",
                },
            ),
            (
                'ぽ',
                Node {
                    transitions: None,
                    output: "po",
                },
            ),
            (
                'ま',
                Node {
                    transitions: None,
                    output: "ma",
                },
            ),
            (
                'み',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "myi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "mye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "mya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "myu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "myo",
                            },
                        ),
                    ]),
                    output: "mi",
                },
            ),
            (
                'む',
                Node {
                    transitions: None,
                    output: "mu",
                },
            ),
            (
                'め',
                Node {
                    transitions: None,
                    output: "me",
                },
            ),
            (
                'も',
                Node {
                    transitions: None,
                    output: "mo",
                },
            ),
            (
                'ゃ',
                Node {
                    transitions: None,
                    output: "ya",
                },
            ),
            (
                'や',
                Node {
                    transitions: None,
                    output: "ya",
                },
            ),
            (
                'ゅ',
                Node {
                    transitions: None,
                    output: "yu",
                },
            ),
            (
                'ゆ',
                Node {
                    transitions: None,
                    output: "yu",
                },
            ),
            (
                'ょ',
                Node {
                    transitions: None,
                    output: "yo",
                },
            ),
            (
                'よ',
                Node {
                    transitions: None,
                    output: "yo",
                },
            ),
            (
                'ら',
                Node {
                    transitions: None,
                    output: "ra",
                },
            ),
            (
                'り',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "ryi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "rye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "rya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "ryu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "ryo",
                            },
                        ),
                    ]),
                    output: "ri",
                },
            ),
            (
                'る',
                Node {
                    transitions: None,
                    output: "ru",
                },
            ),
            (
                'れ',
                Node {
                    transitions: None,
                    output: "re",
                },
            ),
            (
                'ろ',
                Node {
                    transitions: None,
                    output: "ro",
                },
            ),
            (
                'わ',
                Node {
                    transitions: None,
                    output: "wa",
                },
            ),
            (
                'ゐ',
                Node {
                    transitions: None,
                    output: "wi",
                },
            ),
            (
                'ゑ',
                Node {
                    transitions: None,
                    output: "we",
                },
            ),
            (
                'を',
                Node {
                    transitions: None,
                    output: "wo",
                },
            ),
            (
                'ん',
                Node {
                    transitions: Some(vec![
                        (
                            'あ',
                            Node {
                                transitions: None,
                                output: "n\'a",
                            },
                        ),
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "n\'i",
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "n\'u",
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: "n\'e",
                            },
                        ),
                        (
                            'お',
                            Node {
                                transitions: None,
                                output: "n\'o",
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: "n\'ya",
                            },
                        ),
                        (
                            'ゆ',
                            Node {
                                transitions: None,
                                output: "n\'yu",
                            },
                        ),
                        (
                            'よ',
                            Node {
                                transitions: None,
                                output: "n\'yo",
                            },
                        ),
                    ]),
                    output: "n",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "vyi",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "vye",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "vya",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "vyu",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: None,
                                output: "vyo",
                            },
                        ),
                    ]),
                    output: "vu",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "va",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "vi",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "ve",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "vo",
                },
            ),
            (
                '・',
                Node {
                    transitions: None,
                    output: "/",
                },
            ),
            (
                'ー',
                Node {
                    transitions: None,
                    output: "-",
                },
            ),
            (
                '！',
                Node {
                    transitions: None,
                    output: "!",
                },
            ),
            (
                '（',
                Node {
                    transitions: None,
                    output: "(",
                },
            ),
            (
                '）',
                Node {
                    transitions: None,
                    output: ")",
                },
            ),
            (
                '：',
                Node {
                    transitions: None,
                    output: ":",
                },
            ),
            (
                '？',
                Node {
                    transitions: None,
                    output: "?",
                },
            ),
            (
                '［',
                Node {
                    transitions: None,
                    output: "[",
                },
            ),
            (
                '］',
                Node {
                    transitions: None,
                    output: "]",
                },
            ),
            (
                '｛',
                Node {
                    transitions: None,
                    output: "{",
                },
            ),
            (
                '｝',
                Node {
                    transitions: None,
                    output: "}",
                },
            ),
        ]);

        let mut node = Node { transitions, output: "" };
        node.sort();
        node
    };
}
