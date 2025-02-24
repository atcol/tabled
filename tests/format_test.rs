// Copyright (c) 2021 Maxim Zhiburt
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

use tabled::{multiline, table, Cell, Column, Format, Full, Head, Object, Row, Style, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn formatting_full_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        "+------+----------------+-----------------------------+\n",
        "| [id] | [destribution] |           [link]            |\n",
        "+------+----------------+-----------------------------+\n",
        "| [0]  |    [Fedora]    |  [https://getfedora.org/]   |\n",
        "+------+----------------+-----------------------------+\n",
        "| [2]  |   [OpenSUSE]   | [https://www.opensuse.org/] |\n",
        "+------+----------------+-----------------------------+\n",
        "| [3]  | [Endeavouros]  | [https://endeavouros.com/]  |\n",
        "+------+----------------+-----------------------------+\n",
    );

    let table = table!(&data, Format(Full, |s| { format!("[{}]", s) }));

    assert_eq!(table, expected);
}

#[test]
fn formatting_head_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        "| :id | :destribution |           :link           |\n",
        "|-----+---------------+---------------------------|\n",
        "|  0  |    Fedora     |  https://getfedora.org/   |\n",
        "|  2  |   OpenSUSE    | https://www.opensuse.org/ |\n",
        "|  3  |  Endeavouros  | https://endeavouros.com/  |\n",
    );

    let table = table!(
        &data,
        Style::github_markdown(),
        Format(Head, |s| { format!(":{}", s) }),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_row_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        " id  | destribution  |            link             \n",
        "-----+---------------+-----------------------------\n",
        " <0> |   <Fedora>    |  <https://getfedora.org/>   \n",
        " <2> |  <OpenSUSE>   | <https://www.opensuse.org/> \n",
        " <3> | <Endeavouros> | <https://endeavouros.com/>  \n",
    );

    let table = table!(
        &data,
        Style::psql(),
        Format(Row(1..), |s| { format!("<{}>", s) }),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_column_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        " (x) id | destribution |           link            \n",
        "--------+--------------+---------------------------\n",
        " (x) 0  |    Fedora    |  https://getfedora.org/   \n",
        " (x) 2  |   OpenSUSE   | https://www.opensuse.org/ \n",
        " (x) 3  | Endeavouros  | https://endeavouros.com/  \n",
    );

    let table = table!(
        &data,
        Style::psql(),
        Format(Column(..1), |s| { format!("(x) {}", s) }),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_multiline_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 1,
            destribution: "Open\nSUSE",
            link: "https\n://\nwww.opensuse.org/",
        },
        Linux {
            id: 2,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
        Linux {
            id: 3,
            destribution: "Red\nHat\nEnterprise",
            link: "https://redhat.com/",
        },
    ];

    let expected = concat!(
        " (x) id | (x) destribution |           (x) link           \n",
        "--------+------------------+------------------------------\n",
        " (x) 0  |    (x) Fedora    |  (x) https://getfedora.org/  \n",
        " (x) 1  |     (x) Open     |          (x) https           \n",
        "        |     (x) SUSE     |           (x) ://            \n",
        "        |                  |    (x) www.opensuse.org/     \n",
        " (x) 2  | (x) Endeavouros  | (x) https://endeavouros.com/ \n",
        " (x) 3  |     (x) Red      |   (x) https://redhat.com/    \n",
        "        |     (x) Hat      |                              \n",
        "        |  (x) Enterprise  |                              \n",
    );

    let table = table!(
        &data,
        Style::psql(),
        Format(Full, multiline(|s| { format!("(x) {}", s) })),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_cell_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        " (x) id | (x) destribution |         (x) link          \n",
        "--------+------------------+---------------------------\n",
        "   0    |      Fedora      |  https://getfedora.org/   \n",
        "   2    |     OpenSUSE     | https://www.opensuse.org/ \n",
        "   3    |   Endeavouros    | https://endeavouros.com/  \n",
    );

    let table = table!(
        &data,
        Style::psql(),
        Format(Cell(0, 0), |s| { format!("(x) {}", s) }),
        Format(Cell(0, 1), |s| { format!("(x) {}", s) }),
        Format(Cell(0, 2), |s| { format!("(x) {}", s) }),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_and_combination_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        " (x) id | (x) destribution |         (x) link          \n",
        "--------+------------------+---------------------------\n",
        " (x) 0  |      Fedora      |  https://getfedora.org/   \n",
        " (x) 2  |     OpenSUSE     | https://www.opensuse.org/ \n",
        " (x) 3  |   Endeavouros    | https://endeavouros.com/  \n",
    );

    let table = table!(
        &data,
        Style::psql(),
        Format(Column(..1).and(Row(..1)), |s| { format!("(x) {}", s) }),
    );

    assert_eq!(table, expected);
}

#[test]
fn formatting_not_combination_test() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        "  id   | (x) destribution |         (x) link          \n",
        "-------+------------------+---------------------------\n",
        " (x) 0 |      Fedora      |  https://getfedora.org/   \n",
        " (x) 2 |     OpenSUSE     | https://www.opensuse.org/ \n",
        " (x) 3 |   Endeavouros    | https://endeavouros.com/  \n",
    );

    let table = table!(
        &data,
        Style::psql(),
        Format(Column(..1).and(Row(..1)).not(Cell(0, 0)), |s| {
            format!("(x) {}", s)
        }),
    );

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
mod color {

    use super::*;
    use colored::Colorize;

    #[test]
    fn color_column_test() {
        let data = vec![
            Linux {
                id: 0,
                destribution: "Fedora",
                link: "https://getfedora.org/",
            },
            Linux {
                id: 2,
                destribution: "OpenSUSE",
                link: "https://www.opensuse.org/",
            },
            Linux {
                id: 3,
                destribution: "Endeavouros",
                link: "https://endeavouros.com/",
            },
        ];

        let expected = concat!(
            " \u{1b}[31mid\u{1b}[0m | \u{1b}[34mdestribution\u{1b}[0m |           \u{1b}[31mlink\u{1b}[0m            \n",
            "----+--------------+---------------------------\n",
            " \u{1b}[31m0\u{1b}[0m  |    \u{1b}[34mFedora\u{1b}[0m    |  \u{1b}[31mhttps://getfedora.org/\u{1b}[0m   \n",
            " \u{1b}[31m2\u{1b}[0m  |   \u{1b}[34mOpenSUSE\u{1b}[0m   | \u{1b}[31mhttps://www.opensuse.org/\u{1b}[0m \n",
            " \u{1b}[31m3\u{1b}[0m  | \u{1b}[34mEndeavouros\u{1b}[0m  | \u{1b}[31mhttps://endeavouros.com/\u{1b}[0m  \n",
        );

        let table = table!(
            &data,
            Style::psql(),
            Format(Column(..1).and(Column(2..)), |s| { s.red().to_string() }),
            Format(Column(1..2), |s| { s.blue().to_string() }),
        );

        println!("{}", table);

        assert_eq!(table, expected);
    }

    #[test]
    fn color_multiline_test() {
        let data = vec![
            Linux {
                id: 0,
                destribution: "Fedora",
                link: "https://getfedora.org/",
            },
            Linux {
                id: 2,
                destribution: "OpenSUSE",
                link: "https://www.opensuse.org/",
            },
            Linux {
                id: 3,
                destribution: "Endeavouros",
                link: "https://endeavouros.com/",
            },
            Linux {
                id: 4,
                destribution: "Red\nHat\nEnterprise",
                link: "https://redhat.com/",
            },
        ];

        let expected = concat!(
            " \u{1b}[31mid\u{1b}[0m | \u{1b}[34mdestribution\u{1b}[0m |           \u{1b}[32mlink\u{1b}[0m            \n",
            "----+--------------+---------------------------\n",
            " \u{1b}[31m0\u{1b}[0m  |    \u{1b}[34mFedora\u{1b}[0m    |  \u{1b}[32mhttps://getfedora.org/\u{1b}[0m   \n",
            " \u{1b}[31m2\u{1b}[0m  |   \u{1b}[34mOpenSUSE\u{1b}[0m   | \u{1b}[32mhttps://www.opensuse.org/\u{1b}[0m \n",
            " \u{1b}[31m3\u{1b}[0m  | \u{1b}[34mEndeavouros\u{1b}[0m  | \u{1b}[32mhttps://endeavouros.com/\u{1b}[0m  \n",
            " \u{1b}[31m4\u{1b}[0m  |     \u{1b}[34mRed\u{1b}[0m      |    \u{1b}[32mhttps://redhat.com/\u{1b}[0m    \n",
            "    |     \u{1b}[34mHat\u{1b}[0m      |                           \n",
            "    |  \u{1b}[34mEnterprise\u{1b}[0m  |                           \n",
        );

        let table = table!(
            &data,
            Style::psql(),
            Format(Column(..1), multiline(|s| { s.red().to_string() })),
            Format(Column(1..2), multiline(|s| { s.blue().to_string() })),
            Format(Column(2..), multiline(|s| { s.green().to_string() })),
        );

        println!("{}", table);

        assert_eq!(table, expected);
    }
}
