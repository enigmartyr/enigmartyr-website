use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Portraiture {
    pub sketch: Asset,
    pub normal: Asset,
    pub source: &'static str
}

#[derive(Clone, PartialEq)]
pub struct Name {
    pub full: &'static str,
    pub last: &'static str,
}

#[derive(PartialEq)]
pub struct Appointment {
    pub term: (u32, u32),
    pub name: &'static str,
    pub subs: &'static [Appointment]
}

pub const DEAD: u32 = u32::MAX;
pub const NULL: u32 = u32::MIN;

#[derive(Clone, PartialEq)]
pub struct Person {
    pub name: Name,
    pub span: (u32, u32),
    pub portraiture: Option<Portraiture>,
    pub appointment: &'static [Appointment],
}

pub const GEORGE_YEH: Person = Person {
    name: Name {
        full: "“George” Kung-chao Yeh (葉公超)",
        last: "Yeh",
    },
    span: (1904, 1981),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/george_yeh.gif"),
        normal: asset!("assets/portrait/george_yeh.webp"),
        source: "<i>叶公超.</i> Photograph. Accessed September 14, 2024. https://k.sina.cn/article_5765389469_157a4dc9d001017eyq.html"
    }),
    appointment: &[
        Appointment { term: (1919, 1924), name: "<b>Student</b>, Amherst College, Amherst", subs: &[
            Appointment { term: (1919, 1924), name: "Bachelor of Arts, English", subs: &[]}
        ]},
        Appointment { term: (1924, 1926), name: "<b>Student</b>, University of Cambridge, Cambridge", subs: &[
            Appointment { term: (1924, 1926), name: "Master of Arts, Indo-European Linguistics", subs: &[]}
        ]},
        Appointment { term: (1937, 1938), name: "<b>Scholar<b>, ⟨Changsha Temporary University|⟩, Changsha", subs: &[
            Appointment { term: (1897, 1938), name: "<b>Director</b>, Department of Foreign Literature", subs: &[]}
        ]},
        Appointment { term: (1938, 1939), name: "<b>Scholar<b>, ⟨National Southwestern Associated University|Yunnan Normal University⟩, Kunming", subs: &[
            Appointment { term: (1938, 1939), name: "<b>Director</b>, Department of Foreign Literature", subs: &[]}
        ]},
        Appointment { term: (1946, 1978), name: "<b>Bureaucrat<b>, ❰⟨CN|TW⟩❱ Ministry of Foreign Affairs", subs: &[
            Appointment { term: (1946, 1947), name: "<b>Counselor</b>, European Department", subs: &[]},
            Appointment { term: (1946, 1947), name: "<b>Director General</b>, European Department", subs: &[]},
            Appointment { term: (1947, 1949), name: "<b>Minister, Vice</b>", subs: &[]},
            Appointment { term: (1949, 1949), name: "<b>Minister, Deputy</b>", subs: &[]},
            Appointment { term: (1949, 1958), name: "<b>Minister of Foreign Affairs</b>", subs: &[]},
            Appointment { term: (1958, 1961), name: "<b>Ambassador</b>, United States", subs: &[]},
            Appointment { term: (1961, 1978), name: "<b>Minister without Portfolio</b>", subs: &[]},
        ]},
        Appointment { term: (1978, DEAD), name: "<b>Bureaucrat<b>, ❰⟨CN|TW⟩❱ Office of the President", subs: &[
            Appointment { term: (1978, DEAD), name: "<b>Advisor</b>", subs: &[]}
        ]}
    ]
};

pub const JOHANN_BLUNTSCHLI: Person = Person {
    name: Name {
        full: "Johann Kaspar Bluntschli",
        last: "Bluntschli",
    },
    span: (1808, 1881),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/johann_bluntschli.gif"),
        normal: asset!("assets/portrait/johann_bluntschli.webp"),
        source: "<i>Johann Kaspar Bluntschli.</i> Photograph. Accessed September 22, 2024. https://theonlineportraitgallery.com/portrait/johann-kaspar-bluntschli/"
    }),
    appointment: &[
        Appointment { term: (1826, 1827), name: "<b>Student</b>, ⟨Political Institute|University of Zürich⟩, Zürich", subs: &[]},
        Appointment { term: (1827, 1828), name: "<b>Student</b>, University of Berlin, Berlin", subs: &[]},
        Appointment { term: (1828, 1829), name: "<b>Student</b>, ⟨Rhine University|University of Bonn⟩, Bonn", subs: &[
            Appointment { term: (1828, 1829), name: "Doctor of Jurisprudence", subs: &[]}
        ]},
        Appointment { term: (1830, NULL), name: "<b>Scholar</b>, ⟨Political Institute|University of Zürich⟩, Zürich", subs: &[
            Appointment { term: (1830, 1833), name: "<b>Instructor</b>, Roman Law", subs: &[]},
            Appointment { term: (1833, 1836), name: "<b>Professor, Associate</b>, Roman Law", subs: &[]},
            Appointment { term: (1836, NULL), name: "<b>Professor</b>, Roman Law", subs: &[]}
        ]},
        Appointment { term: (1837, 1847), name: "<b>Politician</b>, District (Canton) of Zürich, Switzerland", subs: &[
            Appointment { term: (1844, 1847), name: "<b>President</b>, Great Council", subs: &[]},
        ]},
        Appointment { term: (1848, 1861), name: "<b>Scholar</b>, University of Munich, Germany", subs: &[
            Appointment { term: (1848, 1861), name: "<b>Professor</b>, Constitutional Law", subs: &[]}
        ]},
        Appointment { term: (1861, DEAD), name: "<b>Scholar</b>, Heidelberg University, Heidelberg", subs: &[
            Appointment { term: (1861, DEAD), name: "<b>Professor</b>, Constitutional Law", subs: &[]}
        ]},
        Appointment { term: (1861, 1868), name: "<b>Politician</b>, Upper Chamber, ⟨Grand Duchy of Baden|⟩", subs: &[
            Appointment { term: (1861, 1868), name: "<b>Deputy</b>", subs: &[]},
        ]},
        Appointment { term: (1868, NULL), name: "<b>Politician</b>, ⟨German Customs Union|⟩", subs: &[]},
        Appointment { term: (1873, DEAD), name: "<b>Scholar</b>, Institute of International Law, Ghent", subs: &[]},
    ]
};

pub const FREEMAN_SNOW: Person = Person {
    name: Name {
        full: "Freeman Clarke Snow",
        last: "Snow",
    },
    span: (1841, 1894),
    portraiture: None,
    appointment: &[
        Appointment { term: (1861, 1862), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1861, 1862), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1861, 1862), name: "(OR-01) <b>Private</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1869, 1873), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1869, 1873), name: "Bachelor of Arts, International Law and Constitutional History", subs: &[]}
        ]},
        Appointment { term: (1874, 1876), name: "<b>Scholar</b>, United States Naval Academy, Annapolis", subs: &[
            Appointment { term: (1874, 1876), name: "<b>Professor, Assistant</b>, History", subs: &[]},
            Appointment { term: (1874, 1876), name: "<b>Professor, Assistant</b>, Law", subs: &[]}
        ]},
        Appointment { term: (1876, NULL), name: "<b>Student</b>, France", subs: &[]},
        Appointment { term: (NULL, NULL), name: "<b>Student</b>, Heidelberg University, Heidelberg", subs: &[]},
        Appointment { term: (NULL, 1877), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (NULL, 1877), name: "Bachelor of Laws", subs: &[]},
            Appointment { term: (NULL, 1877), name: "Master of Arts, History", subs: &[]},
            Appointment { term: (NULL, 1877), name: "Doctor of Philosophy, History", subs: &[]}
        ]},
        Appointment { term: (1880, DEAD), name: "<b>Scholar</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1880, 1884), name: "<b>Instructor</b>, Forensics", subs: &[]},
            Appointment { term: (1881, 1883), name: "<b>Instructor</b>, History", subs: &[]},
            Appointment { term: (1886, DEAD), name: "<b>Instructor</b>, International Law", subs: &[]},
            Appointment { term: (1888, 1888), name: "<b>Instructor</b>, French Language", subs: &[]},
            Appointment { term: (1892, 1892), name: "<b>Instructor</b>, English Literature", subs: &[]}
        ]}
    ]
};

pub const HERBERT_HOOVER: Person = Person {
    name: Name {
        full: "Herbert Clark Hoover",
        last: "Hoover",
    },
    span: (1874, 1964),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/herbert_hoover.gif"),
        normal: asset!("assets/portrait/herbert_hoover.webp"),
        source: "<<i>Herbert Hoover.</i> Photograph. Accessed September 18, 2024. https://www.loc.gov/item/2016885717/"
    }),
    appointment: &[
        Appointment { term: (1891, 1895), name: "<b>Student</b>, Stanford University, Stanford", subs: &[
            Appointment { term: (1891, 1895), name: "Bachelor of Science, Geology", subs: &[]}
        ]},
        Appointment { term: (1917, 1918), name: "<b>Bureaucrat</b>, ❰US❱ ⟨United States Food Administration|⟩", subs: &[]},
        Appointment { term: (1921, 1928), name: "<b>Politician</b>, ❰US❱ Department of Commerce", subs: &[
            Appointment { term: (1921, 1928), name: "<b>Secretary of Commerce</b>", subs: &[]}
        ]},
        Appointment { term: (1929, 1933), name: "<b>Politician</b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1929, 1933), name: "<b>President of the United States</b>", subs: &[]}
        ]}
    ]
};

pub const JACOB_JAVITS: Person = Person {
    name: Name {
        full: "Jacob Koppel Javits",
        last: "Javits",
    },
    span: (1904, 1986),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/jacob_javits.gif"),
        normal: asset!("assets/portrait/jacob_javits.webp"),
        source: "<i>Javits, Jacob K. Honorable.</i> Photograph. Accessed September 16, 2024. https://hdl.loc.gov/loc.pnp/hec.21754"
    }),
    appointment: &[
        Appointment { term: (1920, 1923), name: "<b>Student</b>, Columbia University, New York", subs: &[
            Appointment { term: (1920, 1923), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1923, 1926), name: "<b>Student</b>, New York University, New York", subs: &[
            Appointment { term: (1923, 1926), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1927, 1942), name: "<b>Legal Practitioner</b>, “Javits & Javits”", subs: &[]},
        Appointment { term: (1942, 1945), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1942, 1945), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1942, 1945), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1947, 1954), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1947, 1954), name: "<b>Representative</b>, New York", subs: &[]}
        ]},
        Appointment { term: (1955, 1957), name: "<b>Legal Officer</b>, ❰US-NY❱ Office of the Attorney General", subs: &[
            Appointment { term: (1955, 1957), name: "<b>Attorney General</b>", subs: &[]}
        ]},
        Appointment { term: (1957, 1981), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1957, 1981), name: "<b>Senator</b>, New York", subs: &[]}
        ]}
    ]
};

pub const CLIFFORD_CASE_2: Person = Person {
    name: Name {
        full: "Clifford Philip Case Jr.",
        last: "Case",
    },
    span: (1904 , 1982),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/clifford_case_2.gif"),
        normal: asset!("assets/portrait/clifford_case_2.webp"),
        source: "<i>CASE, Clifford Philip.</i> Photograph. Accessed September 17, 2024. https://bioguide.congress.gov/search/bio/c000220"
    }),
    appointment: &[
        Appointment { term: (1921, 1925), name: "<b>Student</b>, Rutgers University, New Brunswick", subs: &[
            Appointment { term: (1921, 1925), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1925, 1928), name: "<b>Student</b>, Columbia University, New York", subs: &[
            Appointment { term: (1925, 1928), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1928, 1953), name: "<b>Legal Practitioner</b>, “Simpson Thacher & Bartlett”", subs: &[]},
        Appointment { term: (1938, 1942), name: "<b>Politician</b>, Municipality (City) of Rahway, New Jersey", subs: &[
            Appointment { term: (1938, 1942), name: "<b>Council Member</b>, Rahway Common Council", subs: &[]}
        ]},
        Appointment { term: (1943, 1945), name: "<b>Politician</b>, ❰US-NJ❱ New Jersey General Assembly", subs: &[
            Appointment { term: (1943, 1945), name: "<b>Assemblyman</b>, Union County", subs: &[]}
        ]},
        Appointment { term: (1944, 1953), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1944, 1953), name: "<b>Representative</b>, New Jersey", subs: &[]}
        ]},
        Appointment { term: (1955, 1979), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1955, 1979), name: "<b>Senator</b>, New Jersey", subs: &[]}
        ]},
        Appointment { term: (1979, DEAD), name: "<b>Legal Practitioner</b>, “Curtis, Mallet-Prevost, Colt & Mosle”", subs: &[]},
        Appointment { term: (1979, DEAD), name: "<b>Scholar</b>, Rutgers University, New Brunswick", subs: &[
            Appointment { term: (1979, DEAD), name: "<b>Lecturer</b>, Politics", subs: &[]}
        ]}
    ]
};

pub const PHILIP_HART: Person = Person {
    name: Name {
        full: "Philip Aloysius Hart",
        last: "Hart",
    },
    span: (1912, 1976),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/philip_hart.gif"),
        normal: asset!("assets/portrait/philip_hart.webp"),
        source: "<i>Philip A. Hart (D-MI).</i> Photograph. Accessed September 17, 2024. https://www.senate.gov/about/images/hart-philip-aloysius.htm"
    }),
    appointment: &[
        Appointment { term: (1931, 1934), name: "<b>Student</b>, Georgetown University, Washington", subs: &[
            Appointment { term: (1931, 1934), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1934, 1937), name: "<b>Student</b>, University of Michigan, Ann Arbor", subs: &[
            Appointment { term: (1934, 1937), name: "Doctor of Jurisprudence", subs: &[]}
        ]},
        Appointment { term: (1938, 1941), name: "<b>Legal Practitioner</b>, “Beaumont, Smith & Harris”", subs: &[]},
        Appointment { term: (1941, 1946), name: "<b>Serviceman</b>, ❰US❱ Department of Defence", subs: &[
            Appointment { term: (1941, 1946), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1941, 1946), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1946, 1949), name: "<b>Legal Practitioner</b>, “Monaghan, Hart & Crawmer”", subs: &[]},
        Appointment { term: (1949, 1952), name: "<b>Bureaucrat</b>, ❰US-MI❱ ⟨Department of Commerce|Department of Licensing and Regulatory Affairs⟩", subs: &[
            Appointment { term: (1949, 1951), name: "<b>Commissioner</b>, ⟨Corporation and Securities Commission|⟩", subs: &[]},
            Appointment { term: (1951, 1952), name: "<b>Director</b>, ⟨Office of Price Stabilization|⟩", subs: &[]}
        ]},
        Appointment { term: (1952, 1953), name: "<b>Legal Officer</b>, ❰US❱ Department of Justice", subs: &[
            Appointment { term: (1952, 1953), name: "<b>U.S. Attorney</b>, District Court for the Eastern District of Michigan", subs: &[]}
        ]},
        Appointment { term: (1953, 1954), name: "<b>Legal Practitioner</b>, ❰US-MI❱ Michigan Office of the Governor", subs: &[
            Appointment { term: (1953, 1954), name: "<b>Legal Advisor</b>", subs: &[]}
        ]},
        Appointment { term: (1955, 1959), name: "<b>Politician</b>, ❰US-MI❱ Michigan Office of the Governor", subs: &[
            Appointment { term: (1955, 1959), name: "<b>Governor, Lieutenant</b>", subs: &[]}
        ]},
        Appointment { term: (1959, DEAD), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1959, DEAD), name: "<b>Senator</b>, Michigan", subs: &[]}
        ]}
    ]
};

pub const HAROLD_HUGHES: Person = Person {
    name: Name {
        full: "Harold Everett Hughes",
        last: "Hughes",
    },
    span: (1922, 1996),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/harold_hughes.gif"),
        normal: asset!("assets/portrait/harold_hughes.webp"),
        source: "<i>Gov. Harold Everett Hughes.</i> Photograph. Accessed September 17, 2024. https://www.nga.org/governor/harold-everett-hughes/"
    }),
    appointment: &[
        Appointment { term: (1942, 1945), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1942, 1945), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1942, 1945), name: "(OR-01) <b>Private</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1958, 1962), name: "<b>Politician</b>, ❰US-IA❱ ⟨Iowa State Commerce Commission|Iowa Utilities Commission⟩", subs: &[
            Appointment { term: (NULL, 1962), name: "<b>Chairman</b>", subs: &[]}
        ]},
        Appointment { term: (1963, 1969), name: "<b>Politician</b>, ❰US-IA❱ Iowa Office of the Governor", subs: &[
            Appointment { term: (1963, 1969), name: "<b>Governor</b>", subs: &[]}
        ]},
        Appointment { term: (1969, 1975), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1969, 1975), name: "<b>Senator</b>, Michigan", subs: &[
                Appointment { term: (NULL, NULL), name: "<b>Chairman</b>, Special Subcommittee on Alcoholism and Narcotics", subs: &[]}
            ]}
        ]},
        Appointment { term: (1975, 1976), name: "<b>Bureaucrat</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1975, 1976), name: "<b>Consultant</b>, Committee on the Judiciary", subs: &[]}
        ]}
    ]
};

pub const DANIEL_INOUYE: Person = Person {
    name: Name {
        full: "Daniel Ken Inouye",
        last: "Inouye",
    },
    span: (1924, 2012),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/daniel_inouye.gif"),
        normal: asset!("assets/portrait/daniel_inouye.webp"),
        source: "<i>Sen. Daniel K. Inouye.</i> Photograph. Accessed September 17, 2024. https://www.nbcnews.com/politics/politics-news/daniel-inouye-senates-most-senior-senator-dies-88-flna1c7655673"
    }),
    appointment: &[
        Appointment { term: (1943, 1947), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1943, NULL), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1943, NULL), name: "(OR-01) <b>Private</b>", subs: &[]},
                Appointment { term: (1944, 1944), name: "(OR-04) <b>Corporal</b>", subs: &[]},
                Appointment { term: (1944, NULL), name: "(OR-05) <b>Sergeant</b>", subs: &[]}
            ]},
            Appointment { term: (NULL, 1947), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (NULL, 1947), name: "(OR-02) <b>Captain</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1947, 1934), name: "<b>Student</b>, University of Hawaiʻi, Mānoa", subs: &[
            Appointment { term: (1931, 1934), name: "Bachelor of Arts, Government and Economics", subs: &[]}
        ]},
        Appointment { term: (1947, 1934), name: "<b>Student</b>, George Washington University, Washington", subs: &[
            Appointment { term: (1931, 1934), name: "Doctor of Jurisprudence", subs: &[]}
        ]},
        Appointment { term: (1958, 1962), name: "<b>Politician</b>, ❰US-IA❱ ⟨Iowa State Commerce Commission|Iowa Utilities Commission⟩", subs: &[
            Appointment { term: (NULL, 1962), name: "<b>Chairman</b>", subs: &[]}
        ]},
        Appointment { term: (1963, 1969), name: "<b>Politician</b>, ❰US-IA❱ Iowa Office of the Governor", subs: &[
            Appointment { term: (1963, 1969), name: "<b>Governor</b>", subs: &[]}
        ]},
        Appointment { term: (1969, 1975), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1969, 1975), name: "<b>Senator</b>, Iowa", subs: &[
                Appointment { term: (NULL, NULL), name: "<b>Chairman</b>, Special Subcommittee on Alcoholism and Narcotics", subs: &[]}
            ]}
        ]},
        Appointment { term: (1975, 1976), name: "<b>Bureaucrat</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1975, 1976), name: "<b>Consultant</b>, Committee on the Judiciary", subs: &[]}
        ]}
    ]
};

pub const EDWARD_KENNEDY: Person = Person {
    name: Name {
        full: "Edward “Ted” Moore Kennedy",
        last: "Kennedy",
    },
    span: (1932, 2009),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/edward_kennedy.gif"),
        normal: asset!("assets/portrait/edward_kennedy.webp"),
        source: "<i>Senator Ted Kennedy.</i> Photograph. Accessed September 17, 2024. https://www.seattlepi.com/local/seattle-history/slideshow/P-I-photos-from-the-1970s-5577.php"
    }),
    appointment: &[
        Appointment { term: (1950, 1951), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[]},
        Appointment { term: (1951, 1953), name: "<b>Serviceman</b>, ❰US❱ Department of Defense", subs: &[
            Appointment { term: (1951, 1953), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1951, NULL), name: "(OR-01) <b>Private</b>", subs: &[]},
                Appointment { term: (NULL, 1953), name: "(OR-02) <b>Private First Class</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1953, 1956), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1953, 1956), name: "Bachelor of Arts, History and Government", subs: &[]}
        ]},
        Appointment { term: (1956, 1959), name: "<b>Student</b>, University of Virginia, Charlottesville", subs: &[
            Appointment { term: (1956, 1959), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1961, 1962), name: "<b>Legal Officer</b>, Suffolk County, Massachusetts", subs: &[
            Appointment { term: (1961, 1962), name: "<b>District Attorney, Assistant</b>", subs: &[]}
        ]},
        Appointment { term: (1962, 2009), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1962, 2009), name: "<b>Senator</b>, Massachusetts", subs: &[
                Appointment { term: (1965, 1981), name: "<b>Chairman</b>, Subcommittee on Refugees and Escapees", subs: &[]},
                Appointment { term: (1971, 1981), name: "<b>Chairman</b>, Subcommittee on Health and Scientific Research", subs: &[]},
                Appointment { term: (1978, 2009), name: "<b>Chairman</b>, Committee on the Judiciary", subs: &[]},
                Appointment { term: (1986, 1995), name: "<b>Chairman</b>, ⟨Committee on Labor and Human Resources|Committee on Health, Education, Labor and Pensions⟩", subs: &[]},
                Appointment { term: (2001, 2003), name: "<b>Chairman</b>, Committee on Health, Education, Labor and Pensions", subs: &[]},
                Appointment { term: (2004, 2006), name: "<b>Chairman</b>, Subcommittee on Immigration, Citizenship and Border Safety", subs: &[]},
                Appointment { term: (2007, DEAD), name: "<b>Chairman</b>, Committee on Health, Education, Labor and Pensions", subs: &[]},
            ]}
        ]}
    ]
};

pub const WALTER_MONDALE: Person = Person {
    name: Name {
        full: "Walter Frederick Mondale",
        last: "Mondale",
    },
    span: (1928, 2021),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/walter_mondale.gif"),
        normal: asset!("assets/portrait/walter_mondale.webp"),
        source: "<i>Vice President Elect Walter Mondale.</i> Photograph. Accessed September 3, 2024. https://www.loc.gov/item/2017658303/"
    }),
    appointment: &[
        Appointment { term: (1948, 1950), name: "<b>Student</b>, Macalester College, Saint Paul", subs: &[]},
        Appointment { term: (1950, 1951), name: "<b>Student</b>, University of Minnesota, Minneapolis", subs: &[
            Appointment { term: (1950, 1951), name: "Bachelor of Arts, Political Science", subs: &[]}
        ]},
        Appointment { term: (1951, 1953), name: "<b>Serviceman</b>, ❰US❱ Department of Defense", subs: &[
            Appointment { term: (1951, 1953), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1951, NULL), name: "(OR-01) <b>Private</b>", subs: &[]},
                Appointment { term: (NULL, 1953), name: "(OR-04) <b>Corporal</b>", subs: &[]},
            ]}
        ]},
        Appointment { term: (1953, 1956), name: "<b>Student</b>, University of Minnesota, Minneapolis", subs: &[
            Appointment { term: (1953, 1956), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1956, 1960), name: "<b>Legal Practitioner</b>, “Larson, Loevinger, Lindquist & Freeman”", subs: &[]},
        Appointment { term: (1960, 1964), name: "<b>Legal Officer</b>, ❰US-MN❱ Office of the Attorney General", subs: &[
            Appointment { term: (1960, 1964), name: "<b>Attorney General</b>", subs: &[]},
        ]},
        Appointment { term: (1964, 1976), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1964, 1976), name: "<b>Senator</b>, Minnesota", subs: &[
                Appointment { term: (1970, 1973), name: "<b>Chairman</b>, Select Committee on Equal Education Opportunity", subs: &[]}
            ]}
        ]},
        Appointment { term: (1977, 1981), name: "<b>Politician</b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1977, 1981), name: "<b>President, Vice</b>", subs: &[]}
        ]},
        Appointment { term: (1993, 1996), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1993, 1996), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1993, 1996), name: "<b>Ambassador</b>, Japan", subs: &[]}
            ]}
        ]}
    ]
};

pub const ADLAI_STEVENSON_3: Person = Person {
    name: Name {
        full: "Adlai Ewing Stevenson III",
        last: "Stevenson",
    },
    span: (1930, 2021),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/adlai_stevenson_3.gif"),
        normal: asset!("assets/portrait/adlai_stevenson_3.webp"),
        source: "<i>Sen. Adlai Stevenson III.</i> Photograph. Accessed September 17, 2024. https://www.loc.gov/item/2017646210/"
    }),
    appointment: &[
        Appointment { term: (1949, 1952), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1949, 1952), name: "Bachelor of Arts, Government", subs: &[]}
        ]},
        Appointment { term: (1952, 1961), name: "<b>Serviceman</b>, ❰US❱ Department of Defense", subs: &[
            Appointment { term: (1952, NULL), name: "<b>Enlisted</b>, Active Component, ❰US❱ Marine Corps", subs: &[
                Appointment { term: (1952, NULL), name: "(OR-01) <b>Private</b>", subs: &[]}
            ]},
            Appointment { term: (NULL, 1954), name: "<b>Commissioned</b>, Active Component, ❰US❱ Marine Corps", subs: &[
                Appointment { term: (NULL, 1954), name: "(OF-01) <b>Lieutenant, First</b>", subs: &[]}
            ]},
            Appointment { term: (1954, 1961), name: "<b>Commissioned</b>, Reserve Component, ❰US❱ Marine Corps", subs: &[
                Appointment { term: (NULL, 1961), name: "(OF-02) <b>Captain</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1954, 1957), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1954, 1957), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1958, 1964), name: "<b>Legal Practitioner</b>, “Brown & Platt”", subs: &[]},
        Appointment { term: (1965, 1967), name: "<b>Politician</b>, ❰US-IL❱ House of Representatives", subs: &[
            Appointment { term: (1965, 1967), name: "<b>Representative</b>", subs: &[]}
        ]},
        Appointment { term: (1967, 1970), name: "<b>Politician</b>, ❰US-IL❱ Office of the State Treasurer", subs: &[
            Appointment { term: (1967, 1970), name: "<b>Treasurer</b>", subs: &[]}
        ]},
        Appointment { term: (1970, 1981), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1970, 1981), name: "<b>Senator</b>, Illinois", subs: &[
                Appointment { term: (NULL, NULL), name: "<b>Chairman</b>, Subcommittee on Science, Technology and Space", subs: &[]},
                Appointment { term: (NULL, NULL), name: "<b>Chairman</b>, Subcommittee on International Finance", subs: &[]},
                Appointment { term: (NULL, NULL), name: "<b>Chairman</b>, Subcommittee on the Collection and Production of Intelligence", subs: &[]},
                Appointment { term: (NULL, NULL), name: "<b>Chairman</b>, Subcommittee on Oil and Gas Production", subs: &[]},
                Appointment { term: (NULL, NULL), name: "<b>Chairman</b>, Select Committee on Ethics", subs: &[]},
            ]}
        ]}
    ]
};

pub const WILLIAM_SYMINGTON_3: Person = Person {
    name: Name {
        full: "William Stuart Symington III",
        last: "Symington",
    },
    span: (1901, 1988),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/william_symington_3.gif"),
        normal: asset!("assets/portrait/william_symington_3.webp"),
        source: "<i>W. Stuart Symington.</i> Photograph. Accessed September 17, 2024. https://www.maxwell.af.mil/News/Display/Article/2937152/freedom-to-serve/"
    }),
    appointment: &[
        Appointment { term: (1918, 1919), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1918, NULL), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1918, NULL), name: "(OR-02) <b>Private First Class</b>", subs: &[]}
            ]},
            Appointment { term: (NULL, NULL), name: "<b>Cadet</b>, Officers Training School, ⟨Camp Zachary Taylor|⟩", subs: &[]},
            Appointment { term: (NULL, 1919), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (NULL, 1919), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1919, 1923), name: "<b>Student</b>, Yale University, New Haven", subs: &[
            Appointment { term: (1919, 1923), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1945, 1946), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Office of War Mobilization|⟩", subs: &[
            Appointment { term: (1945, 1945), name: "<b>Chairman</b>, ⟨Surplus Property Board|⟩", subs: &[]},
            Appointment { term: (1945, 1946), name: "<b>Administrator</b>, ⟨Surplus War Property Administration|⟩", subs: &[]}
        ]},
        Appointment { term: (1946, 1950), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1946, 1947), name: "<b>⟨Assistant Secretary of War for Air|Secretary of the Air Force⟩</b>", subs: &[]},
            Appointment { term: (1947, 1950), name: "<b>Secretary of the Air Force</b>", subs: &[]},
        ]},
        Appointment { term: (1952, 1976), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1952, 1976), name: "<b>Senator</b>, Missouri", subs: &[]}
        ]}
    ]
};

pub const HARRISON_WILLIAMS_2: Person = Person {
    name: Name {
        full: "Harrison “Pete” Arlington Williams Jr.",
        last: "Williams",
    },
    span: (1919, 2001),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/harrison_williams_2.gif"),
        normal: asset!("assets/portrait/harrison_williams_2.webp"),
        source: "<i>Harrison Williams.</i> Photograph. Accessed September 17, 2024. https://www.senate.gov/artandhistory/history/common/image/HarrisonWilliams.htm"
    }),
    appointment: &[
        Appointment { term: (1938, 1941), name: "<b>Student</b>, Oberlin College, Oberlin", subs: &[
            Appointment { term: (1938, 1941), name: "Bachelor of Arts, Economics", subs: &[]}
        ]},
        Appointment { term: (1925, 1929), name: "<b>Student</b>, Georgetown University, Washington", subs: &[]},
        Appointment { term: (1941, 1945), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1941, 1943), name: "<b>Enlisted</b>, Reserve Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1941, 1943), name: "(OR-03) <b>Seaman</b>", subs: &[]}
            ]},
            Appointment { term: (1943, 1945), name: "<b>Commissioned</b>, Reserve Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1943, 1945), name: "(OF-01) <b>Lieutenant Junior Grade</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1946, 1948), name: "<b>Student</b>, Columbia University, New York", subs: &[
            Appointment { term: (1946, 1948), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1953, 1956), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1953, 1956), name: "Bachelor of Arts, History and Government", subs: &[]}
        ]},
        Appointment { term: (1956, 1959), name: "<b>Student</b>, University of Virginia, Charlottesville", subs: &[
            Appointment { term: (1956, 1959), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1951, 1952), name: "<b>Legal Practitioner</b>, “Cox & Walburg”", subs: &[]},
        Appointment { term: (1951, 1952), name: "<b>Scholar</b>, Rutgers University, New Brunswick", subs: &[
            Appointment { term: (1951, 1952), name: "<b>Lecturer</b>, Business Law", subs: &[]}
        ]},
        Appointment { term: (1953, 1957), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1953, 1957), name: "<b>Representative</b>, New Jersey", subs: &[]}
        ]},
        Appointment { term: (1959, 1982), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1959, 1982), name: "<b>Senator</b>, New Jersey", subs: &[
                Appointment { term: (1967, 1971), name: "<b>Chairman</b>, Special Committee on Aging", subs: &[]}
            ]}
        ]}
    ]
};

pub const SAMUEL_WILLIAMS: Person = Person {
    name: Name {
        full: "Samuel Wells Williams",
        last: "Williams",
    },
    span: (1812, 1884),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/samuel_williams.gif"),
        normal: asset!("assets/portrait/samuel_williams.webp"),
        source: "<i>Samuel Wells Williams.</i> Photograph. Accessed February 8, 2025. https://www.facebook.com/photo/?fbid=2454834787955850"
    }),
    appointment: &[
        Appointment { term: (1830, 1832), name: "<b>Student</b>, ⟨Rensselaer Institute|Rensselaer Polytechnic Institute⟩, Troy", subs: &[
            Appointment { term: (1830, 1832), name: "Bachelor of Science", subs: &[]}
        ]},
        Appointment { term: (1853, 1976), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1853, 1976), name: "<b>Diplomat</b>, ❰US❱ ⟨Consular Service|Foreign Service⟩", subs: &[
                Appointment { term: (1853, 1856), name: "<b>Student Interpreter</b>, Tokyo, Japan", subs: &[]},
                Appointment { term: (1856, 1860), name: "<b>Secretary</b>, ⟨Peking|Beijing⟩, China", subs: &[]},
                Appointment { term: (1860, 1876), name: "<b>Chargé d'Affaires</b>, ⟨Peking|Beijing⟩, China", subs: &[]},
            ]},
            Appointment { term: (1877, NULL), name: "<b>Scholar</b>, Yale University, New Haven", subs: &[
                Appointment { term: (1877, NULL), name: "<b>Professor</b>, Chinese Language and Literature", subs: &[]},
            ]}
        ]}
    ]
};

pub const HARRY_TRUMAN: Person = Person {
    name: Name {
        full: "Harry S Truman",
        last: "Truman",
    },
    span: (1884, 1972),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/harry_truman.gif"),
        normal: asset!("assets/portrait/harry_truman.webp"),
        source: "<i>Harry S. Truman.</i> Photograph. Accessed September 15, 2024. https://www.nytimes.com/2022/03/11/books/review/the-trials-of-harry-s-truman-jeffrey-frank.html"
    }),
    appointment: &[
        Appointment { term: (1917, 1953), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1905, 1911), name: "<b>Enlisted</b>, Reserve Component, ❰US❱ Army", subs: &[
                Appointment { term: (1905, 0000), name: "(OR-01) <b>Private</b>", subs: &[]},
                Appointment { term: (0000, 1911), name: "(OR-04) <b>Corporal</b>", subs: &[]}
            ]},
            Appointment { term: (1917, 1919), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1917, 1918), name: "(OF-01) <b>Lieutenant, First</b>", subs: &[]},
                Appointment { term: (1918, 1919), name: "(OF-02) <b>Captain</b>", subs: &[]}
            ]},
            Appointment { term: (1920, 1953), name: "<b>Commissioned</b>, Reserve Component, ❰US❱ Army", subs: &[
                Appointment { term: (1920, 1925), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1925, 1932), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]},
                Appointment { term: (1932, 1953), name: "(OF-05) <b>Colonel</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1922, 1935), name: "<b>Judicial Officer</b>, District of Jackson County, Missouri", subs: &[
            Appointment { term: (1922, 1925), name: "<b>Judge</b>", subs: &[]},
            Appointment { term: (1927, 1935), name: "<b>Judge, Presiding</b>", subs: &[]}
        ]},
        Appointment { term: (1935, 1945), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1935, 1945), name: "<b>Senator</b>, Missouri", subs: &[]}
        ]},
        Appointment { term: (1945, 1953), name: "<b>Politician</b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1945, 1945), name: "<b>President, Vice</b>", subs: &[]},
            Appointment { term: (1945, 1953), name: "<b>President of the United States</b>", subs: &[]}
        ]}
    ]
};

pub const LOUIS_JOHNSON: Person = Person {
    name: Name {
        full: "Louis Arthur Johnson",
        last: "Johnson",
    },
    span: (1891, 1966),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/louis_johnson.gif"),
        normal: asset!("assets/portrait/louis_johnson.webp"),
        source: "<i>Asst. Sec. of War Louis A. Johnson.</i> Photograph. Accessed August 4, 2024. https://www.loc.gov/item/2017828566/"
    }),
    appointment: &[
        Appointment { term: (1908, 1912), name: "<b>Student</b>, University of Virginia, Charlottesville", subs: &[
            Appointment { term: (1908, 1912), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1916, 1940), name: "<b>Politician</b>, ❰US-WV❱ House of Delegates", subs: &[
            Appointment { term: (1916, 1916), name: "<b>Delegate</b>, Harrison County", subs: &[
                Appointment { term: (1916, 1916), name: "<b>Chairman</b>, Judiciary Committee", subs: &[]}
            ]}
        ]},
        Appointment { term: (1917, 0000), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1917, 1917), name: "<b>Cadet</b>, Officers Training School, Fort Benjamin Harrison", subs: &[]},
            Appointment { term: (1917, 1919), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1917, 1919), name: "(OF-02) <b>Captain</b>", subs: &[]}
            ]},
            Appointment { term: (1919, 0000), name: "<b>Commissioned</b>, Reserve Component, ❰US❱ Army", subs: &[
                Appointment { term: (1919, 1928), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1928, 0000), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1937, 1940), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1937, 1940), name: "<b>Secretary, ⟨Assistant|Deputy⟩</b>", subs: &[]},
        ]},
        Appointment { term: (1942, 1942), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1942, 1942), name: "<b>Minister</b> and “Personal Representative of the President”, India", subs: &[]}
        ]},
        Appointment { term: (1942, 1947), name: "<b>Bureaucrat</b>, ❰US❱ Department of Justice", subs: &[
            Appointment { term: (1942, 1947), name: "<b>Clerk</b>, ⟨Office of Alien Property Custodian|⟩", subs: &[]}
        ]},
        Appointment { term: (1949, 1950), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1949, 1950), name: "<b>⟨Secretary of War|Secretary of Defense⟩</b>", subs: &[]}
        ]}
    ]
};

pub const DOUGLAS_MACARTHUR: Person = Person {
    name: Name {
        full: "Douglas MacArthur",
        last: "MacArthur",
    },
    span: (1880, 1964),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/douglas_macarthur.gif"),
        normal: asset!("assets/portrait/douglas_macarthur.webp"),
        source: "<i>Douglas MacArthur.</i> Photograph. Accessed August 4, 2024. https://www.loc.gov/item/2016859437/"
    }),
    appointment: &[
        Appointment { term: (1899, 1951), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1899, 1903), name: "<b>Cadet</b>, United States Military Academy, West Point", subs: &[]},
            Appointment { term: (1903, 1951), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1903, 1904), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]},
                Appointment { term: (1904, 1911), name: "(OF-01) <b>Lieutenant, First</b>", subs: &[]},
                Appointment { term: (1911, 1915), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1915, 1917), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1917, 1918), name: "(OF-05) <b>Colonel</b>", subs: &[]},
                Appointment { term: (1918, 1925), name: "(OF-06) <b>General, Brigadier</b>", subs: &[]},
                Appointment { term: (1925, 1941), name: "(OF-07) <b>General, Major</b>", subs: &[]},
                Appointment { term: (1941, 1941), name: "(OF-08) <b>General, Lieutenant</b>", subs: &[]},
                Appointment { term: (1941, 1944), name: "(OF-09) <b>General</b>", subs: &[]},
                Appointment { term: (1944, 1951), name: "(OF-10) <b>General of the Army</b>", subs: &[]}
            ]},
            Appointment { term: (1919, 1922), name: "<b>Superintendent</b>, United States Military Academy, West Point", subs: &[]},
            Appointment { term: (1930, 1935), name: "<b>Chief of Staff</b>, Department of the Army", subs: &[]},
            Appointment { term: (1935, 1941), name: "<b>Military Advisor</b>, Philippines", subs: &[]},
            Appointment { term: (1945, 1951), name: "<b>Supreme Allied Commander</b>, Allied Powers", subs: &[]},
            Appointment { term: (1950, 1951), name: "<b>Governor</b>, Ryukyu Islands", subs: &[]}
        ]},
    ]
};

pub const OMAR_BRADLEY: Person = Person {
    name: Name {
        full: "Omar Nelson Bradley",
        last: "Bradley",
    },
    span: (1893, 1981),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/omar_bradley.gif"),
        normal: asset!("assets/portrait/omar_bradley.webp"),
        source: "<i>Bradley, Omar, General.</i> Photograph. Accessed August 7, 2024. https://www.loc.gov/resource/hec.21734/"
    }),
    appointment: &[
        Appointment { term: (1911, 1953), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1911, 1915), name: "<b>Cadet</b>, United States Military Academy, West Point", subs: &[]},
            Appointment { term: (1915, 1951), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1915, 1916), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]},
                Appointment { term: (1916, 1917), name: "(OF-01) <b>Lieutenant, First</b>", subs: &[]},
                Appointment { term: (1917, 1918), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1920, 1922), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1922, 1924), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1924, 1936), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1936, 1941), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]},
                Appointment { term: (1941, 1942), name: "(OF-06) <b>General, Brigadier</b>", subs: &[]},
                Appointment { term: (1942, 1943), name: "(OF-07) <b>General, Major</b>", subs: &[]},
                Appointment { term: (1943, 1944), name: "(OF-08) <b>General, Lieutenant</b>", subs: &[]},
                Appointment { term: (1944, 1945), name: "(OF-07) <b>General, Major</b>", subs: &[]},
                Appointment { term: (1945, 1950), name: "(OF-09) <b>General</b>", subs: &[]},
                Appointment { term: (1950, 1953), name: "(OF-10) <b>General of the Army</b>", subs: &[]}
            ]},
            Appointment { term: (1948, 1949), name: "<b>Chief of Staff</b>, ❰US❱ Department of the Army", subs: &[]},
            Appointment { term: (1949, 1951), name: "<b>Chairman</b>, NATO Military Committee", subs: &[]},
            Appointment { term: (1949, 1953), name: "<b>Chairman</b>, Joint Chiefs of Staff", subs: &[]},
        ]}
    ]
};

pub const SIDNEY_SOUERS: Person = Person {
    name: Name {
        full: "Sidney William Souers",
        last: "Souers",
    },
    span: (1892, 1973),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/sidney_souers.gif"),
        normal: asset!("assets/portrait/sidney_souers.webp"),
        source: "<i>Admiral Sidney W. Souers.</i> Photograph. Accessed August 7, 2024. https://www.trumanlibrary.gov/photograph-records/70-5111"
    }),
    appointment: &[
        Appointment { term: (1911, 1912), name: "<b>Student</b>, Purdue University, West Lafayette", subs: &[]},
        Appointment { term: (1913, 1914), name: "<b>Student</b>, Miami University, Ohio", subs: &[
            Appointment { term: (1913, 1914), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1911, 1953), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1929, 1940), name: "<b>Commissioned</b>, Reserve Component, Navy", subs: &[
                Appointment { term: (1929, 1940), name: "(OF-03) <b>Commander, Lieutenant</b>", subs: &[]}
            ]},
            Appointment { term: (1940, 1946), name: "<b>Commissioned</b>, Active Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1940, 1942), name: "(OF-03) <b>Commander, Lieutenant</b>", subs: &[]},
                Appointment { term: (1942, 1945), name: "(OF-04) <b>Commander</b>", subs: &[]},
                Appointment { term: (1945, 1946), name: "(OF-07) <b>⟨Admiral, Rear|Rear Admiral Upper Half⟩</b>", subs: &[]}
            ]},
            Appointment { term: (1944, 1946), name: "<b>Director, Deputy</b>, Office of Naval Intelligence", subs: &[]},
            Appointment { term: (1944, 1946), name: "<b>Chief, Deputy</b>, Office of Naval Intelligence", subs: &[]}
        ]},
        Appointment { term: (1946, 1946), name: "<b>Bureaucrat</b>, ❰US❱ Central Intelligence Agency", subs: &[
            Appointment { term: (1946, 1946), name: "<b>Director</b>, Central Intelligence Group", subs: &[]}
        ]},
        Appointment { term: (1947, 1950), name: "<b>Bureaucrat</b>, ❰US❱ National Security Council", subs: &[
            Appointment { term: (1947, 1950), name: "<b>Secretary, Executive</b>", subs: &[]}
        ]},
        Appointment { term: (1950, 1953), name: "<b>Bureaucrat</b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1950, 1953), name: "<b>Advisor</b>, Military and Foreign Affairs", subs: &[]}
        ]}
    ]
};

pub const ALEXANDER_HABERSHAM: Person = Person {
    name: Name {
        full: "Alexander Wylly Habersham",
        last: "Habersham",
    },
    span: (1826, 1883),
    portraiture: None,
    appointment: &[
        Appointment { term: (1845, 1860), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1845, 1847), name: "<b>Midshipman</b>, United States Naval Academy", subs: &[]},
            Appointment { term: (1847, 1860), name: "<b>Commissioned</b>, Active Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1847, NULL), name: "(OF-01) ⟨<b>Midshipman, Passed</b>|<b>Ensign</b>⟩", subs: &[]},
                Appointment { term: (NULL, 1855), name: "(OF-01) ⟨<b>Master</b>|<b>Lieutenant (Junior Grade)</b>⟩", subs: &[]},
                Appointment { term: (1855, 1860), name: "(OF-02) <b>Lieutenant</b>", subs: &[]},
            ]}
        ]}
    ]
};

pub const MATTHEW_PERRY: Person = Person {
    name: Name {
        full: "Matthew Calbraith Perry",
        last: "Perry",
    },
    span: (1794, 1858),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/matthew_perry.gif"),
        normal: asset!("assets/portrait/matthew_perry.webp"),
        source: "<i>Commodore Matthew Calbraith Perry.</i> Photograph. Accessed August 7, 2024. https://commons.wikimedia.org/wiki/File:Commodore_Matthew_Calbraith_Perry.png"
    }),
    appointment: &[
        Appointment { term: (1809, 1858), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1809, 1855), name: "<b>Commissioned</b>, Active Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1809, 1815), name: "(OF-01) ⟨<b>Midshipman, Passed</b>|<b>Ensign</b>⟩", subs: &[]},
                Appointment { term: (1815, 1824), name: "(OF-02) <b>Lieutenant</b>", subs: &[]},
                Appointment { term: (1824, 1826), name: "(OF-03) ⟨<b>Commandant, Lieutenant</b>|<b>Commander, Lieutenant</b>⟩", subs: &[]},
                Appointment { term: (1826, 1837), name: "(OF-04) ⟨<b>Commandant, Master</b>|<b>Commander</b>⟩", subs: &[]},
                Appointment { term: (1837, 1841), name: "(OF-05) <b>Captain</b>", subs: &[]},
                Appointment { term: (1841, 1855), name: "(OF-06) ⟨<b>Commodore</b>|<b>Rear Admiral (Lower Half)</b>⟩", subs: &[]}
            ]},
            Appointment { term: (1855, 1858), name: "<b>Commissioned</b>, Reserve Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1855, 1858), name: "(OF-07) <b>Rear Admiral</b>", subs: &[]},
            ]}
        ]}
    ]
};

pub const DWIGHT_EISENHOWER: Person = Person {
    name: Name {
        full: "Dwight David Eisenhower",
        last: "Eisenhower",
    },
    span: (1890, 1969),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/dwight_eisenhower.gif"),
        normal: asset!("assets/portrait/dwight_eisenhower.webp"),
        source: "<i>Dwight D. Eisenhower.</i> Photograph. Accessed December 20, 2024. https://www.nato.int/cps/en/natohq/declassified_137961.htm"
    }),
    appointment: &[
        Appointment { term: (1911, 1952), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1911, 1915), name: "<b>Cadet</b>, United States Military Academy", subs: &[]},
            Appointment { term: (1915, 1951), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1915, 1916), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]},
                Appointment { term: (1916, 1917), name: "(OF-01) <b>Lieutenant, First</b>", subs: &[]},
                Appointment { term: (1917, 1918), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1918, 1918), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1918, 1920), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]},
                Appointment { term: (1920, 1920), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1920, 1922), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1922, 1924), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1924, 1936), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1936, 1941), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]},
                Appointment { term: (1941, 1941), name: "(OF-05) <b>Colonel</b>", subs: &[]},
                Appointment { term: (1941, 1942), name: "(OF-06) <b>General, Brigadier</b>", subs: &[]},
                Appointment { term: (1942, 1942), name: "(OF-07) <b>General, Major</b>", subs: &[]},
                Appointment { term: (1942, 1943), name: "(OF-08) <b>General, Lieutenant</b>", subs: &[]},
                Appointment { term: (1943, 1944), name: "(OF-09) <b>General</b>", subs: &[]},
                Appointment { term: (1944, 1946), name: "(OF-10) <b>General of the Army</b>", subs: &[]}
            ]},
            Appointment { term: (1945, 1948), name: "<b>Chief of Staff</b>, ❰US❱ Army", subs: &[]},
            Appointment { term: (1951, 1952), name: "<b>Supreme Allied Commander</b>, NATO", subs: &[]}
        ]},
        Appointment { term: (1953, 1961), name: "<b>Politician</b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1953, 1961), name: "<b>President of the United States</b>", subs: &[]}
        ]}
    ]
};

pub const JOHN_SPARKMAN: Person = Person {
    name: Name {
        full: "John Jackson Sparkman",
        last: "Sparkman",
    },
    span: (1899, 1985),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/john_sparkman.gif"),
        normal: asset!("assets/portrait/john_sparkman.webp"),
        source: "<i>John Jackson Sparkman.</i> Photograph. Accessed August 4, 2024. https://www.loc.gov/item/2016877618/"
    }),
    appointment: &[
        Appointment { term: (1917, 1924), name: "<b>Student</b>, University of Alabama, Tuscaloosa", subs: &[
            Appointment { term: (1917, 1921), name: "Bachelor of Arts, History", subs: &[]},
            Appointment { term: (1921, 1923), name: "Bachelor of Laws", subs: &[]},
            Appointment { term: (1923, 1924), name: "Master of Arts, History", subs: &[]}
        ]},
        Appointment { term: (1937, 1946), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1937, 1946), name: "<b>Representative</b>, Alabama", subs: &[]},
        ]},
        Appointment { term: (1946, 1979), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1946, 1979), name: "<b>Senator</b>, Alabama", subs: &[
                Appointment { term: (1955, 1967), name: "<b>Chairman</b>, Committee on ⟨Small Business|Small Business and Entrepreneurship⟩", subs: &[]},
                Appointment { term: (1967, 1975), name: "<b>Chairman</b>, Committee on ⟨Banking|Banking, Housing, and Urban Affairs⟩", subs: &[]},
                Appointment { term: (1975, 1979), name: "<b>Chairman</b>, Committee on Foreign Relations", subs: &[]},
            ]}
        ]}
    ]
};

pub const HOWARD_SMITH: Person = Person {
    name: Name {
        full: "Howard Alexander Smith",
        last: "Smith",
    },
    span: (1880, 1966),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/howard_smith.gif"),
        normal: asset!("assets/portrait/howard_smith.webp"),
        source: "<i>Howard Alexander Smith.</i> Photograph. Accessed August 4, 2024. https://bioguide.congress.gov/search/bio/S000553"
    }),
    appointment: &[
        Appointment { term: (1897, 1901), name: "<b>Student</b>, Princeton University, Princeton", subs: &[
            Appointment { term: (1897, 1901), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1901, 1904), name: "<b>Student</b>, Columbia University, New York", subs: &[
            Appointment { term: (1901, 1904), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1897, 1901), name: "<b>Scholar<b>, Princeton University, Princeton", subs: &[
            Appointment { term: (1820, 1927), name: "<b>Secretary, Executive</b>", subs: &[]},
            Appointment { term: (1927, 1930), name: "<b>Lecturer</b>, Politics", subs: &[]}
        ]},
        Appointment { term: (1944, 1959), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1944, 1959), name: "<b>Senator</b>, New Jersey", subs: &[
                Appointment { term: (1953, 1955), name: "<b>Chairman</b>, ⟨Committee on Labor and Public Welfare|Committee on Heath, Education, Labor, and Pensions⟩", subs: &[]}
            ]}
        ]}
    ]
};

pub const JULIUS_HOLMES: Person = Person {
    name: Name {
        full: "Julius Cecil Holmes",
        last: "Holmes",
    },
    span: (1899, 1968),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/julius_holmes.gif"),
        normal: asset!("assets/portrait/julius_holmes.webp"),
        source: "<i>Julius Cecil Holmes.</i> Photograph. Accessed August 4, 2024. https://commons.wikimedia.org/wiki/File:Julius_C._Holmes.jpg"
    }),
    appointment: &[
        Appointment { term: (1917, 1922), name: "<b>Student</b>, University of Kansas, Lawrence", subs: &[
            Appointment { term: (1917, 1922), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1925, 1937), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1925, 1937), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1925, 1927), name: "<b>Consul, Vice</b>, Marseille, France", subs: &[]},
                Appointment { term: (1927, 1929), name: "<b>Consul, Vice</b>, Izmir, Turkey", subs: &[]},
                Appointment { term: (1929, 1930), name: "<b>Consul, Vice</b>, Tirana, Albania", subs: &[]},
                Appointment { term: (1929, 1930), name: "<b>Secretary, Third</b>, Tirana, Albania", subs: &[]},
                Appointment { term: (1930, 1934), name: "<b>Secretary, Third</b>, Bucharest, Romania", subs: &[]},
            ]},
            Appointment { term: (1934, 1937), name: "<b>Chief, Assistant</b>, Office of the Chief of Protocol", subs: &[]}
        ]},
        Appointment { term: (1942, 1945), name: "<b>Serviceman</b>, ❰US❱ Department of Defence", subs: &[
            Appointment { term: (1942, 1945), name: "<b>Commissioned</b>, Reserve Component, ❰US❱ Army", subs: &[
                Appointment { term: (1942, 1942), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1942, 1942), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]},
                Appointment { term: (1942, 1943), name: "(OF-05) <b>Colonel</b>", subs: &[]},
                Appointment { term: (1943, 1945), name: "(OF-06) <b>General, Brigadier</b>",   subs: &[]}
            ]}
        ]},
        Appointment { term: (1948, 1965), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1948, 1965), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1948, 1954), name: "<b>Minister</b>, London, United Kingdom", subs: &[]},
                Appointment { term: (1948, 1954), name: "<b>Special Assistant</b>, ⟨Bureau of European Affairs|Bureau of European and Eurasian Affairs⟩", subs: &[]},
                Appointment { term: (1955, 1956), name: "<b>Consul General</b>, Tangier, Morocco", subs: &[]},
                Appointment { term: (1956, 1959), name: "<b>Special Assistant</b>, Office of the Secretary of State", subs: &[]},
                Appointment { term: (1959, 1961), name: "<b>Minister</b>, Hong Kong", subs: &[]},
                Appointment { term: (1959, 1961), name: "<b>Consul General</b>, Hong Kong", subs: &[]},
                Appointment { term: (1959, 1961), name: "<b>Consul General</b>, Macau", subs: &[]},
                Appointment { term: (1961, 1965), name: "<b>Ambassador</b>, Iran", subs: &[]}
            ]}
        ]}
    ]
};

pub const JOHN_DULLES: Person = Person {
    name: Name {
        full: "John Foster Dulles",
        last: "Dulles",
    },
    span: (1888, 1959),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/julius_holmes.gif"),
        normal: asset!("assets/portrait/julius_holmes.webp"),
        source: "<i>John Foster Dulles.</i> Photograph. Accessed August 4, 2024. https://bioguide.congress.gov/search/bio/D000522"
    }),
    appointment: &[
        Appointment { term: (1917, 1919), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defence⟩", subs: &[
            Appointment { term: (1917, 1919), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1917, 1918), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1918, 1919), name: "(OF-03) <b>Major</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1949, 1949), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1949, 1949), name: "<b>Senator</b>, New York", subs: &[]}
        ]},
        Appointment { term: (1950, 1959), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1950, 1950), name: "<b>Consultant</b>", subs: &[]},
            Appointment { term: (1950, 1951), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1950, 1951), name: "<b>Ambassador-at-large</b>", subs: &[]}
            ]},
            Appointment { term: (1953, 1959), name: "<b>Secretary of State</b>", subs: &[]}
        ]}
    ]
};

pub const KENNETH_KRENTZ: Person = Person {
    name: Name {
        full: "Kenneth Carl Krentz",
        last: "Krentz",
    },
    span: (1899, 1962),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/kenneth_krentz.gif"),
        normal: asset!("assets/portrait/kenneth_krentz.webp"),
        source: "<i>Kenneth C. Krentz.</i> Photograph. Accessed August 8, 2024. https://www.newspapers.com/image/300962762/"
    }),
    appointment: &[
        Appointment { term: (1918, 1919), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defence⟩", subs: &[
            Appointment { term: (1918, 1919), name: "<b>Enlisted</b>, Reserve Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1918, 1919), name: "(OR-01) <b>⟨Seaman, Apprentice|Seaman, Recruit⟩</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1926, 1943), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1926, 1943), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1926, 1929), name: "<b>Clerk</b>, Hong Kong, ⟨United Kingdom|China⟩", subs: &[]},
                Appointment { term: (1929, 1932), name: "<b>Consul, Vice</b>, Hong Kong, ⟨United Kingdom|China⟩", subs: &[]},
                Appointment { term: (1932, 1937), name: "<b>Consul, Vice</b>, Kobe, Japan", subs: &[]},
                Appointment { term: (1937, 1938), name: "<b>Consul</b>, Osaka, Japan", subs: &[]},
                Appointment { term: (1938, 1940), name: "<b>Consul</b>, ⟨Canton|Guangdong⟩, China", subs: &[]},
                Appointment { term: (1940, 1942), name: "<b>Consul</b>, ⟨Mukden|Shenyang⟩, China", subs: &[]},
                Appointment { term: (1942, 1943), name: "<b>Consul</b>, ⟨Bombay|Mumbai⟩, India", subs: &[]},
            ]},
        ]},
        Appointment { term: (1943, 1944), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Office of Strategic Services|Central Intelligence Agency⟩", subs: &[
            Appointment { term: (1943, 1944), name: "<b>Assistant</b>, Board of Research and Analysis", subs: &[]}
        ]},
        Appointment { term: (1944, 1957), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1944, 1946), name: "<b>Chief, Assistant</b>, Division of Foreign Service Administration", subs: &[]},
            Appointment { term: (1946, 1949), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1946, 1947), name: "<b>Consul General</b>, ⟨Hankow|Hankou⟩, China", subs: &[]},
                Appointment { term: (1947, 1949), name: "<b>Consul General</b>, Taipei, ⟨Republic of China|Taiwan⟩", subs: &[]}
            ]},
            Appointment { term: (1949, 1953), name: "<b>Clerk</b>, Policy Planning Staff", subs: &[]},
            Appointment { term: (1953, 1957), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1953, 1957), name: "<b>Consul General</b>, London, England", subs: &[]},
            ]}
        ]}
    ]
};

pub const WALTER_GEORGE: Person = Person {
    name: Name {
        full: "Walter Franklin George",
        last: "George",
    },
    span: (1878, 1957),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/walter_george.webp"),
        sketch: asset!("assets/portrait/walter_george.gif"),
        source: "<i>Walter Franklin George.</i> Photograph. Accessed August 4, 2024. https://bioguide.congress.gov/search/bio/G000131"
    }),
    appointment: &[
        Appointment { term: (1922, 1957), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1922, 1957), name: "<b>Senator</b>, Georgia", subs: &[
                Appointment { term: (1940, 1942), name: "<b>Chairman</b>, Committee on Foreign Relations", subs: &[]},
                Appointment { term: (1941, 1953), name: "<b>Chairman</b>, Committee on Finance", subs: &[]},
                Appointment { term: (1955, 1957), name: "<b>Chairman</b>, Committee on Foreign Relations", subs: &[]},
                Appointment { term: (1955, 1957), name: "<b>President pro tempore</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1957, DEAD), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1957, DEAD), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1957, DEAD), name: "<b>Ambassador-at-large</b>, NATO", subs: &[]}
            ]}
        ]}
    ]
};

pub const MICHAEL_MANSFIELD: Person = Person {
    name: Name {
        full: "Michael Joseph Mansfield",
        last: "Mansfield",
    },
    span: (1903, 2001),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/michael_mansfield.webp"),
        sketch: asset!("assets/portrait/michael_mansfield.gif"),
        source: "<i>Mike Mansfield.</i> Photograph. Accessed August 4, 2024. https://www.mtmemory.org/nodes/view/20145"
    }),
    appointment: &[
        Appointment { term: (1918, 1922), name: "<b>Serviceman</b>, ❰US❱ Department of Defence", subs: &[
            Appointment { term: (1918, 1919), name: "<b>Enlisted</b>, Active Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1918, 1919), name: "(OR-01) <b>⟨Seaman, Apprentice|Seaman, Recruit⟩</b>", subs: &[]}
            ]},
            Appointment { term: (1919, 1920), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1919, 1920), name: "(OR-01) <b>Private</b>", subs: &[]}
            ]},
            Appointment { term: (1920, 1922), name: "<b>Enlisted</b>, Active Component, ❰US❱ Marine Corps", subs: &[
                Appointment { term: (1920, 1922), name: "(OR-02) <b>Private First Class</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1943, 1953), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1943, 1953), name: "<b>Representative</b>, Montana", subs: &[
                Appointment { term: (1949, 1951), name: "<b>Chairman</b>, Special Committee on Campaign Expenditures", subs: &[]}
            ]},
        ]},
        Appointment { term: (1953, 1977), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1953, 1977), name: "<b>Senator</b>, Montana", subs: &[
                Appointment { term: (1960, 1963), name: "<b>Chairman</b>, Committee on Rules and Administration", subs: &[]},
                Appointment { term: (1973, 1974), name: "<b>Chairman</b>, Special Committee on Secret and Confidential Documents", subs: &[]}
            ]}
        ]},
        Appointment { term: (1977, 1988), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1977, 1988), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1977, 1988), name: "<b>Ambassador</b>, Japan", subs: &[]}
            ]}
        ]}
    ]
};

pub const THOMAS_LANE: Person = Person {
    name: Name {
        full: "Thomas Joseph Lane",
        last: "Lane",
    },
    span: (1898, 1994),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/thomas_lane.webp"),
        sketch: asset!("assets/portrait/thomas_lane.gif"),
        source: "“Thomas J. Lane” In <i>Pocket Congressional Directory</i>, 1959:63. USGPO, 1959. https://www.google.com/books/edition/Pocket_Congressional_Directory/AtJFAQAAMAAJ"
    }),
    appointment: &[
        Appointment { term: (NULL, NULL), name: "<b>Serviceman</b>, ❰US❱ Department of Defence", subs: &[
            Appointment { term: (NULL, NULL), name: "<b>Active</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (NULL, NULL), name: "(OF-01) <b>Private</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1921, 1925), name: "<b>Student</b>, Suffolk University, Boston", subs: &[
            Appointment { term: (1921, 1925), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1927, 1938), name: "<b>Politician</b>, ❰US-MA❱ House of Representatives", subs: &[
            Appointment { term: (1927, 1938), name: "<b>Representative</b>", subs: &[]}
        ]},
        Appointment { term: (1939, 1941), name: "<b>Politician</b>, ❰US-MA❱ Senate", subs: &[
            Appointment { term: (1939, 1941), name: "<b>Senator</b>", subs: &[]}
        ]},
        Appointment { term: (1941, 1963), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1941, 1963), name: "<b>Representative</b>, Massachusetts", subs: &[]}
        ]},
        Appointment { term: (1965, 1977), name: "<b>Legal Practitioner</b>, ❰US-MA❱ Massachusetts Office of the Governor", subs: &[
            Appointment { term: (1965, 1977), name: "<b>Legal Advisor</b>", subs: &[]}
        ]}
    ]
};

pub const WALTER_JUDD: Person = Person {
    name: Name {
        full: "Walter Henry Judd",
        last: "Judd",
    },
    span: (1898, 1994),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/walter_judd.webp"),
        sketch: asset!("assets/portrait/walter_judd.gif"),
        source: "“Walter H. Judd” In <i>Pocket Congressional Directory</i>, 1955:70. USGPO, 1955. https://www.google.com/books/edition/Pocket_Congressional_Directory/ADuHAAAAMAAJ"
    }),
    appointment: &[
        Appointment { term: (1916, 1918), name: "<b>Student</b>, University of Nebraska, Lincoln", subs: &[]},
        Appointment { term: (1918, 1919), name: "<b>Serviceman</b>, ❰US❱ Department of Defence", subs: &[
            Appointment { term: (1918, 1920), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1918, 1919), name: "(OR-01) <b>Private</b>", subs: &[]}
            ]},
            Appointment { term: (1919, 1920), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1919, 1920), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1920, 1923), name: "<b>Student</b>, University of Nebraska, Lincoln", subs: &[
            Appointment { term: (1920, 1920), name: "Bachelor of Arts", subs: &[]},
            Appointment { term: (1920, 1923), name: "Doctor of Medicine", subs: &[]}
        ]},
        Appointment { term: (1943, 1963), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1943, 1963), name: "<b>Representative</b>, Minnesota", subs: &[]}
        ]}
    ]
};

pub const USHER_BURDICK: Person = Person {
    name: Name {
        full: "Usher Lloyd Burdick",
        last: "Burdick",
    },
    span: (1879, 1960),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/usher_burdick.webp"),
        sketch: asset!("assets/portrait/usher_burdick.gif"),
        source: "<i>Usher L. Burdick.</i> Photograph. Accessed August 5, 2024. https://statemuseum.nd.gov/photobook/details?I%20RecID=PH_I_91178"
    }),
    appointment: &[
        Appointment { term: (1896, 1900), name: "<b>Student</b>, ⟨Mayville Normal School|Mayville State University⟩, Mayville", subs: &[
            Appointment { term: (1896, 1900), name: "Bachelor of Arts, Teaching", subs: &[]},
        ]},
        Appointment { term: (1900, 1904), name: "<b>Student</b>, University of Minnesota, Minneapolis", subs: &[
            Appointment { term: (1902, 1904), name: "Bachelor of Laws", subs: &[]},
        ]},
        Appointment { term: (1904, 1907), name: "<b>Legal Practitioner</b>, “Burdick's Law Office”", subs: &[]},
        Appointment { term: (1907, 1911), name: "<b>Politician</b>, ❰US-ND❱ North Dakota House of Representatives", subs: &[
            Appointment { term: (1907, 1911), name: "<b>Representative</b>", subs: &[]}
        ]},
        Appointment { term: (1911, 1913), name: "<b>Politician</b>, ❰US-ND❱ North Dakota Office of the Governor", subs: &[
            Appointment { term: (1911, 1913), name: "<b>Governor, Lieutenant</b>", subs: &[]}
        ]},
        Appointment { term: (1913, 1920), name: "<b>Legal Officer</b>, District of Williams County, North Dakota", subs: &[
            Appointment { term: (1913, 1915), name: "<b>State's Attorney</b>", subs: &[]},
            Appointment { term: (1915, 1920), name: "<b>Special Prosecutor</b>", subs: &[]}
        ]},
        Appointment { term: (1929, 1932), name: "<b>Legal Officer</b>, ❰US❱ Department of Justice", subs: &[
            Appointment { term: (1929, 1932), name: "<b>U.S. Attorney, Assistant</b>, District Court for the District of North Dakota", subs: &[]}
        ]},
        Appointment { term: (1935, 1959), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1935, 1945), name: "<b>Representative</b>, North Dakota", subs: &[]},
            Appointment { term: (1945, 1959), name: "<b>Representative</b>, North Dakota", subs: &[]}
        ]},
    ]
};

pub const FORREST_SHERMAN: Person = Person {
    name: Name {
        full: "Forrest Percival Sherman",
        last: "Sherman",
    },
    span: (1896, 1951),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/forrest_sherman.webp"),
        sketch: asset!("assets/portrait/forrest_sherman.gif"),
        source: "<i>Captain Forrest P. Sherman.</i> Photograph. Accessed September 16, 2024. https://www.history.navy.mil/our-collections/photography/us-people/s/sherman-forrest-p.html"
    }),
    appointment: &[
        Appointment { term: (1914, 1951), name: "<b>Serviceman</b>, ❰US❱ Department of Defence", subs: &[
            Appointment { term: (1914, 1917), name: "<b>Cadet</b>, United States Naval Academy, Annapolis", subs: &[]},
            Appointment { term: (1917, 1951), name: "<b>Commissioned</b>, Active Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1917, 1922), name: "(OF-01) <b>Ensign</b>", subs: &[]},
                Appointment { term: (1922, 1930), name: "(OF-02) <b>Lieutenant</b>", subs: &[]},
                Appointment { term: (1930, 1937), name: "(OF-03) <b>Commander, Lieutenant</b>", subs: &[]},
                Appointment { term: (1937, 1942), name: "(OF-03) <b>Commander</b>", subs: &[]},
                Appointment { term: (1942, 1943), name: "(OF-05) <b>Captain</b>", subs: &[]},
                Appointment { term: (1943, 1945), name: "(OF-07) <b>⟨Admiral, Rear|Rear Admiral Upper Half⟩</b>", subs: &[]},
                Appointment { term: (1945, 1949), name: "(OF-08) <b>Admiral, Vice</b>", subs: &[]},
                Appointment { term: (1949, DEAD), name: "(OF-08) <b>Admiral</b>", subs: &[]}
            ]},
            Appointment { term: (1949, DEAD), name: "<b>Chief of Naval Operations</b>", subs: &[]}
        ]}
    ]
};

pub const WILLIAM_KNOWLAND: Person = Person {
    name: Name {
        full: "William Fife Knowland",
        last: "Knowland",
    },
    span: (1908, 1974),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/william_knowland.webp"),
        sketch: asset!("assets/portrait/william_knowland.gif"),
        source: "Passaglia, Elio, and Karma Beal. “William F. Knowland.” In <i>A Unique Institution: The National Bureau of Standards, 1950-1969, 925:91. Spec. Pub. (NIST SP). Accessed August 5, 2024. https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication925.pdf"
    }),
    appointment: &[
        Appointment { term: (1925, 1929), name: "<b>Student</b>, University of California, Berkeley", subs: &[
            Appointment { term: (1925, 1929), name: "Bachelor of Arts, Political Science", subs: &[]}
        ]},
        Appointment { term: (1933, 1935), name: "<b>Politician</b>, ❰US-CA❱ California State Assembly", subs: &[
            Appointment { term: (1933, 1935), name: "<b>Assemblyman</b>, 14th district", subs: &[]}
        ]},
        Appointment { term: (1935, 1939), name: "<b>Politician</b>, ❰US-CA❱ California State Senate", subs: &[
            Appointment { term: (1935, 1939), name: "<b>Senator</b>, 16th district", subs: &[]}
        ]},
        Appointment { term: (1942, 1945), name: "<b>Serviceman</b>, ❰US❱ Department of Defence", subs: &[
            Appointment { term: (1942, 1942), name: "<b>Enlisted</b>, Active Component, Navy", subs: &[
                Appointment { term: (1942, 1942), name: "(OR-01) <b>Private</b>", subs: &[]}
            ]},
            Appointment { term: (1942, 1943), name: "<b>Cadet</b>, Officer Candidate School, ⟨Fort Benning|Fort Moore⟩", subs: &[]},
            Appointment { term: (1943, 1945), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1943, 1945), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1945, 1959), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1945, 1959), name: "<b>Senator</b>, California", subs: &[]}
        ]}
    ]
};

pub const WAYNE_MORSE: Person = Person {
    name: Name {
        full: "Wayne Lyman Morse",
        last: "Morse",
    },
    span: (1900, 1974),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/wayne_morse.webp"),
        sketch: asset!("assets/portrait/wayne_morse.gif"),
        source: "“Wayne L. Morse” In <i>Pocket Congressional Directory</i>, 1953:114. USGPO, 1953. https://www.google.com/books/edition/Pocket_Congressional_Directory/qlvNxFEn5UEC"
    }),
    appointment: &[
        Appointment { term: (1919, 1923), name: "<b>Student</b>, University of Wisconsin, Madison", subs: &[
            Appointment { term: (1919, 1923), name: "Bachelor of Arts, Philosophy", subs: &[]},
            Appointment { term: (1923, 1924), name: "Master of Arts, Speech", subs: &[]}
        ]},
        Appointment { term: (1923, 1929), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1923, 1929), name: "<b>Commissioned</b>, Reserve Component, ❰US❱ Army", subs: &[
                Appointment { term: (1923, 1929), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1925, 1928), name: "<b>Student</b>, University of Minnesota, Minneapolis", subs: &[
            Appointment { term: (1925, 1928), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1925, 1928), name: "<b>Scholar</b>, University of Minnesota, Minneapolis", subs: &[
            Appointment { term: (1925, 1928), name: "<b>Lecturer</b>, Argumentation", subs: &[]}
        ]},
        Appointment { term: (1928, 1929), name: "<b>Student</b>, Columbia University, New York", subs: &[
            Appointment { term: (1928, 1929), name: "Doctor of Juridical Science", subs: &[]}
        ]},
        Appointment { term: (1929, 1944), name: "<b>Scholar</b>, University of Oregon, Eugene", subs: &[
            Appointment { term: (1929, 1929), name: "<b>Professor, Assistant</b>, Law", subs: &[]},
            Appointment { term: (1929, 1931), name: "<b>Professor, Associate</b>, Law", subs: &[]},
            Appointment { term: (1931, 1929), name: "<b>Professor</b>, Law", subs: &[]},
            Appointment { term: (1931, 1944), name: "<b>Dean</b>", subs: &[]}
        ]},
        Appointment { term: (1945, 1969), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1945, 1969), name: "<b>Senator</b>, Oregon", subs: &[]}
        ]}
    ]
};

pub const CAREY_KEFAUVER: Person = Person {
    name: Name {
        full: "Kefauver",
        last: "Yeh",
    },
    span: (1903, 1963),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/carey_kefauver.webp"),
        sketch: asset!("assets/portrait/carey_kefauver.gif"),
        source: "<i>Carey Estes Kefauver.</i> Photograph. Accessed August 5, 2024. https://bioguide.congress.gov/search/bio/K000044"
    }),
    appointment: &[
        Appointment { term: (1920, 1924), name: "<b>Student</b>, University of Wisconsin, Madison", subs: &[
            Appointment { term: (1920, 1924), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1925, 1927), name: "<b>Student</b>, Yale University, New Haven", subs: &[
            Appointment { term: (1925, 1927), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1927, NULL), name: "<b>Legal Practitioner</b>, “Cooke, Swaney & Cooke”", subs: &[]},
        Appointment { term: (NULL, NULL), name: "<b>Legal Practitioner</b>, “Sizer, Chambliss & Kefauver”", subs: &[]},
        Appointment { term: (NULL, 1939), name: "<b>Legal Practitioner</b>, “Duggan, McDonald, & Kefauver”", subs: &[]},
        Appointment { term: (1939, 1949), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1939, 1949), name: "<b>Representative</b>, Tennessee", subs: &[
                Appointment { term: (1946, 1946), name: "<b>Chairman</b>, Select Committee on Small Business", subs: &[]}
            ]}
        ]},
        Appointment { term: (1949, DEAD), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1949, DEAD), name: "<b>Senator</b>, Tennessee", subs: &[
                Appointment { term: (1950, 1951), name: "<b>Chairman</b>, Special Committee to Investigate Crime in Interstate Commerce", subs: &[]},
                Appointment { term: (1953, 1953), name: "<b>Chairman</b>, Subcommittee on Juvenile Delinquency", subs: &[]},
                Appointment { term: (1957, DEAD), name: "<b>Chairman</b>, Subcommittee on Antitrust and Monopoly", subs: &[]}
            ]}
        ]}
    ]
};

pub const HUBERT_HUMPHREY: Person = Person {
    name: Name {
        full: "Hubert Horatio Humphrey",
        last: "Humphrey",
    },
    span: (1911, 1978),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/hubert_humphrey.webp"),
        sketch: asset!("assets/portrait/hubert_humphrey.gif"),
        source: "Hubert H. Humphrey.</i> Photograph. Accessed September 14, 2024. https://www.bu.edu/hhh/about/hubert-h-humphrey/"
    }),
    appointment: &[
        Appointment { term: (1937, 1939), name: "<b>Student</b>, University of Minnesota, Minneapolis", subs: &[
            Appointment { term: (1937, 1939), name: "Bachelor of Arts, Political Science", subs: &[]}
        ]},
        Appointment { term: (1939, 1940), name: "<b>Student</b>, Louisiana State University, Baton Rouge", subs: &[
            Appointment { term: (1939, 1940), name: "Master of Arts, Political Science", subs: &[]}
        ]},
        Appointment { term: (1943, 1944), name: "<b>Scholar</b>, Macalester College, Saint Paul", subs: &[
            Appointment { term: (1943, 1944), name: "<b>Professor</b>, Political Science", subs: &[]}
        ]},
        Appointment { term: (1945, 1949), name: "<b>Politician</b>, Municipality (City) of Minneapolis, Minnesota", subs: &[
            Appointment { term: (1945, 1949), name: "<b>Mayor</b>, Minneapolis City Hall", subs: &[]}
        ]},
        Appointment { term: (1949, 1964), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1949, 1964), name: "<b>Senator</b>, Minnesota", subs: &[]}
        ]},
        Appointment { term: (1965, 1969), name: "<b>Politician</b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1965, 1969), name: "<b>President, Vice</b>", subs: &[]}
        ]},
        Appointment { term: (1971, DEAD), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1971, DEAD), name: "<b>Senator</b>, Minnesota", subs: &[]}
        ]},
    ]
};

pub const HERBERT_LEHMAN: Person = Person {
    name: Name {
        full: "Herbert Henry Lehman",
        last: "Lehman",
    },
    span: (1878, 1963),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/herbert_lehman.webp"),
        sketch: asset!("assets/portrait/herbert_lehman.gif"),
        source: "Herbert H. Lehman.</i> Photograph. Accessed September 14, 2024. https://www.gettyimages.com/detail/1357447764"
    }),
    appointment: &[
        Appointment { term: (1895, 1899), name: "<b>Student</b>, Williams College, Williamson", subs: &[
            Appointment { term: (1895, 1899), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1917, 1917), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1917, 1917), name: "<b>Textile Expert</b>, Department of the Navy", subs: &[]}
        ]},
        Appointment { term: (1917, 1919), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1917, 1919), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1917, 1918), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1918, 1918), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1918, 1919), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1928, 1942), name: "<b>Politician</b>, ❰US-NY❱ New York Office of the Governor", subs: &[
            Appointment { term: (1928, 1932), name: "<b>Governor, Lieutenant</b>", subs: &[]},
            Appointment { term: (1932, 1942), name: "<b>Governor</b>", subs: &[]}
        ]},
        Appointment { term: (1942, 1946), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1942, 1943), name: "<b>Director</b>, ⟨Office of Foreign Relief and Rehabilitation Operations|⟩", subs: &[]},
            Appointment { term: (1942, 1946), name: "<b>Director General</b>, ⟨United Nations Relief and Rehabilitation Administration|⟩", subs: &[]}
        ]},
        Appointment { term: (1949, 1956), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1949, 1956), name: "<b>Senator</b>, New York", subs: &[]}
        ]}
    ]
};

pub const DONALD_EDGAR: Person = Person {
    name: Name {
        full: "Donald Dixon Edgar",
        last: "Edgar",
    },
    span: (1907, 1972),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/donald_edgar.webp"),
        sketch: asset!("assets/portrait/donald_edgar.gif"),
        source: "<i>Donald Dixon Edgar.</i> Photograph. Accessed September 18, 2024. https://www.ae-memoir.com/photogallery"
    }),
    appointment: &[
        Appointment { term: (1924, 1928), name: "<b>Student</b>, Williams College, Williamstown", subs: &[
            Appointment { term: (1924, 1928), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1930, 1962), name: "<b>Bureaucrat, ❰US❱ Department of State", subs: &[
            Appointment { term: (1930, 1940), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1930, 1931), name: "<b>Consul, Vice</b>, Kingston, Canada", subs: &[]},
                Appointment { term: (1931, 1933), name: "<b>Consul, Vice</b>, Hong Kong", subs: &[]},
                Appointment { term: (1933, 1936), name: "<b>Consul, Vice</b>, Havana, Cuba", subs: &[]},
                Appointment { term: (1936, 1937), name: "<b>Secretary, Third</b>, Ciudad Trujillo, Dominican Republic", subs: &[]},
                Appointment { term: (1937, 1940), name: "<b>Consul</b>, Geneva, Switzerland", subs: &[]}
            ]},
            Appointment { term: (1940, 1941), name: "<b>Undercover Operative</b>, “The Edgar Clay Company”", subs: &[]},
            Appointment { term: (1942, 1943), name: "<b>Clerk</b>, ⟨American Hemisphere Exports Office|⟩", subs: &[]},
            Appointment { term: (1943, 1944), name: "<b>Clerk</b>, ⟨Division of Exports and Requirements|⟩", subs: &[]},
            Appointment { term: (1944, 1944), name: "<b>Clerk</b>, ⟨Board of Economic Warfare|⟩", subs: &[]},
            Appointment { term: (1944, 1945), name: "<b>Clerk</b>, ⟨Office of American Republics|⟩", subs: &[]},
            Appointment { term: (1945, 1946), name: "<b>Clerk</b>", subs: &[]},
            Appointment { term: (1946, 1947), name: "<b>Chief</b>, ⟨Central Intelligence Group|Central Intelligence Agency⟩", subs: &[]},
            Appointment { term: (1947, 1957), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1947, 1948), name: "<b>Consul</b>, Taipei, ⟨Japan|Taiwan⟩", subs: &[]},
                Appointment { term: (1948, 1950), name: "<b>Consul General</b>, Taipei, ⟨Japan|Taiwan⟩", subs: &[]},
                Appointment { term: (1950, 1951), name: "<b>Secretary, First</b>, Rome, Italy", subs: &[]},
                Appointment { term: (1953, 1955), name: "<b>Consul General</b>, Alexandria, Egypt", subs: &[]},
                Appointment { term: (1955, 1957), name: "<b>Counsellor</b>, Rio de Janeiro, Brazil", subs: &[]}
            ]},
            Appointment { term: (1957, 1959), name: "<b>Director</b>, ⟨International Educational Exchange Service|⟩", subs: &[]},
            Appointment { term: (1959, 1962), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1959, 1962), name: "<b>Consul General</b>, Marseille, France", subs: &[]}
            ]},
        ]},
        Appointment { term: (1947, 1962), name: "<b>Bureaucrat</b>, ❰US❱ Office of the Director of National Intelligence", subs: &[
            Appointment { term: (1947, 1962), name: "<b>Undercover Operative</b>, ❰US❱ Central Intelligence Agency", subs: &[]},
        ]}
    ]
};

pub const DEAN_ACHESON: Person = Person {
    name: Name {
        full: "Dean Gooderham Acheson",
        last: "Acheson",
    },
    span: (1893, 1971),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/dean_acheson.webp"),
        sketch: asset!("assets/portrait/dean_acheson.gif"),
        source: "<i>Dean Acheson.</i> Photograph. Accessed September 18, 2024. https://alphahistory.com/coldwar/dean-acheson/"
    }),
    appointment: &[
        Appointment { term: (1912, 1915), name: "<b>Student</b>, Yale University, New Haven", subs: &[
            Appointment { term: (1912, 1915), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1915, 1918), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1915, 1918), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1916, 1917), name: "<b>Serviceman</b>, ❰US❱ Department of Defence", subs: &[
            Appointment { term: (1916, 1917), name: "<b>Enlisted</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1916, 1917), name: "(OR-01) Private", subs: &[]}
            ]},
        ]},
        Appointment { term: (1919, 1921), name: "<b>Legal Practitioner</b>, Office of Supreme Court Justice Louis Brandeis", subs: &[
            Appointment { term: (1919, 1921), name: "<b>Law Clerk</b>", subs: &[]},
        ]},
        Appointment { term: (1921, 1946), name: "<b>Legal Practitioner</b>, “Covington & Burling”", subs: &[]},
        Appointment { term: (1933, 1933), name: "<b>Bureaucrat</b>, ❰US❱ Department of the Treasury", subs: &[
            Appointment { term: (1933, 1933), name: "<b>Secretary, Under</b>", subs: &[]}
        ]},
        Appointment { term: (1941, 1953), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1941, 1945), name: "<b>Secretary, Assistant</b>, Office of Legislative Affairs", subs: &[]},
            Appointment { term: (1945, 1947), name: "<b>Secretary, Under</b>", subs: &[]},
            Appointment { term: (1949, 1953), name: "<b>Secretary of State</b>", subs: &[]}
        ]},
    ]
};

pub const MICHAEL_MCDERMOTT: Person = Person {
    name: Name {
        full: "Michael “Mack” James McDermott",
        last: "McDermott",
    },
    span: (1894, 1955),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/michael_mcdermott.webp"),
        sketch: asset!("assets/portrait/michael_mcdermott.gif"),
        source: "<i>Michael J. McDermott.</i> Photograph. Accessed September 18, 2024. https://www.newspapers.com/image/869758026/"
    }),
    appointment: &[
        Appointment { term: (1917, 1920), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1917, 1920), name: "<b>Clerk</b>", subs: &[]}
        ]},
        Appointment { term: (1920, 1954), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1920, 1924), name: "<b>Clerk</b>", subs: &[]},
            Appointment { term: (1924, 1927), name: "<b>Chief, Assistant</b>, ⟨Division of Current Information|⟩", subs: &[]},
            Appointment { term: (1927, 1944), name: "<b>Chief</b>, ⟨Division of Current Information|⟩", subs: &[]},
            Appointment { term: (1944, 1953), name: "<b>Special Assistant</b>, Office of Press Relations", subs: &[]},
            Appointment { term: (1953, 1954), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1953, 1954), name: "<b>Ambassador</b>, El Salvador", subs: &[]}
            ]}
        ]}
    ]
};

pub const DAVID_ABSHIRE: Person = Person {
    name: Name {
        full: "David Manker Abshire",
        last: "Abshire",
    },
    span: (1926, 2014),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/david_abshire.webp"),
        sketch: asset!("assets/portrait/david_abshire.gif"),
        source: "<i>David M. Abshire.</i> Photograph. Accessed September 16, 2024. https://www.rlounsbery.org/post/remembering-david-m-abshire-1926-2014"
    }),
    appointment: &[
        Appointment { term: (1947, 1955), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1947, 1951), name: "<b>Cadet</b>, United States Military Academy, West Point", subs: &[]},
            Appointment { term: (1951, 1955), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1951, 1955), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]},
            ]},
        ]},
        Appointment { term: (1955, 1959), name: "<b>Scholar</b>, Georgetown University, Washington", subs: &[
            Appointment { term: (1955, 1959), name: "<b>Professor, Adjunct</b>, Foreign Service", subs: &[]}
        ]},
        Appointment { term: (1955, 1959), name: "<b>Student</b>, Georgetown University, Washington", subs: &[
            Appointment { term: (1955, 1959), name: "Doctor of Philosophy, History", subs: &[]}
        ]},
        Appointment { term: (1970, 1973), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1970, 1973), name: "<b>Secretary, Assistant</b>, ⟨Bureau of Congressional Relations|Bureau of Legislative Affairs⟩", subs: &[]},
        ]},
        Appointment { term: (1974, 1977), name: "<b>Bureaucrat</b>, ❰US❱ ⟨United States Information Agency|⟩", subs: &[
            Appointment { term: (1974, 1977), name: "<b>Chairman</b>, Board for International Broadcasting", subs: &[]}
        ]},
        Appointment { term: (1981, 1982), name: "<b>Bureaucrat</b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1981, 1982), name: "<b>Advisor</b>, ⟨President's Foreign Intelligence Advisory Board|President's Intelligence Advisory Board⟩", subs: &[]},
        ]},
        Appointment { term: (1983, 1987), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1983, 1987), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1983, 1987), name: "<b>Ambassador-at-large</b>, NATO", subs: &[]}
            ]}
        ]},
        Appointment { term: (1987, 1991), name: "<b>Bureaucrat</b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1987, 1987), name: "<b>Special Counselor</b>", subs: &[]},
            Appointment { term: (1991, 1991), name: "<b>Advisor</b>, ⟨President's Task Force on U.S. Government International Broadcasting|⟩", subs: &[]}
        ]},
    ]
};

pub const JACK_MCFALL: Person = Person {
    name: Name {
        full: "Jack Kirkham McFall",
        last: "McFall",
    },
    span: (1905, 1990),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/jack_mcfall.webp"),
        sketch: asset!("assets/portrait/jack_mcfall.gif"),
        source: "<i>Jack K. McFall.</i> Photograph. Accessed September 16, 2024. https://commons.wikimedia.org/wiki/File:Jack_K_McFall.jpg"
    }),
    appointment: &[
        Appointment { term: (1925, 1929), name: "<b>Student</b>, Georgetown University, Washington", subs: &[
            Appointment { term: (1925, 1929), name: "Bachelor of Science, Foreign Service", subs: &[]}
        ]},
        Appointment { term: (1925, 1928), name: "<b>Bureaucrat</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1925, 1928), name: "<b>Secretary, Assistant</b>, Office of Senator Arthur Robinson", subs: &[]}
        ]},
        Appointment { term: (1929, 1933), name: "<b>Student</b>, ⟨National University School of Law|Georgetown University⟩, Washington", subs: &[
            Appointment { term: (1929, 1933), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1928, 1941), name: "<b>Bureaucrat</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1928, 1941), name: "<b>Secretary, Executive</b>, Committee on Appropriations", subs: &[]}
        ]},
        Appointment { term: (1941, 1945), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1941, 1945), name: "<b>Commissioned</b>, Active Component, ❰US❱ Navy", subs: &[
                Appointment { term: (1941, 1945), name: "(OF-04) <b>Commander</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1946, 1947), name: "<b>Bureaucrat</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1946, 1947), name: "<b>Secretary, Executive</b>, Committee on Appropriations", subs: &[]}
        ]},
        Appointment { term: (1948, 1955), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1948, 1949), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1948, 1949), name: "<b>Consul</b>, Montreal, Canada", subs: &[]},
                Appointment { term: (1949, 1949), name: "<b>Consul</b>, Athens, Greece", subs: &[]},
                Appointment { term: (1949, 1949), name: "<b>Secretary, First</b>, Athens, Greece", subs: &[]},
            ]},
            Appointment { term: (1949, 1952), name: "<b>Secretary, Assistant</b>, ⟨Bureau of Congressional Relations|Bureau of Legislative Affairs⟩", subs: &[]},
            Appointment { term: (1952, 1955), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1952, 1955), name: "⟨<b>Minister Plenipotentiary</b>|<b>Ambassador</b>⟩, Finland", subs: &[]},
            ]}
        ]}
    ]
};

pub const JOSEPH_BALLANTINE: Person = Person {
    name: Name {
        full: "Joseph William Ballantine",
        last: "Ballantine",
    },
    span: (1888, 1973),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/joseph_ballantine.webp"),
        sketch: asset!("assets/portrait/joseph_ballantine.gif"),
        source: "<i>Joseph William Ballantine.</i> Photograph. Accessed September 18, 2024. https://www.ancestry.com/mediaui-viewer/collection/1030/tree/193165012/person/402515483278/media/652a771b-2926-48ac-9877-95ab95273be6"
    }),
    appointment: &[
        Appointment { term: (1905, 1909), name: "<b>Student</b>, Amherst College, Amherst", subs: &[
            Appointment { term: (1905, 1909), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1909, 1947), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1909, 1947), name: "<b>Diplomat</b>, ❰US❱ ⟨Consular Service|Foreign Service⟩", subs: &[
                Appointment { term: (1909, 1911), name: "<b>Student Interpreter</b>, Tokyo, Japan", subs: &[]},
                Appointment { term: (1911, 1912), name: "<b>Consul, Deputy</b>, Kobe, Japan", subs: &[]},
                Appointment { term: (1912, 1914), name: "<b>Consul, Deputy</b>, ⟨Taihoku|Taipei⟩, ⟨Japan|Taiwan⟩", subs: &[]},
                Appointment { term: (1914, 1914), name: "<b>Consul, Deputy</b>, Yokohama, Japan", subs: &[]},
                Appointment { term: (1921, 1923), name: "<b>Consul</b>, ⟨Dairen|Dalian⟩, China", subs: &[]},
                Appointment { term: (1923, 1929), name: "<b>Consul</b>, Tokyo, Japan", subs: &[]},
                Appointment { term: (1930, 1932), name: "<b>Consul</b>, ⟨Canton|Guangdong⟩, China", subs: &[]},
                Appointment { term: (1932, 1934), name: "<b>Consul General</b>, ⟨Canton|Guangdong⟩, China", subs: &[]},
                Appointment { term: (1934, NULL), name: "<b>Consul General</b>, ⟨Mukden|Shenyang⟩, China", subs: &[]}
            ]},
            Appointment { term: (1944, 1945), name: "<b>Director</b>, Office of Far Eastern Affairs", subs: &[]},
            Appointment { term: (1945, 1947), name: "<b>Special Assistant</b>", subs: &[]}
        ]}
    ]
};

pub const WILLIAM_REISMAN: Person = Person {
    name: Name {
        full: "William Michael Reisman",
        last: "Reisman",
    },
    span: (1939, NULL),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/william_reisman.webp"),
        sketch: asset!("assets/portrait/william_reisman.gif"),
        source: "<i>W. Michael Reisman.</i> Photograph. Accessed December 21, 2024. https://onlineexhibits.library.yale.edu/s/YJIL-50/item/19817"
    }),
    appointment: &[
        Appointment { term: (1956, 1960), name: "<b>Student</b>, Johns Hopkins University, Baltimore", subs: &[
            Appointment { term: (1956, 1960), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1960, 1963), name: "<b>Student</b>, Hebrew University, Jerusalem", subs: &[
            Appointment { term: (1960, 1963), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1963, 1965), name: "<b>Student</b>, Yale University, New Haven", subs: &[
            Appointment { term: (1963, 1964), name: "Master of Laws", subs: &[]},
            Appointment { term: (1964, 1965), name: "Doctor of the Science of Law", subs: &[]}
        ]},
        Appointment { term: (1965, NULL), name: "<b>Scholar</b>, Yale University, New Haven", subs: &[
            Appointment { term: (1965, 1969), name: "<b>Research Assistant</b>, School of International Law", subs: &[]},
            Appointment { term: (1969, 1972), name: "<b>Professor, Associate</b>, School of International Law", subs: &[]},
            Appointment { term: (1972, 1982), name: "<b>Professor</b>, School of International Law", subs: &[]},
            Appointment { term: (1982, 1998), name: "<b>Professor, <i>Wesley Newcomb Hohfeld</i></b>, School of International Law", subs: &[]},
            Appointment { term: (1998, NULL), name: "<b>Professor Emeritus, <i>Myres Smith McDougal</i></b>, School of International Law", subs: &[]}
        ]}
    ]
};

pub const DAVID_ROWE: Person = Person {
    name: Name {
        full: "David Nelson Rowe",
        last: "Rowe",
    },
    span: (1905, 1985),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/david_rowe.webp"),
        sketch: asset!("assets/portrait/david_rowe.gif"),
        source: "<i>Dr. David Nelson Rowe.</i> Photograph. Accessed December 21, 2024. https://www.newspapers.com/image/107800588/"
    }),
    appointment: &[
        Appointment { term: (1923, 1927), name: "<b>Student</b>, Princeton University, Princeton", subs: &[
            Appointment { term: (1923, 1927), name: "Bachelor of Arts, Political Science", subs: &[]}
        ]},
        Appointment { term: (1927, 1930), name: "<b>Student</b>, University of Southern California, Los Angeles", subs: &[
            Appointment { term: (1927, 1930), name: "Master of Arts, History", subs: &[]}
        ]},
        Appointment { term: (1930, 1935), name: "<b>Student</b>, University of Chicago, Chicago", subs: &[
            Appointment { term: (1930, 1935), name: "Doctor of Philosophy, History", subs: &[]}
        ]},
        Appointment { term: (1935, 1937), name: "<b>Scholar</b>, Harvard University, Cambridge", subs: &[]},
        Appointment { term: (1937, 1938), name: "<b>Scholar</b>, College of Chinese Studies, Beijing", subs: &[]},
        Appointment { term: (1938, 1943), name: "<b>Scholar</b>, Princeton University, Princeton", subs: &[
            Appointment { term: (1938, 1943), name: "<b>Lecturer</b>, Far Eastern Affairs", subs: &[]}
        ]},
        Appointment { term: (1941, NULL), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1941, NULL), name: "<b>Diplomat</b>, ❰US❱ Foreign Service", subs: &[
                Appointment { term: (1941, NULL), name: "<b>Assistant</b>, ⟨Chungking|Chongqing⟩, China", subs: &[]},
            ]},
        ]},
        Appointment { term: (NULL, 1942), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Office of Strategic Services|Central Intelligence Agency⟩", subs: &[
            Appointment { term: (NULL, 1942), name: "<b>Assistant</b>, Board of Research and Analysis", subs: &[]}
        ]},
        Appointment { term: (1945, 1974), name: "<b>Scholar</b>, Yale University, New Haven", subs: &[
            Appointment { term: (1945, 1950), name: "<b>Professor, Assistant</b>, International Relations", subs: &[]},
            Appointment { term: (1950, 1974), name: "<b>Professor</b>, Political Science", subs: &[]}
        ]},
        Appointment { term: (1964, NULL), name: "<b>Serviceman</b>, ❰US❱ Department of Defense", subs: &[
            Appointment { term: (1964, NULL), name: "<b>Commissioned</b>, Reserve Component, ❰US❱ Army", subs: &[
                Appointment { term: (1964, NULL), name: "(OF-05) Colonel", subs: &[]}
            ]},
        ]},
    ]
};

pub const CHARLES_MARSHALL: Person = Person {
    name: Name {
        full: "Charles Burton Marshall",
        last: "Marshall",
    },
    span: (1908, 1999),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/charles_marshall.webp"),
        sketch: asset!("assets/portrait/charles_marshall.gif"),
        source: "<i>Charles Burton Marshall.</i> Photograph. Accessed December 21, 2024. https://ancestors.familysearch.org/en/L2P7-297/charles-burton-marshall-1908-1999"
    }),
    appointment: &[
        Appointment { term: (1930, 1933), name: "<b>Student</b>, ⟨Texas College of Mines and Metallurgy|University of Texas, El Paso⟩", subs: &[]},
        Appointment { term: (1933, 1934), name: "<b>Student</b>, University of Texas, Austin", subs: &[
            Appointment { term: (1933, 1934), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1934, 1938), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1934, 1938), name: "Doctor of Philosophy, Government", subs: &[]}
        ]},
        Appointment { term: (1938, 1942), name: "<b>Scholar</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1938, 1942), name: "<b>Instructor</b>, Government", subs: &[]}
        ]},
        Appointment { term: (1942, 1945), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1942, 1945), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1942, 0000), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (0000, 1945), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1946, 1947), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1946, 1947), name: "<b>Consultant</b>, ⟨Intergovernmental Committee on Refugees|United Nations High Commissioner for Refugees⟩", subs: &[]}
        ]},
        Appointment { term: (1947, 1950), name: "<b>Bureaucrat</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1947, 1950), name: "<b>Consultant</b>, Committee on Foreign Affairs", subs: &[]}
        ]},
        Appointment { term: (1950, 1953), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1950, 1953), name: "<b>Member</b>, Policy Planning Staff", subs: &[]}
        ]}
    ]
};

pub const CHARLES_LEGENDRE: Person = Person {
    name: Name {
        full: "Charles Guillaum Joseph Émile LeGendre",
        last: "LeGendre",
    },
    span: (1830, 1899),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/charles_legendre.webp"),
        sketch: asset!("assets/portrait/charles_legendre.gif"),
        source: "<i>C.W. Le Gendre.</i> Photograph. Accessed December 21, 2024. https://www.loc.gov/pictures/resource/cwpb.05131/"
    }),
    appointment: &[
        Appointment { term: (NULL, NULL), name: "<b>Student</b>, ⟨Royal College of Rheims|University of Reims⟩, Rheims", subs: &[]},
        Appointment { term: (NULL, NULL), name: "<b>Student</b>, ⟨University of Paris|⟩, Paris", subs: &[
            Appointment { term: (NULL, NULL), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1861, 1865), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1861, 1865), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1861, 1862), name: "(OF-03) <b>Major</b>", subs: &[]},
                Appointment { term: (1862, 1863), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]},
                Appointment { term: (1863, NULL), name: "(OF-05) <b>Colonel</b>", subs: &[]},
                Appointment { term: (NULL, 1865), name: "(OF-06) <b>General, Brigadier</b>", subs: &[]},
            ]},
        ]},
        Appointment { term: (1866, 1872), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1866, 1872), name: "<b>Diplomat</b>, ❰US❱ ⟨Consular Service|Foreign Service⟩", subs: &[
                Appointment { term: (1866, 1872), name: "<b>Consul</b>, ⟨Amoy|Xiamen⟩, ⟨Great Qing|The People's Republic of China⟩", subs: &[]}
            ]}
        ]},
        Appointment { term: (1872, 1875), name: "<b>Bureaucrat</b>, ❰JP❱ Ministry of Foreign Affairs (外務省)", subs: &[
            Appointment { term: (1872, 1875), name: "<b>Advisor</b>, ❰JP❱ Bureau of Taiwan Aborigine Affairs (蕃地事務局)", subs: &[]}
        ]},
        Appointment { term: (1890, DEAD), name: "<b>Bureaucrat</b>, ❰KR❱ ⟨Ministry of the Interior|Ministry of the Interior and Safety⟩", subs: &[
            Appointment { term: (1890, DEAD), name: "<b>Minister, Deputy</b>, ⟨Imperial Household Department|⟩", subs: &[]}
        ]}
    ]
};

pub const CHARLES_BRADLEY: Person = Person {
    name: Name {
        full: "Charles William Bradley",
        last: "Bradley",
    },
    span: (1807, 1865),
    portraiture: None,
    appointment: &[
        Appointment { term: (1825, NULL), name: "<b>Student</b>, ⟨Washington College|Trinity College⟩, Hartford", subs: &[]},
        Appointment { term: (NULL, 1830), name: "<b>Student</b>, ⟨General Protestant Episcopal Theological Seminary|General Theological Seminary⟩, New York", subs: &[
            Appointment { term: (NULL, 1830), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (NULL, 1846), name: "<b>Student</b>, ⟨Hobart College|Hobart and William Smith Colleges⟩, Geneva", subs: &[
            Appointment { term: (NULL, 1846), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1843, 1844), name: "<b>Bureaucrat</b>, ❰US-CT❱ Connecticut House of Representatives", subs: &[
            Appointment { term: (1843, 1844), name: "<b>Clerk</b>", subs: &[]}
        ]},
        Appointment { term: (1846, 1847), name: "<b>Politician</b>, ❰US-CT❱ Office of the Secretary of the State", subs: &[
            Appointment { term: (1846, 1847), name: "<b>Secretary of the State</b>", subs: &[]}
        ]},
        Appointment { term: (1849, 1860), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1849, 1860), name: "<b>Diplomat</b>, ❰US❱ ⟨Consular Service|Foreign Service⟩", subs: &[
                Appointment { term: (1849, 1854), name: "<b>Consul</b>, ⟨Amoy|Xiamen⟩, ⟨Great Qing|The People's Republic of China⟩", subs: &[]},
                Appointment { term: (1854, 1857), name: "<b>Consul</b>, Singapore, ⟨The United Kingdom|The Republic of Singapore⟩", subs: &[]},
                Appointment { term: (1857, 1860), name: "<b>Consul</b>, ⟨Ningpo|Ningbo⟩, ⟨Great Qing|The People's Republic of China⟩", subs: &[]}
            ]}
        ]},
        Appointment { term: (1860, 1863), name: "<b>Bureaucrat</b>, ❰CN❱ Ministry of Revenue (Great Qing)", subs: &[
            Appointment { term: (1860, 1863), name: "<b>Customs Officer</b>, ❰CN❱ ⟨Imperial Maritime Customs Service|Chinese Maritime Customs Service⟩", subs: &[
                Appointment { term: (1860, 1863), name: "<b>Tidewaiter</b>, Hankou, ⟨Great Qing|The People's Republic of China⟩", subs: &[]},
            ]}
        ]},
    ]
};

pub const EDWARD_HOUSE: Person = Person {
    name: Name {
        full: "Edward Howard House",
        last: "House",
    },
    span: (1836, 1901),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/edward_house.webp"),
        sketch: asset!("assets/portrait/edward_house.gif"),
        source: "<i>Edward Howard House.</i> Photograph. Accessed February 21, 2025. https://commons.wikimedia.org/wiki/File:EdwardHowardHouse.jpg"
    }),
    appointment: &[
        Appointment { term: (1854, 1858), name: "<b>Journalist</b>, <i>The Boston Courier</i>", subs: &[]},
        Appointment { term: (1858, 1877), name: "<b>Journalist</b>, <i>The New York Tribune</i>", subs: &[
            Appointment { term: (1860, 1861), name: "<b>Correspondent, Foreign</b>, Paris, France", subs: &[]},
            Appointment { term: (1861, 1865), name: "<b>Correspondent, War</b>, American Civil War", subs: &[]},
            Appointment { term: (1870, 1877), name: "<b>Correspondent, Foreign</b>, Tokyo, Japan", subs: &[]},
            Appointment { term: (1874, 1874), name: "<b>Correspondent, War</b>, Taiwan Expedition", subs: &[]},
        ]},
        Appointment { term: (1870, 1880), name: "<b>Scholar</b>, ⟨Western Learning College|University of Tokyo⟩, Tokyo", subs: &[
            Appointment { term: (1870, 1880), name: "<b>Professor</b>, English Language", subs: &[]}
        ]},
        Appointment { term: (1870, 1880), name: "<b>Bureaucrat</b>, ❰JP❱ Ministry of Education (文部省)", subs: &[
            Appointment { term: (1877, 1880), name: "<b>Advisor</b>", subs: &[]},
        ]},
        Appointment { term: (1877, 1880), name: "<b>Journalist</b>, <i>Tokio Times</i>", subs: &[]},
        Appointment { term: (1882, 1885), name: "<b>Journalist</b>, <i>Japan Mail</i>", subs: &[]},
        Appointment { term: (1882, 1885), name: "<b>Scholar</b>, University of Tokyo, Tokyo", subs: &[
            Appointment { term: (1882, 1885), name: "<b>Professor</b>, English Language", subs: &[]}
        ]},
    ]
};

pub const WILLIAM_FOSTER: Person = Person {
    name: Name {
        full: "William Chapman Foster",
        last: "Foster",
    },
    span: (1897, 1984),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/william_foster.webp"),
        sketch: asset!("assets/portrait/william_foster.gif"),
        source: "<i>William Chapman Foster.</i> Photograph. Accessed September 14, 2024. https://mitmuseum.mit.edu/collections/person/8522"
    }),
    appointment: &[
        Appointment { term: (1915, 1917), name: "<b>Student</b>, Massachusetts Institute of Technology, Cambridge", subs: &[]},
        Appointment { term: (1917, 1919), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1917, 1919), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1917, 1919), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]},
            ]},
        ]},
        Appointment { term: (1919, 1920), name: "<b>Student</b>, Massachusetts Institute of Technology, Cambridge", subs: &[
            Appointment { term: (1919, 1920), name: "Bachelor of Engineering, Chemical Engineering", subs: &[]}
        ]},
        Appointment { term: (1944, 1946), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1944, NULL), name: "<b>Clerk</b>, Purchase Division, Army Service Forces, ❰US❱ Army", subs: &[]},
            Appointment { term: (NULL, NULL), name: "<b>Director, Assistant</b>, Purchase Division, Army Service Forces, ❰US❱ Army", subs: &[]},
            Appointment { term: (NULL, NULL), name: "<b>Director, Deputy</b>, Purchase Division, Army Service Forces, ❰US❱ Army", subs: &[]},
            Appointment { term: (NULL, NULL), name: "<b>Director</b>, Purchase Division, Army Service Forces, ❰US❱ Army", subs: &[]},
            Appointment { term: (NULL, 1946), name: "<b>Chairman</b>, Purchase Policy Advisory Committee, Army Service Forces, ❰US❱ Army", subs: &[]}
        ]},
        Appointment { term: (1946, 1948), name: "<b>Bureaucrat</b>, ❰US❱ Department of Commerce", subs: &[
            Appointment { term: (1946, 1948), name: "<b>Secretary, Under</b>", subs: &[]}
        ]},
        Appointment { term: (1950, 1951), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Economic Cooperation Administration|⟩", subs: &[
            Appointment { term: (1950, 1951), name: "<b>Administrator</b>", subs: &[]}
        ]},
        Appointment { term: (1951, 1953), name: "<b>Bureaucrat</b>, ❰US❱ Department of Defense", subs: &[
            Appointment { term: (1951, 1953), name: "<b>Secretary, Deputy</b>", subs: &[]}
        ]},
        Appointment { term: (1962, 1968), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Arms Control and Disarmament Agency|⟩", subs: &[
            Appointment { term: (1962, 1968), name: "<b>Director</b>", subs: &[]}
        ]}
    ]
};

pub const HUBERT_GRAVES: Person = Person {
    name: Name {
        full: "Hubert Ashton Graves",
        last: "Graves",
    },
    span: (1894, 1972),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/hubert_graves.webp"),
        sketch: asset!("assets/portrait/hubert_graves.gif"),
        source: "<i>Sir Hubert Ashton Graves.</i> Photograph. Accessed January 25, 2025. https://www.npg.org.uk/collections/search/portrait/mw222448/Sir-Hubert-Ashton-Graves"
    }),
    appointment: &[
        Appointment { term: (1915, NULL), name: "<b>Serviceman</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[
            Appointment { term: (1915, 1918), name: "<b>Commissioned</b>, Active Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1915, 1918), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]},
                Appointment { term: (1918, NULL), name: "(OF-01) <b>Lieutenant</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1923, 1926), name: "<b>Bureaucrat</b>, ❰UK❱ ⟨Inland Revenue|Revenue and Customs⟩", subs: &[
            Appointment { term: (1923, 1926), name: "<b>Clerk</b>", subs: &[]}
        ]},
        Appointment { term: (1926, 1955), name: "<b>Bureaucrat</b>, ❰UK❱ ⟨Foreign Office|Foreign, Commonwealth and Development Office⟩", subs: &[
            Appointment { term: (1926, 1955), name: "<b>Diplomat</b>, ❰UK❱ Diplomatic Service", subs: &[
                Appointment { term: (1926, 1928), name: "<b>Student Interpreter</b>", subs: &[]},
                Appointment { term: (1928, 1938), name: "<b>Consul, Vice</b>, Japan", subs: &[]},
                Appointment { term: (1938, NULL), name: "<b>Consul</b>, Osaka, Japan", subs: &[]},
                Appointment { term: (1946, 1951), name: "<b>Counselor</b>, Washington, D.C.", subs: &[]},
                Appointment { term: (1951, 1954), name: "<b>Ambassador</b>, ⟨Indochinese Federation|⟩", subs: &[]},
                Appointment { term: (1954, 1955), name: "<b>Ambassador</b>, ⟨The Republic of Vietnam|The Socialist Republic of Vietnam⟩", subs: &[]}
            ]}
        ]}
    ]
};

pub const ANTHONY_EDEN: Person = Person {
    name: Name {
        full: "Robert Anthony Eden",
        last: "Eden",
    },
    span: (1897, 1977),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/robert_eden.webp"),
        sketch: asset!("assets/portrait/robert_eden.gif"),
        source: "<i>Anthony Eden.</i> Photograph. Accessed January 25, 2025. https://www.iwm.org.uk/collections/item/object/205022126"
    }),
    appointment: &[
        Appointment { term: (1915, 1923), name: "<b>Serviceman</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[
            Appointment { term: (1915, 1921), name: "<b>Commissioned</b>, Active Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1915, 1917), name: "(OF-01) <b>Lieutenant, Second</b>", subs: &[]},
                Appointment { term: (1917, 1919), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (1920, 1921), name: "(OF-01) <b>Lieutenant</b>", subs: &[]},
                Appointment { term: (1921, 1921), name: "(OF-02) <b>Captain</b>", subs: &[]}
            ]},
            Appointment { term: (1921, 1923), name: "<b>Commissioned</b>, Reserve Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1921, 0000), name: "(OF-02) <b>Captain</b>", subs: &[]},
                Appointment { term: (0000, 1923), name: "(OF-03) <b>Major</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1919, 1922), name: "<b>Student</b>, University of Oxford, Oxford", subs: &[
            Appointment { term: (1919, 1922), name: "Bachelor of Arts, Oriental Languages", subs: &[]}
        ]},
        Appointment { term: (1923, 1957), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1923, 1957), name: "<b>Member of Parliament (MP)</b>, Electoral District of Warwick and Leamington", subs: &[
                Appointment { term: (1931, 1934), name: "<b>Secretary of State, Under</b>, ⟨Foreign Office|Foreign, Commonwealth, and Development Office⟩", subs: &[]},
                Appointment { term: (1934, 1945), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1935, 1938), name: "⟨<b>Secretary of State for Foreign Affairs</b>|<b>Secretary of State for Foreign, Commonwealth and Development Affairs</b>⟩", subs: &[]},
                    Appointment { term: (1939, 1940), name: "⟨<b>Secretary of State for Dominion Affairs</b>|<b>Secretary of State for Foreign, Commonwealth and Development Affairs</b>⟩", subs: &[]},
                    Appointment { term: (1939, 1940), name: "⟨<b>Secretary of State for War</b>|<b>Secretary of State for Defence</b>⟩", subs: &[]},
                    Appointment { term: (1940, 1945), name: "⟨<b>Secretary of State for Foreign Affairs</b>|<b>Secretary of State for Foreign, Commonwealth and Development Affairs</b>⟩", subs: &[]},
                ]},
                Appointment { term: (1951, 1957), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1951, 1955), name: "⟨<b>Secretary of State for Foreign Affairs</b>|<b>Secretary of State for Foreign, Commonwealth and Development Affairs</b>⟩", subs: &[]},
                    Appointment { term: (1951, 1955), name: "<b>Prime Minister, Deputy</b>", subs: &[]},
                    Appointment { term: (1955, 1957), name: "<b>Prime Minister</b>", subs: &[]}
                ]}
            ]},
        ]},
        Appointment { term: (1934, DEAD), name: "<b>Privy Councilor</b>, ❰UK❱ Privy Council", subs: &[]},
        Appointment { term: (1961, DEAD), name: "<b>Politician</b>, ❰UK❱ House of Lords", subs: &[
            Appointment { term: (1961, DEAD), name: "<b>Peer</b>, Earl of Avon", subs: &[]}
        ]}
    ]
};

pub const HEDLEY_BULL: Person = Person {
    name: Name {
        full: "Hedley Norman Bull",
        last: "Bull",
    },
    span: (1932, 1985),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/hedley_bull.webp"),
        sketch: asset!("assets/portrait/hedley_bull.gif"),
        source: "<i>Hedley Bull.</i> Photograph. Accessed January 25, 2025. https://ecpr.eu/Prizes/PrizeDetails.aspx?PrizeID=11"
    }),
    appointment: &[
        Appointment { term: (1949, 1952), name: "<b>Student</b>, University of Sydney, Sydney", subs: &[
            Appointment { term: (1949, 1952), name: "Bachelor of Arts, History and Philosophy", subs: &[]}
        ]},
        Appointment { term: (1953, 1955), name: "<b>Student</b>, University of Sydney, Sydney", subs: &[
            Appointment { term: (1953, 0000), name: "Bachelor of Philosophy, Politics", subs: &[]},
            Appointment { term: (0000, 1955), name: "Master of Arts, Politics", subs: &[]}
        ]},
        Appointment { term: (1955, 1967), name: "<b>Scholar</b>, University of London, London", subs: &[
            Appointment { term: (1955, 1958), name: "<b>Lecturer, Assistant</b>, International Relations", subs: &[]},
            Appointment { term: (1959, 1962), name: "<b>Lecturer</b>, International Relations", subs: &[]},
            Appointment { term: (1963, 1967), name: "<b>Lecturer, Principal</b>, International Relations", subs: &[]}
        ]},
        Appointment { term: (1963, 1964), name: "<b>Scholar</b>, Princeton University, Princeton", subs: &[
            Appointment { term: (1963, 1964), name: "<b>Research Associate</b>, International Affairs", subs: &[]}
        ]},
        Appointment { term: (1964, 1967), name: "<b>Bureaucrat</b>, ❰UK❱ ⟨Foreign Office|Foreign, Commonwealth and Development Office⟩", subs: &[
            Appointment { term: (1964, 1967), name: "<b>Director</b>, ⟨Arms Control and Disarmament Research Unit|⟩", subs: &[]}
        ]},
        Appointment { term: (1967, 1977), name: "<b>Scholar</b>, Australian National University, Canberra", subs: &[
            Appointment { term: (1967, 1977), name: "<b>Professor</b>, International Relations", subs: &[]}
        ]},
        Appointment { term: (1970, 1971), name: "<b>Scholar</b>, Columbia University, New York", subs: &[
            Appointment { term: (1970, 1971), name: "<b>Professor</b>, War and Peace Studies", subs: &[]}
        ]},
        Appointment { term: (1974, 1975), name: "<b>Scholar</b>, Jawaharlal Nehru University, Delhi", subs: &[
            Appointment { term: (1974, 1975), name: "<b>Professor</b>, International Politics and Organization", subs: &[]}
        ]},
        Appointment { term: (1977, 1985), name: "<b>Scholar</b>, University of Oxford, Oxford", subs: &[
            Appointment { term: (1977, 1985), name: "<b>Professor, <i>Montague Burton</i></b>, International Relations", subs: &[]},
        ]},
    ]
};

pub const WINSTON_CHURCHILL: Person = Person {
    name: Name {
        full: "Winston Leonard Spencer Churchill",
        last: "Churchill",
    },
    span: (1874, 1964),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/winston_churchill.webp"),
        sketch: asset!("assets/portrait/winston_churchill.gif"),
        source: "<i>Prime Minister Winston Churchill of Great Britain.</i> Photograph. Accessed January 25, 2025. https://www.loc.gov/item/2017871963/"
    }),
    appointment: &[
        Appointment { term: (1893, 1924), name: "<b>Serviceman</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[
            Appointment { term: (1893, 1895), name: "<b>Cadet</b>, ⟨Royal Military College|Royal Military Academy⟩, Sandhurst", subs: &[]},
            Appointment { term: (1895, 1924), name: "<b>Commissioned</b>, Active Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1895, 1896), name: "(OF-01) Lieutenant, Second", subs: &[]},
                Appointment { term: (1896, 1900), name: "(OF-01) Lieutenant", subs: &[]}
            ]},
            Appointment { term: (1902, 1924), name: "<b>Commissioned</b>, Reserve Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1902, 1905), name: "(OF-02) Captain", subs: &[]},
                Appointment { term: (1905, 1916), name: "(OF-03) Major", subs: &[]},
                Appointment { term: (1916, 1916), name: "(OF-04) Colonel, Lieutenant", subs: &[]},
                Appointment { term: (1916, 1924), name: "(OF-03) Major", subs: &[]},
            ]}
        ]},
        Appointment { term: (1900, 1964), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1900, 1906), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Oldham|⟩", subs: &[]},
            Appointment { term: (1906, 1908), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Manchester North West|⟩", subs: &[
                Appointment { term: (1906, 1908), name: "<b>Secretary of State, Under</b>, ❰UK❱ ⟨Colonial Office|Foreign, Commonwealth and Development Office⟩", subs: &[]}
            ]},
            Appointment { term: (1908, 1922), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Dundee|⟩", subs: &[
                Appointment { term: (1910, 1911), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1910, 1911), name: "<b>Secretary of State for the Home Department</b>", subs: &[]},
                    Appointment { term: (1911, 1915), name: "⟨<b>First Lord of the Admiralty</b>|<b>Secretary of State for Defence</b>⟩", subs: &[]},
                    Appointment { term: (1915, 1915), name: "<b>Chancellor of the Duchy of Lancaster</b>", subs: &[]},
                ]},
                Appointment { term: (1917, 1919), name: "⟨<b>Minister of Munitions</b>|⟩", subs: &[]},
                Appointment { term: (1919, 1922), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1919, 1921), name: "⟨<b>Secretary of State for Air</b>|<b>Secretary of State for Defence</b>⟩", subs: &[]},
                    Appointment { term: (1919, 1921), name: "⟨<b>Secretary of State for War</b>|<b>Secretary of State for Defence</b>⟩", subs: &[]},
                    Appointment { term: (1921, 1922), name: "⟨<b>Secretary of State for the Colonies</b>|<b>Secretary of State for Foreign, Commonwealth and Development Affairs</b>⟩", subs: &[]},
                ]},
            ]},
            Appointment { term: (1924, 1945), name: "<b>Member of Parliament (MP)</b>, Electoral District of Epping", subs: &[
                Appointment { term: (1924, 1929), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1924, 1929), name: "<b>Chancellor of the Exchequer</b>", subs: &[]},
                ]},
                Appointment { term: (1939, 1945), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1939, 1940), name: "<b>Chancellor of the Exchequer</b>", subs: &[]},
                    Appointment { term: (1940, 1942), name: "<b>Leader of the House of Commons</b>", subs: &[]},
                    Appointment { term: (1940, 1945), name: "<b>Prime Minister</b>", subs: &[]}
                ]}
            ]},
            Appointment { term: (1945, 1964), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Woodford|⟩", subs: &[
                Appointment { term: (1945, 1951), name: "<b>Shadow Minister</b>, ❰UK❱ Shadow Cabinet", subs: &[
                    Appointment { term: (1945, 1951), name: "<b>Leader of the Opposition</b>", subs: &[]}
                ]},
                Appointment { term: (1951, 1955), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1951, 1952), name: "⟨<b>Minister of Defense</b>|<b>Secretary of State for Defence</b>⟩", subs: &[]},
                    Appointment { term: (1951, 1955), name: "<b>Prime Minister</b>", subs: &[]}
                ]}
            ]}
        ]},
        Appointment { term: (1908, DEAD), name: "<b>Privy Councilor</b>, ❰UK❱ Privy Council", subs: &[
            Appointment { term: (1908, 1910), name: "<b>President</b>, Board of Trade", subs: &[]}
        ]}
    ]
};

pub const HERBERT_MORRISON: Person = Person {
    name: Name {
        full: "Herbert Stanley Morrison",
        last: "Morrison",
    },
    span: (1888, 1965),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/herbert_morrison.webp"),
        sketch: asset!("assets/portrait/herbert_morrison.gif"),
        source: "<i>Herbert Stanley Morrison, Baron Morrison of Lambeth.</i> Photograph. Accessed January 25, 2025. https://www.npg.org.uk/collections/search/portrait/mw102294/Herbert-Stanley-Morrison-Baron-Morrison-of-Lambeth"
    }),
    appointment: &[
        Appointment { term: (1919, 1921), name: "<b>Politician</b>, Municipality (Borough) of ⟨Hackney, England|⟩", subs: &[
            Appointment { term: (1920, 1921), name: "<b>Mayor</b>, ⟨Hackney Borough Council|⟩", subs: &[]}
        ]},
        Appointment { term: (1922, 1940), name: "<b>Politician</b>, District (Region) of Greater London, England", subs: &[
            Appointment { term: (1934, 1940), name: "⟨<b>Leader</b>, London County Council|<b>Mayor</b>, Greater London Authority⟩", subs: &[]},
        ]},
        Appointment { term: (1923, 1924), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1923, 1924), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Hackney South|⟩", subs: &[]},
        ]},
        Appointment { term: (1929, 1931), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1929, 1931), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Hackney South|⟩", subs: &[
                Appointment { term: (1929, 1931), name: "⟨<b>Minister of Transport</b>|<b>Secretary of State for Transport</b>⟩", subs: &[]}
            ]}
        ]},
        Appointment { term: (1935, 1951), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1935, 1945), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Hackney South|⟩", subs: &[
                Appointment { term: (1940, 1940), name: "<b>⟨Minister of Supply|⟩</b>", subs: &[]},
                Appointment { term: (1940, 1945), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1940, 1945), name: "<b>Secretary of State for the Home Department</b>", subs: &[]},
                ]},
            ]},
            Appointment { term: (1945, 1959), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Lewisham South|⟩", subs: &[
                Appointment { term: (1945, 1951), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                    Appointment { term: (1945, 1951), name: "<b>Leader of the House of Commons</b>", subs: &[]},
                    Appointment { term: (1951, 1951), name: "⟨<b>Secretary of State for Foreign Affairs</b>|<b>Secretary of State for Foreign, Commonwealth and Development Affairs</b>⟩", subs: &[]}
                ]}
            ]}
        ]},
        Appointment { term: (1940, DEAD), name: "<b>Privy Councilor</b>, ❰UK❱ Privy Council", subs: &[
            Appointment { term: (1945, 1951), name: "<b>Lord President of the Council</b>", subs: &[]},
        ]},
        Appointment { term: (1959, DEAD), name: "<b>Politician</b>, ❰UK❱ House of Lords", subs: &[
            Appointment { term: (1959, DEAD), name: "<b>Peer</b>, Baron of Lambeth", subs: &[]},
        ]}
    ]
};

pub const JANET_LEE: Person = Person {
    name: Name {
        full: "Janet Lee",
        last: "Lee",
    },
    span: (1904, 1988),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/janet_lee.webp"),
        sketch: asset!("assets/portrait/janet_lee.gif"),
        source: "<i>Jennie Lee.</i> Photograph. Accessed January 25, 2025. https://heritage.humanists.uk/jennie-lee/"
    }),
    appointment: &[
        Appointment { term: (1922, 1927), name: "<b>Student</b>, University of Edinburgh", subs: &[
            Appointment { term: (1922, NULL), name: "Bachelor of Arts, Education", subs: &[]},
            Appointment { term: (NULL, 1926), name: "Master of Arts, Education", subs: &[]},
            Appointment { term: (1926, 1927), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1929, 1931), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1929, 1931), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨North Lanarkshire|⟩", subs: &[]}
        ]},
        Appointment { term: (1940, 1945), name: "<b>Bureaucrat</b>, ❰UK❱ ⟨Ministry of Aircraft Production|⟩", subs: &[
            Appointment { term: (1940, 1945), name: "<b>Political Correspondent</b>", subs: &[]},
        ]},
        Appointment { term: (1945, 1970), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1945, 1970), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Cannock|⟩", subs: &[
                Appointment { term: (1964, 1965), name: "<b>Secretary of State, Under</b>, ❰UK❱ ⟨Ministry of Public Building and Works|⟩", subs: &[]},
                Appointment { term: (1964, 1970), name: "⟨<b>Minister of State for the Arts</b>|<b>Minister of State</b>, ❰UK❱ Department for Digital, Culture, Media and Sport⟩", subs: &[]},
                Appointment { term: (1965, 1967), name: "<b>Secretary of State, Under</b>, ❰UK❱ ⟨Department of Education and Science|Department for Education and Skills⟩", subs: &[]},
                Appointment { term: (1967, 1970), name: "<b>Minister of State</b>, ❰UK❱ ⟨Department of Education and Science|Department for Education⟩", subs: &[]},
            ]}
        ]},
        Appointment { term: (1966, DEAD), name: "<b>Privy Councilor</b>, ❰UK❱ Privy Council", subs: &[]},
        Appointment { term: (1970, DEAD), name: "<b>Politician</b>, ❰UK❱ House of Lords", subs: &[
            Appointment { term: (1970, DEAD), name: "<b>Peer</b>, Baroness of Asheridge", subs: &[]}
        ]}
    ]
};

pub const WILLIAM_PICKERING: Person = Person {
    name: Name {
        full: "William Alexander Pickering",
        last: "Pickering",
    },
    span: (1840, 1907),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/william_pickering.webp"),
        sketch: asset!("assets/portrait/william_pickering.gif"),
        source: "<i>William A. Pickering.</i> Photograph. Accessed January 25, 2025. https://commons.wikimedia.org/wiki/File:William_A._Pickering.jpg"
    }),
    appointment: &[
        Appointment { term: (1862, 1867), name: "<b>Bureaucrat</b>, ❰CN❱ Ministry of Revenue (Great Qing)", subs: &[
            Appointment { term: (1862, 1867), name: "<b>Customs Officer</b>, ❰CN❱ ⟨Imperial Maritime Customs Service|Chinese Maritime Customs Service⟩", subs: &[
                Appointment { term: (1862, 1863), name: "<b>Tidewaiter</b>, Fuzhou, ⟨Great Qing|The People's Republic of China⟩", subs: &[]},
                Appointment { term: (1863, NULL), name: "<b>Tidewaiter</b>, ⟨Tamsui|New Taipei City⟩, ⟨Great Qing|Taiwan⟩", subs: &[]},
                Appointment { term: (NULL, NULL), name: "<b>Tidewaiter</b>, Keelung, ⟨Great Qing|Taiwan⟩", subs: &[]},
                Appointment { term: (NULL, 1865), name: "<b>Tidewaiter</b>, ⟨Takou|Kaohsiung⟩, ⟨Great Qing|Taiwan⟩", subs: &[]},
                Appointment { term: (1865, 1867), name: "<b>Chief</b>, ⟨Anping|Tainan⟩, ⟨Great Qing|Taiwan⟩", subs: &[]},
            ]}
        ]},
        Appointment { term: (1871, 1889), name: "<b>Bureaucrat</b>, ❰UK❱ Colonial Office", subs: &[
            Appointment { term: (1871, 1889), name: "<b>Interpreter, Chinese</b>, The Straight Settlements", subs: &[]},
            Appointment { term: (1874, NULL), name: "<b>Police Magistrate</b>, The Straight Settlements", subs: &[]},
            Appointment { term: (1874, NULL), name: "<b>Justice of the Peace</b>, The Straight Settlements", subs: &[]},
            Appointment { term: (1877, 1889), name: "<b>Protector of Chinese</b>, The Straight Settlements", subs: &[]}
        ]},
    ]
};

pub const RUTHERFORD_ALCOCK: Person = Person {
    name: Name {
        full: "John Rutherford Alcock",
        last: "Alcock",
    },
    span: (1809, 1897),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/rutherford_alcock.webp"),
        sketch: asset!("assets/portrait/rutherford_alcock.gif"),
        source: "<i>Rutherford Alcock.</i> Photograph. Accessed January 25, 2025. https://commons.wikimedia.org/wiki/File:Rutherford_Alcock,_Lock_%26_Whitfield_woodburytype,_1876-84.jpg"
    }),
    appointment: &[
        Appointment { term: (1836, 1837), name: "<b>Serviceman</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[
            Appointment { term: (1836, 1837), name: "<b>Commissioned</b>, Active Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1836, 1837), name: "(OF-01) Lieutenant", subs: &[]}
            ]}
        ]},
        Appointment { term: (1843, 1870), name: "<b>Bureaucrat</b>, ❰UK❱ ⟨Foreign Office|Foreign, Commonwealth and Development Office⟩", subs: &[
            Appointment { term: (1843, 1870), name: "<b>Diplomat</b>, ❰UK❱ ⟨Consulate Service|Diplomatic Service⟩", subs: &[
                Appointment { term: (1843, 1844), name: "b>Secretary</b>, ⟨Amoy|Xiamen⟩, ⟨Great Qing|People's Republic of China⟩", subs: &[]},
                Appointment { term: (1844, 1846), name: "b>Consul</b>, Fuzhou, ⟨Great Qing|People's Republic of China⟩", subs: &[]},
                Appointment { term: (1846, 1856), name: "b>Consul</b>, Shanghai, ⟨Great Qing|People's Republic of China⟩", subs: &[]},
                Appointment { term: (1856, 1859), name: "b>Consul</b>, ⟨Canton|Guangdong⟩, ⟨Great Qing|People's Republic of China⟩", subs: &[]},
                Appointment { term: (1859, 1860), name: "b>Consul General</b>, Tokyo, Japan", subs: &[]},
            ]},
            Appointment { term: (1843, 1870), name: "<b>Diplomat</b>, ❰UK❱ Diplomatic Service", subs: &[
                Appointment { term: (1860, 1865), name: "b>Minister Plenipotentiary</b>, Japan", subs: &[]},
                Appointment { term: (1865, 1870), name: "b>Minister Plenipotentiary</b>, ⟨Great Qing|People's Republic of China⟩", subs: &[]}
            ]}
        ]}
    ]
};

pub const OLIVER_FRANKS: Person = Person {
    name: Name {
        full: "Oliver Shewell Franks",
        last: "Franks",
    },
    span: (1905, 1992),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/oliver_franks.webp"),
        sketch: asset!("assets/portrait/oliver_franks.gif"),
        source: "<i>Oliver Shewell Franks.</i> Photograph. Accessed January 25, 2025. https://www.npg.org.uk/collections/search/portrait/mw100853/"
    }),
    appointment: &[
        Appointment { term: (1923, 1927), name: "<b>Student</b>, University of Oxford, Oxford", subs: &[
            Appointment { term: (1922, 1927), name: "Bachelor of Arts, Classics", subs: &[]},
        ]},
        Appointment { term: (1937, 1939), name: "<b>Scholar</b>, University of Glasgow, Glasgow", subs: &[
            Appointment { term: (1937, 1939), name: "<b>Professor</b>, Moral Philosophy", subs: &[]},
        ]},
        Appointment { term: (1939, 1946), name: "<b>Bureaucrat</b>, ❰UK❱ Civil Service", subs: &[
            Appointment { term: (1939, NULL), name: "<b>Clerk</b>, ⟨Ministry of Supply|⟩", subs: &[]},
            Appointment { term: (NULL, 1946), name: "<b>Secretary, Permanent</b>, ⟨Ministry of Supply|⟩", subs: &[]}
        ]},
        Appointment { term: (1948, 1952), name: "<b>Bureaucrat</b>, ❰UK❱ ⟨Foreign Office|Foreign, Commonwealth and Development Office⟩", subs: &[
            Appointment { term: (1948, 1952), name: "<b>Diplomat</b>, ❰UK❱ Diplomatic Service", subs: &[
                Appointment { term: (1948, 1952), name: "<b>Ambassador</b>, The United States of America", subs: &[]}
            ]},
        ]},
        Appointment { term: (1949, DEAD), name: "<b>Privy Councilor</b>, ❰UK❱ Privy Council", subs: &[]},
        Appointment { term: (1962, DEAD), name: "<b>Politician</b>, ❰UK❱ House of Lords", subs: &[
            Appointment { term: (1962, DEAD), name: "<b>Peer</b>, Baron of Headington", subs: &[]}
        ]}
    ]
};

pub const CLEMENT_ATTLEE: Person = Person {
    name: Name {
        full: "Clement Richard Attlee",
        last: "Attlee",
    },
    span: (1883, 1967),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/clement_attlee.webp"),
        sketch: asset!("assets/portrait/clement_attlee.gif"),
        source: "<i>British Prime Minister Clement Attlee.</i> Photograph. Accessed January 25, 2025. https://www.trumanlibrary.gov/photograph-records/99-48"
    }),
    appointment: &[
        Appointment { term: (1901, 1904), name: "<b>Student</b>, University of Oxford, Oxford", subs: &[
            Appointment { term: (1901, 1904), name: "Bachelor of Arts, Modern History", subs: &[]}
        ]},
        Appointment { term: (1912, 1914), name: "<b>Scholar</b>, University of London", subs: &[
            Appointment { term: (1912, 1914), name: "<b>Lecturer</b>, Social Administration", subs: &[]}
        ]},
        Appointment { term: (1914, 1919), name: "<b>Serviceman</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[
            Appointment { term: (1914, 1919), name: "<b>Commissioned</b>, Active Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1914, 1915), name: "(OF-01) Lieutenant", subs: &[]},
                Appointment { term: (1915, 1917), name: "(OF-02) Captain", subs: &[]},
                Appointment { term: (1917, 1919), name: "(OF-03) Major", subs: &[]}
            ]}
        ]},
        Appointment { term: (1919, 1920), name: "<b>Politician</b>, Municipality (Borough) of ⟨Stepney, England|⟩", subs: &[
            Appointment { term: (1919, 1920), name: "<b>Mayor</b>, Stepney Borough Council", subs: &[]}
        ]},
        Appointment { term: (1922, 1955), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1922, 1950), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Limehouse|⟩", subs: &[
                Appointment { term: (1924, 1924), name: "<b>Secretary of State, Under</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[]}
            ]},
            Appointment { term: (1950, 1955), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Walthamstow West|⟩", subs: &[]},
            Appointment { term: (1930, 1931), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                Appointment { term: (1930, 1931), name: "<b>Chancellor of the Duchy of Lancaster</b>", subs: &[]},
                Appointment { term: (1931, 1931), name: "<b>⟨Postmaster General|⟩</b>", subs: &[]}
            ]},
            Appointment { term: (1935, 1940), name: "<b>Shadow Minister</b>, ❰UK❱ Shadow Cabinet", subs: &[
                Appointment { term: (1935, 1940), name: "<b>Leader of the Opposition</b>", subs: &[]},
            ]},
            Appointment { term: (1942, 1950), name: "<b>Cabinet Minister</b>, ❰UK❱ Cabinet", subs: &[
                Appointment { term: (1942, 1943), name: "⟨<b>Secretary of State for Dominion Affairs</b>|<b>Secretary of State for Foreign, Commonwealth and Development Affairs</b>⟩", subs: &[]},
                Appointment { term: (1942, 1945), name: "<b>Prime Minister, Deputy</b>", subs: &[]},
                Appointment { term: (1945, 1951), name: "<b>Prime Minister</b>", subs: &[]},
                Appointment { term: (1945, 1946), name: "⟨<b>Minister of Defense</b>|<b>Secretary of State for Defence</b>⟩", subs: &[]},
            ]},
            Appointment { term: (1951, 1955), name: "<b>Shadow Minister</b>, ❰UK❱ Shadow Cabinet", subs: &[
                Appointment { term: (1951, 1955), name: "<b>Leader of the Opposition</b>", subs: &[]},
            ]}
        ]},
        Appointment { term: (1935, DEAD), name: "<b>Privy Councilor</b>, ❰UK❱ Privy Council", subs: &[
            Appointment { term: (1940, 1941), name: "<b>Lord Keeper of the Privy Seal</b>", subs: &[]},
            Appointment { term: (1943, 1945), name: "<b>Lord President of the Council</b>", subs: &[]},
        ]},
        Appointment { term: (1955, DEAD), name: "<b>Politician</b>, ❰UK❱ House of Lords", subs: &[
            Appointment { term: (1955, DEAD), name: "<b>Peer</b>, Earl Attlee", subs: &[]}
        ]}
    ]
};

pub const GEORGE_FITZMAURICE: Person = Person {
    name: Name {
        full: "George John Charles Petty-Fitzmaurice",
        last: "Petty-Fitzmaurice",
    },
    span: (1912, 1999),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/george_fitzmaurice.webp"),
        sketch: asset!("assets/portrait/george_fitzmaurice.gif"),
        source: "<i>Lord Lansdowne.</i> Photograph. Accessed January 25, 2025. https://www.gettyimages.com/detail/818839658"
    }),
    appointment: &[
        Appointment { term: (1940, 1944), name: "<b>Serviceman</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[
            Appointment { term: (1940, 1944), name: "<b>Commissioned</b>, Active Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1940, 1942), name: "(OF-01) Lieutenant", subs: &[]},
                Appointment { term: (1942, 1944), name: "(OF-02) Captain", subs: &[]},
                Appointment { term: (1944, 1944), name: "(OF-03) Major", subs: &[]}
            ]}
        ]},
        Appointment { term: (1944, DEAD), name: "<b>Politician</b>, ❰UK❱ House of Lords", subs: &[
            Appointment { term: (1944, DEAD), name: "<b>Peer</b>, Marquess of Lansdowne", subs: &[
                Appointment { term: (1958, 1962), name: "<b>Secretary of State, Under</b>, ❰UK❱ ⟨Foreign Office|Foreign, Commonwealth and Development Office⟩", subs: &[]},
                Appointment { term: (1962, 1964), name: "<b>Minister of State</b>, ❰UK❱ ⟨Colonial Office|Foreign, Commonwealth and Development Office⟩", subs: &[]},
            ]}
        ]},
        Appointment { term: (1950, 1950), name: "<b>Judicial Officer</b>, District (County) of Perthshire, Scotland", subs: &[
            Appointment { term: (1950, 1950), name: "<b>Justice of the Peace</b>", subs: &[]}
        ]},
        Appointment { term: (1964, DEAD), name: "<b>Privy Councilor</b>, ❰UK❱ Privy Council", subs: &[]},
    ]
};

pub const HAROLD_NUTTING: Person = Person {
    name: Name {
        full: "Harold Anthony Nutting",
        last: "Nutting",
    },
    span: (1920, 1999),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/harold_nutting.webp"),
        sketch: asset!("assets/portrait/harold_nutting.gif"),
        source: "<i>Anthony Nutting.</i> Photograph. Accessed January 25, 2025. https://www.gettyimages.com/detail/992102778"
    }),
    appointment: &[
        Appointment { term: (NULL, NULL), name: "<b>Student</b>, University of Cambridge, Cambridge", subs: &[
            Appointment { term: (NULL, NULL), name: "Bachelor of Arts, Agriculture", subs: &[]}
        ]},
        Appointment { term: (1939, 1940), name: "<b>Serviceman</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[
            Appointment { term: (1939, 1940), name: "<b>Enlisted</b>, Active Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1939, 1940), name: "(OR-01) Private", subs: &[]}
            ]}
        ]},
        Appointment { term: (1940, 1945), name: "<b>Bureaucrat</b>, ❰UK❱ ⟨Foreign Office|Foreign, Commonwealth and Development Office⟩", subs: &[
            Appointment { term: (1940, 1945), name: "<b>Undercover Operative</b>, ❰UK❱ ⟨Foreign Intelligence Service|Secret Intelligence Service⟩", subs: &[
                Appointment { term: (1940, 1941), name: "<b>Attaché</b>, British Embassy, Paris", subs: &[]},
                Appointment { term: (1941, 1944), name: "<b>Attaché</b>, British Embassy, Madrid", subs: &[]},
                Appointment { term: (1944, 1945), name: "<b>Attaché</b>, British Embassy, Rome", subs: &[]}
            ]},
        ]},
        Appointment { term: (1945, 1956), name: "<b>Politician</b>, ❰UK❱ House of Commons", subs: &[
            Appointment { term: (1945, 1956), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Melton|⟩", subs: &[
                Appointment { term: (1951, 1954), name: "<b>Secretary of State, Under</b>, ❰UK❱ ⟨Foreign Office|Foreign, Commonwealth and Development Office⟩", subs: &[]},
                Appointment { term: (1954, 1956), name: "<b>Minister of State</b>, ❰UK❱ ⟨Foreign Office|Foreign, Commonwealth and Development Office⟩", subs: &[]},
            ]}
        ]},
        Appointment { term: (1954, DEAD), name: "<b>Privy Councilor</b>, ❰UK❱ Privy Council", subs: &[]},
    ]
};

pub const LESTER_PEARSON: Person = Person {
    name: Name {
        full: "Lester Bowles Pearson",
        last: "Pearson",
    },
    span: (1897, 1972),
    portraiture: Some(Portraiture {
        normal: asset!("assets/portrait/lester_pearson.webp"),
        sketch: asset!("assets/portrait/lester_pearson.gif"),
        source: "<i>Lester B. Pearson.</i> Photograph. Accessed January 25, 2025. https://commons.wikimedia.org/wiki/File:Lester_B._Pearson_(1963_ABC_press_photo).jpg"
    }),
    appointment: &[
        Appointment { term: (1914, 1915), name: "<b>Student</b>, University of Toronto, Toronto", subs: &[]},
        Appointment { term: (1915, 1917), name: "<b>Serviceman</b>, ❰CA❱ ⟨Department of Militia and Defence|Canadian Armed Forces⟩", subs: &[
            Appointment { term: (1915, 1917), name: "<b>Enlisted</b>, Active Component, ❰CA❱ Army", subs: &[
                Appointment { term: (1915, NULL), name: "(OR-01) Private", subs: &[]},
                Appointment { term: (NULL, 1917), name: "(OR-03) Corporal", subs: &[]}
            ]}
        ]},
        Appointment { term: (1917, 1918), name: "<b>Serviceman</b>, ❰UK❱ ⟨War Office|Ministry of Defence⟩", subs: &[
            Appointment { term: (1917, 1918), name: "<b>Commissioned</b>, Active Component, ❰UK❱ Army", subs: &[
                Appointment { term: (1917, 1918), name: "(OF-01) Lieutenant", subs: &[]}
            ]}
        ]},
        Appointment { term: (1918, 1919), name: "<b>Student</b>, University of Toronto, Toronto", subs: &[
            Appointment { term: (1918, 1919), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1921, 1925), name: "<b>Student</b>, University of Oxford, Oxford", subs: &[
            Appointment { term: (1921, 1923), name: "Bachelor of Arts, Modern History", subs: &[]},
            Appointment { term: (1923, 1925), name: "Master of Arts, Modern History", subs: &[]}
        ]},
        Appointment { term: (1925, 1928), name: "<b>Scholar</b>, University of Toronto, Toronto", subs: &[
            Appointment { term: (1925, 1928), name: "<b>Professor, Assistant</b>, Modern History", subs: &[]}
        ]},
        Appointment { term: (1928, 1946), name: "<b>Bureaucrat</b>, ❰CA❱ ⟨Department of External Affairs|Department of Foreign Affairs, Trade and Development (Global Affairs Canada)⟩", subs: &[
            Appointment { term: (1928, 1946), name: "<b>Diplomat</b>, ❰CA❱ Diplomatic Corps Services", subs: &[
                Appointment { term: (1928, 1935), name: "<b>Secretary, First</b>, Ottawa, Canada", subs: &[]},
                Appointment { term: (1935, 1941), name: "<b>Secretary, First</b>, London, England", subs: &[]},
                Appointment { term: (1942, 1944), name: "<b>Minister Counsellor</b>, Washington, DC", subs: &[]},
                Appointment { term: (1944, 1945), name: "<b>Minister Plenipotentiary</b>, Washington, DC", subs: &[]},
                Appointment { term: (1945, 1946), name: "<b>Ambassador</b>, The United States of America", subs: &[]}
            ]},
            Appointment { term: (1946, 1948), name: "<b>Secretary of State, Under</b>, ❰CA❱ ⟨Department of External Affairs|Department of Foreign Affairs, Trade and Development (Global Affairs Canada)⟩", subs: &[]}
        ]},
        Appointment { term: (1948, 1968), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1948, 1968), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Algoma East|⟩", subs: &[
                Appointment { term: (1948, 1957), name: "<b>Cabinet Minister</b>, ❰CA❱ Cabinet", subs: &[
                    Appointment { term: (1948, 1957), name: "⟨<b>Secretary of State for External Affairs</b>|<b>Minister of Foreign Affairs</b>⟩, ❰CA❱ ⟨Department of External Affairs|Department of Foreign Affairs, Trade and Development (Global Affairs Canada)⟩", subs: &[
                        Appointment { term: (1952, 1953), name: "<b>President</b>, ❰NATO❱ General Assembly", subs: &[]}
                    ]},
                ]},
                Appointment { term: (1963, 1968), name: "<b>Cabinet Minister</b>, ❰CA❱ Cabinet", subs: &[
                    Appointment { term: (1963, 1968), name: "Prime Minister", subs: &[]}
                ]}
            ]}
        ]},
        Appointment { term: (1948, DEAD), name: "<b>Privy Councilor</b>, ❰CA❱ Privy Council", subs: &[
            Appointment { term: (1931, 1931), name: "<b>Secretary</b>, Royal Commission on Trading in Grain Futures", subs: &[]},
            Appointment { term: (1934, 1935), name: "<b>Secretary</b>, Royal Commission on Price Spreads", subs: &[]}
        ]}
    ]
};

pub const SOLON_LOW: Person = Person {
    name: Name {
        full: "Solon Earl Low",
        last: "Low",
    },
    span: (1900, 1962),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/solon_low.gif"),
        normal: asset!("assets/portrait/solon_low.webp"),
        source: "<i>Solon E. Low.</i> Photograph. Accessed January 25, 2025. https://www.newspapers.com/image/481781771/"
    }),
    appointment: &[
        Appointment { term: (NULL, NULL), name: "<b>Bureaucrat</b>, Municipality (Village) of Arrowwood, Alberta", subs: &[
            Appointment { term: (NULL, NULL), name: "<b>Secretary</b>, Board of Trade", subs: &[]}
        ]},
        Appointment { term: (NULL, NULL), name: "<b>Bureaucrat</b>, Municipality (Village) of Stirling, Alberta", subs: &[
            Appointment { term: (NULL, NULL), name: "<b>President</b>, Board of Trade", subs: &[]}
        ]},
        Appointment { term: (1935, 1945), name: "<b>Politician</b>, ❰CA-AB❱ Alberta Legislative Assembly", subs: &[
            Appointment { term: (1935, 1940), name: "<b>Member of the Legislative Assembly (MLA)</b>, Warner", subs: &[
                Appointment { term: (1937, 1940), name: "<b>Cabinet Minister</b>, ❰CA-AB❱ Executive Council (Cabinet)", subs: &[
                    Appointment { term: (1937, 1940), name: "⟨<b>Provincial Treasurer</b>|<b>Minister of Finance</b>⟩", subs: &[]},
                ]}
            ]},
            Appointment { term: (1940, 1944), name: "<b>Member of the Legislative Assembly (MLA)</b>, Vegreville", subs: &[
                Appointment { term: (1940, 1944), name: "<b>Cabinet Minister</b>, ❰CA-AB❱ Executive Council (Cabinet)", subs: &[
                    Appointment { term: (1940, 1942), name: "⟨<b>Provincial Treasurer</b>|<b>Minister of Finance</b>⟩", subs: &[]},
                    Appointment { term: (1942, 1944), name: "<b>Minister of Education</b>", subs: &[]},
                ]},
            ]},
            Appointment { term: (1944, 1945), name: "<b>Member of the Legislative Assembly (MLA)</b>, Warner", subs: &[]}
        ]},
        Appointment { term: (1945, 1958), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1945, 1958), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Peace River|⟩", subs: &[]}
        ]},
        Appointment { term: (1961, DEAD), name: "<b>Judicial Officer</b>, ❰CA-AB❱ ⟨Provincial Court of Alberta|Alberta Court of Justice⟩", subs: &[
            Appointment { term: (1961, DEAD), name: "<b>Judge</b>", subs: &[]}
        ]},
    ]
};

pub const FREDERICK_LOW: Person = Person {
    name: Name {
        full: "Frederick Ferdinand Low",
        last: "Low",
    },
    span: (1828, 1894),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/frederick_low.gif"),
        normal: asset!("assets/portrait/frederick_low.webp"),
        source: "<i>Frederick Ferdinand Low.</i> Photograph. Accessed February 6, 2025. https://hd.housedivided.dickinson.edu/node/42862"
    }),
    appointment: &[
        Appointment { term: (1862, 1863), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1862, 1863), name: "<b>Representative</b>, California", subs: &[]}
        ]},
        Appointment { term: (1863, 1867), name: "<b>Politician</b>, ❰US-CA❱ California Office of the Governor", subs: &[
            Appointment { term: (1863, 1867), name: "<b>Governor</b>", subs: &[]}
        ]},
        Appointment { term: (1869, 1873), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1869, 1873), name: "<b>Diplomat</b>, ❰US❱ ⟨Diplomatic Service|Foreign Service⟩", subs: &[
                Appointment { term: (1869, 1873), name: "<b>Minister Plenipotentiary</b>, ⟨Great Qing|People's Republic of China⟩", subs: &[]}
            ]}
        ]},
    ]
};


pub const CHARLES_DELONG: Person = Person {
    name: Name {
        full: "Charles Egbert DeLong",
        last: "DeLong",
    },
    span: (1832, 1876),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/charles_delong.gif"),
        normal: asset!("assets/portrait/charles_delong.webp"),
        source: "<i>Charles DeLong.</i> Photograph. Accessed February 7, 2025. https://commons.wikimedia.org/wiki/File:Charles_DeLong.jpg"
    }),
    appointment: &[
        Appointment { term: (1858, 1859), name: "<b>Politician</b>, ❰US-CA❱ California State Assembly", subs: &[
            Appointment { term: (1858, 1859), name: "<b>Assemblyman</b>, ⟨Yuba County|3rd district⟩", subs: &[]}
        ]},
        Appointment { term: (1861, 1862), name: "<b>Politician</b>, ❰US-CA❱ California State Senate", subs: &[
            Appointment { term: (1861, 1862), name: "<b>Senator</b>, ⟨Yuba County|1st district⟩", subs: &[]}
        ]},
        Appointment { term: (1869, 1873), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1869, 1873), name: "<b>Diplomat</b>, ❰US❱ ⟨Diplomatic Service|Foreign Service⟩", subs: &[
                Appointment { term: (1869, 1872), name: "⟨<b>Minister Resident</b>|⟩, ⟨Empire of Japan|⟩", subs: &[]},
                Appointment { term: (1872, 1873), name: "⟨<b>Minister Plenipotentiary</b>|⟩, ⟨Empire of Japan|⟩", subs: &[]}
            ]}
        ]},
        Appointment { term: (1871, 1871), name: "<b>Bureaucrat</b>, ❰⟨<b>HI</b>|US-HI⟩❱ ⟨Ministry of Foreign Affairs|⟩", subs: &[
            Appointment { term: (1871, 1871), name: "<b>Diplomat</b>, ❰⟨<b>HI</b>|US-HI⟩❱ ⟨Diplomatic Corps|⟩", subs: &[
                Appointment { term: (1871, 1871), name: "⟨<b>Minister Plenipotentiary</b>|⟩, ⟨Empire of Japan|⟩", subs: &[]}
            ]}
        ]},
    ]
};

pub const MAJOR_COLDWELL: Person = Person {
    name: Name {
        full: "Major James William Coldwell",
        last: "Coldwell",
    },
    span: (1888, 1974),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/major_coldwell.gif"),
        normal: asset!("assets/portrait/major_coldwell.webp"),
        source: "<i>M. J. Coldwell.</i> Photograph. Accessed January 25, 2025. https://www.gettyimages.com/detail/50498241"
    }),
    appointment: &[
        Appointment { term: (1922, 1932), name: "<b>Politician</b>, Municipality (City) of Regina, Saskatchewan", subs: &[
            Appointment { term: (1922, 1932), name: "<b>Alderman</b>, Regina City Council", subs: &[]}
        ]},
        Appointment { term: (1929, 1930), name: "<b>Politician</b>, ❰CA-SK❱ Saskatchewan Legislative Assembly", subs: &[
            Appointment { term: (1929, 1930), name: "<b>Chairman</b>, Public Service Inquiry Commission", subs: &[]}
        ]},
        Appointment { term: (1935, 1958), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1935, 1958), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Rosetown—Biggar|⟩", subs: &[]}
        ]},
        Appointment { term: (1964, 1966), name: "<b>Bureaucrat</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1964, 1966), name: "<b>Member</b>, Advisory Committee to Study Curtailment of Election Expenditures", subs: &[]}
        ]},
        Appointment { term: (1964, DEAD), name: "<b>Privy Councilor</b>, ❰CA❱ Privy Council", subs: &[
            Appointment { term: (1966, 1969), name: "<b>Commissioner</b>, Royal Commission on Security", subs: &[]}
        ]},
    ]
};

pub const PAUL_MARTIN_SR: Person = Person {
    name: Name {
        full: "Paul Joseph James Guillaume Martin Sr.",
        last: "Martin",
    },
    span: (1903, 1992),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/paul_martin_1.gif"),
        normal: asset!("assets/portrait/paul_martin_1.webp"),
        source: "<i>Paul Martin Sr.</i> Photograph. Accessed January 31, 2025. https://canadaehx.com/2023/02/01/paul-martin-sr/"
    }),
    appointment: &[
        Appointment { term: (1918, 1921), name: "<b>Student</b>, Saint Alexander's College, Gatineau", subs: &[]},
        Appointment { term: (1921, 1928), name: "<b>Student</b>, St. Michael's College, Toronto", subs: &[
            Appointment { term: (1921, 1925), name: "Bachelor of Arts, Philosophy", subs: &[]},
            Appointment { term: (1925, 1928), name: "Master of Arts, Philosophy", subs: &[]}
        ]},
        Appointment { term: (1925, 1928), name: "<b>Student</b>, Osgoode Hall Law School, Toronto", subs: &[
            Appointment { term: (1925, 1928), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1925, 1928), name: "<b>Legal Practitioner</b>, “Henderson & McGuire”", subs: &[]},
        Appointment { term: (1928, 1929), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1928, 1929), name: "Master of Laws", subs: &[]}
        ]},
        Appointment { term: (1930, 1933), name: "<b>Legal Practitioner</b>, “McTague, Clark, Springsteen, Racine, & Spencer”", subs: &[]},
        Appointment { term: (1930, DEAD), name: "<b>Scholar</b>, ⟨Assumption College|Assumption University⟩, Windsor", subs: &[
            Appointment { term: (1930, NULL), name: "<b>Lecturer</b>, Political Science", subs: &[]},
            Appointment { term: (NULL, DEAD), name: "<b>Professor, Adjunct</b>, Political Science", subs: &[]}
        ]},
        Appointment { term: (1933, 1944), name: "<b>Legal Practitioner</b>, “Martin & Laird”", subs: &[]},
        Appointment { term: (1944, 1950), name: "<b>Legal Practitioner</b>, “Martin, Laird & Easton”", subs: &[]},
        Appointment { term: (1950, 1957), name: "<b>Legal Practitioner</b>, “Martin, Laird, Easton & Cowan”", subs: &[]},
        Appointment { term: (1957, 1966), name: "<b>Legal Practitioner</b>, “Martin, Laird, Easton, Cowan & Chauvin”", subs: &[]},
        Appointment { term: (1967, 1972), name: "<b>Legal Practitioner</b>, “Martin, Laird & Cowan”", subs: &[]},
        Appointment { term: (1935, 1966), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1935, 1968), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Essex East|⟩", subs: &[
                Appointment { term: (1943, 1945), name: "⟨<b>Assistant, Parliamentary</b>|<b>Secretary, Parliamentary</b>⟩, ❰CA❱ ⟨Department of Labour|Employment and Social Development Canada⟩", subs: &[]},
                Appointment { term: (1945, 1950), name: "<b>Cabinet Minister</b>, ❰CA❱ Cabinet", subs: &[
                    Appointment { term: (1945, 1946), name: "⟨<b>Secretary of State for Canada</b>|⟩", subs: &[]},
                    Appointment { term: (1946, 1957), name: "⟨<b>Minister of National Health and Welfare</b>|<b>Minister of Health</b>⟩, ❰CA❱ ⟨Department of National Health and Welfare|Health Canada⟩", subs: &[]},
                    Appointment { term: (1947, 1948), name: "<b>Chairman</b>, Special Committee on Prices", subs: &[]},
                    Appointment { term: (1950, 1950), name: "<b>Minister of Labour</b>, ❰CA❱ ⟨Department of Labour|Employment and Social Development Canada⟩", subs: &[]},
                ]},
                Appointment { term: (1963, 1968), name: "<b>Cabinet Minister</b>, ❰CA❱ Cabinet", subs: &[
                    Appointment { term: (1963, 1968), name: "⟨<b>Secretary of State for External Affairs</b>|<b>Minister of Foreign Affairs</b>⟩, ❰CA❱ ⟨Department of External Affairs|Department of Foreign Affairs, Trade and Development (Global Affairs Canada)⟩", subs: &[]},
                ]}
            ]}
        ]},
        Appointment { term: (1945, DEAD), name: "<b>Privy Councilor</b>, ❰CA❱ Privy Council", subs: &[]},
        Appointment { term: (1968, 1974), name: "<b>Politician</b>, ❰CA❱ Senate", subs: &[
            Appointment { term: (1968, 1974), name: "<b>Senator</b>, Windsor—Walkerville", subs: &[
                Appointment { term: (1968, 1974), name: "<b>Cabinet Minister</b>, ❰CA❱ Cabinet", subs: &[
                    Appointment { term: (1968, 1974), name: "⟨<b>Leader of the Government in the Senate</b>|<b>Representative of the Government in the Senate</b>⟩", subs: &[]},
                ]}
            ]}
        ]},
        Appointment { term: (1974, 1979), name: "<b>Bureaucrat</b>, ❰CA❱ ⟨Department of External Affairs|Department of Foreign Affairs, Trade and Development (Global Affairs Canada)⟩", subs: &[
            Appointment { term: (1974, 1979), name: "<b>Diplomat</b>, ❰CA❱ Diplomatic Corps Services", subs: &[
                Appointment { term: (1974, 1979), name: "<b>Ambassador (High Commissioner)</b>, The United Kingdom of Great Britain and Northern Ireland", subs: &[]}
            ]}
        ]},
    ]
};

pub const THOMAS_REID: Person = Person {
    name: Name {
        full: "Thomas Reid",
        last: "Reid",
    },
    span: (1886, 1968),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/thomas_reid.gif"),
        normal: asset!("assets/portrait/thomas_reid.webp"),
        source: "<i>The Hon. Thomas Reid.</i> Photograph. Accessed January 31, 2025. https://lop.parl.ca/sites/ParlInfo/default/en_CA/People/Profile?personId=15145"
    }),
    appointment: &[
        Appointment { term: (1922, 1930), name: "<b>Politician</b>, Municipality (City) of Surrey, British Columbia", subs: &[
            Appointment { term: (1922, 1924), name: "<b>Council Member</b>, Surrey City Council", subs: &[]},
            Appointment { term: (1924, 1930), name: "<b>Reeve</b>, Surrey City Council", subs: &[]}
        ]},
        Appointment { term: (1930, 1949), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1930, 1949), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨New Westminster|⟩", subs: &[
                Appointment { term: (1945, 1947), name: "<b>Chairman</b>, Standing Committee on Marine and Fisheries", subs: &[]},
                Appointment { term: (1948, 1948), name: "⟨<b>Assistant, Parliamentary</b>|<b>Secretary, Parliamentary</b>⟩, ❰CA❱ ⟨Department of Fisheries|Fisheries and Oceans Canada⟩", subs: &[]},
                Appointment { term: (1948, 1949), name: "⟨<b>Assistant, Parliamentary</b>|<b>Secretary, Parliamentary</b>⟩, ❰CA❱ ⟨Department of National Revenue|Canada Revenue Agency⟩", subs: &[]},
                Appointment { term: (1949, 1949), name: "⟨<b>Assistant, Parliamentary</b>|<b>Secretary, Parliamentary</b>⟩, ❰CA❱ ⟨Department of National Health and Welfare|Health Canada⟩", subs: &[]},
            ]}
        ]},
        Appointment { term: (1949, 1967), name: "<b>Politician</b>, ❰CA❱ Senate", subs: &[
            Appointment { term: (1949, 1967), name: "<b>Senator</b>, British Columbia", subs: &[
                Appointment { term: (1955, 1955), name: "<b>Chairman</b>, Special Committee on Traffic in Narcotic Drugs in Canada", subs: &[]}
            ]}
        ]}
    ]
};

pub const ALISTAIR_STEWART: Person = Person {
    name: Name {
        full: "Alistair McLeod Stewart",
        last: "Stewart",
    },
    span: (1905, 1970),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/alistair_steward.gif"),
        normal: asset!("assets/portrait/alistair_steward.webp"),
        source: "<i>Alistair Stewart.</i> Photograph. Accessed February 2, 2025. https://www.newspapers.com/image/39288714/"
    }),
    appointment: &[
        Appointment { term: (NULL, NULL), name: "<b>Student</b>, University of Edinburgh, Edinburgh", subs: &[
            Appointment { term: (NULL, NULL), name: "Bachelor of Science, Agriculture", subs: &[]}
        ]},
        Appointment { term: (1945, 1958), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1945, 1958), name: "<b>Member of Parliament (MP)</b>, Electoral District of Winnipeg North", subs: &[]}
        ]},
    ]
};

pub const HAZEN_ARGUE: Person = Person {
    name: Name {
        full: "Hazen Robert Argue",
        last: "Argue",
    },
    span: (1921, 1991),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/hazen_argue.gif"),
        normal: asset!("assets/portrait/hazen_argue.webp"),
        source: "<i>Hazen Argue.</i> Photograph. Accessed January 31, 2025. https://commons.wikimedia.org/wiki/File:Hazen_Argue.jpg"
    }),
    appointment: &[
        Appointment { term: (1940, 1944), name: "<b>Student</b>, University of Saskatchewan, Saskatoon", subs: &[
            Appointment { term: (1940, 1944), name: "Bachelor of Science, Agriculture", subs: &[]}
        ]},
        Appointment { term: (1945, 1949), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1945, 1949), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Wood Mountain|⟩", subs: &[]},
        ]},
        Appointment { term: (1949, 1963), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1949, 1963), name: "<b>Member of Parliament (MP)</b>, Electoral District of Assiniboia", subs: &[]},
        ]},
        Appointment { term: (1966, 1991), name: "<b>Politician</b>, ❰CA❱ Senate", subs: &[
            Appointment { term: (1966, 1991), name: "<b>Senator</b>, Saskatchewan", subs: &[
                Appointment { term: (1972, 1979), name: "<b>Chairman</b>, Standing Committee on Agriculture", subs: &[]},
                Appointment { term: (1980, 1984), name: "<b>Cabinet Minister</b>, ❰CA❱ Cabinet", subs: &[
                    Appointment { term: (1980, 1984), name: "<b>Minister of State</b>, Canadian Wheat Board, ❰CA❱ ⟨Department of Agriculture|Agriculture and Agri-Food Canada⟩", subs: &[]},
                ]},
                Appointment { term: (1986, 1988), name: "<b>Chairman</b>, Special Committee on Preventive Health Care", subs: &[]},
            ]}
        ]},
        Appointment { term: (1980, DEAD), name: "<b>Privy Councilor</b>, ❰CA❱ Privy Council", subs: &[]},
    ]
};

pub const ALLISTER_GROSART: Person = Person {
    name: Name {
        full: "Allister Herbert George Grosart",
        last: "Grosart",
    },
    span: (1906, 1984),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/allister_grosart.gif"),
        normal: asset!("assets/portrait/allister_grosart.webp"),
        source: "<i>Grosart, Allister.</i> Photograph. Accessed January 31, 2025. https://digitalarchive.tpl.ca/objects/269889/grosart-allister"
    }),
    appointment: &[
        Appointment { term: (1923, 1927), name: "<b>Student</b>, University of Toronto, Toronto", subs: &[
            Appointment { term: (1923, 1927), name: "Bachelor of Arts, Political Science", subs: &[]}
        ]},
        Appointment { term: (1939, 1945), name: "<b>Serviceman</b>, ❰CA❱ ⟨Department of Militia and Defence|Canadian Armed Forces⟩", subs: &[
            Appointment { term: (1939, 1945), name: "<b>Commissioned</b>, Reserve Component, ❰CA❱ Army", subs: &[
                Appointment { term: (1939, NULL), name: "(OF-01) Lieutenant", subs: &[]},
                Appointment { term: (NULL, NULL), name: "(OF-02) Captain", subs: &[]},
                Appointment { term: (NULL, 1945), name: "(OF-03) Major", subs: &[]}
            ]}
        ]},
        Appointment { term: (1962, 1981), name: "<b>Politician</b>, ❰CA❱ Senate", subs: &[
            Appointment { term: (1962, 1981), name: "<b>Senator</b>, Ontario", subs: &[
                Appointment { term: (1973, 1976), name: "<b>Chairman, Vice</b>, Standing Committee on Foreign Affairs", subs: &[]},
                Appointment { term: (1979, 1979), name: "<b>Chairman</b>, Standing Joint Committee on the Restaurant of Parliament", subs: &[]},
                Appointment { term: (1979, 1979), name: "<b>Chairman</b>, Standing Joint Committee on the Library of Parliament", subs: &[]},
                Appointment { term: (1979, 1980), name: "<b>Speaker of the Senate</b>", subs: &[]}
            ]}
        ]},
        Appointment { term: (1981, DEAD), name: "<b>Privy Councilor</b>, ❰CA❱ Privy Council", subs: &[]},
    ]
};

pub const MITCHELL_SHARP: Person = Person {
    name: Name {
        full: "Mitchell William Sharp",
        last: "Sharp",
    },
    span: (1911, 2004),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/mitchell_sharp.gif"),
        normal: asset!("assets/portrait/mitchell_sharp.webp"),
        source: "<i>Mitchell Sharp.</i> Photograph. Accessed February 1, 2025. https://digitalarchive.tpl.ca/objects/310950/mitchell-sharp-commissioner-northern-pipeline-agency"
    }),
    appointment: &[
        Appointment { term: (1930, 1934), name: "<b>Student</b>, University of Manitoba, Winnipeg", subs: &[
            Appointment { term: (1930, 1934), name: "Bachelor of Arts, Economics", subs: &[]}
        ]},
        Appointment { term: (1942, 1958), name: "<b>Bureaucrat</b>, ❰CA❱ ⟨Civil Service|Public Service⟩", subs: &[
            Appointment { term: (1942, 1958), name: "<b>Civil Servant</b>, ❰CA❱ ⟨Department of Trade and Commerce|Department of Foreign Affairs, Trade and Development (Global Affairs Canada)⟩", subs: &[
                Appointment { term: (1942, 1946), name: "<b>Clerk</b>", subs: &[]},
                Appointment { term: (1946, 1947), name: "<b>Executive Assistant</b>", subs: &[]},
                Appointment { term: (1947, 1951), name: "<b>Director</b>, Economic Policy Division", subs: &[]},
                Appointment { term: (1951, 1957), name: "<b>Deputy Minister, Associate</b>", subs: &[]},
                Appointment { term: (1957, 1958), name: "<b>Deputy Minister</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1963, 1978), name: "<b>Politician</b>, ❰CA❱ House of Commons", subs: &[
            Appointment { term: (1963, 1978), name: "<b>Member of Parliament (MP)</b>, Electoral District of ⟨Eglinton|⟩", subs: &[
                Appointment { term: (1963, 1974), name: "<b>Cabinet Minister</b>, ❰CA❱ Cabinet", subs: &[
                    Appointment { term: (1963, 1966), name: "⟨<b>Minister of Trade and Commerce</b>|<b>Minister for International Trade</b>⟩, ❰CA❱ ⟨Department of Trade and Commerce|Department of Foreign Affairs, Trade and Development (Global Affairs Canada)⟩", subs: &[]},
                    Appointment { term: (1965, 1968), name: "<b>Minister of Finance</b>, ❰CA❱ Department of Finance", subs: &[]},
                    Appointment { term: (1968, 1974), name: "⟨<b>Secretary of State for External Affairs</b>|<b>Minister of Foreign Affairs</b>⟩, ❰CA❱ ⟨Department of External Affairs|Department of Foreign Affairs, Trade and Development (Global Affairs Canada)⟩", subs: &[]},
                ]},
                Appointment { term: (1974, 1976), name: "<b>Leader of the Government in the House of Commons</b>", subs: &[]},
                Appointment { term: (1974, 1978), name: "<b>Chairman</b>, Standing Committee of Selection", subs: &[]},
                Appointment { term: (1974, 1978), name: "<b>Chairman</b>, Standing Committee on Procedure and Organization", subs: &[]},
            ]}
        ]},
        Appointment { term: (1963, DEAD), name: "<b>Privy Councilor</b>, ❰CA❱ Privy Council", subs: &[
            Appointment { term: (1974, 1976), name: "<b>President of the King's Privy Council for Canada</b>", subs: &[]},
            Appointment { term: (1993, 2003), name: "<b>Advisor</b>, ❰CA❱ Office of the Prime Minister", subs: &[]},
        ]},
    ]
};

pub const ANSON_BURLINGAME: Person = Person {
    name: Name {
        full: "Anson Burlingame",
        last: "Burlingame",
    },
    span: (1820, 1870),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/anson_burlingame.gif"),
        normal: asset!("assets/portrait/anson_burlingame.webp"),
        source: "<i>Anson Burlingame.</i> Photograph. Accessed February 1, 2025. https://www.findagrave.com/memorial/7500072/anson-burlingame"
    }),
    appointment: &[
        Appointment { term: (1838, 1841), name: "<b>Student</b>, University of Michigan, Detroit", subs: &[
            Appointment { term: (1838, 1841), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1844, 1846), name: "<b>Student</b>, Harvard University, Cambridge", subs: &[
            Appointment { term: (1844, 1846), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1852, 1854), name: "<b>Politician</b>, ❰US-MA❱ Massachusetts Senate", subs: &[
            Appointment { term: (1852, 1854), name: "<b>Senator</b>", subs: &[]}
        ]},
        Appointment { term: (1855, 1861), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1855, 1861), name: "<b>Representative</b>, Massachusetts", subs: &[]}
        ]},
        Appointment { term: (1862, 1867), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1862, 1867), name: "<b>Diplomat</b>, ❰US❱ ⟨Diplomatic Service|Foreign Service⟩", subs: &[
                Appointment { term: (1862, NULL), name: "⟨<b>Minister Resident</b>|<b>Minister Counselor</b>⟩, ⟨Great Qing|People's Republic of China⟩", subs: &[]},
                Appointment { term: (NULL, 1867), name: "⟨<b>Minister Plenipotentiary</b>|<b>Ambassador Plenipotentiary</b>⟩, ⟨Great Qing|People's Republic of China⟩", subs: &[]}
            ]}
        ]},
        Appointment { term: (1867, DEAD), name: "<b>Bureaucrat</b>, ❰CN❱ ⟨Grand Council (軍機處)|⟩", subs: &[
            Appointment { term: (1867, DEAD), name: "<b>Diplomat</b>, ❰CN❱ ⟨Zongli Yamen (總理衙門)|⟩", subs: &[
                Appointment { term: (1867, DEAD), name: "⟨<b>Minister Plenipotentiary</b>|<b>Ambassador-at-large</b>⟩", subs: &[]}
            ]}
        ]}
    ]
};

pub const CHARLES_DENBY: Person = Person {
    name: Name {
        full: "Charles Harvey Denby",
        last: "Denby",
    },
    span: (1830, 1904),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/charles_denby.gif"),
        normal: asset!("assets/portrait/charles_denby.webp"),
        source: "<i>Charles Denby.</i> Photograph. Accessed February 1, 2025. https://commons.wikimedia.org/wiki/File:Charles_Denby_-_circa_1906_(cropped).jpg"
    }),
    appointment: &[
        Appointment { term: (NULL, NULL), name: "<b>Student</b>, ⟨Georgetown College|Georgetown University⟩, Washington", subs: &[]},
        Appointment { term: (NULL, 1850), name: "<b>Student</b>, Virginia Military Institute, Lexington", subs: &[
            Appointment { term: (NULL, 1850), name: "Bachelor of Arts", subs: &[]}
        ]},
        Appointment { term: (1856, 1857), name: "<b>Politician</b>, ❰US-IN❱ Indiana House of Representatives", subs: &[
            Appointment { term: (1856, 1857), name: "<b>Representative</b>, Vanderburgh County", subs: &[]}
        ]},
        Appointment { term: (1861, 1863), name: "<b>Serviceman</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1861, 1863), name: "<b>Commissioned</b>, Active Component, ❰US❱ Army", subs: &[
                Appointment { term: (1861, 1862), name: "(OF-04) <b>Colonel, Lieutenant</b>", subs: &[]},
                Appointment { term: (1862, 1863), name: "(OF-05) <b>Colonel</b>", subs: &[]}
            ]},
        ]},
        Appointment { term: (1885, 1898), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1885, 1898), name: "<b>Diplomat</b>, ❰US❱ ⟨Diplomatic Service|Foreign Service⟩", subs: &[
                Appointment { term: (1885, 1898), name: "⟨<b>Minister Plenipotentiary</b>|<b>Ambassador Plenipotentiary</b>⟩, ⟨Great Qing|People's Republic of China⟩", subs: &[]}
            ]}
        ]}
    ]
};

pub const PETER_PARKER: Person = Person {
    name: Name {
        full: "Peter Parker",
        last: "Parker",
    },
    span: (1804, 1888),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/peter_parker.gif"),
        normal: asset!("assets/portrait/peter_parker.webp"),
        source: "<i>Hon. Parker.</i> Photograph. Accessed February 19, 2025. https://catalog.archives.gov/id/528706"
    }),
    appointment: &[
        Appointment { term: (1826, 1830), name: "<b>Student</b>, Amherst College, Amherst", subs: &[]},
        Appointment { term: (1830, 1834), name: "<b>Student</b>, Yale University, New Haven", subs: &[
            Appointment { term: (1830, 1831), name: "Bachelor of Arts", subs: &[]},
            Appointment { term: (1831, 1834), name: "Doctor of Medicine", subs: &[]}
        ]},
        Appointment { term: (1845, 1857), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1845, 1857), name: "<b>Diplomat</b>, ❰US❱ ⟨Diplomatic Service|Foreign Service⟩", subs: &[
                Appointment { term: (1845, 1847), name: "<b>Secretary</b>, Macao, ⟨Great Qing|People's Republic of China⟩", subs: &[]},
                Appointment { term: (1847, 1855), name: "<b>Charge d’Affaires</b>, Macao, ⟨Great Qing|People's Republic of China⟩", subs: &[]},
                Appointment { term: (1855, 1857), name: "⟨<b>Minister Plenipotentiary</b>|<b>Ambassador Plenipotentiary</b>⟩, Macao, ⟨Great Qing|People's Republic of China⟩", subs: &[]}
            ]}
        ]},
        Appointment { term: (1857, 1865), name: "<b>Bureaucrat<b>, ❰US❱ Office of the President", subs: &[
            Appointment { term: (1857, 1865), name: "<b>Advisor</b>", subs: &[]}
        ]}
    ]
};

pub const GIDEON_NYE_2: Person = Person {
    name: Name {
        full: "Gideon Nye, Jr",
        last: "Nye",
    },
    span: (1812, 1888),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/gideon_nye_2.gif"),
        normal: asset!("assets/portrait/gideon_nye_2.webp"),
        source: "<i>Gidean Nye.</i> Photograph. Accessed February 19, 2025. https://m.thepaper.cn/kuaibao_detail.jsp?contid=23281055"
    }),
    appointment: &[
        Appointment { term: (1845, NULL), name: "<b>Bureaucrat</b>, ❰CL❱ Ministry of Foreign Affairs", subs: &[
            Appointment { term: (1845, NULL), name: "<b>Diplomat</b>, ❰CL❱ Diplomatic Corps", subs: &[
                Appointment { term: (1845, NULL), name: "b>Consul</b>, ⟨Canton|Guangzhou⟩, ⟨Great Qing|People's Republic of China⟩", subs: &[]}
            ]}
        ]},
        Appointment { term: (1857, DEAD), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1857, DEAD), name: "<b>Diplomat</b>, ❰US❱ ⟨Diplomatic Service|Foreign Service⟩", subs: &[
                Appointment { term: (1857, DEAD), name: "<b>Consul, Vice</b>, Macao, ⟨Great Qing|People's Republic of China⟩", subs: &[]}
            ]}
        ]}
    ]
};

pub const CHARLES_CONRAD: Person = Person {
    name: Name {
        full: "Charles Magill Conrad",
        last: "Conrad",
    },
    span: (1804, 1878),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/charles_conrad.gif"),
        normal: asset!("assets/portrait/charles_conrad.webp"),
        source: "<i>Charles Magill Conrad.</i> Photograph. Accessed February 19, 2025. https://commons.wikimedia.org/wiki/File:Charles_Magill_Conrad.jpg"
    }),
    appointment: &[
        Appointment { term: (1839, 1840), name: "<b>Politician</b>, ❰US-LA❱ Louisiana House of Representatives", subs: &[
            Appointment { term: (1839, 1840), name: "<b>v</b>", subs: &[]}
        ]},
        Appointment { term: (1841, 1843), name: "<b>Politician</b>, ❰US❱ Senate", subs: &[
            Appointment { term: (1841, 1843), name: "<b>Senator</b>, Louisiana", subs: &[]}
        ]},
        Appointment { term: (1849, 1850), name: "<b>Politician</b>, ❰US❱ House of Representatives", subs: &[
            Appointment { term: (1849, 1850), name: "<b>Representative</b>, Louisiana", subs: &[]}
        ]},
        Appointment { term: (1850, 1853), name: "<b>Bureaucrat</b>, ❰US❱ ⟨Department of War|Department of Defense⟩", subs: &[
            Appointment { term: (1850, 1853), name: "<b>⟨Secretary of War|Secretary of Defence⟩</b>", subs: &[]},
            Appointment { term: (1851, 1851), name: "<b>Secretary of the Navy</b>", subs: &[]},
        ]},
        Appointment { term: (1852, 1852), name: "<b>Bureaucrat</b>, ❰US❱ Department of State", subs: &[
            Appointment { term: (1852, 1852), name: "<b>Secretary State</b>", subs: &[]},
        ]},
    ]
};

pub const ALLEN_DEVANEY: Person = Person {
    name: Name {
        full: "Allen Charles Devaney",
        last: "Devaney",
    },
    span: (1904, 1956),
    portraiture: None,
    appointment: &[
        Appointment { term: (1924, 1928), name: "<b>Student</b>, Georgetown College, Georgetown", subs: &[
            Appointment { term: (1924, 1928), name: "Bachelor of Laws", subs: &[]}
        ]},
        Appointment { term: (1928, 1942), name: "<b>Judicial Officer</b>, ❰US❱ Department of Justice", subs: &[
            Appointment { term: (1928, NULL), name: "<b>Law Clerk</b>, Office of the Solicitor of the Treasury", subs: &[]},
            Appointment { term: (1928, NULL), name: "<b>Law Clerk</b>, Office of the Solicitor of Commerce", subs: &[]},
            Appointment { term: (NULL, 1942), name: "<b>Chief Examiner</b>, ❰US❱ Immigration and Naturalization Service", subs: &[]},
        ]},
        Appointment { term: (1943, DEAD), name: "<b>Bureaucrat</b>, ❰US❱ Immigration and Naturalization Service", subs: &[
            Appointment { term: (1943, 1953), name: "<b>Commissioner, Assistant</b>, Adjudications Division", subs: &[]},
            Appointment { term: (1953, DEAD), name: "<b>Commissioner, Assistant</b>, Inspections and Examinations Division", subs: &[]},
        ]},
    ]
};

pub const SHEN_BAOZHEN: Person = Person {
    name: Name {
        full: "Shen Baozhen (沈葆楨)",
        last: "Shen (沈)",
    },
    span: (1820, 1879),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/shen_baozhen.gif"),
        normal: asset!("assets/portrait/shen_baozhen.webp"),
        source: "<i>Shen Baozhen.</i> Photograph. Accessed February 10, 2025. https://commons.wikimedia.org/wiki/File:%E6%B2%88%E8%91%86%E6%A5%A8%EF%BC%88%E9%81%A0%E6%9D%B1%E6%9C%88%E5%88%8A%E6%9C%89%E8%AA%A4%EF%BC%89.jpg"
    }),
    appointment: &[
        Appointment { term: (NULL, 1847), name: "<b>Student</b>", subs: &[
            Appointment { term: (NULL, 1836), name: "County Examination (秀才科)", subs: &[]},
            Appointment { term: (1836, 1840), name: "Provincial Examination (鄉試)", subs: &[]},
            Appointment { term: (1840, 1847), name: "Metropolitan Examination (會試)", subs: &[]}
        ]},
        Appointment { term: (1836, DEAD), name: "<b>Scholar</b>", subs: &[
            Appointment { term: (1836, 1840), name: "<b>Xiucai (秀才)</b>", subs: &[]},
            Appointment { term: (1840, 1847), name: "<b>Juren (舉人)</b>", subs: &[]},
            Appointment { term: (1847, DEAD), name: "<b>Jinshi (進士)</b>", subs: &[]}
        ]},
        Appointment { term: (1855, DEAD), name: "<b>Bureaucrat</b>", subs: &[
            Appointment { term: (1854, 1855), name: "<b>Investigating Censor (监察御史)</b>, Province (省) of Jiangnan (江南)", subs: &[]},
            Appointment { term: (1855, 1855), name: "<b>Investigating Censor (监察御史)</b>, Province (省) of Guizhou (贵州)", subs: &[]},
            Appointment { term: (1855, 1856), name: "<b>Magistrate, Prefectural (知府)</b>, District (Prefecture; 府) of Jiujiang (九江), Province (省) of Jiangxi (江西)", subs: &[]},
            Appointment { term: (1856, 1857), name: "<b>Magistrate, Prefectural (知府)</b>, District (Prefecture; 府) of Guangxin (广信), Province (省) of Jiangxi (江西)", subs: &[]},
            Appointment { term: (1857, 1859), name: "<b>Circuit Intendant (Taotai; 道臺)</b>, Super-District (Circuit; 道) of Guangrao-Jiunan (广饶九南), Province (省) of Jiangxi (江西)", subs: &[]},
            Appointment { term: (1860, 1860), name: "<b>Circuit Intendant (Taotai; 道臺)</b>, Super-District (Circuit; 道) of Jinan-Ganning (吉南贛寧), Province (省) of Jiangxi (江西)", subs: &[]},
            Appointment { term: (1861, 1865), name: "<b>Governor, Provincial (巡撫)</b>, Province (省) of Jiangxi (江西)", subs: &[]},
            Appointment { term: (1861, 1865), name: "<b>Right Censor-in-chief, Vice (右副都御史)</b>", subs: &[]},
            Appointment { term: (NULL, NULL), name: "<b>Minister of War, Vice (兵部侍郎)</b>", subs: &[]},
            Appointment { term: (1867, 1875), name: "<b>Minister of Fuzhou Naval Affairs (福州船政大臣)</b>, Mawei Shipyard (馬尾造船廠)", subs: &[]},
            Appointment { term: (1874, 1875), name: "<b>Minister for Taiwan Maritime Defense and Foreign Affairs (辦理臺灣等處海防兼理各國事務大臣)</b>", subs: &[]},
            Appointment { term: (1875, NULL), name: "<b>Minister of Commerce (辦理通商事务大臣)</b>", subs: &[]},
            Appointment { term: (1875, DEAD), name: "<b>Governor-General (總督)</b>, Multi-Province Region (总督区) of Liangjiang (兩江)", subs: &[]},
        ]}
    ]
};

pub const LI_HONGZHANG: Person = Person {
    name: Name {
        full: "Li Hongzhang (李鴻章)",
        last: "Li (李)",
    },
    span: (1823, 1901),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/li_hongzhang.gif"),
        normal: asset!("assets/portrait/li_hongzhang.webp"),
        source: "<i>Li Hung Chang.</i> Photograph. Accessed February 10, 2025. https://commons.wikimedia.org/wiki/File:Li_Hung_Chang_in_1896.jpg"
    }),
    appointment: &[
        Appointment { term: (NULL, 1847), name: "<b>Student</b>", subs: &[
            Appointment { term: (NULL, 1843), name: "County Examination (秀才科)", subs: &[]},
            Appointment { term: (1843, 1844), name: "Provincial Examination (鄉試)", subs: &[]},
            Appointment { term: (1844, 1847), name: "Metropolitan Examination (鄉試)", subs: &[]}
        ]},
        Appointment { term: (1843, DEAD), name: "<b>Scholar</b>", subs: &[
            Appointment { term: (1843, 1844), name: "<b>Xiucai (秀才)</b>", subs: &[]},
            Appointment { term: (1844, 1847), name: "<b>Juren (舉人)</b>", subs: &[]},
            Appointment { term: (1847, DEAD), name: "<b>Jinshi (會試)</b>", subs: &[]}
        ]},
        Appointment { term: (1853, 1894), name: "<b>Serviceman</b>", subs: &[
            Appointment { term: (1853, 1861), name: "<b>Commissioned</b>, Active Component, ⟨Xiang Army (湘軍)|⟩", subs: &[]},
            Appointment { term: (1861, 1894), name: "<b>Commissioned</b>, Active Component, ⟨Huai Army (淮軍)|⟩", subs: &[]},
            Appointment { term: (1879, 1894), name: "<b>Commissioned</b>, Active Component, ⟨Beiyang Fleet (北洋舰队)|⟩", subs: &[]}
        ]},
        Appointment { term: (1859, DEAD), name: "<b>Bureaucrat</b>", subs: &[
            Appointment { term: (1859, 1859), name: "<b>Circuit Intendant (Taotai; 道臺)</b>, Super-District (Circuit; 道) of Yanjianshao (延建邵), Province (省) of Fujian (福建)", subs: &[]},
            Appointment { term: (1860, 1860), name: "<b>Circuit Intendant (Taotai; 道臺)</b>, Super-District (Circuit; 道) of Jinan-Gannan (吉南贛南), Province (省) of Fujian (福建)", subs: &[]},
            Appointment { term: (1862, 1867), name: "<b>Governor, Provincial (巡撫)</b>, Province (省) of Jiangsu (江蘇)", subs: &[]},
            Appointment { term: (1862, 1863), name: "<b>Governor-General (總督)</b>, Multi-Province Region (总督区) of Liangguang (兩廣)", subs: &[]},
            Appointment { term: (1863, 1866), name: "<b>Minister of Commerce for the Five Ports (五口通商大臣)</b>", subs: &[]},
            Appointment { term: (1863, 1866), name: "<b>Minister of Commerce for the Southern Coast (南洋通商大臣)</b>", subs: &[]},
            Appointment { term: (1865, 1866), name: "<b>Governor-General (總督)</b>, Multi-Province Region (总督区) of Liangjiang (兩江)", subs: &[]},
            Appointment { term: (1867, 1870), name: "<b>Governor-General (總督)</b>, Multi-Province Region (总督区) of Huguang (湖廣)", subs: &[]},
            Appointment { term: (1867, 1870), name: "<b>Minister of War (兵部尚書)</b>", subs: &[]},
            Appointment { term: (1868, 1872), name: "<b>Grand Secretary, Deputy (協辦大學士)</b>", subs: &[]},
            Appointment { term: (1869, 1869), name: "<b>Governor, Provincial (巡撫)</b>, Province (省) of Hubei (湖北)", subs: &[]},
            Appointment { term: (1870, 1882), name: "<b>Governor-General (總督)</b>, Multi-Province Region (总督区) of Zhili (直隸)", subs: &[]},
            Appointment { term: (1870, 1882), name: "<b>Minister of Commerce for the Northern Coast (北洋通商大臣)</b>", subs: &[]},
            Appointment { term: (1871, 1871), name: "<b>Minister Plenipotentiary (全权大臣)</b>, ⟨Empire of Japan|⟩", subs: &[]},
            Appointment { term: (1872, 1874), name: "<b>Grand Secretary of the Wuying Hall (武英殿大學士)</b>", subs: &[]},
            Appointment { term: (1874, 1874), name: "<b>Minister Plenipotentiary (全权大臣)</b>, ⟨Peruvian Republic|⟩", subs: &[]},
            Appointment { term: (1874, 1882), name: "<b>Grand Secretary of the Wenhua Hall (文華殿大學士)</b>", subs: &[]},
            Appointment { term: (1875, 1876), name: "<b>Minister Plenipotentiary (全权大臣)</b>, United Kingdom", subs: &[]},
            Appointment { term: (1880, 1880), name: "<b>Minister Plenipotentiary (全权大臣)</b>, ⟨Empire of Brazil|⟩", subs: &[]},
            Appointment { term: (1883, 1895), name: "<b>Governor-General (總督)</b>, Multi-Province Region (总督区) of Zhili (直隸)", subs: &[]},
            Appointment { term: (1883, 1895), name: "<b>Minister of Commerce for the Northern Coast (北洋通商大臣)</b>", subs: &[]},
            Appointment { term: (1884, 1886), name: "<b>Minister Plenipotentiary (全权大臣)</b>, ⟨French Third Republic|⟩", subs: &[]},
            Appointment { term: (1885, 1885), name: "<b>Minister Plenipotentiary (全权大臣)</b>, ⟨Empire of Japan|⟩", subs: &[]},
            Appointment { term: (1887, 1887), name: "<b>Minister Plenipotentiary (全权大臣)</b>, ⟨Kingdom of Portugal|⟩", subs: &[]},
            Appointment { term: (1895, 1895), name: "<b>Minister Plenipotentiary (全权大臣)</b>, ⟨Empire of Japan|⟩", subs: &[]},
            Appointment { term: (1895, 1901), name: "<b>Ambassador-at-large (頭等專使大臣)</b>, Western Powers", subs: &[]},
            Appointment { term: (1899, 1899), name: "<b>Minister of Commerce (办理通商事务大臣)</b>", subs: &[]},
            Appointment { term: (1899, 1900), name: "<b>Governor-General (總督)</b>, Multi-Province Region (总督区) of Liangguang (兩廣)", subs: &[]},
            Appointment { term: (1900, DEAD), name: "<b>Governor-General (總督)</b>, Multi-Province Region (总督区) of Zhili (直隸)", subs: &[]},
            Appointment { term: (1900, DEAD), name: "<b>Minister of Commerce for the Northern Coast (北洋通商大臣)</b>", subs: &[]},
            Appointment { term: (1900, 1901), name: "<b>Minister Plenipotentiary (全权大臣)</b>", subs: &[]},
            Appointment { term: (1901, DEAD), name: "<b>Minister of the Zongli Yamen (总理各国事务衙门大臣)</b>", subs: &[]},
            Appointment { term: (1901, DEAD), name: "<b>Minister of Government Affairs (督辦政務處大臣)</b>", subs: &[]},
        ]}
    ]
};

pub const DING_RICHANG: Person = Person {
    name: Name {
        full: "Ding Richang (丁日昌)",
        last: "Ding (丁)",
    },
    span: (1823, 1882),
    portraiture: Some(Portraiture {
        sketch: asset!("assets/portrait/ding_richang.gif"),
        normal: asset!("assets/portrait/ding_richang.webp"),
        source: "<i>丁日昌.</i> Photograph. Accessed February 14, 2025. https://www.tongxianghuicn.com/picture/870723.jhtml?libId=1682"
    }),
    appointment: &[
        Appointment { term: (NULL, 1842), name: "<b>Student</b>", subs: &[
            Appointment { term: (NULL, 1842), name: "County Examination (秀才科)", subs: &[]}
        ]},
        Appointment { term: (1842, DEAD), name: "<b>Scholar</b>", subs: &[
            Appointment { term: (1842, DEAD), name: "<b>Xiucai (秀才)</b>", subs: &[]}
        ]},
        Appointment { term: (1859, 1861), name: "<b>Bureaucrat</b>", subs: &[
            Appointment { term: (1859, 1861), name: "<b>Magistrate, County (知縣)</b>, Sub-District (County; 县) of Wan'an (万安), Province (省) of Jiangxi (江西)", subs: &[]},
            Appointment { term: (1861, 1861), name: "<b>Magistrate, County (知縣)</b>, Sub-District (County; 县) of Luling (庐陵), Province (省) of Jiangxi (江西)", subs: &[]},
        ]},
        Appointment { term: (1861, 1863), name: "<b>Serviceman</b>", subs: &[
            Appointment { term: (1861, 1863), name: "<b>Commissioned</b>, Active Component, ⟨Huai Army (淮軍)|⟩", subs: &[]}
        ]},
        Appointment { term: (1862, NULL), name: "<b>Bureaucrat</b>", subs: &[
            Appointment { term: (1862, 1863), name: "<b>Magistrate, County (知縣)</b>, Sub-District (County; 县) of Wan'an (万安), Province (省) of Jiangxi (江西)", subs: &[]},
            Appointment { term: (1863, 1865), name: "<b>Magistrate, District (知州)</b>, District (州) of Zhili (直隶), Province (省) of Hebei (河北)", subs: &[]},
            Appointment { term: (1865, NULL), name: "<b>General Manager (總辦)</b>, Jiangnan Machinery Manufacturing Bureau (江南機器製造總局)", subs: &[]},
            Appointment { term: (1865, 1867), name: "<b>Circuit Intendant (Taotai; 道臺)</b>, Super-District (Circuit; 道) of Susongtai (苏松太), Province (省) of Jiangsu (江蘇)", subs: &[]},
            Appointment { term: (1867, 1870), name: "<b>Governor, Provincial (巡撫)</b>, Province (省) of Jiangsu (江蘇)", subs: &[]},
            Appointment { term: (1875, 1878), name: "<b>Minister of Fuzhou Naval Affairs (福州船政大臣)</b>, Mawei Shipyard (馬尾造船廠)", subs: &[]},
            Appointment { term: (1875, 1878), name: "<b>Governor, Provincial (巡撫)</b>, Province (省) of Fujian (福建)", subs: &[]},
            Appointment { term: (NULL, NULL), name: "<b>Right Censor-in-chief, Vice (右副都御史)</b>", subs: &[]},
            Appointment { term: (1878, NULL), name: "<b>Right Censor-in-chief, Vice (右副都御史)</b>", subs: &[]},
            Appointment { term: (1879, DEAD), name: "<b>Minister of the Zongli Yamen (总理各国事务衙门大臣)</b>", subs: &[]},
            Appointment { term: (1879, DEAD), name: "<b>Governor-General (總督)</b>", subs: &[]},
        ]}
    ]
};