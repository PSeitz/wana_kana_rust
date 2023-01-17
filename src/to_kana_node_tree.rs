#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub transitions: Vec<(char, Node)>,
    pub output: Option<&'static str>,
}

impl Node {
    pub(crate) fn get<'a>(&self, chars: &'a [char]) -> (Option<&'static str>, usize) {
        let mut i = 0;
        let mut curr_node = self;
        for char in chars.iter() {
            if let Some(trans_node) = curr_node.find_transition_node(char.to_ascii_lowercase()) {
                curr_node = trans_node;
            } else {
                break;
            }
            i += 1;
        }

        if let Some(_output) = curr_node.output {
            (curr_node.output, i)
        } else {
            (None, 0)
        }
    }

    pub(crate) fn find_transition_node(&self, char: char) -> Option<&Node> {
        self.transitions.iter().find(|&t| t.0 == char).map(|t| &t.1)
        // self.transitions.binary_search_by_key(&char, |t|
        // t.0).ok().map(|index|&self.transitions[index].1)
    }

    fn find_transition_mut(&mut self, char: char) -> Option<&mut (char, Node)> {
        self.transitions.iter_mut().find(|t| t.0 == char)
    }

    fn sort(&mut self) {
        self.transitions.sort_by_key(|el| el.0);
        for el in &mut self.transitions {
            el.1.sort();
        }
    }
}

#[test]
fn test_node_tree() {
    let chars = ['a'];
    assert_eq!(TO_KANA_NODE_TREE.get(&chars).0, Some("あ"));
}

lazy_static! {
    pub(crate) static ref TO_KANA_NODE_TREE: Node = {
        let transitions = vec![
            (
                '!',
                Node {
                    transitions: vec![],
                    output: Some("！"),
                },
            ),
            (
                '(',
                Node {
                    transitions: vec![],
                    output: Some("（"),
                },
            ),
            (
                ')',
                Node {
                    transitions: vec![],
                    output: Some("）"),
                },
            ),
            (
                ',',
                Node {
                    transitions: vec![],
                    output: Some("、"),
                },
            ),
            (
                '-',
                Node {
                    transitions: vec![],
                    output: Some("ー"),
                },
            ),
            (
                '.',
                Node {
                    transitions: vec![],
                    output: Some("。"),
                },
            ),
            (
                '/',
                Node {
                    transitions: vec![],
                    output: Some("・"),
                },
            ),
            (
                ':',
                Node {
                    transitions: vec![],
                    output: Some("："),
                },
            ),
            (
                '?',
                Node {
                    transitions: vec![],
                    output: Some("？"),
                },
            ),
            (
                '[',
                Node {
                    transitions: vec![],
                    output: Some("［"),
                },
            ),
            (
                ']',
                Node {
                    transitions: vec![],
                    output: Some("］"),
                },
            ),
            (
                'a',
                Node {
                    transitions: vec![],
                    output: Some("あ"),
                },
            ),
            (
                'b',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ば"),
                            },
                        ),
                        (
                            'b',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っば"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っべ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っび"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぼ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぶ"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っびゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っびぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っびぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っびょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っびゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("べ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("び"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ぼ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ぶ"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("びゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("びぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("びぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("びょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("びゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'c',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("か"),
                            },
                        ),
                        (
                            'c',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っか"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っけ"),
                                        },
                                    ),
                                    (
                                        'h',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っち"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちゅ"),
                                                    },
                                                ),
                                                (
                                                    'y',
                                                    Node {
                                                        transitions: vec![
                                                            (
                                                                'a',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っちゃ"),
                                                                },
                                                            ),
                                                            (
                                                                'e',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っちぇ"),
                                                                },
                                                            ),
                                                            (
                                                                'i',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っちぃ"),
                                                                },
                                                            ),
                                                            (
                                                                'o',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っちょ"),
                                                                },
                                                            ),
                                                            (
                                                                'u',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っちゅ"),
                                                                },
                                                            ),
                                                        ],
                                                        output: None,
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っき"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っこ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っく"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("け"),
                            },
                        ),
                        (
                            'h',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ち"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちゅ"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ちゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ちぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ちぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ちょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ちゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("き"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("こ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("く"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'd',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("だ"),
                            },
                        ),
                        (
                            'd',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っだ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っで"),
                                        },
                                    ),
                                    (
                                        'h',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っでゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っでぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っでぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っでょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っでゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぢ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っど"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っづ"),
                                        },
                                    ),
                                    (
                                        'w',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っどぁ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っどぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っどぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っどぉ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っどぅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぢゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぢぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぢぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぢょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぢゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("で"),
                            },
                        ),
                        (
                            'h',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("でゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("でぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("でぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("でょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("でゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ぢ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ど"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("づ"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("どぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("どぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("どぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("どぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("どぅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぢゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぢぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぢぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぢょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぢゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'e',
                Node {
                    transitions: vec![],
                    output: Some("え"),
                },
            ),
            (
                'f',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ふぁ"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("ふぇ"),
                            },
                        ),
                        (
                            'f',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っふぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っふぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っふぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っふぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っふ"),
                                        },
                                    ),
                                    (
                                        'w',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふぁ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふぉ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふぅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っふゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ふぃ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ふぉ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ふ"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふぅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ふゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'g',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("が"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("げ"),
                            },
                        ),
                        (
                            'g',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っが"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っげ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぎ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っご"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぐ"),
                                        },
                                    ),
                                    (
                                        'w',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぐぁ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぐぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぐぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぐぉ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぐぅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぎゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぎぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぎぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぎょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぎゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ぎ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ご"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ぐ"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぐぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぐぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぐぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぐぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぐぅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぎゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぎぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぎぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぎょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぎゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'h',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("は"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("へ"),
                            },
                        ),
                        (
                            'h',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っは"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っへ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っひ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っほ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っふ"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っひゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っひぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っひぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っひょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っひゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ひ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ほ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ふ"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ひゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ひぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ひぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ひょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ひゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'i',
                Node {
                    transitions: vec![],
                    output: Some("い"),
                },
            ),
            (
                'j',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("じゃ"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("じぇ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("じ"),
                            },
                        ),
                        (
                            'j',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っじゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っじぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っじ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っじょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っじゅ"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("じょ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("じゅ"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'k',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("か"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("け"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("き"),
                            },
                        ),
                        (
                            'k',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っか"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っけ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っき"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っこ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っく"),
                                        },
                                    ),
                                    (
                                        'w',
                                        Node {
                                            transitions: vec![(
                                                'a',
                                                Node {
                                                    transitions: vec![],
                                                    output: Some("っくぁ"),
                                                },
                                            )],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っきゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っきぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っきぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っきょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っきゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("こ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("く"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![(
                                    'a',
                                    Node {
                                        transitions: vec![],
                                        output: Some("くぁ"),
                                    },
                                )],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("きゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("きぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("きぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("きょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("きゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'l',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ぁ"),
                            },
                        ),
                        (
                            'c',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ヵ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ヶ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("ぇ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ぃ"),
                            },
                        ),
                        (
                            'k',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ヵ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ヶ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ぉ"),
                            },
                        ),
                        (
                            't',
                            Node {
                                transitions: vec![
                                    (
                                        's',
                                        Node {
                                            transitions: vec![(
                                                'u',
                                                Node {
                                                    transitions: vec![],
                                                    output: Some("っ"),
                                                },
                                            )],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ぅ"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![(
                                    'a',
                                    Node {
                                        transitions: vec![],
                                        output: Some("ゎ"),
                                    },
                                )],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'm',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ま"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("め"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("み"),
                            },
                        ),
                        (
                            'm',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っま"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っめ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っみ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っも"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っむ"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っみゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っみぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っみぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っみょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っみゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("も"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("む"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("みゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("みぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("みぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("みょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("みゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'n',
                Node {
                    transitions: vec![
                        (
                            '\'',
                            Node {
                                transitions: vec![],
                                output: Some("ん"),
                            },
                        ),
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("な"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("ね"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("に"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("の"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ぬ"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("にゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("にぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("にぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("にょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("にゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: Some("ん"),
                },
            ),
            (
                'o',
                Node {
                    transitions: vec![],
                    output: Some("お"),
                },
            ),
            (
                'p',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ぱ"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("ぺ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ぴ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ぽ"),
                            },
                        ),
                        (
                            'p',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぱ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぺ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぴ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぽ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぷ"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぴゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぴぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぴぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぴょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っぴゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ぷ"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぴゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぴぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぴぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぴょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぴゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'q',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("くぁ"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("くぇ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("くぃ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("くぉ"),
                            },
                        ),
                        (
                            'q',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っくぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っくぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っくぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っくぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っくぅ"),
                                        },
                                    ),
                                    (
                                        'w',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくぁ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくぉ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくぅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っくゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("くぅ"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くぅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("くゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'r',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ら"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("れ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("り"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ろ"),
                            },
                        ),
                        (
                            'r',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っら"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っれ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っり"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っろ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っる"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っりゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っりぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っりぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っりょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っりゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("る"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("りゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("りぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("りぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("りょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("りゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                's',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("さ"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("せ"),
                            },
                        ),
                        (
                            'h',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("し"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しゅ"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("しゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("しぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("しぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("しょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("しゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("し"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("そ"),
                            },
                        ),
                        (
                            's',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っさ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っせ"),
                                        },
                                    ),
                                    (
                                        'h',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っし"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしゅ"),
                                                    },
                                                ),
                                                (
                                                    'y',
                                                    Node {
                                                        transitions: vec![
                                                            (
                                                                'a',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っしゃ"),
                                                                },
                                                            ),
                                                            (
                                                                'e',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っしぇ"),
                                                                },
                                                            ),
                                                            (
                                                                'i',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っしぃ"),
                                                                },
                                                            ),
                                                            (
                                                                'o',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っしょ"),
                                                                },
                                                            ),
                                                            (
                                                                'u',
                                                                Node {
                                                                    transitions: vec![],
                                                                    output: Some("っしゅ"),
                                                                },
                                                            ),
                                                        ],
                                                        output: None,
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っし"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っそ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っす"),
                                        },
                                    ),
                                    (
                                        'w',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っすぁ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っすぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っすぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っすぉ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っすぅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っしゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("す"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("すぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("すぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("すぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("すぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("すぅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("しゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                't',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("た"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("て"),
                            },
                        ),
                        (
                            'h',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("てゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("てぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("てぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("てょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("てゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ち"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("と"),
                            },
                        ),
                        (
                            's',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("つぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("つぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("つぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("つぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("つ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            't',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("った"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("って"),
                                        },
                                    ),
                                    (
                                        'h',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ってゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ってぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ってぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ってょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("ってゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っち"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っと"),
                                        },
                                    ),
                                    (
                                        's',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っつぁ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っつぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っつぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っつぉ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っつ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っつ"),
                                        },
                                    ),
                                    (
                                        'w',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っとぁ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っとぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っとぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っとぉ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っとぅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っちゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("つ"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("とぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("とぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("とぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("とぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("とぅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ちゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'u',
                Node {
                    transitions: vec![],
                    output: Some("う"),
                },
            ),
            (
                'v',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ゔぁ"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("ゔぇ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ゔぃ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ゔぉ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ゔ"),
                            },
                        ),
                        (
                            'v',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っゔぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っゔぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っゔぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っゔぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っゔ"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っゔゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っゔぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っゔぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っゔょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っゔゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゔゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゔぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゔぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゔょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゔゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'w',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("わ"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("うぇ"),
                            },
                        ),
                        (
                            'h',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("うぁ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("うぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("うぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("うぉ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("う"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("うぃ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("を"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("う"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っわ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っうぇ"),
                                        },
                                    ),
                                    (
                                        'h',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っうぁ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っうぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っうぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っうぉ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っう"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っうぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っを"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っう"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'x',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ぁ"),
                            },
                        ),
                        (
                            'c',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ヵ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ヶ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("ぇ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("ぃ"),
                            },
                        ),
                        (
                            'k',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ヵ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ヶ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'n',
                            Node {
                                transitions: vec![],
                                output: Some("ん"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ぉ"),
                            },
                        ),
                        (
                            't',
                            Node {
                                transitions: vec![
                                    (
                                        's',
                                        Node {
                                            transitions: vec![(
                                                'u',
                                                Node {
                                                    transitions: vec![],
                                                    output: Some("っ"),
                                                },
                                            )],
                                            output: None,
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ぅ"),
                            },
                        ),
                        (
                            'w',
                            Node {
                                transitions: vec![(
                                    'a',
                                    Node {
                                        transitions: vec![],
                                        output: Some("ゎ"),
                                    },
                                )],
                                output: None,
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("ゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'y',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("や"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("いぇ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("い"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("よ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ゆ"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っや"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っいぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っい"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っよ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っゆ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                'z',
                Node {
                    transitions: vec![
                        (
                            'a',
                            Node {
                                transitions: vec![],
                                output: Some("ざ"),
                            },
                        ),
                        (
                            'e',
                            Node {
                                transitions: vec![],
                                output: Some("ぜ"),
                            },
                        ),
                        (
                            'i',
                            Node {
                                transitions: vec![],
                                output: Some("じ"),
                            },
                        ),
                        (
                            'o',
                            Node {
                                transitions: vec![],
                                output: Some("ぞ"),
                            },
                        ),
                        (
                            'u',
                            Node {
                                transitions: vec![],
                                output: Some("ず"),
                            },
                        ),
                        (
                            'y',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じゃ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じぇ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じぃ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じょ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("じゅ"),
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                        (
                            'z',
                            Node {
                                transitions: vec![
                                    (
                                        'a',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っざ"),
                                        },
                                    ),
                                    (
                                        'e',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぜ"),
                                        },
                                    ),
                                    (
                                        'i',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っじ"),
                                        },
                                    ),
                                    (
                                        'o',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っぞ"),
                                        },
                                    ),
                                    (
                                        'u',
                                        Node {
                                            transitions: vec![],
                                            output: Some("っず"),
                                        },
                                    ),
                                    (
                                        'y',
                                        Node {
                                            transitions: vec![
                                                (
                                                    'a',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじゃ"),
                                                    },
                                                ),
                                                (
                                                    'e',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじぇ"),
                                                    },
                                                ),
                                                (
                                                    'i',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじぃ"),
                                                    },
                                                ),
                                                (
                                                    'o',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじょ"),
                                                    },
                                                ),
                                                (
                                                    'u',
                                                    Node {
                                                        transitions: vec![],
                                                        output: Some("っじゅ"),
                                                    },
                                                ),
                                            ],
                                            output: None,
                                        },
                                    ),
                                ],
                                output: None,
                            },
                        ),
                    ],
                    output: None,
                },
            ),
            (
                '{',
                Node {
                    transitions: vec![],
                    output: Some("｛"),
                },
            ),
            (
                '}',
                Node {
                    transitions: vec![],
                    output: Some("｝"),
                },
            ),
            (
                '~',
                Node {
                    transitions: vec![],
                    output: Some("〜"),
                },
            ),
            (
                '‘',
                Node {
                    transitions: vec![],
                    output: Some("「"),
                },
            ),
            (
                '’',
                Node {
                    transitions: vec![],
                    output: Some("」"),
                },
            ),
            (
                '“',
                Node {
                    transitions: vec![],
                    output: Some("『"),
                },
            ),
            (
                '”',
                Node {
                    transitions: vec![],
                    output: Some("』"),
                },
            ),
        ];
        let mut tree = Node {
            transitions,
            output: None,
        };
        tree.sort();
        tree
    };
    pub(crate) static ref TO_KANA_NODE_TREE_OBSOLETE: Node = {
        let mut tree = TO_KANA_NODE_TREE.clone();
        let w = tree.find_transition_mut('w').unwrap();
        w.1.transitions.retain(|x| x.0 != 'i' && x.0 != 'e');
        w.1.transitions.push((
            'i',
            Node {
                transitions: vec![],
                output: Some("ゐ"),
            },
        ));
        w.1.transitions.push((
            'e',
            Node {
                transitions: vec![],
                output: Some("ゑ"),
            },
        ));
        tree.sort();
        tree
    };
    pub(crate) static ref TO_KANA_NODE_TREE_IMEMODE: Node = {
        let mut tree = TO_KANA_NODE_TREE.clone();
        let w = tree.find_transition_mut('n').unwrap();
        w.1.transitions.push((
            'n',
            Node {
                transitions: vec![],
                output: Some("ん"),
            },
        ));
        w.1.transitions.push((
            ' ',
            Node {
                transitions: vec![],
                output: Some("ん"),
            },
        ));
        w.1.output = None;
        tree.sort();
        tree
    };
}
