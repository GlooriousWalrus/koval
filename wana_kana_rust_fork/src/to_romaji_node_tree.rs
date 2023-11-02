#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub transitions: Option<Vec<(char, Node)>>,
    pub output: &'static str,
}

impl Node {
    pub(crate) fn get<'a>(&self, chars: &'a [char]) -> (&'static str, usize) {
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
    pub(crate) static ref TO_ROMAJI_NODE_TREE: Node = {
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
                    output: "'",
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
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "ай",
                            },
                        ),
                    ]),
                    output: "а",
                },
            ),
            (
                'あ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "ай",
                            },
                        ),
                    ]),
                    output: "а",
                },
            ),
            (
                'ぃ',
                Node {
                    transitions: Some(vec![
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "i\'є",
                            },
                        ),
                    ]),
                    output: "і",
                },
            ),
            (
                'い',
                Node {
                    transitions: Some(vec![
                        (
                            'え',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "ієї",
                                        },
                                    ),
                                ]),
                                output: "іє",
                            },
                        ),
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "ії",
                            },
                        ),
                        (
                            'ち',
                            Node {
                                transitions: None,
                                output: "їчі",
                            },
                        ),
                    ]),
                    output: "і",
                },
            ),
            (
                'ぅ',
                Node {
                    transitions: None,
                    output: "у",
                },
            ),
            (
                'う',
                Node {
                    transitions: None,
                    output: "у",
                },
            ),
            (
                'ぇ',
                Node {
                    transitions: None,
                    output: "е",
                },
            ),
            (
                'え',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "ей",
                            },
                        ),
                    ]),
                    output: "е",
                },
            ),
            (
                'ぉ',
                Node {
                    transitions: None,
                    output: "о",
                },
            ),
            (
                'お',
                Node {
                    //DOUBLE VOWEL
                    // transitions: None,
                    transitions: Some(vec![
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "о",
                                //output: "о\u{304}",
                            },
                        ),
                    ]),
                    output: "о",
                },
            ),
            (
                'か',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ち',
                                        Node {
                                            transitions: None,
                                            output: "каїчі",
                                        },
                                    ),
                                ]),
                                output: "кай",
                            },
                        ),
                        (
                            'あ',
                            Node {
                                transitions: None,
                                output: "ка",
                            }
                        ),
                    ]),
                    output: "ка",
                },
            ),
            (
                'が',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ち',
                                        Node {
                                            transitions: None,
                                            output: "ґаїчі",
                                        },
                                    ),
                                    (
                                        'も',
                                        Node {
                                            transitions: None,
                                            output: "ґаїмо",
                                        },
                                    ),
                                ]),
                                output: "ґай",
                            },
                        ),
                    ]),
                    output: "ґа",
                },
            ),
            (
                'き',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ち',
                                        Node {
                                            transitions: None,
                                            output: "кіїчі",
                                        },
                                    ),
                                ]),
                                output: "кій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "к\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "кя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "кю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "кьо",
                                        },
                                    ),
                                ]),
                                output: "кьо",
                            },
                        ),
                    ]),
                    output: "кі",
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
                                output: "ґій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "ґ\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "ґя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "ґю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "ґьо",
                                        },
                                    ),
                                ]),
                                output: "ґьо",
                            },
                        ),
                    ]),
                    output: "ґі",
                },
            ),
            (
                'く',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'な',
                                        Node {
                                            transitions: None,
                                            output: "куїна",
                                        },
                                    ),
                                ]),
                                output: "куй",
                            },
                        ),
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "куй",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "к\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "кя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "кю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "кьо",
                                        },
                                    ),
                                ]),
                                output: "кьо",
                            },
                        ),
                    ]),
                    output: "ку",
                },
            ),
            (
                'ぐ',
                Node {
                    transitions: Some(vec![
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "ґу",
                            },
                        ),
                    ]),
                    output: "ґу",
                },
            ),
            (
                'け',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "кей",
                            },
                        ),
                    ]),
                    output: "ке",
                },
            ),
            (
                'げ',
                Node {
                    transitions: None,
                    output: "ґе",
                },
            ),
            (
                'こ',
                Node {
                    transitions: None,
                    output: "ко",
                },
            ),
            (
                'ご',
                Node {
                    transitions: None,
                    output: "ґо",
                },
            ),
            (
                'さ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "сай",
                            },
                        ),
                    ]),
                    output: "са",
                },
            ),
            (
                'ざ',
                Node {
                    transitions: None,
                    output: "дза",
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
                                output: "шій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "ш\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "шя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "шю",
                                        },
                                    ),
                                ]),
                                output: "шю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "шьо",
                                        },
                                    ),
                                ]),
                                output: "шьо",
                            },
                        ),
                    ]),
                    output: "ші",
                },
            ),
            (
                'じ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'え',
                                        Node {
                                            transitions: None,
                                            output: "джіїє",
                                        },
                                    ),
                                ]),
                                output: "джій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "дж\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "джя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "джю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "джьо",
                                        },
                                    ),
                                ]),
                                output: "джьо",
                            },
                        ),
                    ]),
                    output: "джі",
                },
            ),
            (
                'す',
                Node {
                    transitions: None,
                    output: "су",
                },
            ),
            (
                'ず',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "дзуй"
                            },
                        ),
                    ]),
                    output: "дзу",
                },
            ),
            (
                'せ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "сей",
                            },
                        ),
                    ]),
                    output: "се",
                },
            ),
            (
                'ぜ',
                Node {
                    transitions: None,
                    output: "дзе",
                },
            ),
            (
                'そ',
                Node {
                    transitions: Some(vec![
                        (
                            'う',
                            Node {
                                transitions: Some(vec![
                                    // (
                                    //     'い',
                                    //     Node {
                                    //         transitions: None,
                                    //         output: "сой",
                                    //     },
                                    // ),
                                ]),
                                output: "со",
                            },
                        ),
                    ]),
                    output: "со",
                },
            ),
            (
                'ぞ',
                Node {
                    transitions: Some(vec![
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "дзо",
                            },
                        ),
                    ]),
                    output: "дзо",
                },
            ),
            (
                'た',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "тай",
                            },
                        ),
                    ]),
                    output: "та",
                },
            ),
            (
                'だ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "дай",
                            },
                        ),
                    ]),
                    output: "да",
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
                                output: "чій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "ч\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "чя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "чю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                //DOUBLE VOWEL
                                // transitions: None,
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "чьо"
                                            // output: "чьо\u{304}"
                                        },
                                    ),
                                ]),
                                output: "чьо",
                            },
                        ),
                    ]),
                    output: "чі",
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
                                output: "джій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "дж\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "джя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "джю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "джьо",
                                        },
                                    ),
                                ]),
                                output: "джьо",
                            },
                        ),
                    ]),
                    output: "джі",
                },
            ),

            (
                'つ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ど',
                                        Node {
                                            transitions: None,
                                            output: "цуїдо",
                                        },
                                    ),
                                ]),
                                output: "цуй",
                            },
                        ),
                    ]),
                    output: "цу",
                },
            ),
            (
                'づ',
                Node {
                    transitions: None,
                    output: "дзу",
                },
            ),
            (
                'て',
                Node {
                    transitions: None,
                    output: "те",
                },
            ),
            (
                'で',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "ді",
                            },
                        ),
                    ]),
                    output: "де",
                },
            ),
            (
                'と',
                Node {
                    transitions: Some(vec![
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "то",
                            },
                        ),
                    ]),
                    output: "то",
                },
            ),
            (
                'ど',
                Node {
                    transitions: Some(vec![
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "до",
                            },
                        ),
                    ]),
                    output: "до",
                },
            ),
            (
                'な',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "най",
                            },
                        ),
                    ]),
                    output: "на",
                },
            ),
            (
                'に',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "нії",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "н\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "ня",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "ню",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "ньо",
                                        },
                                    ),
                                ]),
                                output: "ньо",
                            },
                        ),
                    ]),
                    output: "ні",
                },
            ),
            (
                'ぬ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "нуй",
                            },
                        ),
                    ]),
                    output: "ну",
                },
            ),
            (
                'ね',
                Node {
                    // transitions: None,
                    transitions: Some(vec![
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: "не",
                            },
                        ),
                    ]),
                    output: "не",
                },
            ),
            (
                'の',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "ной",
                            },
                        ),
                    ]),
                    output: "но",
                },
            ),
            (
                'は',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "хай",
                            },
                        ),
                    ]),
                    output: "ха",
                },
            ),
            (
                'ば',
                Node {
                    // transitions: None,
                    transitions: Some(vec![
                        (
                            'あ',
                            Node {
                                transitions: None,
                                output: "ба"
                            },
                        ),
                    ]),
                    output: "ба",
                },
            ),
            (
                'ぱ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "пай"
                            }
                        ),
                    ]),
                    output: "па",
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
                                output: "хій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "х\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "хя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "хю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "хьо",
                                        },
                                    ),
                                ]),
                                output: "хьо",
                            },
                        ),
                    ]),
                    output: "хі",
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
                                output: "бій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "б\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "бя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "бю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "бьо",
                                        },
                                    ),
                                ]),
                                output: "бьо",
                            },
                        ),
                    ]),
                    output: "бі",
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
                                output: "пій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "п\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "пя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "пю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "пьо",
                                        },
                                    ),
                                ]),
                                output: "пьо",
                            },
                        ),
                    ]),
                    output: "пі",
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
                                output: "фій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "ф\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "фя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "фю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "фьо",
                                        },
                                    ),
                                ]),
                                output: "фьо",
                            },
                        ),
                    ]),
                    output: "фу",
                },
            ),
            (
                'ぶ',
                Node {
                    transitions: None,
                    output: "бу",
                },
            ),
            (
                'ぷ',
                Node {
                    transitions: None,
                    output: "пу",
                },
            ),
            (
                'へ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "хей",
                            },
                        ),
                    ]),
                    output: "хе",
                },
            ),
            (
                'べ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "бей",
                            },
                        ),
                    ]),
                    output: "бе",
                },
            ),
            (
                'ぺ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "пей",
                            },
                        ),
                    ]),
                    output: "пе",
                },
            ),
            (
                'ほ',
                Node {
                    transitions: None,
                    output: "хо",
                },
            ),
            (
                'ぼ',
                Node {
                    transitions: None,
                    output: "бо",
                },
            ),
            (
                'ぽ',
                Node {
                    transitions: None,
                    output: "по",
                },
            ),
            (
                'ま',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "май",
                            },
                        ),
                    ]),
                    output: "ма",
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
                                output: "мій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "м\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "мя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "мю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "мьо",
                                        },
                                    ),
                                ]),
                                output: "мьо",
                            },
                        ),
                    ]),
                    output: "мі",
                },
            ),
            (
                'む',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ち',
                                        Node {
                                            transitions: None,
                                            output: "муїчі",
                                        },
                                    ),
                                ]),
                                output: "муй",
                            },
                        ),
                    ]),
                    output: "му",
                },
            ),
            (
                'め',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "мей",
                            },
                        ),
                    ]),
                    output: "ме",
                },
            ),
            (
                'も',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "мой",
                            },
                        ),
                    ]),
                    output: "мо",
                },
            ),
            (
                'ゃ',
                Node {
                    transitions: None,
                    output: "я",
                },
            ),
            (
                'や',
                Node {
                    transitions: None,
                    output: "я",
                },
            ),
            (
                'ゅ',
                Node {
                    transitions: None,
                    output: "ю",
                },
            ),
            (
                'ゆ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "юй",
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "юї",
                                        },
                                    ),
                                ]),
                                output: "юу",
                            },
                        ),
                    ]),
                    output: "ю",
                },
            ),
            (
                'ょ',
                Node {
                    transitions: None,
                    // transitions: Some(vec![
                    //     (
                    //         'う',
                    //         Node {
                    //             transitions: None,
                    //             output: "ьо\u{304}"
                    //         },
                    //     ),
                    // ]),
                    output: "i",
                },
            ),
            (
                'よ',
                Node {
                    // DOUBLE VOWEL
                    transitions: Some(vec![
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "йо",
                            },
                        ),
                    ]),
                    output: "йо",
                },
            ),
            (
                'ら',
                Node {
                    transitions: None,
                    output: "ра",
                },
            ),
            (
                'り',
                Node {
                    transitions: Some(vec![
                        (
                            'ぃ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ち',
                                        Node {
                                            transitions: None,
                                            output: "ріїчі",
                                        },
                                    ),
                                ]),
                                output: "рій",
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: "р\'є",
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: "ря",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "рю",
                                        },
                                    ),
                                ]),
                                output: "рю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "рьо",
                                        },
                                    ),
                                ]),
                                output: "рьо",
                            },
                        ),
                    ]),
                    output: "рі",
                },
            ),
            (
                'る',
                Node {
                    transitions: None,
                    output: "ру",
                },
            ),
            (
                'れ',
                Node {
                    transitions: None,
                    output: "ре",
                },
            ),
            (
                'ろ',
                Node {
                    transitions: Some(vec![
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "ро",
                            },
                        ),
                    ]),
                    output: "ро",
                },
            ),
            (
                'ゎ',
                Node {
                    transitions: None,
                    output: "ва",
                },
            ),
            (
                'わ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ず',
                                        Node {
                                            transitions: None,
                                            output: "ваїдзу",
                                        },
                                    ),
                                ]),
                                output: "вай",
                            },
                        ),
                    ]),
                    output: "ва",
                },
            ),
            (
                'ゐ',
                Node {
                    transitions: None,
                    output: "ві",
                },
            ),
            (
                'ゑ',
                Node {
                    transitions: None,
                    output: "ве",
                },
            ),
            (
                'を',
                Node {
                    transitions: None,
                    output: "во",
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
                                output: "н\'ї",
                            },
                        ),
                        (
                            '「',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "н\'ї",
                                        },
                                    ),
                                    (
                                        'あ',
                                        Node {
                                            transitions: None,
                                            output: "n\'a",
                                        },
                                    ),
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "н\'у",
                                        },
                                    ),
                                    (
                                        'え',
                                        Node {
                                            transitions: None,
                                            output: "н\'є",
                                        },
                                    ),
                                    (
                                        'お',
                                        Node {
                                            transitions: None,
                                            output: "н\'о",
                                        },
                                    ),
                                    (
                                        'や',
                                        Node {
                                            transitions: None,
                                            output: "н\'я",
                                        },
                                    ),
                                    (
                                        'ゆ',
                                        Node {
                                            transitions: None,
                                            output: "н\'ю",
                                        },
                                    ),
                                    (
                                        'よ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "н\'йо",
                                                    },
                                                ),
                                            ]),
                                            output: "н\'йо",
                                        },
                                    ),
                                ]),
                                output: "н\'",
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "н\'у",
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: "н\'є",
                            },
                        ),
                        (
                            'お',
                            Node {
                                transitions: None,
                                output: "н\'о",
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: "н\'я",
                            },
                        ),
                        (
                            'ゆ',
                            Node {
                                transitions: None,
                                output: "н\'ю",
                            },
                        ),
                        (
                            'よ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "н\'йо",
                                        },
                                    ),
                                ]),
                                output: "н\'йо",
                            },
                        ),
                        (
                            'ま',
                            Node {
                                transitions: None,
                                output: "мма",
                            },
                        ),
                        (
                            'み',
                            Node {
                                transitions: Some(vec![
                                (
                                    'ゃ',
                                    Node {
                                            transitions: None,
                                            output: "ммя"
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ммю"
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ммьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ммьо"
                                        },
                                    )
                                ],),
                                output: "ммі",
                            },
                        ),
                        (
                            'む',
                            Node {
                                transitions: None,
                                output: "мму",
                            },
                        ),
                        (
                            'め',
                            Node {
                                transitions: None,
                                output: "мме",
                            },
                        ),
                        (
                            'も',
                            Node {
                                transitions: None,
                                output: "ммо",
                            },
                        ),
                        (
                            'ぱ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "мпай"
                                        }
                                    ),
                                ]),
                                output: "мпа"
                            },
                        ),
                        (
                            'ぴ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "мпя"
                                        }
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "мпю"
                                        }
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "мпьо",
                                                    },
                                                ),
                                            ]),
                                            output: "мпьо"
                                        }
                                    ),
                                ]),
                                output: "мпі"
                            },
                        ),
                        (
                            'ぷ',
                            Node {
                                transitions: None,
                                output: "мпу"
                            },
                        ),
                        (
                            'ぺ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "мпей",
                                        },
                                    ),
                                ]),
                                output: "мпе"
                            },
                        ),
                        (
                            'ぽ',
                            Node {
                                transitions: None,
                                output: "мпо"
                            },
                        ),
                        (
                            'ば',
                            Node {
                                transitions: None,
                                output: "мба"
                            },
                        ),
                        (
                            'び',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "мбя"
                                        }
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "мбю"
                                        }
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "мбьо",
                                                    },
                                                ),
                                            ]),
                                            output: "мбьо"
                                        }
                                    ),
                                ]),
                                output: "мбі"
                            },
                        ),
                        (
                            'ぶ',
                            Node {
                                transitions: None,
                                output: "мбу"
                            },
                        ),
                        (
                            'べ',
                            Node {
                                transitions: None,
                                output: "мбе"
                            },
                        ),
                        (
                            'ぼ',
                            Node {
                                transitions: None,
                                output: "мбо"
                            },
                        ),
                        (
                            'ぼ',
                            Node {
                                transitions: None,
                                output: "мбо"
                            },
                        ),
                    ]),
                    output: "н",
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
                                output: "вій",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "в\'є",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "вя",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "вю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "вьо",
                                        },
                                    ),
                                ]),
                                output: "вьо",
                            },
                        ),
                    ]),
                    output: "ву",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "ва",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "ві",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: Some(vec![
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "вей",
                            },
                        ),
                    ]),
                    output: "ве",
                },
            ),
            (
                'ゔ',
                Node {
                    transitions: None,
                    output: "во",
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
                                output: "а",
                            },
                        ),
                        (
                            'あ',
                            Node {
                                transitions: None,
                                output: "а",
                            },
                        ),
                        (
                            'ぃ',
                            Node {
                                transitions: None,
                                output: "і",
                            },
                        ),
                        (
                            'い',
                            Node {
                                transitions: None,
                                output: "і",
                            },
                        ),
                        (
                            'ぅ',
                            Node {
                                transitions: None,
                                output: "у",
                            },
                        ),
                        (
                            'う',
                            Node {
                                transitions: None,
                                output: "у",
                            },
                        ),
                        (
                            'ぇ',
                            Node {
                                transitions: None,
                                output: "е",
                            },
                        ),
                        (
                            'え',
                            Node {
                                transitions: None,
                                output: "е",
                            },
                        ),
                        (
                            'ぉ',
                            Node {
                                transitions: None,
                                output: "о",
                            },
                        ),
                        (
                            'お',
                            Node {
                                transitions: None,
                                output: "о",
                            },
                        ),
                        (
                            'か',
                            Node {
                                // transitions: None,
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "ккай"
                                        }
                                    )
                                ]),
                                output: "кка",
                            },
                        ),
                        (
                            'が',
                            Node {
                                transitions: None,
                                output: "ґґа",
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
                                            output: "ккій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "кк\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ккя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ккю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ккьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ккьо",
                                        },
                                    ),
                                ]),
                                output: "ккі",
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
                                            output: "ґґій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "ґґ\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ґґя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ґґю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ґґьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ґґьо",
                                        },
                                    ),
                                ]),
                                output: "ґґі",
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
                                            output: "ккій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "кк\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ккя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ккю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ккьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ккьо",
                                        },
                                    ),
                                ]),
                                output: "кку",
                            },
                        ),
                        (
                            'ぐ',
                            Node {
                                transitions: None,
                                output: "ґґу",
                            },
                        ),
                        (
                            'け',
                            Node {
                                transitions: None,
                                output: "кке",
                            },
                        ),
                        (
                            'げ',
                            Node {
                                transitions: None,
                                output: "ґґе",
                            },
                        ),
                        (
                            'こ',
                            Node {
                                transitions: None,
                                output: "кко",
                            },
                        ),
                        (
                            'ご',
                            Node {
                                transitions: None,
                                output: "ґґо",
                            },
                        ),
                        (
                            'さ',
                            Node {
                                transitions: None,
                                output: "сса",
                            },
                        ),
                        (
                            'ざ',
                            Node {
                                transitions: None,
                                output: "ддза",
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
                                            output: "шшій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "шш\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "шшя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "шшю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "шшьо",
                                                    },
                                                ),
                                            ]),
                                            output: "шшьо",
                                        },
                                    ),
                                ]),
                                output: "шші",
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
                                            output: "дджій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "ддж\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "дджя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "дджю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "дджьо",
                                                    },
                                                ),
                                            ]),
                                            output: "дджьо",
                                        },
                                    ),
                                ]),
                                output: "дджі",
                            },
                        ),
                        (
                            'す',
                            Node {
                                transitions: None,
                                output: "ссу",
                            },
                        ),
                        (
                            'ず',
                            Node {
                                transitions: None,
                                output: "ддзу",
                            },
                        ),
                        (
                            'せ',
                            Node {
                                transitions: None,
                                output: "ссе",
                            },
                        ),
                        (
                            'ぜ',
                            Node {
                                transitions: None,
                                output: "ддзе",
                            },
                        ),
                        (
                            'そ',
                            Node {
                                transitions: None,
                                output: "ссо",
                            },
                        ),
                        (
                            'ぞ',
                            Node {
                                transitions: None,
                                output: "ддзо",
                            },
                        ),
                        (
                            'た',
                            Node {
                                transitions: None,
                                output: "тта",
                            },
                        ),
                        (
                            'だ',
                            Node {
                                transitions: None,
                                output: "дда",
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
                                            output: "ччій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "чч\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ччя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ччю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ччьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ччьо",
                                        },
                                    ),
                                ]),
                                output: "ччі",
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
                                            output: "дджій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "ддж\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "дджя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "дджю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "дджьо",
                                                    },
                                                ),
                                            ]),
                                            output: "дджьо",
                                        },
                                    ),
                                ]),
                                output: "дджі",
                            },
                        ),
                        (
                            'つ',
                            Node {
                                transitions: None,
                                output: "ццу",
                            },
                        ),
                        (
                            'づ',
                            Node {
                                transitions: None,
                                output: "ддзу",
                            },
                        ),
                        (
                            'て',
                            Node {
                                transitions: None,
                                output: "тте",
                            },
                        ),
                        (
                            'で',
                            Node {
                                transitions: None,
                                output: "дде",
                            },
                        ),
                        (
                            'と',
                            Node {
                                transitions: None,
                                output: "тто",
                            },
                        ),
                        (
                            'ど',
                            Node {
                                transitions: None,
                                output: "ддо",
                            },
                        ),
                        (
                            'な',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "най",
                                        },
                                    ),
                                ]),
                                output: "на",
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
                                            output: "ній",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "н\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ня",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ню",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            transitions: None,
                                            output: "нью",
                                        },
                                    ),
                                ]),
                                output: "ні",
                            },
                        ),
                        (
                            'ぬ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "нуй",
                                        },
                                    ),
                                ]),
                                output: "ну",
                            },
                        ),
                        (
                            'ね',
                            Node {
                                transitions: None,
                                output: "не",
                            },
                        ),
                        (
                            'の',
                            Node {
                                transitions: None,
                                output: "но",
                            },
                        ),
                        (
                            'は',
                            Node {
                                transitions: None,
                                output: "хха",
                            },
                        ),
                        (
                            'ば',
                            Node {
                                transitions: None,
                                output: "бба",
                            },
                        ),
                        (
                            'ぱ',
                            Node {
                                transitions: None,
                                output: "ппа",
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
                                            output: "ххій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "хх\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ххя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ххю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ххьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ххьо",
                                        },
                                    ),
                                ]),
                                output: "ххі",
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
                                            output: "ббій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "бб\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ббя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ббю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ббьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ббьо",
                                        },
                                    ),
                                ]),
                                output: "ббі",
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
                                            output: "ппій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "пп\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ппя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ппю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ппьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ппьо",
                                        },
                                    ),
                                ]),
                                output: "ппі",
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
                                            output: "ффій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "фф\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ффя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ффю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ффьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ффьо",
                                        },
                                    ),
                                ]),
                                output: "ффу",
                            },
                        ),
                        (
                            'ぶ',
                            Node {
                                transitions: None,
                                output: "ббу",
                            },
                        ),
                        (
                            'ぷ',
                            Node {
                                transitions: None,
                                output: "ппу",
                            },
                        ),
                        (
                            'へ',
                            Node {
                                transitions: None,
                                output: "ххе",
                            },
                        ),
                        (
                            'べ',
                            Node {
                                transitions: None,
                                output: "ббе",
                            },
                        ),
                        (
                            'ぺ',
                            Node {
                                transitions: None,
                                output: "ппе",
                            },
                        ),
                        (
                            'ほ',
                            Node {
                                transitions: None,
                                output: "ххо",
                            },
                        ),
                        (
                            'ぼ',
                            Node {
                                transitions: None,
                                output: "ббо",
                            },
                        ),
                        (
                            'ぽ',
                            Node {
                                transitions: None,
                                output: "ппо",
                            },
                        ),
                        (
                            'ま',
                            Node {
                                transitions: None,
                                output: "мма",
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
                                            output: "ммій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "мм\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ммя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ммю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ммьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ммьо",
                                        },
                                    ),
                                ]),
                                output: "ммі",
                            },
                        ),
                        (
                            'む',
                            Node {
                                transitions: None,
                                output: "мму",
                            },
                        ),
                        (
                            'め',
                            Node {
                                transitions: None,
                                output: "мме",
                            },
                        ),
                        (
                            'も',
                            Node {
                                transitions: None,
                                output: "ммо",
                            },
                        ),
                        (
                            'ゃ',
                            Node {
                                transitions: None,
                                output: "я",
                            },
                        ),
                        (
                            'や',
                            Node {
                                transitions: None,
                                output: "я",
                            },
                        ),
                        (
                            'ゅ',
                            Node {
                                transitions: None,
                                output: "ю",
                            },
                        ),
                        (
                            'ゆ',
                            Node {
                                transitions: Some(vec![
                                    (
                                        'い',
                                        Node {
                                            transitions: None,
                                            output: "юй",
                                        },
                                    ),
                                ]),
                                output: "ю",
                            },
                        ),
                        (
                            'ょ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "ьо",
                                        },
                                    ),
                                ]),
                                output: "ьо",
                            },
                        ),
                        (
                            'よ',
                            Node {
                                // DOUBLE VOWEL
                                transitions: Some(vec![
                                    (
                                        'う',
                                        Node {
                                            transitions: None,
                                            output: "ьо",
                                        },
                                    ),
                                ]),
                                output: "ьо",
                            },
                        ),
                        (
                            'ら',
                            Node {
                                transitions: None,
                                output: "рра",
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
                                            output: "ррій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "рр\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "рря",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ррю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ррьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ррьо",
                                        },
                                    ),
                                ]),
                                output: "ррі",
                            },
                        ),
                        (
                            'る',
                            Node {
                                transitions: None,
                                output: "рру",
                            },
                        ),
                        (
                            'れ',
                            Node {
                                transitions: None,
                                output: "рре",
                            },
                        ),
                        (
                            'ろ',
                            Node {
                                transitions: None,
                                output: "рро",
                            },
                        ),
                        (
                            'わ',
                            Node {
                                transitions: None,
                                output: "вва",
                            },
                        ),
                        (
                            'ゐ',
                            Node {
                                transitions: None,
                                output: "вві",
                            },
                        ),
                        (
                            'ゑ',
                            Node {
                                transitions: None,
                                output: "вве",
                            },
                        ),
                        (
                            'を',
                            Node {
                                transitions: None,
                                output: "вво",
                            },
                        ),
                        (
                            'ん',
                            Node {
                                transitions: None,
                                output: "н",
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
                                            output: "ввій",
                                        },
                                    ),
                                    (
                                        'ぇ',
                                        Node {
                                            transitions: None,
                                            output: "вв\'є",
                                        },
                                    ),
                                    (
                                        'ゃ',
                                        Node {
                                            transitions: None,
                                            output: "ввя",
                                        },
                                    ),
                                    (
                                        'ゅ',
                                        Node {
                                            transitions: None,
                                            output: "ввю",
                                        },
                                    ),
                                    (
                                        'ょ',
                                        Node {
                                            // DOUBLE VOWEL
                                            transitions: Some(vec![
                                                (
                                                    'う',
                                                    Node {
                                                        transitions: None,
                                                        output: "ввьо",
                                                    },
                                                ),
                                            ]),
                                            output: "ввьо",
                                        },
                                    ),
                                ]),
                                output: "вву",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "вва",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "вві",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "вве",
                            },
                        ),
                        (
                            'ゔ',
                            Node {
                                transitions: None,
                                output: "вво",
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
        ]);

        let mut node = Node {
            transitions,
            output: "",
        };
        node.sort();
        node
    };
}
