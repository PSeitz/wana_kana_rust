#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub transitions: Option<Vec<(char, Node)>>,
    pub output: &'static str,
}

impl Node {
    pub(crate) fn get(&self, chars: &[char]) -> (&'static str, usize) {
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

    pub(crate) fn find_transition_node(&self, char: char) -> Option<&Node> {
        if let Some(t) = &self.transitions {
            t.binary_search_by_key(&char, |t| t.0)
                .ok()
                .map(|index| &t[index].1)
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
    pub(crate) static ref HALFWIDTH_KATAKANA_TO_HIRAGANA_NODE_TREE: Node = {
        let transitions = Some(vec![
            (
                '\u{3000}',
                Node {
                    transitions: None,
                    output: " ",
                },
            ),
            (
                'ｧ',
                Node {
                    transitions: None,
                    output: "ぁ",
                },
            ),
            (
                'ｱ',
                Node {
                    transitions: None,
                    output: "あ",
                },
            ),
            (
                'ｨ',
                Node {
                    transitions: None,
                    output: "ぃ",
                },
            ),
            (
                'ｲ',
                Node {
                    transitions: None,
                    output: "い",
                },
            ),
            (
                'ｩ',
                Node {
                    transitions: None,
                    output: "ぅ",
                },
            ),
            (
                'ｳ',
                Node {
                    output: "う",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ゔ",
                        },
                    )]),
                },
            ),
            (
                'ｪ',
                Node {
                    transitions: None,
                    output: "ぇ",
                },
            ),
            (
                'ｴ',
                Node {
                    transitions: None,
                    output: "え",
                },
            ),
            (
                'ｫ',
                Node {
                    transitions: None,
                    output: "ぉ",
                },
            ),
            (
                'ｵ',
                Node {
                    transitions: None,
                    output: "お",
                },
            ),
            (
                'ｶ',
                Node {
                    output: "か",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "が",
                        },
                    )]),
                },
            ),
            (
                'ｷ',
                Node {
                    output: "き",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ぎ",
                        },
                    )]),
                },
            ),
            (
                'ｸ',
                Node {
                    output: "く",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ぐ",
                        },
                    )]),
                },
            ),
            (
                'ｹ',
                Node {
                    output: "け",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "げ",
                        },
                    )]),
                },
            ),
            (
                'ｺ',
                Node {
                    output: "こ",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ご",
                        },
                    )]),
                },
            ),
            (
                'ｻ',
                Node {
                    output: "さ",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ざ",
                        },
                    )]),
                },
            ),
            (
                'ｼ',
                Node {
                    output: "し",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "じ",
                        },
                    )]),
                },
            ),
            (
                'ｽ',
                Node {
                    output: "す",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ず",
                        },
                    )]),
                },
            ),
            (
                'ｾ',
                Node {
                    output: "せ",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ぜ",
                        },
                    )]),
                },
            ),
            (
                'ｿ',
                Node {
                    output: "そ",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ぞ",
                        },
                    )]),
                },
            ),
            (
                'ﾀ',
                Node {
                    output: "た",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "だ",
                        },
                    )]),
                },
            ),
            (
                'ﾁ',
                Node {
                    output: "ち",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ぢ",
                        },
                    )]),
                },
            ),
            (
                'ｯ',
                Node {
                    transitions: None,
                    output: "っ",
                },
            ),
            (
                'ﾂ',
                Node {
                    output: "つ",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "づ",
                        },
                    )]),
                },
            ),
            (
                'ﾃ',
                Node {
                    output: "て",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "で",
                        },
                    )]),
                },
            ),
            (
                'ﾄ',
                Node {
                    output: "と",
                    transitions: Some(vec![(
                        'ﾞ',
                        Node {
                            transitions: None,
                            output: "ど",
                        },
                    )]),
                },
            ),
            (
                'ﾅ',
                Node {
                    transitions: None,
                    output: "な",
                },
            ),
            (
                'ﾆ',
                Node {
                    transitions: None,
                    output: "に",
                },
            ),
            (
                'ﾇ',
                Node {
                    transitions: None,
                    output: "ぬ",
                },
            ),
            (
                'ﾈ',
                Node {
                    transitions: None,
                    output: "ね",
                },
            ),
            (
                'ﾉ',
                Node {
                    transitions: None,
                    output: "の",
                },
            ),
            (
                'ﾊ',
                Node {
                    output: "は",
                    transitions: Some(vec![
                        (
                            'ﾞ',
                            Node {
                                transitions: None,
                                output: "ば",
                            },
                        ),
                        (
                            'ﾟ',
                            Node {
                                transitions: None,
                                output: "ぱ",
                            },
                        ),
                    ]),
                },
            ),
            (
                'ﾋ',
                Node {
                    output: "ひ",
                    transitions: Some(vec![
                        (
                            'ﾞ',
                            Node {
                                transitions: None,
                                output: "び",
                            },
                        ),
                        (
                            'ﾟ',
                            Node {
                                transitions: None,
                                output: "ぴ",
                            },
                        ),
                    ]),
                },
            ),
            (
                'ﾌ',
                Node {
                    output: "ふ",
                    transitions: Some(vec![
                        (
                            'ﾞ',
                            Node {
                                transitions: None,
                                output: "ぶ",
                            },
                        ),
                        (
                            'ﾟ',
                            Node {
                                transitions: None,
                                output: "ぷ",
                            },
                        ),
                    ]),
                },
            ),
            (
                'ﾍ',
                Node {
                    output: "へ",
                    transitions: Some(vec![
                        (
                            'ﾞ',
                            Node {
                                transitions: None,
                                output: "べ",
                            },
                        ),
                        (
                            'ﾟ',
                            Node {
                                transitions: None,
                                output: "ぺ",
                            },
                        ),
                    ]),
                },
            ),
            (
                'ﾎ',
                Node {
                    output: "ほ",
                    transitions: Some(vec![
                        (
                            'ﾞ',
                            Node {
                                transitions: None,
                                output: "ぼ",
                            },
                        ),
                        (
                            'ﾟ',
                            Node {
                                transitions: None,
                                output: "ぽ",
                            },
                        ),
                    ]),
                },
            ),
            (
                'ﾏ',
                Node {
                    transitions: None,
                    output: "ま",
                },
            ),
            (
                'ﾐ',
                Node {
                    transitions: None,
                    output: "み",
                },
            ),
            (
                'ﾑ',
                Node {
                    transitions: None,
                    output: "む",
                },
            ),
            (
                'ﾒ',
                Node {
                    transitions: None,
                    output: "め",
                },
            ),
            (
                'ﾓ',
                Node {
                    transitions: None,
                    output: "も",
                },
            ),
            (
                'ｬ',
                Node {
                    transitions: None,
                    output: "ゃ",
                },
            ),
            (
                'ﾔ',
                Node {
                    transitions: None,
                    output: "や",
                },
            ),
            (
                'ｭ',
                Node {
                    transitions: None,
                    output: "ゅ",
                },
            ),
            (
                'ﾕ',
                Node {
                    transitions: None,
                    output: "ゆ",
                },
            ),
            (
                'ｮ',
                Node {
                    transitions: None,
                    output: "ょ",
                },
            ),
            (
                'ﾖ',
                Node {
                    transitions: None,
                    output: "よ",
                },
            ),
            (
                'ﾗ',
                Node {
                    transitions: None,
                    output: "ら",
                },
            ),
            (
                'ﾘ',
                Node {
                    transitions: None,
                    output: "り",
                },
            ),
            (
                'ﾙ',
                Node {
                    transitions: None,
                    output: "る",
                },
            ),
            (
                'ﾚ',
                Node {
                    transitions: None,
                    output: "れ",
                },
            ),
            (
                'ﾛ',
                Node {
                    transitions: None,
                    output: "ろ",
                },
            ),
            (
                'ﾜ',
                Node {
                    transitions: None,
                    output: "わ",
                },
            ),
            (
                'ｦ',
                Node {
                    transitions: None,
                    output: "を",
                },
            ),
            (
                'ﾝ',
                Node {
                    transitions: None,
                    output: "ん",
                },
            ),
            (
                'ｰ',
                Node {
                    transitions: None,
                    output: "ー",
                },
            ),
            (
                '｡',
                Node {
                    transitions: None,
                    output: "。",
                },
            ),
            (
                '､',
                Node {
                    transitions: None,
                    output: "、",
                },
            ),
            (
                '｢',
                Node {
                    transitions: None,
                    output: "「",
                },
            ),
            (
                '｣',
                Node {
                    transitions: None,
                    output: "」",
                },
            ),
            (
                '･',
                Node {
                    transitions: None,
                    output: "・",
                },
            ),
        ]);

        let mut node = Node {
            transitions,
            output: "",
        };
        node.sort();
        node
    };
}
