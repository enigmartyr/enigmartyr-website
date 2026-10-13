/*
pub(crate) use crate::generic::data::file::File;
use dioxus::prelude::*;
use std::hash::Hasher;
use std::hash::{DefaultHasher, Hash};

pub const NYDH_1851_10_12: File =  File {
    internal_source: asset!("assets/docs/us/nydh_1851_10_12.pdf"),
    external_source: "https://www.newspapers.com/image/843812753/",
    source_descript: "“The Missing Bark Coquette” in The New York Herald, 12 Oct 1851",
    source_citation: "<mark>New York Herald</mark>, 12 Oct 1851, p. 1",
};
pub const NYDH_1857_04_18: File =  File {
    internal_source: asset!("assets/docs/us/nydh_1857_04_18.pdf"),
    external_source: "https://www.newspapers.com/image/843799789/",
    source_descript: "“The Missing Clipper Ship Highflyer” in The New York Herald, 18 Apr 1857",
    source_citation: "<mark>New York Herald</mark>, 18 Apr 1857, p. 1"
};
pub const GAZETTE_1867_08_24_ARMY_NAVY_JOURNAL: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1867_08_24_army_navy_journal.pdf"),
    external_source: "https://archive.org/details/sim_armed-forces-journal_1867-08-24_5_1/",
    source_descript: "",
    source_citation: ""
};
pub const GAZETTE_1874_04_24_PENINSULA_COURIER: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1874_04_24_peninsular_courier.pdf"),
    external_source: "https://www.newspapers.com/image/1182588540/",
    source_descript: "Steere, Joseph, “Tamsui, Formosa, December 1st, 1873” (Part 1) in The Peninsular Courier, 24 Apr 1874",
    source_citation: "<mark>The Peninsular Courier</mark>, 18 Apr 1874, p. 1"
};
pub const GAZETTE_1874_05_01_PENINSULA_COURIER: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1874_05_01_peninsular_courier.pdf"),
    external_source: "https://www.newspapers.com/image/1182588572/",
    source_descript: "Steere, Joseph, “Tamsui, Formosa, December 1st, 1873” (Part 2) in The Peninsular Courier, 1 May 1874",
    source_citation: "<mark>The Peninsular Courier</mark>, 1 May 1874, p. 1"
};
pub const GAZETTE_1874_05_15_PENINSULA_COURIER: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1874_05_15_peninsular_courier.pdf"),
    external_source: "https://www.newspapers.com/image/1182588663/",
    source_descript: "Steere, Joseph, “Tamsui, Formosa, December 1st, 1873” (Part 3) in The Peninsular Courier, 15 May 1874",
    source_citation: "<mark>The Peninsular Courier</mark>, 15 May 1874, p. 4"
};
pub const GAZETTE_1874_05_22_PENINSULA_COURIER: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1874_05_22_peninsular_courier.pdf"),
    external_source: "https://www.newspapers.com/image/1182588671/",
    source_descript: "Steere, Joseph, “Tamsui, Formosa, December 1st, 1873” (Part 4) in The Peninsular Courier, 22 May 1874",
    source_citation: "<mark>The Peninsular Courier</mark>, 22 May 1874, p. 1"
};
pub const GAZETTE_1874_05_29_PENINSULA_COURIER: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1874_05_22_peninsular_courier.pdf"),
    external_source: "https://www.newspapers.com/image/1182588704/",
    source_descript: "Steere, Joseph, “Takao, Formosa, January 2nd, 1873” in The Peninsular Courier, 29 May 1874",
    source_citation: "<mark>The Peninsular Courier</mark>, 29 May 1874, p. 1"
};
pub const GAZETTE_1874_06_05_PENINSULA_COURIER: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1874_06_05_peninsular_courier.pdf"),
    external_source: "https://www.newspapers.com/image/1182588733/",
    source_descript: "Steere, Joseph, “Taiwanfu, January 10th, 1874” in The Peninsular Courier, 5 Jun 1874",
    source_citation: "<mark>The Peninsular Courier</mark>, 5 Jun 1874, p. 1,4"
};
pub const GAZETTE_1874_06_12_PENINSULA_COURIER: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1874_06_12_peninsular_courier.pdf"),
    external_source: "https://www.newspapers.com/image/1182588770/",
    source_descript: "“Taiwanfu, February 1st, 1874” in The Peninsular Courier, 12 Jun 1874",
    source_citation: "<mark>The Peninsular Courier</mark>, 12 Jun 1874, p. 1"
};

/* pub const GAZETTE_1874_07_03_PENINSULA_COURIER: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1874_07_03_peninsular_courier.pdf"),
    external_source: "https://www.newspapers.com/image/1182588873/",
    source_descript: "Steere, Joseph, “Canton, China, April 10th, 1874” in The Peninsular Courier, 3 Jul 1874",
    source_citation: "<mark>The Peninsular Courier</mark>, 3 Jul 1874, p. 1",

};*/
pub const GAZETTE_1949_02_08_RICHMOND_INDEPENDENT: File =  File {
    internal_source: asset!("assets/docs/us/gazette_1949_02_08_richmond_independent.pdf"),
    external_source: "https://www.newspapers.com/image/1035593184/",
    source_descript: "International News Service: “Lapham Urges More ECA Aid to Formosa”",
    source_citation: "<mark>The Richmond Independent</mark>, 8 Feb 1949, p. 9",
};

pub const GAZETTE_GEO_MAG_V04: File =  File {
    internal_source: asset!("assets/docs/uk/gazette_geo_mag_v04.pdf"),
    external_source: "https://www.google.com/books/edition/uk/bq7sErB9_w4C/",
    source_descript: "Morrison, James, ‘A Description of the Island of Formosa: With Some Remarks on Its Past History, Its Present Condition, and Its Future Prospects’",
    source_citation: "<mark>The Geographical Magazine</mark> (London: Trubner & Co. Ludgate Hill, 1977), vol. 4 p. 260-266,293-296,319-322",
};
pub const BOOK_ATTLEE: File =  File {
    internal_source: asset!("assets/docs/uk/book_attlee.pdf"),
    external_source: "https://archive.org/details/primeministerrem0000fran/",
    source_descript: "[Prime Minister of the United Kingdom] Clement Richard Attlee, “A Prime Minister Remembers”",
    source_citation: "",
};
pub const BOOK_BAX: File =  File {
    internal_source: asset!("assets/docs/uk/book_bax.pdf"),
    external_source: "https://www.google.com/books/edition/uk/jTl_FFVHm4oC/",
    source_descript: "The Eastern Seas Being a Narrative of the Voyage of H. M. S. “Dwarf” in China, Japan and Formosa. With a Description of the Coast of Russian Tartary and Eastern Siberia, from the Corea to the River Amur",
    source_citation: "Bax, Bonham, <mark>The Eastern Seas</mark> (London: John Murray, 1875)",
};
pub const BOOK_HULL: File =  File {
    internal_source: asset!("assets/docs/us/book_hull.pdf"),
    external_source: "https://archive.org/details/bwb_P9-BIB-290/",
    source_descript: "[US Secretary of State] Cordell Hull, “Toward Victory in the Orient”",
    source_citation: "",
};
pub const CA_BOOK_DCER_V016: File =  File {
    internal_source: asset!("assets/docs/ca/book_dcer_v016.pdf"),
    external_source: "https://gac.canadiana.ca/view/ooe.b2217569F_016/",
    source_descript: "Department of Foreign Affairs and International Trade, “Documents on Canadian External relations”",
    source_citation: "",
};
pub const CA_BOOK_DCER_V017: File =  File {
    internal_source: asset!("assets/docs/ca/book_dcer_v016.pdf"),
    external_source: "https://gac.canadiana.ca/view/ooe.b2217569F_017/",
    source_descript: "Department of Foreign Affairs and International Trade, “Documents on Canadian External relations”",
    source_citation: "",
};
pub const UK_BDFA_04_E_08: File =  File {
    internal_source: asset!("assets/docs/uk/bdfa_04_e_10.pdf"),
    external_source: "https://www.google.com/books/edition/uk/Hq4xgfGCo4UC/",
    source_descript: "British Documents on Foreign Affairs: China, Japan, Siam, Indo-China and Korea; January 1949—December 1949",
    source_citation: "<mark>British Documents on Foreign Affairs</mark> (Washington, D.C.: University Publications of America, 2003), pt. 4 ser. E vol. 8 p. 157",
};
pub const UK_BDFA_04_E_10: File =  File {
    internal_source: asset!("assets/docs/uk/bdfa_04_e_10.pdf"),
    external_source: "https://www.google.com/books/edition/uk/J7-Sy18LogsC/",
    source_descript: "British Documents on Foreign Affairs: China, Japan, Siam, Indo-China and Korea; January 1950—December 1950",
    source_citation: "<mark>British Documents on Foreign Affairs</mark> (Washington, D.C.: University Publications of America, 2003), pt. 4 ser. E vol. 10",
};
pub const UK_BDFA_05_E_03: File =  File {
    internal_source: asset!("assets/docs/uk/bdfa_05_e_03.pdf"),
    external_source: "https://www.google.com/books/edition/uk/7OePAAAAMAAJ/",
    source_descript: "British Documents on Foreign Affairs: China, Japan, Siam, Indo-China and Korea; January 1952—December 1952",
    source_citation: "<mark>British Documents on Foreign Affairs</mark> (Washington, D.C.: University Publications of America, 2006), pt. 5 ser. E vol. 3",
};
pub const UK_PARLIAMENT_038_S02_HANSARD_N0439: File =  File {
    internal_source: asset!("assets/docs/uk/parliament_038_s02_hansard_n0439.pdf"),
    external_source: "https://www.google.com/books/edition/uk/LdctAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const UK_PARLIAMENT_038_S05_HANSARD_N0469: File =  File {
    internal_source: asset!("assets/docs/uk/parliament_038_s05_hansard_n0469.pdf"),
    external_source: "https://www.google.com/books/edition/uk/w2UdAQAAMAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const UK_PARLIAMENT_039_S02_HANSARD_N0484: File =  File {
    internal_source: asset!("assets/docs/uk/parliament_039_s02_hansard_n0484.pdf"),
    external_source: "https://www.google.com/books/edition/uk/9fDrixCYEcUC/",
    source_descript: "",
    source_citation: "",
};
pub const UK_PARLIAMENT_039_S02_HANSARD_N0487: File =  File {
    internal_source: asset!("assets/docs/uk/parliament_039_s02_hansard_n0487.pdf"),
    external_source: "https://www.google.com/books/edition/uk/FFgdAQAAMAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const UK_PARLIAMENT_040_S01_HANSARD_N0495: File =  File {
    internal_source: asset!("assets/docs/uk/parliament_040_s01_hansard_n0495.pdf"),
    external_source: "https://www.google.com/books/edition/uk/cNctAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const UK_PARLIAMENT_040_S03_HANSARD_N0530: File =  File {
    internal_source: asset!("assets/docs/uk/parliament_040_s03_hansard_n0530.pdf"),
    external_source: "https://www.google.com/books/edition/uk/ibnIdCGuh60C/",
    source_descript: "",
    source_citation: "",
};
pub const UK_PARLIAMENT_040_S04_HANSARD_N0536: File =  File {
    internal_source: asset!("assets/docs/uk/parliament_040_s04_hansard_n0536.pdf"),
    external_source: "https://www.google.com/books/edition/uk/98UtAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const UK_MEMO_1873_05_15_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1873_05_15_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235570",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 120: The Japanese Ambassador; His Pretentions,” 15 May 1873",
    source_citation: "",
};
pub const UK_MEMO_1873_05_25_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1873_05_25_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235570",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 131: The Japanese Ambassador; Visit to and Conversation with; Objects of His Embassy,” 25 May 1873",
    source_citation: "",
};
pub const UK_MEMO_1873_06_04_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1873_06_04_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235570",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Telegraph No. 143: Japanese Audience; Corea; Formosa,” 4 Jun 1873",
    source_citation: "",
};
pub const UK_MEMO_1873_08_20_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1873_08_20_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235571",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 175: Japanese Embassy in China; The Audience,” 20 Aug 1873",
    source_citation: "",
};
pub const UK_MEMO_1874_04_24_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_04_24_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 62: Japanese Descent on Formosa,” 24 Apr 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_04_30_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_04_30_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 72: Japanese Expedition to Formosa,” 30 Apr 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_05_01_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_05_01_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 74: Japan, Formosa, and China,” 1 May 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_05_12_GREGORY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_05_12_gregory.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4384189",
    source_descript: "[Consul of British Consulate in Kaohsiung] William Gregory, “Dispatch No. 9: Arrival of Japanese Expedition in Formosa,” 12 May 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_05_31_GREGORY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_05_31_gregory.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Consul of British Consulate in Kaohsiung] William Gregory, “Dispatch No. 10: The Japanese Expedition, in Continuation of No. 9,” 31 May 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_05_31_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_05_31_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 96: Japanese Landed in Formosa,” 31 May 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_06_10_WADE_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_06_10_wade_1.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 99: Further on Japanese Expedition to Formosa,” 10 Jun 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_06_10_WADE_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_06_10_wade_2.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 100: Appointment of High Commissioner to Formosa Beneficial to Trade,” 10 Jun 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_06_12_GREGORY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_06_12_gregory.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Consul of British Consulate in Kaohsiung] William Gregory, “Dispatch No. 12: The Japanese Expedition, Continued,” 12 Jun 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_06_22_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_06_22_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235590",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 116: Story of Iron-clad Project,” 22 Jun 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_06_24_WADE_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_06_24_wade_1.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235590",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 123: The War in Formosa,” 24 Jun 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_06_24_WADE_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_06_24_wade_2.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235590",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 125: French Minister Leaving; His Policy,” 24 Jun 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_07_04_GREGORY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_07_04_gregory.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Consul of British Consulate in Kaohsiung] William Gregory, “Dispatch No. 14: The Japanese Expedition, Continued; Notifications; Actions of Chinese Authorities,” 5 Jul 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_07_09_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_07_09_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235590",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 133: Japan and Formosa,” 9 Jul 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_07_10_WADE_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_07_10_wade_1.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235590",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 136: Japanese Also in North of Formosa,” 10 Jul 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_07_10_WADE_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_07_10_wade_2.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235590",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 139: Japan; No Declaration of War,” 10 Jul 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_07_12_GREGORY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_07_12_gregory.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Consul of British Consulate in Kaohsiung] William Gregory, “Dispatch No. 15: Japanese Expedition; Sends Copy of Dispatches to Mr. Wade with Translation of Dispatch from Imperial Commissioner to Japanese General; etc.,” 12 Jul 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_07_24_GREGORY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_07_24_gregory.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235589",
    source_descript: "[Consul of British Consulate in Kaohsiung] William Gregory, “Dispatch No. 16: The Japanese Expedition; Sunday Notes, in Copy of Dispatch to Legation,” 24 Jul 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_07_25_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_07_25_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235590",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 151: Text of February; Sale of Ironclad,” 25 Jul 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_07_31_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_07_31_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235590",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 157: Russian Opinion of Chinese and Japanese,” 31 Jul 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_08_08_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_08_08_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235591",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 157: Arrival of and Conversation with the Japanese Minister,” 8 Aug 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_08_10_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_08_10_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235591",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 176: Prince of Kung; Correspondence on Japan,” 10 Aug 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_08_13_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_08_13_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235591",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 178: Formosa; Japanese Mission Instructions,” 13 Aug 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_09_03_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_09_03_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235591",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 186: Japan & China; Conferences and State of Mind of Chinese Government,” 3 Sep 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_09_16_WADE_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_09_16_wade_1.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235591",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 195: Japanese High Commissioner Okubo Arrived; Received by Prince of Kung; Visits Exchanged and Conversations respecting State of Affairs,” 16 Sep 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_09_16_WADE_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_09_16_wade_2.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235591",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 198: General View of State of Affairs in China; Probabilities as regards War with Japan; Japanese in Formosa; Sedition; Security of the Ports,” 16 Sep 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_10_02_WADE_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_10_02_wade_1.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 201: China & Japan; Interview with Japanese Commissioner,” 2 Oct 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_10_02_WADE_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_10_02_wade_2.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 202: Japan & China; The Minister of the Yamen Came to Talk over the Situation,” 2 Oct 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_10_16_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_10_16_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 206: Japan & China; State of Affairs Drawing to a Close,” 16 Oct 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_10_30_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_10_30_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 207: China and Japan; Agreement Come to with Regard to Formosa,” 30 Oct 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_11_01_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_11_01_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 208: Japan & China; Agreement for Evacuation of Formosa Signed,” 1 Nov 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_11_05_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_11_05_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 209: Japan & China; Tone of Agreement,” 5 Nov 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_11_16_WADE_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_11_16_wade_1.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 219: Agreement Communicated by Prince of Kung,” 16 Nov 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_11_16_WADE_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_11_16_wade_2.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 220: Formosa; Admiral Requested to Support Agreement and Watch Coast,” 16 Nov 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_11_16_WADE_3: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_11_16_wade_3.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 221: Formosa; Visit of Ministers to Return Thanks,” 16 Nov 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_11_16_WADE_4: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_11_16_wade_4.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 222: Japan & China; History of Negotiations for Settlement of Formosan Question,” 16 Nov 1874",
    source_citation: "",
};
pub const UK_MEMO_1874_11_27_WADE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1874_11_27_wade.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/C4235592",
    source_descript: "[Minister Plenipotentiary of British Legation in Beijing] Thomas Francis Wade, “Dispatch No. 223: History of Difficulties and Negotiations Concerning the Formosan Affair,” 16 Nov 1874",
    source_citation: "",
};
pub const UK_MEMO_1950_07_03_YOUNGER: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1950_07_03_younger.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/D7655988",
    source_descript: "Ernest Bevin, “Formosa; Memorandum by the Minister of State”, 03 Jul 1950",
    source_citation: "Cabinet Papers, <mark>CP (50) 156</mark> (Cabinet Office, 1950)",
};
pub const UK_MEMO_1950_08_21_BEVEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1950_08_21_bevin.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/D7656026",
    source_descript: "Ernest Bevin, “Formosa; Memorandum by the Secretary of State for Foreign Affairs”, 21 Aug 1950",
    source_citation: "Cabinet Papers, <mark>CP (50) 194</mark> (Cabinet Office, 1950)",
};
pub const UK_MEMO_1950_11_17_BEVEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1950_11_17_bevin.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/D7656107",
    source_descript: "Ernest Bevin, “Formosa; Memorandum by the Secretary of State for Foreign Affairs”, 17 Nov 1950",
    source_citation: "Cabinet Papers, <mark>CP (50) 276</mark> (Cabinet Office, 1950)",
};
pub const UK_MEMO_1951_01_06_STREET: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_01_06_street.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_07_00C0008",
    source_descript: "Vivian Wakefield Street, “Strategic Importance of Formosa",
    source_citation: "",
};
pub const UK_MEMO_1951_03_20_MORRISON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_03_20_morrison.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/D7656233",
    source_descript: "Herbert Morrison, “Japanese Peace Treaty, Chinese Participation and Disposal of Formosa; Memorandum by the Secretary of State for Foreign Affairs”, 20 Mar 1951",
    source_citation: "Cabinet Papers, <mark>CP (51) 78</mark> (Cabinet Office, 1951)",
};
pub const UK_MEMO_1951_05_23_MORRISON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_05_23_morrison.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/D7656291",
    source_descript: "Herbert Morrison, “Japanese Peace Treaty; Memorandum by the Secretary of State for Foreign Affairs”, 23 May 1951",
    source_citation: "Cabinet Papers, <mark>CP (51) 137</mark> (Cabinet Office, 1951)",
};
pub const UK_MEMO_1951_06_07_MORRISON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_06_07_morrison.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/D7656309",
    source_descript: "Herbert Morrison, “Japanese Peace Treaty, Talks with Mr. Dulles; Memorandum by the Secretary of State for Foreign Affairs”, 7 Jun 1951",
    source_citation: "Cabinet Papers, <mark>CP (51) 155</mark> (Cabinet Office, 1951)",
};
pub const UK_MEMO_1951_06_09_MORRISON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_06_09_morrison.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/D7656312",
    source_descript: "Herbert Morrison, “Japanese Peace Treaty: Chinese Participation; Memorandum by the Secretary of State for Foreign Affairs”, 9 Jun 1951",
    source_citation: "Cabinet Papers, <mark>CP (51) 158</mark> (Cabinet Office, 1951)",
};
pub const UK_MEMO_1951_06_16_CRO_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_06_16_cro_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0040",
    source_descript: "Commonwealth Relations Office, “Outward Telegram Y. No. 316: Japanese Peace Treaty”, 16 Jun 1951",
    source_citation: "",
};
pub const UK_MEMO_1951_06_16_CRO_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_06_16_cro_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0040",
    source_descript: "Commonwealth Relations Office, “Outward Telegram Y. No. 317: Japanese Peace Treaty, Chinese Participation and Formosa”, 16 Jun 1951",
    source_citation: "",
};
pub const UK_MEMO_1951_08_15_CLUTTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_08_15_clutton.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "George Lisle Clutton, “Addressed to Foreign Office telegram No: 1060 of 15th August”",
    source_citation: "",
};
pub const UK_MEMO_1951_08_31_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_08_31_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Robert Anthony Eden, “Addressed to Washington telegram No: 3981 of 31st August”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_02_FRANKS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_02_franks.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Oliver Shewell Franks, “Addressed to Foreign Office telegram No. 3 of September 2nd”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_04_JOHNSTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_04_johnston.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Charles Hepburn Johnston, “Japan's Future Relations with Formosa”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_04_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_04_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No: 1090 of 4th September”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_05_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_05_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No: 15 of 5th September”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_13_FRANKS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_13_franks.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Oliver Shewell Franks, “Addressed to Foreign Office telegram No. 945 Saving of 13th September”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_18_YOUNGER: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_18_younger.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Kenneth Gilmour Younger, “Letter to the Japan and Pacific Department”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_20_CLUTTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_20_clutton.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "George Lisle Clutton, “Letter Charles Hepburn Johnston”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_21_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_21_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Robert Heatlie Scott, “Letter to Esler Maberley Dening”",
    source_citation: "",
};
pub const UK_MEMO_1951_09_27_CLUTTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_09_27_clutton.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "George Lisle Clutton, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1951_10_08_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_10_08_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1951_10_15_TOMLINSON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_10_15_tomlinson.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Frank Stanley Tomlinson, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1951_10_19_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_10_19_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Charles Peter Scott, “Letter to George Lisle Clutton”",
    source_citation: "",
};
pub const UK_MEMO_1951_10_31_JOHNSTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_10_31_johnston.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Charles Hepburn Johnston, “For Bipartite Talks: Japan's Future Relations with China”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_12_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_12_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Robert Heatlie Scott, “Letter to Esler Maberley Dening”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_12_JOHNSTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_12_johnston.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Charles Hepburn Johnston, “Japan's Future Relations with China”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_13_JOHNSTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_13_johnston.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Charles Hepburn Johnston, “Japan's Future Relations with China”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_14_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_14_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Charles Peter Scott, “Summary of Record of Meetings Held at the Foreign Office on 13th and 14th November 1951 to Discuss ‘Japan's Future Relations with Formosa.’”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_14_EDEN_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_14_eden_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No: 1332 of 14th November”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_14_EDEN_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_14_eden_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No: 1333 of 14th November”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_14_EDEN_3: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_14_eden_3.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No: 1334 of 14th November”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_15_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_15_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1623 of 15th November”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_16_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_16_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1628 of 16th November”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_17_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_17_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Robert Heatlie Scott, “Japan and Formosa”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_20_BARCLAY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_20_barclay.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Roderick Barclay, “Japan and Formosa”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_20_JOHNSTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_20_johnston.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Charles Hepburn Johnston, “Japan's Relations with China”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_20_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_20_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Robert Anthony Eden, “Addressed to Paris telegram No. 1719 of 20th November”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_22_HARVEY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_22_harvey.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Oliver Charles Harvey, “Addressed to Foreign Office telegram No. 570 of 22nd November”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_24_JOHNSTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_24_johnston.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Charles Hepburn Johnston, “Japanese Peace Treaty: Debate on November 26th”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_26_JOHNSTON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_26_johnston.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Charles Hepburn Johnston, “Letter to Esler Maberley Dening”",
    source_citation: "",
};
pub const UK_MEMO_1951_11_30_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_11_30_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1675 of 30th November”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_08_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_08_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No. 1424 of 8th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_12_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_12_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1755 of 12th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_13_DENING_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_13_dening_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1767 of 13th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_13_DENING_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_13_dening_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1768 of 13th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_13_DENING_3: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_13_dening_3.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1769 of 13th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_14_DENING_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_14_dening_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1771 of 14th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_14_DENING_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_14_dening_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1772 of 14th December”",
    source_citation: "",
};

/* follow UK_MEMO_1952_01_16_SCOTT_1 */
pub const UK_MEMO_1951_12_14_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_14_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Robert Heatlie Scott, “Letter to William Strang”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_14_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_14_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No. 1436 of 14th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_15_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_15_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1781 of 15th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_16_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_16_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No. 6119 of 16th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_17_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_17_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_18_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_18_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1795 of 18th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_18_FRANKS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_18_franks.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Oliver Shewell Franks, “Addressed to Foreign Office telegram No. 3900 of 18th December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_19_BROCKWAY: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_19_Brockway.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Archibald Fenner Brockway, “Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_21_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_21_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 1819 of 21st December”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_27_TOMLINSON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_27_tomlinson.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Frank Stanley Tomlinson, “Letter to John Arthur Pilcher”",
    source_citation: "",
};
pub const UK_MEMO_1951_12_28_PILCHER: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1951_12_28_pilcher.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "John Arthur Pilcher, “The United States and Japan's Relations with China”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_02_NUTTING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_02_Nutting.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Harold Anthony Nutting, “Letter to Archibald Fenner Brockway”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_10_FRANKS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_10_franks.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Oliver Shewell Franks, “Addressed to Foreign Office telegram No. 107 of 10th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_14_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_14_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_16_DENING_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_16_dening_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 70 of 16th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_16_DENING_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_16_dening_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 71 of 16th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_16_DENING_3: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_16_dening_3.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 72 of 16th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_16_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_16_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Robert Anthony Eden, “Addressed to Washington telegram No. 313 of 16th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_16_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_16_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Robert Heatlie Scott, “Letter to William Strang”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_16_STRANG: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_16_strang.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "William Strang, “Letter to Herbert Stanley Morrison”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_16_PILCHER: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_16_pilcher.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "John Arthur Pilcher, “Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_16_FRANKS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_16_franks.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Oliver Shewell Franks, “Addressed to Foreign Office telegram No. 186 of 16th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_17_STRANG: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_17_strang.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "William Strang, “Record of Conversation with the French Ambassador”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_17_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_17_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 79 of 17th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_17_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_17_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Robert Heatlie Scott, “Letter to William Strang”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_18_FRANKS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_18_franks.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Oliver Shewell Franks, “Addressed to Foreign Office telegram No. 201 of 18th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_19_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_19_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 92 of 19th January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_21_TOMLINSON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_21_tomlinson.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Frank Stanley Tomlinson, “Letter to Charles Arthur Evelyn Shuckburgh”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_22_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_22_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "Robert Anthony Eden, “Addressed to Washington telegram No. 434 of 22nd January”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_24_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_24_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_26_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_26_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_26_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_26_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Robert Heatlie Scott, “Letter to Charles Arthur Evelyn Shuckburgh”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_28_DENING_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_28_dening_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_01_28_DENING_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_01_28_dening_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_01_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_01_eden.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Robert Anthony Eden, “Addressed to Tokyo telegram No. 93 of 1st February”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_02_FRANKS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_02_franks.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Oliver Shewell Franks, “Addressed to Foreign Office telegram No. 361 of 2nd February”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_04_DENING_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_04_dening_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_04_DENING_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_04_dening_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_11_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_11_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_12_STRANG: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_12_strang.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "William Strang, “Letter to Oliver Shewell Franks”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_18_DENING_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_18_dening_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_18_DENING_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_18_dening_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_19_FRANKS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_19_franks.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Oliver Shewell Franks, “Letter to James Edwin Webb”",
    source_citation: "",
};
pub const UK_MEMO_1952_02_25_TOMLINSON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_02_25_tomlinson.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "Frank Stanley Tomlinson, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1952_03_05_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_03_05_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Addressed to Foreign Office telegram No. 33 of 5th March”",
    source_citation: "",
};
pub const UK_MEMO_1952_03_10_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_03_10_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Esler Maberley Dening, “Addressed to Foreign Office telegram No. 13 Saving of 10th March”",
    source_citation: "",
};
pub const UK_MEMO_1952_03_11_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_03_11_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const UK_MEMO_1952_03_20_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_03_20_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Addressed to Foreign Office telegram No. 42 of 20th March”",
    source_citation: "",
};
pub const UK_MEMO_1952_03_29_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_03_29_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Addressed to Foreign Office telegram No. 47 of 29th March”",
    source_citation: "",
};
pub const UK_MEMO_1952_04_07_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_04_07_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const UK_MEMO_1952_04_07_MARE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_04_07_mare.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Arthur James de la Mare, “Letter to John Arthur Pilcher”",
    source_citation: "",
};
pub const UK_MEMO_1952_04_22_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_04_22_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Addressed to Foreign Office telegram No. 58 of 22nd April”",
    source_citation: "",
};
pub const UK_MEMO_1952_04_29_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_04_29_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Addressed to Foreign Office telegram No. 62 of 29nd April”",
    source_citation: "",
};
pub const UK_MEMO_1952_05_07_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_05_07_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const UK_MEMO_1952_05_12_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_05_12_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Esler Maberley Dening, “Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const UK_MEMO_1952_06_06_MARE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_06_06_mare.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Arthur James de la Mare, “Letter to John Arthur Pilcher”",
    source_citation: "",
};
pub const UK_MEMO_1952_06_23_MARE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_06_23_mare.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Arthur James de la Mare, “Letter to John Arthur Pilcher”",
    source_citation: "",
};
pub const UK_MEMO_1952_06_30_LARKCOM: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1952_06_30_larkcom.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Eric Herbert Jacobs-Larkcom, “Letter to Arthur James de la Mare”",
    source_citation: "",
};
pub const UK_MEMO_1952_07_01_MARE: File =  File {
        internal_source: asset!("assets/docs/uk/memo_1952_07_01_mare.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0018",
    source_descript: "Arthur James de la Mare, “Letter to John Arthur Pilcher”",
    source_citation: "",
};
pub const UK_MEMO_1953_01_27_DENING: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1953_01_27_dening.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_04C0010",
    source_descript: "Esler Maberley Dening, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1953_04_08_TOMLINSON: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1953_04_08_tomlinson.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_04C0010",
    source_descript: "Frank Stanley Tomlinson, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const UK_MEMO_1953_04_10_SCOTT: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1953_04_10_scott.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_04C0010",
    source_descript: "Robert Heatlie Scott, “Letter to Frank Stanley Tomlinson”",
    source_citation: "",
};
pub const UK_MEMO_1954_11_25_MAKINS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1954_11_25_makins.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_04_00C0014",
    source_descript: "Roger Mellor Makins, “Addressed to Foreign Office telegram No. 2533 of 25th November”",
    source_citation: "",
};
pub const UK_MEMO_1954_11_26_MAKINS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1954_11_26_makins.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_04_00C0014",
    source_descript: "Roger Mellor Makins, “Addressed to Foreign Office telegram No. 2538 of 26th November”",
    source_citation: "",
};
pub const UK_MEMO_1954_11_28_MAKINS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1954_11_28_makins.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_04_00C0014",
    source_descript: "Roger Mellor Makins, “Addressed to Foreign Office telegram No. 2556 of 28th November”",
    source_citation: "",
};
pub const UK_MEMO_1954_08_09_BIS: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1954_08_09_bis.pdf"),
    external_source: "https://www.google.com/books/edition/uk/MBbtuBQWg5IC/",
    source_descript: "British Information Services, “408/2: British Policy in Relation to Formosa”",
    source_citation: "",
};
pub const UK_MEMO_1954_12_01_EDEN_1: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1954_12_01_eden_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_04_00C0014",
    source_descript: "Robert Anthony Eden, “Addressed to Peking telegram No. 1056 of 1st December”",
    source_citation: "",
};
pub const UK_MEMO_1954_12_01_EDEN_2: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1954_12_01_eden_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_04_00C0014",
    source_descript: "Robert Anthony Eden, “Addressed to Washington telegram No. 5904 of 1st December”",
    source_citation: "",
};
pub const UK_MEMO_1954_12_10_ALLEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1954_12_10_allen.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_04_00C0015",
    source_descript: "William Denis Allen, “Letter to Ivone Augustine Kirkpatrick”",
    source_citation: "",
};
pub const UK_MEMO_1954_12_16_RAE: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1954_12_16_rae.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_04_00C0015",
    source_descript: "Charles Robert Angus Rae, “Letter to A. T. Rush”",
    source_citation: "",
};
pub const UK_MEMO_1955_02_11_EDEN: File =  File {
    internal_source: asset!("assets/docs/uk/memo_1955_02_11_eden.pdf"),
    external_source: "https://discovery.nationalarchives.gov.uk/details/r/D7657701",
    source_descript: "Robert Anthony Eden, “Formosa; Note by the Secretary of State for Foreign Affairs”, 11 Feb 1955",
    source_citation: "Cabinet Papers, <mark>CP (55) 38</mark> (Cabinet Office, 1955)",
};
pub const UK_CPCS_V01: File = File {
    internal_source: asset!("assets/docs/uk/cpcs_v01.pdf"),
    source_descript: "China; Political, Commercial, and Social, Volume 1",
    external_source: "https://www.google.com/books/edition/uk/ReEDAAAAQAAJ/",
    source_citation: "Kong Zhaoming. <i>Selected Memorials and Policy Discussions from Four Reigns: Daoguang, Xianfeng, Tongzhi, and Guangxu</i> Taiwan Historical Documents Series. Vol.4, No. 70. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const UK_NMNC_1851: File = File {
    internal_source: asset!("assets/docs/uk/nmnc_1851.pdf"),
    source_descript: "The Nautical Magazine and Naval Chronicle, 1851",
    external_source: "https://www.google.com/books/edition/uk/oxiMSeSPvPgC/",
    source_citation: "Kong Zhaoming. <i>Selected Memorials and Policy Discussions from Four Reigns: Daoguang, Xianfeng, Tongzhi, and Guangxu</i> Taiwan Historical Documents Series. Vol.4, No. 70. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const BOOK_SEBALD: File = File {
    internal_source: asset!("assets/docs/us/book_sebald.pdf"),
    external_source: "https://archive.org/details/withmacarthurinj0000will/",
    source_descript: "[Ambassador to Japan] William Joseph Sebald, “With MacArthur in Japan”",
    source_citation: "",
};
pub const BOOK_SPROSTON: File =  File {
    internal_source: asset!("assets/docs/us/book_sproston.pdf"),
    external_source: "https://digital-archives.sophia.ac.jp/repository/view/repository/20190711001",
    source_descript: "Breese, Kidder, ‘Breese's Description of the Formosa Coal Field’",
    source_citation: "<mark>A Private Journal of John Glendy Sproston, USN.</mark> (Tokyo: Sophia University, 1940), p. 65-77",
};
pub const BOOK_HABERSHAM: File = File {
    internal_source: asset!("assets/docs/us/book_habersham.pdf"),
    source_descript: "The North Pacific Surveying and Exploring Expedition",
    external_source: "https://www.google.com/books/edition/us/oWedEAAAQBAJ/",
    source_citation: "<i>From January to December, 1851</i>                                                 \
    The Chinese Repository. Vol.20                                                                 \
    Kraus Reprint, 1852. Accessed December 22, 2024.",
};
pub const BOOK_HOUSE: File = File {
    internal_source: asset!("assets/docs/us/book_house.pdf"),
    source_descript: "The Japanese Expedition to Formosa",
    external_source: "https://www.google.com/books/edition/us/h35xAAAAMAAJ/",
    source_citation: "<i>From January to December, 1851</i>                                                 \
    The Chinese Repository. Vol.20                                                                 \
    Kraus Reprint, 1852. Accessed December 22, 2024.",
};
pub const BOOK_PICKERING: File = File {
    internal_source: asset!("assets/docs/uk/book_pickering.pdf"),
    source_descript: "Pioneering in Formosa",
    external_source: "https://www.google.com/books/edition/uk/LDcPAAAAYAAJ/",
    source_citation: "",
};
pub const BOOK_BROOKE: File = File {
    internal_source: asset!("assets/docs/uk/book_brooke.pdf"),
    source_descript: "War Diaries, 1939-1945",
    external_source: "https://archive.org/details/wardiaries1939190000alan/",
    source_citation: "",
};
pub const BOOK_CHURCHILL_1: File = File {
    internal_source: asset!("assets/docs/uk/book_churchill_1.pdf"),
    source_descript: "Winston Leonard Spencer Churchill, “The Second World War, Volume 3: The Grand Alliance”",
    external_source: "https://archive.org/details/bwb_S0-CAV-257/",
    source_citation: "",
};
pub const BOOK_CHURCHILL_2: File = File {
    internal_source: asset!("assets/docs/uk/book_churchill_2.pdf"),
    source_descript: "Winston Leonard Spencer Churchill, “The Second World War, Volume 5: Closing the Ring”",
    external_source: "https://archive.org/details/secondworldwarvo0055wins/",
    source_citation: "",
};
pub const BOOK_EDEN: File = File {
    internal_source: asset!("assets/docs/uk/book_eden.pdf"),
    source_descript: "Robert Anthony Eden, “The Second World War, Volume 5: Closing the Ring”",
    external_source: "https://archive.org/details/fullcirclememoir00eden/",
    source_citation: "",
};
pub const BOOK_MOUNTBATTEN: File = File {
    internal_source: asset!("assets/docs/uk/book_mountbatten.pdf"),
    source_descript: "Report to the Combined Chiefs of Staff by the Supreme Allied Commander, South-East Asia, 1943-1945",
    external_source: "https://www.google.com/books/edition/uk/9iLTAAAAMAAJ/",
    source_citation: "",
};
pub const BOOK_THOMSON: File = File {
    internal_source: asset!("assets/docs/uk/book_thomson.pdf"),
    source_descript: "John Thomson: “The Straits of Malacca, Indo-China, and China”",
    external_source: "https://www.google.com/books/edition/uk/p0BmAAAAcAAJ/",
    source_citation: "",
};
pub const SERIEAL_DACR_N0124: File = File {
    internal_source: asset!("assets/docs/uk/serieal_dacr_n0124.pdf"),
    external_source: "https://www.google.com/books/edition/uk/Mk-_GIusIwIC/",
    source_descript: "Her Majesty's Stationary Office: “Diplomatic and Consular Reports, No. 124: Report for the Year 1886 on the Trade of Tamsui (Formosa)”",
    source_citation: "",
};





pub const BOOK_CHINESE_REPO_20: File = File {
    internal_source: asset!("assets/docs/us/chinese_repository_20.pdf"),
    source_descript: "The Chinese Repository, Volume 20",
    external_source: "https://www.google.com/books/edition/The_Chinese_Repository/UQAMAAAAYAAJ/",
    source_citation: "<i>From January to December, 1851</i>                                                 \
    The Chinese Repository. Vol.20                                                                 \
    Kraus Reprint, 1852. Accessed December 22, 2024.",
};
pub const BOOK_BALLANTINE: File = File {
    internal_source: asset!("assets/docs/us/book_ballantine.pdf"),
    external_source: "https://archive.org/details/ballantine-formosa/",
    source_descript: "[Director of the Office of Far Eastern Affairs] Joseph Ballantine, “Formosa: a Problem for United States Foreign Policy”",
    source_citation: "",
};
pub const BOOK_BEVANS_1: File = File {
    internal_source: asset!("assets/docs/us/book_bevans_1.pdf"),
    source_descript: "Treaties and Other International Agreements of the United States of America, Volume 1",
    external_source: "https://www.loc.gov/item/lltreaties-ustbv001/",
    source_citation: "Charles Bevans. <i>Multilateral 1776-1917</i>                                         \
    Treaties and Other International Agreements of the United States of America. Vol.1             \
    United States Government Printing Office, 1968. Accessed September 18, 2024.",
};
pub const BOOK_BEVANS_2: File = File {
    internal_source: asset!("assets/docs/us/book_bevans_2.pdf"),
    source_descript: "Treaties and Other International Agreements of the United States of America, Volume 2",
    external_source: "https://www.loc.gov/item/lltreaties-ustbv002/",
    source_citation: "Charles Bevans. <i>Multilateral 1918-1930</i>                                         \
    Treaties and Other International Agreements of the United States of America. Vol.2             \
    United States Government Printing Office, 1968. Accessed September 18, 2024.",
};
pub const BOOK_BEVANS_3: File = File {
    internal_source: asset!("assets/docs/us/book_bevans_3.pdf"),
    source_descript: "Treaties and Other International Agreements of the United States of America, Volume 3",
    external_source: "https://www.loc.gov/item/lltreaties-ustbv003/",
    source_citation: "Charles Bevans. <i>Multilateral 1931-1945</i>                                         \
    Treaties and Other International Agreements of the United States of America. Vol.3             \
    United States Government Printing Office, 1968. Accessed September 18, 2024.",
};
pub const BOOK_BEVANS_6: File = File {
    internal_source: asset!("assets/docs/us/book_bevans_6.pdf"),
    source_descript: "Treaties and Other International Agreements of the United States of America, Volume 6",
    external_source: "https://www.loc.gov/item/lltreaties-ustbv006/",
    source_citation: "Charles Bevans. <i>Canada-Czechoslovakia</i>                                          \
    Treaties and Other International Agreements of the United States of America. Vol.6             \
    United States Government Printing Office, 1968. Accessed September 18, 2024.",
};
pub const BOOK_BEVANS_9: File = File {
    internal_source: asset!("assets/docs/us/book_bevans_9.pdf"),
    source_descript: "Treaties and Other International Agreements of the United States of America, Volume 9",
    external_source: "https://www.loc.gov/item/lltreaties-ustbv009/",
    source_citation: "Charles Bevans. <i>Iraq—Muscat</i>                                                    \
    Treaties and Other International Agreements of the United States of America. Vol.9             \
    United States Government Printing Office, 1968. Accessed September 18, 2024.",
};
pub const BOOK_DA: File = File {
    internal_source: asset!("assets/docs/us/book_da.pdf"),
    external_source: "https://www.google.com/books/edition/us/LSlfBe3YkH4C/",
    source_descript: "Department of the Army, “Reports of General MacArthur: Volume I”",
    source_citation: "",
};
pub const BOOK_EISENHOWER_1: File = File {
    internal_source: asset!("assets/docs/us/book_eisenhower_1.pdf"),
    external_source: "https://archive.org/details/mandateforchange00eise/",
    source_descript: "(President, US) Dwight David Eisenhower, “The White House Years: Mandate for Change, 1953–1956”",
    source_citation: "",
};
pub const BOOK_EISENHOWER_2: File = File {
    internal_source: asset!("assets/docs/us/book_eisenhower_2.pdf"),
    external_source: "https://archive.org/details/wagingpeace1956100eise/",
    source_descript: "(President, US) Dwight David Eisenhower, “The White House Years: Waging Peace, 1956–1961”",
    source_citation: "",
};
pub const BOOK_HOOVER: File =  File {
    internal_source: asset!("assets/docs/us/book_hoover.pdf"),
    external_source: "https://archive.org/details/40keyquestionsab0000herb_q2h7/",
    source_descript: "Latter from President Hoover to Senator Knowland, December 31, 1949",
    source_citation: "Hoover, Herbert, <mark>40 Key Questions about Our Foreign Policy</mark> (Scarsdale: The Updegraff Press, 1952), p. 13",
};
pub const BOOK_KING: File = File {
    internal_source: asset!("assets/docs/us/book_king.pdf"),
    source_descript: "Fleet Admiral King: A Naval Record",
    external_source: "https://archive.org/details/bwb_T4-AHF-626/",
    source_citation: "",
};
pub const BOOK_LEAHY: File = File {
    internal_source: asset!("assets/docs/us/book_leahy.pdf"),
    source_descript: "William Daniel Leahy, “I Was There: The Personal Story of the Chief of Staff to Presidents Roosevelt and Truman Based on His Notes and Diaries Made at the Time”",
    external_source: "https://archive.org/details/dli.ernet.2304/",
    source_citation: "",
};
pub const BOOK_LI: File = File {
    internal_source: asset!("assets/docs/us/book_li.pdf"),
    source_descript: "Li Zongren, “The Memoirs of Li Tsung-jen”",
    external_source: "https://archive.org/details/memoirsoflitsung0000lizo/",
    source_citation: "",
};
pub const BOOK_RANKIN: File = File {
    internal_source: asset!("assets/docs/us/book_rankin.pdf"),
    external_source: "https://archive.org/details/chinaassignment0000rank/",
    source_descript: "(Ambassador to China) Karl Lott Rankin, “China Assignment”",
    source_citation: "",
};
pub const BOOK_REAGAN_1: File = File {
    internal_source: asset!("assets/docs/us/book_reagan_1.pdf"),
    external_source: "https://archive.org/details/reaganinhisownha0000unse/",
    source_descript: "(President of the United States) Ronald Wilson Reagan, “Reagan, In His Own Hand”",
    source_citation: "",
};
pub const BOOK_REAGAN_2: File = File {
    internal_source: asset!("assets/docs/us/book_reagan_2.pdf"),
    external_source: "https://archive.org/details/reaganspathtovic00reag/",
    source_descript: "(President of the United States) Ronald Wilson Reagan, “Reagan's Path to Victory”",
    source_citation: "",
};
pub const BOOK_REAGAN_3: File = File {
    internal_source: asset!("assets/docs/us/book_reagan_3.pdf"),
    external_source: "https://archive.org/details/reaganlifeinlett0000reag/",
    source_descript: "(President of the United States) Ronald Wilson Reagan, “Reagan: A Life in Letters”",
    source_citation: "",
};
pub const BOOK_REAGAN_4: File = File {
    internal_source: asset!("assets/docs/us/book_reagan_4.pdf"),
    external_source: "https://archive.org/details/americanlife00reag_0/",
    source_descript: "(President of the United States) Ronald Wilson Reagan, “An American Life”",
    source_citation: "",
};
pub const BOOK_REAGAN_5: File = File {
    internal_source: asset!("assets/docs/us/book_reagan_5.pdf"),
    external_source: "https://archive.org/details/reagandiaries0000rona/",
    source_descript: "(President of the United States) Ronald Wilson Reagan, “The Reagan Diaries”",
    source_citation: "",
};
pub const BOOK_STILWELL: File = File {
    internal_source: asset!("assets/docs/us/book_stilwell.pdf"),
    external_source: "https://archive.org/details/stilwellpapersic00stil/",
    source_descript: "Joseph Warren Stilwell, “Stilwell Papers”",
    source_citation: "",
};
pub const BOOK_TRUMAN: File = File {
    internal_source: asset!("assets/docs/us/book_truman.pdf"),
    external_source: "https://archive.org/details/memoirst02trum/",
    // https://archive.org/details/yearsoftrialandh000234mbp/,
    source_descript: "Harry S Truman, “Memoirs by Harry S. Truman: Years of Trial and Hope”",
    source_citation: "",
};
pub const BOOK_MARITIME_CUSTOMS: File = File {
    internal_source: asset!("assets/docs/cn/book_maritime_customs.pdf"),
    source_descript: "Treaties, Conventions, Etc., Between China and Foreign States, Volume 2",
    external_source: "https://www.google.com/books/edition/China/fmAvAQAAMAAJ/",
    source_citation: "<i>Belgium, Sweden and Norway, Sweden, Germany, Portugal, Denmark, The Netherlands , Spain, Italy, Austria-Hungary, Japan, Peru, Brazil , Congo Free State, Mexico, Korea</i> \
    Treaties, Conventions, Etc., Between China and Foreign States. Vol.2                           \
    Statistical Department of the Inspectorate General of Customs, 1917. Accessed February 6, 2025.",
};
pub const BOOK_DOS_BULLETIN_V013_N0327: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v013_n0327.pdf"),
    external_source: "https://www.google.com/books/edition/us/Bmf-ES6JBs8C/",
    source_descript: "Department of State, “Bulletin,” Volume 13, No. 327",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V013_N0330: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v013_n0330.pdf"),
    external_source: "https://www.google.com/books/edition/us/Bmf-ES6JBs8C/",
    source_descript: "Department of State, “Bulletin,” Volume 13, No. 330",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V014_N0345: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v014_n0345.pdf"),
    external_source: "https://www.google.com/books/edition/us/lolNAQAAMAAJ/",
    source_descript: "Department of State, “Bulletin,” Volume 14, No. 345",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V024_N0601: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v024_n0601.pdf"),
    external_source: "https://www.google.com/books/edition/us/3OBPDH5wnokC/",
    source_descript: "Department of State, “Bulletin,” Volume 24, No. 601",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V024_N0607: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v024_n0607.pdf"),
    external_source: "https://www.google.com/books/edition/us/3OBPDH5wnokC/",
    source_descript: "Department of State, “Bulletin,” Volume 24, No. 607",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V024_N0621: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v024_n0621.pdf"),
    external_source: "https://www.google.com/books/edition/us/lYZNAQAAMAAJ/",
    source_descript: "Department of State, “Bulletin,” Volume 24, No. 621",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V025_N0630: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v025_n0630.pdf"),
    external_source: "https://www.google.com/books/edition/us/nqLuhKAZw50C/",
    source_descript: "Department of State, “Bulletin,” Volume 25, No. 630",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V025_N0636: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v025_n0636.pdf"),
    external_source: "https://www.google.com/books/edition/us/2NxpZyfeV3wC/",
    source_descript: "Department of State, “Bulletin,” Volume 25, No. 636",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V025_N0638: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v025_n0638.pdf"),
    external_source: "https://www.google.com/books/edition/us/2NxpZyfeV3wC/",
    source_descript: "Department of State, “Bulletin,” Volume 25, No. 638",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V028_N0711: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v028_n0711.pdf"),
    external_source: "https://www.google.com/books/edition/us/hCgniF4jdf8C/",
    source_descript: "Department of State, “Bulletin,” Volume 28, No. 711",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V030_N0772: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v030_n0772.pdf"),
    external_source: "https://www.google.com/books/edition/us/hNJxpQXvGCgC/",
    source_descript: "Department of State, “Bulletin,” Volume 30, No. 772",
    source_citation: "",
};
pub const BOOK_DOS_BULLETIN_V030_N0780: File =  File {
    internal_source: asset!("assets/docs/us/book_dos_bulletin_v030_n0780.pdf"),
    external_source: "https://www.google.com/books/edition/us/hNJxpQXvGCgC/",
    source_descript: "Department of State, “Bulletin,” Volume 30, No. 780",
    source_citation: "",
};
pub const US_MEMO_1867_04_01_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_04_01_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 17,” 1 Apr 1867",
    source_citation: "",
};
pub const US_MEMO_1867_04_23_BURLINGAME: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_04_23_burlingame.pdf"),
    external_source: "https://catalog.archives.gov/id/189032217/",
    source_descript: "[Minister Plenipotentiary of US Legation in Beijing] Anson Burlingame: “Dispatch No. 137,” 23 Apr 1867",
    source_citation: "",
};
pub const US_MEMO_1867_04_30_BELL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_04_30_bell.pdf"),
    external_source: "https://catalog.archives.gov/id/312048762/",
    source_descript: "[Commander of the US Asiatic Squadron] Henry Haywood Bell: “Dispatch No. 39: Monthly Report of Distribution and Employment of Vessels of the Asiatic Squadron for April 1867,” 30 Apr 1867",
    source_citation: "",
};
pub const US_MEMO_1867_05_01_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_05_01_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 19,” 1 May 1867",
    source_citation: "",
};
pub const US_MEMO_1867_05_11_LEGENDRE_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_05_11_legendre_1.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 20,” 11 May 1867",
    source_citation: "",
};
pub const US_MEMO_1867_05_11_LEGENDRE_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_05_11_legendre_2.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 21,” 11 May 1867",
    source_citation: "",
};
pub const US_MEMO_1867_05_31_BELL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_05_31_bell.pdf"),
    external_source: "https://catalog.archives.gov/id/312048762/",
    source_descript: "[Commander of the US Asiatic Squadron] Henry Haywood Bell: “Dispatch No. 44: Monthly Report of Distribution etc. of Vessels of Squadron for May 1867,” 31 May 1867",
    source_citation: "",
};
pub const US_MEMO_1867_06_03_BELL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_06_03_bell.pdf"),
    external_source: "https://catalog.archives.gov/id/312048762/",
    source_descript: "[Commander of the US Asiatic Squadron] Henry Haywood Bell: “Dispatch No. 46: Orders to Commander Febringer in the U.S.S. Ashuelot Having Been Sent to Formosa,” 3 Jun 1867",
    source_citation: "",
};
pub const US_MEMO_1867_06_05_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_06_05_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 22,” 5 Jun 1867",
    source_citation: "",
};
pub const US_MEMO_1867_06_19_BELL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_06_19_bell.pdf"),
    external_source: "https://catalog.archives.gov/id/312048762/",
    source_descript: "[Commander of the US Asiatic Squadron] Henry Haywood Bell: “Dispatch No. 53: Informs the Department of an Expedition to the South End of the Island of Formosa,” 19 Jun 1867",
    source_citation: "",
};
pub const US_MEMO_1867_06_19_BURLINGAME: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_06_19_burlingame.pdf"),
    external_source: "https://catalog.archives.gov/id/189032217/",
    source_descript: "[Minister Plenipotentiary of US Legation in Beijing] Anson Burlingame: “Dispatch No. 142,” 19 Jun 1867",
    source_citation: "",
};
pub const US_MEMO_1867_06_20_SEWARD: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_06_20_seward.pdf"),
    external_source: "https://catalog.archives.gov/id/149299561/",
    source_descript: "[US Secretary of State] William Henry Seward: “Dispatch No. 202,” 20 Jun 1867",
    source_citation: "",
};
pub const US_MEMO_1867_06_30_BELL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_06_30_bell.pdf"),
    external_source: "https://catalog.archives.gov/id/312048762/",
    source_descript: "[Commander of the US Asiatic Squadron] Henry Haywood Bell: “Dispatch No. 57.5: Monthly Report of Distribution and Employment of Vessels for the Month of June 1867,” 30 Jun 1867",
    source_citation: "",
};
pub const US_MEMO_1867_06_30_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_06_30_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 23,” 30 Jun 1867",
    source_citation: "",
};
pub const US_MEMO_1867_07_05_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_07_05_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 24,” 5 Jul 1867",
    source_citation: "",
};
pub const US_MEMO_1867_07_15_SEWARD: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_07_15_seward.pdf"),
    external_source: "https://catalog.archives.gov/id/149299561/",
    source_descript: "[US Secretary of State] William Henry Seward: “Dispatch No. 206,” 15 Jul 1867",
    source_citation: "",
};
pub const US_MEMO_1867_08_23_SEWARD: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_08_23_seward.pdf"),
    external_source: "https://catalog.archives.gov/id/149299561/",
    source_descript: "[US Secretary of State] William Henry Seward: “Dispatch No. 213,” 23 Aug 1867",
    source_citation: "",
};
pub const US_MEMO_1867_07_30_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_07_30_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 34,” 30 Jul 1867",
    source_citation: "",
};
pub const US_MEMO_1867_09_04_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_09_04_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 35,” 4 Sep 1867",
    source_citation: "",
};
pub const US_MEMO_1867_11_21_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_11_21_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 41,” 21 Nov 1867",
    source_citation: "",
};
pub const US_MEMO_1867_12_11_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1867_12_11_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 42,” 11 Dec 1867",
    source_citation: "",
};
pub const US_MEMO_1868_03_13_WILLIAMS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1868_03_13_williams.pdf"),
    external_source: "https://catalog.archives.gov/id/189032217/",
    source_descript: "[Minister Plenipotentiary of US Legation in Beijing] Samuel Wells Williams: “Dispatch No. 8,” 13 Mar 1868",
    source_citation: "",
};
pub const US_MEMO_1868_04_12_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1868_04_12_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210908978/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 58,” 12 Apr 1868",
    source_citation: "",
};
pub const US_MEMO_1869_05_10_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1869_05_10_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210909486/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 87,” 10 May 1869",
    source_citation: "",
};
pub const US_MEMO_1870_05_07_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1870_05_07_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210909877/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 109,” 7 May 1870",
    source_citation: "",
};
pub const US_MEMO_1870_06_09_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1870_06_09_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210909877/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 114,” 9 Jun 1870",
    source_citation: "",
};
pub const US_MEMO_1872_02_15_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1872_02_15_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210909877/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 163: Intended Visit to Southern Formosa,” 15 Feb 1872",
    source_citation: "",
};
pub const US_MEMO_1872_04_28_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1872_04_28_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210910450/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 168: Dispatch No. 73 to the Minister at Peking,” 28 Apr 1872",
    source_citation: "",
};
pub const US_MEMO_1872_04_29_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1872_04_29_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210910450/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 169: Statement of Facts in Reply to Mr. Low's Dispatch No. 140 to the Honorable Secretary of State,” 29 Apr 1872",
    source_citation: "",
};
pub const US_MEMO_1872_10_07_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1872_10_07_legendre.pdf"),
    external_source: "https://catalog.archives.gov/id/210910450/",
    source_descript: "[Consul of US Consulate in Xiamen] Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 187: Formosa's Affairs; Consul's Leave of Absence,” 7 Oct 1872",
    source_citation: "",
};
pub const US_MEMO_1872_12_18_FISH: File =  File {
    internal_source: asset!("assets/docs/us/memo_1872_12_18_fish.pdf"),
    external_source: "https://catalog.archives.gov/id/149321102/",
    source_descript: "[US Secretary of State] Hamilton Fish: “Dispatch No. 157,” 18 Dec 1874",
    source_citation: "",
};
pub const US_MEMO_1872_12_30_FISH: File =  File {
    internal_source: asset!("assets/docs/us/memo_1872_12_30_fish.pdf"),
    external_source: "https://catalog.archives.gov/id/149321102/",
    source_descript: "[US Secretary of State] Hamilton Fish: “Dispatch No. 165,” 30 Dec 1872",
    source_citation: "",
};
pub const US_MEMO_1873_05_13_LOW: File =  File {
    internal_source: asset!("assets/docs/us/memo_1873_05_13_low.pdf"),
    external_source: "https://catalog.archives.gov/id/189036884/",
    source_descript: "[Minister Plenipotentiary of US Legation in Beijing] Frederick Ferdinand Low: “Dispatch No. 256,” 13 May 1873",
    source_citation: "",
};
pub const US_MEMO_1873_05_22_LOW: File =  File {
    internal_source: asset!("assets/docs/us/memo_1873_05_22_low.pdf"),
    external_source: "https://catalog.archives.gov/id/189036884/",
    source_descript: "[Minister Plenipotentiary of US Legation in Beijing] Frederick Ferdinand Low: “Dispatch No. 259,” 22 May 1873",
    source_citation: "",
};
pub const US_MEMO_1873_06_13_LOW: File =  File {
    internal_source: asset!("assets/docs/us/memo_1873_06_13_low.pdf"),
    external_source: "https://catalog.archives.gov/id/189036884/",
    source_descript: "[Minister Plenipotentiary of US Legation in Beijing] Frederick Ferdinand Low: “Dispatch No. 264,” 13 Jun 1873",
    source_citation: "",
};
pub const US_MEMO_1873_07_10_LOW: File =  File {
    internal_source: asset!("assets/docs/us/memo_1873_07_10_low.pdf"),
    external_source: "https://catalog.archives.gov/id/189036884/",
    source_descript: "[Minister Plenipotentiary of US Legation in Beijing] Frederick Ferdinand Low: “Dispatch No. 271,” 10 Jul 1873",
    source_citation: "",
};
pub const US_MEMO_1874_04_22_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_04_22_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 76: The Japanese Government Preparing to Send and Expedition against Formosa,” 22 Apr 1874",
    source_citation: "",
};
pub const US_MEMO_1874_04_24_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_04_24_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 78: With Reference to Proposed Expedition of Japanese Government to Formosa,” 24 Apr 1874",
    source_citation: "",
};
pub const US_MEMO_1874_05_15_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_05_15_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910450/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 28: Japanese Expedition to Formosa,” 15 May 1874",
    source_citation: "",
};
pub const US_MEMO_1874_05_19_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_05_19_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 82: U.S. Chargé at Peking Writes that the Chinese Government Disclaim Having Given Any Permission to the Japanese to Go to Formosa,” 19 May 1874",
    source_citation: "",
};
pub const US_MEMO_1874_05_26_WILLIAMS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_05_26_williams.pdf"),
    external_source: "https://catalog.archives.gov/id/189037240/",
    source_descript: "[Chargé d'Affaires of US Legation in Beijing] Samuel Wells Williams: “Dispatch No. 33,” 26 May 1874",
    source_citation: "",
};
pub const US_MEMO_1874_05_29_WILLIAMS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_05_29_williams.pdf"),
    external_source: "https://catalog.archives.gov/id/189037240/",
    source_descript: "[Chargé d'Affaires of US Legation in Beijing] Samuel Wells Williams: “Dispatch No. 34,” 29 May 1874",
    source_citation: "",
};
pub const US_MEMO_1874_06_01_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_06_01_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910450/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 30: Japanese Expedition to Formosa,” 1 Jun 1874",
    source_citation: "",
};
pub const US_MEMO_1874_06_03_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_06_03_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 89: Sanjo's Proclamation Concerning Causes of Formosan Expedition Which Has Landed in Formosa and Has Been Attacked by Aborigines,” 6 Jun 1874",
    source_citation: "",
};
pub const US_MEMO_1874_06_03_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_06_03_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910450/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 31: Japanese Invasion of Formosa; Complicity of Citizens of the United States,” 3 Jun 1874",
    source_citation: "",
};
pub const US_MEMO_1874_06_06_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_06_06_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910450/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 32: Japanese in Formosa,” 6 Jun 1874",
    source_citation: "",
};
pub const US_MEMO_1874_06_10_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_06_10_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910869/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 33: Japanese Invasion of Formosa,” 10 Jun 1874",
    source_citation: "",
};
pub const US_MEMO_1874_06_15_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_06_15_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910869/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 34: Japanese in Formosa,” 15 Jun 1874",
    source_citation: "",
};
pub const US_MEMO_1874_06_23_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_06_23_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910869/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 36: Japanese in Formosa,” 23 Jun 1874",
    source_citation: "",
};
pub const US_MEMO_1874_07_11_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_07_11_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910869/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 40: Japanese in Formosa,” 11 Jul 1874",
    source_citation: "",
};
pub const US_MEMO_1874_07_23_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_07_23_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910869/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 41: Absence from Post,” 23 Jul 1874",
    source_citation: "",
};
pub const US_MEMO_1874_08_10_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_08_10_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910869/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 42: Arrest of Charles W. le Gendre,” 10 Aug 1874",
    source_citation: "",
};
pub const US_MEMO_1874_08_15_HENDERSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_08_15_henderson.pdf"),
    external_source: "https://catalog.archives.gov/id/210910869/",
    source_descript: "[Consul of US Consulate in Xiamen] Joseph J. Henderson: “Dispatch No. 44: Arrest of C. W. le Gendre,” 15 Aug 1874",
    source_citation: "",
};
pub const US_MEMO_1874_06_18_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_06_18_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 95: Attitude of China towards Expedition to Formosa by Japanese,” 18 Jun 1874",
    source_citation: "",
};
pub const US_MEMO_1874_07_16_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_07_16_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 98: The Attitude of China on the Formosan Question,” 16 Jul 1874",
    source_citation: "",
};
pub const US_MEMO_1874_07_29_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_07_29_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 105: Telegram of Department Received and Notice Given to American Citizens Not to Engage in Proceedings Hostile to China,” 29 Jul 1874",
    source_citation: "",
};
pub const US_MEMO_1874_07_29_FISH: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_07_29_fish.pdf"),
    external_source: "https://catalog.archives.gov/id/149321102/",
    source_descript: "[US Secretary of State] Hamilton Fish: “Dispatch No. 60,” 29 Jul 1874",
    source_citation: "",
};
pub const US_MEMO_1874_09_01_FISH: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_09_01_fish.pdf"),
    external_source: "https://catalog.archives.gov/id/149321102/",
    source_descript: "[US Secretary of State] Hamilton Fish: “Dispatch No. 67,” 1 Sep 1874",
    source_citation: "",
};
pub const US_MEMO_1874_10_08_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_10_08_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 132: Two Notifications by Japanese Government,” 8 Oct 1874",
    source_citation: "",
};
pub const US_MEMO_1874_10_09_CADWALADER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_10_09_cadwalader.pdf"),
    external_source: "https://catalog.archives.gov/id/149321102/",
    source_descript: "[US Assistant Secretary of State] John Lambert Cadwalader: “Dispatch No. 77,” 9 Oct 1874",
    source_citation: "",
};
pub const US_MEMO_1874_08_18_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_08_18_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188373861/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 112: Arrest of Le Gendre by U.S. Consul at Amoy,” 18 Aug 1874",
    source_citation: "",
};
pub const US_MEMO_1874_11_06_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_11_06_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188374353/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 147: Okuba, Japanese Ambassador, Has Left Peking Having Had a Rupture with Chinese Authorities,” 6 Nov 1874",
    source_citation: "",
};
pub const US_MEMO_1874_11_08_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_11_08_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188374353/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 149: Okuba Has Telegraphed to the Japanese Government that an Agreement Has Been Arrived at between Himself and Chinese Authorities,” 8 Nov 1874",
    source_citation: "",
};
pub const US_MEMO_1874_11_19_BINGHAM: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_11_19_bingham.pdf"),
    external_source: "https://catalog.archives.gov/id/188374353/",
    source_descript: "[Minister Plenipotentiary of US Legation in Tokyo] John Armor Bingham: “Dispatch No. 152: Convention between China and Japan Settling the Formosa Question,” 19 Nov 1874",
    source_citation: "",
};
pub const US_MEMO_1900_09_01_PIERCE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1900_09_01_pierce.pdf"),
    external_source: "https://catalog.archives.gov/id/188750528/",
    source_descript: "[First Secretary of US Embassy in St. Petersburg] Herbert Henry Davis Peirce, “Dispatch No. 303, September 10, 1900”",
    source_citation: "",
};
pub const US_MEMO_1902_02_01_HAY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1902_02_01_hay.pdf"),
    external_source: "https://catalog.archives.gov/id/149291967/",
    source_descript: "[Secretary of State] John Milton Hay, “Memorandum respecting Manchuria,” 1 Feb 1902",
    source_citation: "",
};
pub const US_MEMO_1902_02_03_TOWER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1902_02_03_tower.pdf"),
    external_source: "https://catalog.archives.gov/id/188751543/",
    source_descript: "[US Ambassador to Russia] Charlemagne Tower Jr.: “Dispatch No. 523, February 3, 1902”",
    source_citation: "",
};
pub const US_MEMO_1902_02_10_TOWER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1902_02_10_tower.pdf"),
    external_source: "https://catalog.archives.gov/id/188751543/",
    source_descript: "[US Ambassador to Russia] Charlemagne Tower Jr.: Telegram from St. Petersburg, 10 Feb 1902",
    source_citation: "",
};
pub const US_MEMO_1902_02_12_TOWER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1902_02_12_tower.pdf"),
    external_source: "https://catalog.archives.gov/id/188751543/",
    source_descript: "[US Ambassador to Russia] Charlemagne Tower Jr.: “Dispatch No. 529,” 12 Feb 1902",
    source_citation: "",
};
pub const US_MEMO_1902_02_13_TOWER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1902_02_13_tower.pdf"),
    external_source: "https://catalog.archives.gov/id/188751543/",
    source_descript: "[US Ambassador to Russia] Charlemagne Tower Jr.: Telegram from St. Petersburg, 13 Feb 1902",
    source_citation: "",
};
pub const US_MEMO_1941_12_08_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_08_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/605144866/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Telegram No. 481, Chungking, December 8, 1941”",
    source_citation: "",
};
pub const US_MEMO_1941_12_08_HULL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_08_hull.pdf"),
    external_source: "https://catalog.archives.gov/id/605144866/",
    source_descript: "[US Secretary of State] Cordell Hull: “Telegram No. 5749, Washington, 3 p.m., December 8, 1941”",
    source_citation: "",
};
pub const US_MEMO_1941_12_10_ROOSEVELT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_10_roosevelt.pdf"),
    external_source: "https://catalog.archives.gov/id/27579588/",
    source_descript: "[President of the United States] Franklin Delano Roosevelt: Telegram to [Prime Minister of the United Kingdom] Winston Leonard Spencer Churchill, December 10, 1941",
    source_citation: "",
};
pub const US_MEMO_1941_12_16_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_16_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/605149535/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Telegram No. 240, Chungking, December 16, 1941”",
    source_citation: "",
};
pub const US_MEMO_1941_12_12_WELLES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_12_welles.pdf"),
    external_source: "https://catalog.archives.gov/id/16608671/",
    source_descript: "[US Under Secretary of State] Benjamin Sumner Welles III: Letter to [President of the United States] Franklin Delano Roosevelt, 12 Dec 1941",
    source_citation: "",
};
pub const US_MEMO_1941_12_17_STIMSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_17_stimson.pdf"),
    external_source: "https://catalog.archives.gov/id/16608671/",
    source_descript: "[US Secretary of War] Henry Lewis Stimson: “Memorandum for General Watson,” 17 Dec 1941",
    source_citation: "",
};
pub const US_MEMO_1941_12_22_HULL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_22_hull.pdf"),
    external_source: "https://catalog.archives.gov/id/16608671/",
    source_descript: "[US Secretary of State] Cordell Hull: “Memorandum for the President: Termination of Hostilities,” 22 Dec 1941",
    source_citation: "",
};
pub const US_MEMO_1941_12_29_HULL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_29_hull.pdf"),
    external_source: "https://catalog.archives.gov/id/605144866/",
    source_descript: "[US Secretary of State] Cordell Hull: “Proposed Joint Declaration Against the Axis”, 29 Dec 1941",
    source_citation: "",
};
pub const US_MEMO_1941_12_19_HULL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_19_hull.pdf"),
    external_source: "https://catalog.archives.gov/id/605084180/",
    source_descript: "[US Secretary of State] Cordell Hull: “Memorandum for the President,” 19 Dec 1941",
    source_citation: "",
};
pub const US_MEMO_1941_12_19_SAVAGE_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_19_savage_1.pdf"),
    external_source: "https://catalog.archives.gov/id/605084180/",
    source_descript: "[US Assistant to the Secretary of State] Carlton Raymond Savage: “Memorandum for the Secretary,” 19 Dec 1941",
    source_citation: "",
};
pub const US_MEMO_1941_12_19_SAVAGE_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1941_12_19_savage_2.pdf"),
    external_source: "https://catalog.archives.gov/id/605084180/",
    source_descript: "[US Assistant to the Secretary of State] Carlton Raymond Savage: Memorandum, 19 Dec 1941",
    source_citation: "",
};
pub const US_MEMO_1942_01_01_EARLY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_01_01_early.pdf"),
    external_source: "https://catalog.archives.gov/id/16621006/",
    source_descript: "[Press Secretary] Stephen Tyree Early: “Declaration by United Nations,” 1 Jan 1942",
    source_citation: "",
};
pub const US_MEMO_1942_01_01_ROOSEVELT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_01_01_roosevelt.pdf"),
    external_source: "https://catalog.archives.gov/id/514106/",
    source_descript: "[President of the United States] Franklin Delano Roosevelt: “Declaration by the United Nations,” 1 Jan 1942",
    source_citation: "",
};
pub const US_MEMO_1942_02_11_GEROW: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_02_11_gerow.pdf"),
    external_source: "https://catalog.archives.gov/id/605149535/",
    source_descript: "[US Assistant Chief of Staff, War Plans Division] Leonard Townsend Gerow: “Memorandum for the Adjutant General: Far Eastern Situation,” 11 Feb 1942",
    source_citation: "",
};
pub const US_MEMO_1942_03_07_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_03_07_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/605150308/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Telegram No. 195,” 7 Mar 1942",
    source_citation: "",
};
pub const US_MEMO_1942_04_08_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_04_08_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/605151345/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Despatch No. 355,” 8 Apr 1942",
    source_citation: "",
};
pub const US_MEMO_1942_04_16_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_04_16_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/605151345/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Despatch No. 374,” 16 Apr 1942",
    source_citation: "",
};
pub const US_MEMO_1942_05_26_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_05_26_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/605152186/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Despatch No. 442,” 26 May 1942",
    source_citation: "",
};
pub const US_MEMO_1942_07_14_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_07_14_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/608091523/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Despatch No. 514,” 14 Jul 1942",
    source_citation: "",
};
pub const US_MEMO_1942_01_XX_CCS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1942_01_xx_ccs.pdf"),
    external_source: "https://www.jcs.mil/About/Joint-Staff-History/",
    /// https://www.jcs.mil/Portals/36/Documents/History/WWII/Arcadia3.pdf
    source_descript: "Combined Chiefs of Staff: “Proceedings of the American–British Joint Chiefs of Staff Conferences held in Washington,” Jan 1942",
    source_citation: "",
};
pub const US_MEMO_1943_01_XX_CCS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_01_xx_ccs.pdf"),
    external_source: "https://www.jcs.mil/About/Joint-Staff-History/",
    /// https://www.jcs.mil/Portals/36/Documents/History/WWII/Casablanca3.pdf
    source_descript: "Combined Chiefs of Staff: “Papers and Minutes of Meetings; CASABLANCA Conference,” Jan 1943",
    source_citation: "",
};
pub const US_MEMO_1943_03_09_DAVIES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_03_09_davies.pdf"),
    external_source: "https://catalog.archives.gov/id/16618380/",
    source_descript: "[Political Attaché] John Paton Davies II: “The Stilwell Mission,” 9 Mar 1943",
    source_citation: "",
};
pub const US_MEMO_1943_05_18_CURRIE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_05_18_currie.pdf"),
    external_source: "https://catalog.archives.gov/id/16620109/",
    source_descript: "[US Administrative Assistant to the President] Lauchlin Bernard Currie: “Memorandum for the President: Some Reflections on American Chinese Policy,” 18 May 1943",
    source_citation: "",
};
pub const US_MEMO_1943_10_18_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_10_18_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/605158204/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Despatch No. 1693, October 18, 1943: China's War Effort; Sino-American Relations”",
    source_citation: "",
};
pub const US_MEMO_1943_11_30_STETTINIUS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_11_30_stettinius.pdf"),
    external_source: "https://catalog.archives.gov/id/605158204/",
    source_descript: "[US Under Secretary of State] Edward Reilly Stettinius III: “Ambassador Gauss' despatch #1693 on China's War Effort,” 30 Nov 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_01_EARLY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_01_early.pdf"),
    external_source: "https://catalog.archives.gov/id/605165625/",
    source_descript: "[Press Secretary] Stephen Tyree Early: “Cairo Declaration,” 1 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_02_HULL: File =  File {
        internal_source: asset!("assets/docs/us/memo_1943_12_02_hull.pdf"),
    external_source: "https://catalog.archives.gov/id/16618380/",
    source_descript: "[US Secretary of State] Cordell Hull: “Memorandum for the President: Ambassador Gauss' despatch No. 1693 on China's War Effort,” 2 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_05_HOPKINS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_05_hopkins.pdf"),
    external_source: "https://catalog.archives.gov/id/27579712/",
    source_descript: "[Special Assistant to the President of the United States] Harold Lloyd Hopkins: “Memorandum for the Prime Minister,” 5 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_06_ROOSEVELT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_06_roosevelt.pdf"),
    external_source: "https://catalog.archives.gov/id/27579712/",
    source_descript: "[President of the United States] Franklin Delano Roosevelt: “Personal and Most Secret from the President to the Generalissimo,” 6 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_XX_CCS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_xx_ccs.pdf"),
    external_source: "https://www.jcs.mil/About/Joint-Staff-History/",
    /// https://www.jcs.mil/Portals/36/Documents/History/WWII/Sextant_Eureka3.pdf
    source_descript: "Combined Chiefs of Staff: “Papers and Minutes of Meetings; SEXTANT and EUREKA Conferences”",
    source_citation: "",
};
pub const US_MEMO_1943_12_09_GAUSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_09_gauss.pdf"),
    external_source: "https://catalog.archives.gov/id/16618380/",
    source_descript: "[US Ambassador to the Republic of China] Clarence Edward Gauss: “Telegram No. 2361, December 9, 3 p.m.,” 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_09_HEARN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_09_hearn.pdf"),
    external_source: "https://catalog.archives.gov/id/27579712/",
    source_descript: "[US Chief of Staff of the China–Burma–India Theater] Thomas Guerdon Hearn: “CM-IN-5966,” 9 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_09_SUMMERLIN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_09_summerlin.pdf"),
    external_source: "https://catalog.archives.gov/id/16618380/",
    source_descript: "[US Chief of Protocol] George Thomas Summerlin: “Memorandum for General Watson,” 9 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_17_STILWELL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_17_stilwell.pdf"),
    external_source: "https://catalog.archives.gov/id/750274/",
    source_descript: "[Deputy Supreme Commander of the Allied Powers] Joseph Warren Stilwell: Telegram from Generalissimo Chiang to President Roosevelt, 17 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_18_MORGENTHAU: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_18_morgenthau.pdf"),
    external_source: "https://catalog.archives.gov/id/16618380/",
    source_descript: "[US Secretary of the Treasury] Henry Morgenthau Jr: “Memorandum for the President,” 18 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_18_ROOSEVELT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_18_roosevelt.pdf"),
    external_source: "https://catalog.archives.gov/id/27579712/",
    source_descript: "[President of the United States] Franklin Delano Roosevelt: “Personal and Most Secret from the President to the Generalissimo,” 18 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_20_ROOSEVELT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_20_roosevelt.pdf"),
    external_source: "https://catalog.archives.gov/id/27579712/",
    source_descript: "[President of the United States] Franklin Delano Roosevelt: “Personal and Most Secret from the President for General Stilwell's Eyes Alone,” 20 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_23_HEARN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_23_hearn.pdf"),
    external_source: "https://catalog.archives.gov/id/27579712/",
    source_descript: "[US Chief of Staff of the China–Burma–India Theater] Thomas Guerdon Hearn: “CM-IN-14570,” 23 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_27_ROOSEVELT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_27_roosevelt.pdf"),
    external_source: "https://catalog.archives.gov/id/27579712/",
    source_descript: "[President of the United States] Franklin Delano Roosevelt: “Only for General Stilwell's Eyes for Delivery of the Following Message from the President to Generalissimo Chiang Kai-shek,” 27 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1943_12_31_HANDY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1943_12_31_handy.pdf"),
    external_source: "https://catalog.archives.gov/id/750274/",
    source_descript: "[Deputy Chief of Staff for the Army] Thomas Troy Handy, “Memorandum for the Chief of Staff: Equipping Chinese Divisions,” 31 Dec 1943",
    source_citation: "",
};
pub const US_MEMO_1944_XX_XX_MCCREA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1944_xx_xx_mccrea.pdf"),
    external_source: "https://catalog.archives.gov/id/27579362/",
    source_descript: "[Naval Aide to the President of the United States] John Livingstone McCrea, “The President and US Aid to China,” 1944",
    source_citation: "",
};
pub const US_MEMO_1944_05_04_BALLANTINE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1944_05_04_ballantine.pdf"),
    external_source: "https://www.ndl.go.jp/constitution/e/shiryo/01/001shoshi.html",
    source_descript: "[Director of the Office of Far Eastern Affairs] Joseph Ballantine, “Japan: The Post-War Objectives of the United States in Regard to Japan”",
    source_citation: "",
};
// todo: Get original document from NARA or whoever
pub const US_MEMO_1944_06_22_VINCENT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1944_06_22_vincent.pdf"),
    external_source: "https://catalog.archives.gov/id/750274/",
    source_descript: "[Chief of the Division of Chinese Affairs] John Carter Vincent, “Memorandum of Conversation Conversation between Vice President Henry A. Wallace and President Chiang Kai-shek,” 22 Jun 1944",
    source_citation: "",
};
pub const US_MEMO_1945_03_19_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_03_19_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981524/",
    source_descript: "[Secretary of the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                     [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                     “SWNCC 68: Politico-Military Problems in the Far East: National Composition of Forces to Occupy Formosa”",
    source_citation: "",
};
pub const US_MEMO_1945_05_12_GREW_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_05_12_grew_1.pdf"),
    external_source: "https://catalog.archives.gov/id/605164181/",
    source_descript: "[US Deputy Secretary of State] Joseph Clark Grew, “Memorandum for the Secretary of the Navy,” 12 May 1945",
    source_citation: "",
};
pub const US_MEMO_1945_05_12_GREW_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_05_12_grew_2.pdf"),
    external_source: "https://catalog.archives.gov/id/605164181/",
    source_descript: "[US Deputy Secretary of State] Joseph Clark Grew, “Memorandum for the Secretary of War,” 12 May 1945",
    source_citation: "",
};
pub const US_MEMO_1945_06_11_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_06_11_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                     [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                     “SWNCC 150: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 11 Jun 1945",
    source_citation: "",
};
pub const US_MEMO_1945_07_21_THOMPSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_07_21_thompson.pdf"),
    external_source: "https://catalog.archives.gov/id/134405411/",
    source_descript: "[Second Secretary of US Embassy, London] Llewellyn Edward Thompson Jr., “Meeting of Heads of Government, Berlin, July 21, 1945, 5 p.m.”",
    source_citation: "",
};
pub const US_MEMO_1945_07_22_THOMPSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_07_22_thompson.pdf"),
    external_source: "https://catalog.archives.gov/id/134405442/",
    source_descript: "[Second Secretary of US Embassy, London] Llewellyn Edward Thompson Jr., “Meeting of Heads of Government, Berlin, July 22, 1945, 5 p.m.”",
    source_citation: "",
};
pub const US_MEMO_1945_07_27_THOMPSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_07_22_thompson.pdf"),
    external_source: "https://catalog.archives.gov/id/134405822/",
    source_descript: "[Second Secretary of US Embassy, London] Llewellyn Edward Thompson Jr., “Meeting of Foreign Ministers, Berlin, July 27, 1945, 4 p.m.”",
    source_citation: "",
};
pub const US_MEMO_1945_07_30_SWNCC_SFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_07_30_swncc_sfe.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981524/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 104: Politico-Military Problems of the Far East: National Composition of Forces to Occupy Formosa (Including the Pescadores) in the Post-Defeat Period: Relations of the Military Government of Formosa with China and the Chinese”",
    source_citation: "",
};
pub const US_MEMO_1945_08_03_ROBERTS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_03_roberts.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981524/",
    source_descript: "[Department of War, Representative in the SWNCC Subcommittee for the Far East (SFE)] Frank Needham Roberts, “Memorandum for Mr. H. M. Benninghoff, Captain Lorenzo Sablin: SFE 104 - National Composition of Forces to Occupy Formosa”",
    source_citation: "",
};
pub const US_MEMO_1945_08_06_SWNCC_SFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_06_swncc_sfe.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981524/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 104/1: Politico-Military Problems of the Far East: National Composition of Forces to Occupy Formosa (Including the Pescadores) in the Post-Defeat Period: Relations of the Military Government of Formosa with China and the Chinese”",
        source_citation: "",
};
pub const US_MEMO_1945_08_09_ROSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_09_ross.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/270626624/",
    source_descript: "[Press Secretary] Charles Griffith Ross, Press Release: Radio Address of the President, 9 Aug 1945",
    source_citation: "",
};
pub const US_MEMO_1945_08_10_MCFARLAND: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_10_mcfarland.pdf"),
    external_source: "https://catalog.archives.gov/id/141212654/",
    source_descript: "[Secretary of the Joint Chiefs of Staffs] Andrew Jackson McFarland: “Memorandum for the SWNCC: Draft of Letter to General Wedemeyer,” 10 Aug 1945",
    source_citation: "",
};
pub const US_MEMO_1945_08_11_HURLEY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_11_hurley.pdf"),
    external_source: "https://catalog.archives.gov/id/205715926/",
    source_descript: "[Ambassador to the Republic of China] Patrick Jay Hurley, “Telegram No. 1330, August 11, 4 p.m.”",
    source_citation: "",
};
pub const US_MEMO_1945_08_11_SWNCC_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_11_swncc_1.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee, “Minutes of the Meeting held in Room 200, The State Department Building, on Saturday, 11 August 1945, at 1430”",
    source_citation: "",
};
pub const US_MEMO_1945_08_11_SWNCC_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_11_swncc_1.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                      [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                      [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                      “SWNCC 150/1: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 11 Aug 1945",
    source_citation: "",
};
pub const US_MEMO_1945_08_12_SWNCC_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_12_swncc_1.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee, “Minutes of the Meeting held in Room 4E-886, The Pentagon, on Sunday, 12 August 1945, at 1000”",
    source_citation: "",
};
pub const US_MEMO_1945_08_12_SWNCC_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_12_swncc_2.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                      [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                      [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                      “SWNCC 150/2: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 12 Aug 1945",
    source_citation: "",
};
pub const US_MEMO_1945_08_22_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_22_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                      [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                      [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                      “SWNCC 150/3: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 22 Aug 1945",
    source_citation: "",
};
pub const US_MEMO_1945_08_23_SWNCC_SFE_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_23_swncc_sfe_1.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981524/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, \
                      “SFE 104/2: Politico-Military Problems of the Far East: National Composition of Forces to Occupy Formosa (Including the Pescadores) in the Post-Defeat Period: Relations of the Military Government of Formosa with China and the Chinese,” 23 Aug 1945",
        source_citation: "",
};
pub const US_MEMO_1945_08_23_SWNCC_SFE_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_23_swncc_sfe_2.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981524/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 104/3: Politico-Military Problems of the Far East: National Composition of Forces to Occupy Formosa (Including the Pescadores) in the Post-Defeat Period: Relations of the Military Government of Formosa with China and the Chinese,” 23 Aug 1945",
        source_citation: "",
};
pub const US_MEMO_1945_08_23_SWNCC_SFE_3: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_23_swncc_sfe_3.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 107: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 23 Aug 1945",
        source_citation: "",
};
pub const US_MEMO_1945_08_29_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_29_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981463/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                      [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                      [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                      “SWNCC 181/D: Custody of Japanese Archives and Diplomatic Property”",
        source_citation: "",
};
// TODO: uncertain utility
pub const US_MEMO_1945_08_30_JCAC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_30_jcac.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the Joint Civil Affairs Committee (JCAC)] Warde Mitchell Cameron, “JCAC 48: Basic  Initial Post-Surrender Directive to the SCAP for the Occupation and Control of Japan,” 30 Aug 1945",
    source_citation: "",
};
// TODO: uncertain utility
pub const US_MEMO_1945_08_30_MCFARLAND: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_30_mcfarland.pdf"),
    external_source: "https://catalog.archives.gov/id/141212654/",
    source_descript: "[Secretary of the Joint Chiefs of Staffs] Andrew Jackson McFarland: “Memorandum for the SWNCC: Directive Concerning Policy and Operations in China after Japanese Capitulation,” 30 Aug 1945",
    source_citation: "",
};
pub const US_MEMO_1945_08_30_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_30_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                      [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                      [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                      “SWNCC 150/3 (Revised): Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 30 Aug 1945",
    source_citation: "",
};
pub const US_MEMO_1945_08_31_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_08_31_swncc.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee, “Minutes of the Meeting held in Room 200, The State Department Building, on Friday, 31 August 1945, at 1030”",
    source_citation: "",
};
pub const US_MEMO_1945_09_01_SWNCC_SFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_01_swncc_sfe.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981463/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 117: Custody of Japanese Archives and Diplomatic Property”",
    source_citation: "",
};
pub const US_MEMO_1945_09_03_BYRNES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_03_byrnes.pdf"),
    external_source: "https://catalog.archives.gov/id/205715936/",
    source_descript: "[Secretary of State] James Francis Byrnes, “Memorandum for the President: Chinese Matters Arising out of Conversations with T. V. Soong,” 3 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_03_ELSEY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_03_elsey.pdf"),
    external_source: "https://catalog.archives.gov/id/205715936/",
    source_descript: "George McKee Elsey, “Memorandum for Commodore Vardaman,” September 3, 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_03_HULL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_03_hull.pdf"),
    external_source: "https://catalog.archives.gov/id/205715941/",
    source_descript: "[Acting Deputy Chief of Staff] John Edwin Hull, “Memorandum for the President: United States Commitment for Equipment of the Chinese Army,” 3 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_03_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_03_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981463/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                      [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                      [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                      “SWNCC 181/1: Custody of Japanese Archives and Diplomatic Property,” 3 Sep 1945",
        source_citation: "",
};
pub const US_MEMO_1945_09_04_SWNCC_SFE_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_04_swncc_sfe_1.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981463/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 117/1: Custody of Japanese Archives and Diplomatic Property”",
    source_citation: "",
};
pub const US_MEMO_1945_09_04_SWNCC_SFE_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_04_swncc_sfe_2.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981463/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 117/2: Custody of Japanese Archives and Diplomatic Property”",
    source_citation: "",
};
pub const US_MEMO_1945_09_05_LEAHY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_05_leahy.pdf"),
    external_source: "https://catalog.archives.gov/id/205715941/",
    source_descript: "William Daniel Leahy, “Memorandum for the President,” September 5, 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_06_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_06_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                     [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                     “SWNCC 150/4: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 6 Sep 1945",
    source_citation: "",
};
// TODO obtain original
pub const US_MEMO_1945_09_07_ACHESON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_07_acheson.pdf"),
    external_source: "https://catalog.archives.gov/id/750274/",
    source_descript: "[US Deputy Secretary of State] Dean Gooderham Acheson: Memorandum of Conversation among the President, Dr. T. V. Soong, and Acting Secretary Acheson; 7 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_10_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_10_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981463/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                      [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                      [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                      “SWNCC 181/2: Custody of Japanese Archives and Diplomatic Property,” 10 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_12_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_12_swncc.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee: “Minutes of the Meeting held in Room 200, The State Department Building, on Wednesday, 12 September 1945, at 1500”",
    source_citation: "",
};
pub const US_MEMO_1945_09_13_ACHESON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_13_acheson.pdf"),
    external_source: "https://www.ndl.go.jp/constitution/e/shiryo/01/023shoshi.html",
    source_descript: "[US Deputy Secretary of State] Dean Gooderham Acheson: “Memorandum for the President: Authority of the Supreme Commander for the Allied Powers,” 13 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_17_SWNCC_SFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_17_swncc_sfe.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 107/1: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 17 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_19_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_19_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                     [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                     “SWNCC 150/5: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 19 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_19_SWNCC_SFE_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_19_swncc_sfe_1.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981524/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 104/4: Politico-Military Problems of the Far East: National Composition of Forces to Occupy Formosa (Including the Pescadores) in the Post-Defeat Period: Relations of the Military Government of Formosa with China and the Chinese”",
    source_citation: "",
};
pub const US_MEMO_1945_09_19_SWNCC_SFE_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_19_swncc_sfe_2.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Hugh Dexter Farley, “SFE 107/2: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 19 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_21_SWNCC_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_21_swncc_1.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee: “Minutes of the Meeting held in Room 200, The State Department Building, on Friday, 21 September 1945, at 1030”",
    source_citation: "",
};
pub const US_MEMO_1945_09_21_SWNCC_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_21_swncc_2.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                     [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                     “SWNCC 150/4/A: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 21 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_21_SWNCC_3: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_21_swncc_3.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981463/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                     [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                     “SWNCC 181/3: Custody of Japanese Archives and Diplomatic Property,” 21 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_24_ROSS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_24_ross.pdf"),
    external_source: "https://catalog.archives.gov/id/270626908/",
    source_descript: "[Press Secretary] Charles Griffith Ross: “#276,” 24 Nov 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_24_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_24_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Alexander Davidson Reid, \
                     [Secretary to the SWNCC for the Department of the Navy] Bernard Lige Austin, \
                     “SWNCC 150/6: Politico-Military Problems in the Far East: United States Initial Post-Defeat Policy Relating to Japan,” 24 Sep 1945",
    source_citation: "",
};
pub const US_MEMO_1945_09_25_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_09_25_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981524/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                     [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                     “SWNCC 68/1: Politico-Military Problems in the Far East: National Composition of Forces to Occupy Formosa”",
        source_citation: "",
};
pub const US_MEMO_1945_10_10_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_10_10_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981463/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Charles Weller McCarthy, \
                     [Secretary to the SWNCC for the Department of the Navy] Alvin Franklin Richardson, \
                     “SWNCC 181/4: Custody of Japanese Archives and Diplomatic Property,” 10 Oct 1945",
    source_citation: "",
};
pub const US_MEMO_1945_11_03_MCFARLAND: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_11_03_mcfarland.pdf"),
    external_source: "https://www.ndl.go.jp/constitution/e/shiryo/01/036shoshi.html",
    source_descript: "[Secretary of the Joint Chiefs of Staffs] Andrew Jackson McFarland, \
                     [Deputy Secretary of the Joint Chiefs of Staffs (JCS)] Charles Johnes Moore, \
                     “Basic Directive for Post-Surrender Military Government in Japan Proper”",
    source_citation: "",
};
pub const US_MEMO_1945_11_27_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_11_27_swncc.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee: “Minutes of the Meeting held in Room 200, The State Department Building, on Tuesday, 27 September 1945, at 1430”",
    source_citation: "",
};
pub const US_MEMO_1945_12_07_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_12_07_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Alexander Davidson Reid, \
                     [Secretary to the SWNCC for the Department of the Navy] Bernard Lige Austin, \
                     “SWNCC 150/7/D: Post-Surrender Policy for Japan,” 7 Dec 1945",
    source_citation: "",
};
pub const US_MEMO_1945_12_14_SWNCC_SFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_12_14_swncc_sfe.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Alvin Franklin Richardson, “SFE 107/3: Post-Surrender Policy for Japan,” 14 Dec 1945",
    source_citation: "",
};
pub const US_MEMO_1945_12_18_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_12_18_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Alexander Davidson Reid, \
                     [Secretary to the SWNCC for the Department of the Navy] Bernard Lige Austin, \
                     “SWNCC 150/8: Post-Surrender Policy for Japan,” 12 Dec 1945",
    source_citation: "",
};
pub const US_MEMO_1945_12_19_SWNCC_SFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_12_19_swncc_sfe.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981527/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Alvin Franklin Richardson, “SFE 107/4: Post-Surrender Policy for Japan,” 19 Dec 1945",
    source_citation: "",
};
pub const US_MEMO_1945_12_20_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1945_12_20_swncc.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee: “Minutes of the Meeting held in Room 200, The State Department Building, on Thursday, 20 December 1945, at 1430”",
    source_citation: "",
};
pub const US_MEMO_1946_01_29_ALLEN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1946_01_29_allen.pdf"),
    external_source: "https://www.cas.go.jp/jp/ryodo_eg/shiryo/takeshima/detail/t1946012900101.html",
    source_descript: "[US Assistant Adjutant General to SCAP] Harold W. Allen, “SCAPIN-677: Governmental and Administrative Separation of Certain Outlying Areas from Japan,” 29 Jan 1946",
    source_citation: "",
};
pub const US_MEMO_1946_03_05_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1946_03_05_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981458/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Alexander Davidson Reid, \
                     [Secretary to the SWNCC for the Department of the Navy] Bernard Lige Austin, \
                     “SWNCC 272/D: Sovereignty of Formosa”",
        source_citation: "",
};
pub const US_MEMO_1946_03_07_SWNCC_SFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1946_03_07_swncc_sfe.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981458/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Alvin Franklin Richardson, “SFE 168: Sovereignty of Formosa”",
    source_citation: "",
};
pub const US_MEMO_1946_03_08_BORTON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1946_03_08_borton.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981458/",
    source_descript: "[Chief of the Division of Japanese Affairs] Hugh Borton, Memorandum for [Deputy Director of the Office of Far Eastern Affairs] James Kedzie Penfield",
    source_citation: "",
};
pub const US_MEMO_1946_03_12_SWNCC_SFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1946_03_12_swncc_sfe.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981458/",
    source_descript: "[Secretary of the SWNCC Subcommittee for the Far East (SFE)] Alvin Franklin Richardson, “SFE 168/1: Sovereignty of Formosa”",
    source_citation: "",
};
pub const US_MEMO_1946_03_11_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1946_03_11_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981458/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Raymond Edwin Cox, \
                     [Secretary to the SWNCC for the Department of War] Alexander Davidson Reid, \
                     [Secretary to the SWNCC for the Department of the Navy] Bernard Lige Austin, \
                     “SWNCC 272/1: Sovereignty of Formosa”",
        source_citation: "",
};
pub const US_MEMO_1946_03_22_FITCH: File =  File {
    internal_source: asset!("assets/docs/us/memo_1946_03_22_fitch.pdf"),
    external_source: "https://catalog.archives.gov/id/430646575/",
    source_descript: "[US Adjutant General to SCAP] Burdette Mase Fitch, “SCAPIN-841: Governmental and Administrative Separation of Certain Outlying Areas from Japan,” 22 Mar 1946",
    source_citation: "",
};
pub const US_MEMO_1946_03_24_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1946_03_24_swncc.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/13981458/",
    source_descript: "[Secretary to the SWNCC for the Department of State] Harold Wilcox Moseley, \
                      [Secretary to the SWNCC for the Department of War] Alexander Davidson Reid, \
                          [Secretary to the SWNCC for the Department of the Navy] Bernard Lige Austin, \
                          “Decision on SWNCC 272/1: Sovereignty of Formosa”",
        source_citation: "",
};
pub const US_MEMO_1947_01_29_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1947_01_29_swncc.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee: “Minutes of the Meeting held in Room 285, The State Department Building, on Wednesday, 29 January 1947, at 1430”",
    source_citation: "",
};
pub const US_MEMO_1947_02_19_SWNCC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1947_02_19_swncc.pdf"),
    external_source: "https://catalog.archives.gov/id/178696266/",
    source_descript: "State-War-Navy Coordinating Committee: “Minutes of the Meeting held in Room 285, The State Department Building, on Wednesday, 19 February 1947, at 1430”",
    source_citation: "",
};
pub const US_MEMO_1947_03_12_MCLENNAN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1947_03_12_mclennan.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "[Chief of Staff for Intelligence, G-2 (Intelligence), Philippines–Ryukyu Command] Donald Glover McLennan, “Conditions in Formosa”",
    source_citation: "",
};
pub const US_MEMO_1947_03_12_FEC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1947_03_12_fec.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "G-2 (Intelligence), Far East Command, “Summary of Available Information on Formosa (Taiwan) Uprising”",
    source_citation: "",
};
pub const US_MEMO_1947_05_26_HERR: File =  File {
    internal_source: asset!("assets/docs/us/memo_1947_05_26_herr.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_55_00C0028",
    source_descript: "George Kerr, “Probability of Communist Penetration in Formosa”",
    source_citation: "",
};
pub const US_MEMO_1947_08_11_BLAKE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1947_08_11_blake.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_55_00C0028",
    source_descript: "Ralph Joseph Blake, “A Brief Resume on Postwar Conditions on the Island of Taiwan”",
    source_citation: "",
};
pub const US_MEMO_1947_08_21_BLAKE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1947_08_21_blake.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_55_00C0028",
    source_descript: "Ralph Joseph Blake, “Telegram Received, August 21, 1947”",
    source_citation: "",
};
pub const US_MEMO_1947_11_14_CIA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1947_11_14_cia.pdf"),
    external_source: "https://catalog.archives.gov/id/225248097/",
    source_descript: "Central Intelligence Agency: “ORE 44: The Japanese Peace Treaty Problems, Issues, and Reactions,” 14 Nov 1947",
    source_citation: "",
};
pub const US_MEMO_1948_01_01_CIA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1948_01_01_cia.pdf"),
    external_source: "https://catalog.archives.gov/id/486499023/",
    source_descript: "Central Intelligence Agency: “SR 8: China,” 1948",
    source_citation: "",
};
pub const US_MEMO_1948_12_16_CIA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1948_12_16_cia.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/cia-rdp67-00059a000500070001-8",
    source_descript: "Central Intelligence Agency: “CIA 12–48: Review of the World Situation”",
    source_citation: "",
};
pub const US_MEMO_1948_12_17_CIA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1948_12_17_cia.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/cia-rdp67-00059a000100120009-8",
    source_descript: "Central Intelligence Agency: “Minuted of Intelligence Advisory Committee Meeting,” 17 Dec 1948",
    source_citation: "",
};
pub const US_MEMO_1949_01_19_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_01_19_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0091",
    source_descript: "National Security Council, “NSC 37/1: The Position of the United States with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_01_28_WILLOUGHBY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_01_28_willoughby.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "[Chief of Staff for Intelligence, G-2 (Intelligence), Far East Command] Charles Andrew Willoughby, Transcript of Statement by [Inspector General, Chinese Maritime Customs Service] Lester Knox Little",
    source_citation: "",
};
pub const US_MEMO_1949_02_03_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_02_03_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0092",
    source_descript: "National Security Council, “NSC 37/2: The Current Position of the United States with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_02_10_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_02_10_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0093",
    source_descript: "National Security Council, “NSC 37/3: The Strategic Importance of Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_02_18_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_02_18_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0094",
    source_descript: "National Security Council, “NSC 37/4: The Current Position of the United States with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_03_01_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_03_01_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0095",
    source_descript: "National Security Council, “NSC 37/5: Supplemental Measures with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_03_07_DUMOULIN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_03_07_dumoulin.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "[G-2 (Intelligence), Far East Command] Alfred Kirk duMoulin, “Formosan League for Re-Emancipation,” October 3, 1949",
    source_citation: "",
};
pub const US_MEMO_1949_04_28_ACHESON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_04_28_acheson.pdf"),
    external_source: "https://archive.org/details/dos-report_s-11949-52-supp/",
    source_descript: "[Secretary of State] Dean Gooderham Acheson: “Meeting in the President's Office with Senators Wherry and Bridges on China,” 28 Apr 1949",
    source_citation: "",
};
pub const US_MEMO_1949_08_05_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_08_05_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0096",
    source_descript: "National Security Council, “NSC 37/6: Current Position of the US with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_08_22_BRIDGES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_08_22_nsc.pdf"),
    external_source: "https://history.nebraska.gov/collection_section/kenneth-s-wherry-1892-1951-rg3559-am/",
    // https://history.nebraska.gov/wp-content/uploads/2021/08/doc_3559am-B17-Press-Release-August-21-22-1949.pdf
    source_descript: "[US Senator] Henry Styles Bridges, “Memorandum on the White Paper on U.S. Relations with China,” 22 Aug 1949",
    source_citation: "",
};
pub const US_MEMO_1949_08_22_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_08_22_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0097",
    source_descript: "National Security Council, “NSC 37/7: The Position of the United States with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_08_30_KO: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_08_30_ko.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8342156/",
    source_descript: "[G-2 (Intelligence), Far East Command] Mr. \"KO\", “Outline of Observations of Conditions in Formosa by a Certain Japanese,” August 30, 1949",
    source_citation: "",
};
pub const US_MEMO_1949_08_31_KO: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_08_31_ko.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8342156/",
    source_descript: "[G-2 (Intelligence), Far East Command] Mr. \"KO\", “Outline of Observations of Conditions in Formosa by a Certain Japanese,” August 31, 1949",
    source_citation: "",
};
pub const US_MEMO_1949_08_31_JESSUP: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_08_31_jessup.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_35_00C0008",
    source_descript: "Philip Caryl Jessup, “Letter to Deputy to Assistant Secretary of State for Occupied Areas Walter Wilds”",
    source_citation: "",
};
pub const US_MEMO_1949_10_03_LACEY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_10_03_lacey.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "[G-2 (Intelligence), Far East Command] Arthur Titman Lacey, “Formosa,” October 3, 1949",
    source_citation: "",
};
pub const US_MEMO_1949_10_06_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_10_06_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0098",
    source_descript: "National Security Council, “NSC 37/8: The Position of the United States with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_10_06_WALKER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_10_06_walker.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "[Chief of Staff for Intelligence, G-2 (Intelligence), 25th Infantry Division, I Corps] George LaVerne Walker, “Report on Formosa”",
    source_citation: "",
};
pub const US_MEMO_1949_10_08_ROBERTSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_10_08_robertson.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_99_00C0008",
    source_descript: "Walter Spencer Robertson, “Letter to Philip Caryl Jessup”",
    source_citation: "",
};
pub const US_MEMO_1949_10_20_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_10_20_nsc.pdf"),
    external_source: "https://catalog.archives.gov/id/750721",
    source_descript: "National Security Council: “Memorandum for the President,” 20 Oct 1949",
    source_citation: "",
};
pub const US_MEMO_1949_11_03_WILLOUGHBY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_11_03_willoughby.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "[Chief of Staff for Intelligence, G-2 (Intelligence), Far East Command] Charles Andrew Willoughby, “Possible Coup D'Etat on Taiwan”",
    source_citation: "",
};
pub const US_MEMO_1949_11_10_RINALDUCCI: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_11_10_rinalducci.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "[G-2 (Intelligence), Far East Command] Ralph Joseph Rinalducci, “Report on Formosa,” September 10, 1949",
    source_citation: "",
};
pub const US_MEMO_1949_11_26_WILLOUGHBY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_11_26_willoughby.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/9850613/",
    source_descript: "[Chief of Staff for Intelligence, G-2 (Intelligence), Far East Command] Charles Andrew Willoughby, “Staff Visit to Taiwan”",
    source_citation: "",
};
pub const US_MEMO_1949_11_30_RINALDUCCI: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_11_30_rinalducci.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/8341943/",
    source_descript: "[G-2 (Intelligence), Far East Command] Ralph Joseph Rinalducci, “Report on Formosa,” September 30, 1949",
    source_citation: "",
};
pub const US_MEMO_1949_12_27_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_12_27_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0090",
    source_descript: "National Security Council, “NSC 37/9: Possible United States Military Action toward Taiwan not Involving Major Military Forces”",
    source_citation: "",
};
pub const US_MEMO_1949_12_29_FEAREY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_12_29_fearey.pdf"),
    external_source: "https://db.history.go.kr/contemp/hihd/level.do?levelId=hihd_1949_12_29_0020",
    source_descript: "Robert Appleton Fearley, Memorandum for Deputy Director of the Office of Far East Affairs (John Moore Allison), “Japanese Treaty”",
    source_citation: "",
};
pub const US_MEMO_1949_12_30_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1949_12_30_nsc.pdf"),
    external_source: "https://catalog.archives.gov/id/750721",
    source_descript: "National Security Council: “Memorandum for the President,” 30 Dec 1949",
    source_citation: "",
};
pub const US_MEMO_1950_01_04_MCFALL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_01_04_mcfall.pdf"),
    external_source: "https://catalog.archives.gov/id/183391290/",
    source_descript: "[Assistant Secretary of State for Legislative Affairs] Jack Kirkham McFall: “Memorandum of Conversation: Formosa Problem”",
    source_citation: "",
};
pub const US_MEMO_1950_01_05_BATTLE_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_01_05_battle_1.pdf"),
    external_source: "https://catalog.archives.gov/id/183391299/",
    source_descript: "[US Special Assistant to the Secretary of State] Lucius Durham Battle: “Meeting with the President,” 5 Jan 1950",
    source_citation: "",
};
pub const US_MEMO_1950_01_05_BATTLE_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_01_05_battle_2.pdf"),
    external_source: "https://catalog.archives.gov/id/183391305/",
    source_descript: "[US Special Assistant to the Secretary of State] Lucius Durham Battle: “Meeting with the President,” 5 Jan 1950   ",
    source_citation: "",
};
pub const US_MEMO_1950_05_22_EVANS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_05_22_evans.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_100_00C0029",
    source_descript: "Allan Evans, “Informal Comments on Statements on Formosa”",
    source_citation: "",
};
pub const US_MEMO_1950_06_14_MACARTHUR_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_06_14_macarthur_1.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/9850613/",
    source_descript: "Douglas MacArthur, “Memorandum on Formosa,” June 14, 1950",
    source_citation: "",
};
pub const US_MEMO_1950_06_14_MACARTHUR_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_06_14_macarthur_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_35_00C0008",
    source_descript: "Douglas MacArthur, “Memorandum on Formosa,” June 14, 1950",
    source_citation: "",
};
pub const US_MEMO_1950_06_27_PRESS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_06_27_press.pdf"),
    external_source: "https://catalog.archives.gov/id/229036774/",
    source_descript: "White House Office of the Press Secretary: “Statement by the President”, 27 Jun 1950",
    source_citation: "",
};
pub const US_MEMO_1950_06_28_WHITE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_06_28_white.pdf"),
    external_source: "https://catalog.archives.gov/id/406140747/",
    source_descript: "Lincoln White, “Memorandum of the Press and Radio News Conference, Wednesday, June 28, 195O”",
    source_citation: "",
};
pub const US_MEMO_1950_07_10_STRONG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_10_strong.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0014",
    source_descript: "Robert Campbell Strong, “Telegram No. 34, July 10, Noon”",
    source_citation: "",
};
pub const US_MEMO_1950_07_11_STUART: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_11_stuart.pdf"),
    external_source: "https://catalog.archives.gov/id/406141141/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1950_07_14_STUART: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_14_stuart.pdf"),
    external_source: "https://catalog.archives.gov/id/406141155/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1950_07_25_FREEMAN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_25_freeman.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0014",
    source_descript: "Fulton Freeman, “Problem Arising out of President's Statement of June 27 with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1950_07_27_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_27_nsc.pdf"),
    external_source: "https://catalog.archives.gov/id/750722/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1950_07_28_DAVIS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_28_davis.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS128_04_05C0008",
    source_descript: "Arthur Cayley Davis, “Letter to Secretary of Defense Louis Arthur Johnson”",
    source_citation: "",
};
pub const US_MEMO_1950_07_29_JOHNSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_29_johnson.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0014",
    source_descript: "Louis Arthur Johnson, “Letter to Secretary of State Dean Gooderham Acheson”",
    source_citation: "",
};
pub const US_MEMO_1950_07_29_ACHESON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_29_acheson.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0014",
    source_descript: "Dean Gooderham Acheson, “Letter to Secretary of State Louis Arthur Johnson”",
    source_citation: "",
};
pub const US_MEMO_1950_07_29_STRONG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_07_29_strong.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0014",
    source_descript: "Robert Campbell Strong, “Telegram No. 153, July , 6.p.m.”",
    source_citation: "",
};
pub const US_MEMO_1950_08_01_ALMOND: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_01_almond.pdf"),
    external_source: "https://dl.ndl.go.jp/en/pid/9850613/",
    source_descript: "[Chief of Staff for Personnel, G-1 (Personnel), Far East Command] Edward Mallory Almond, “Command and Staff Visit to Formosa: 31 July–1 August 1950”",
    source_citation: "",
};
pub const US_MEMO_1950_08_02_JOHNSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_02_johnson.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0014",
    source_descript: "Louis Arthur Johnson, “Letter to Secretary of State Dean Gooderham Acheson”",
    source_citation: "",
};
pub const US_MEMO_1950_08_02_LAY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_02_lay.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS128_04_05C0008",
    source_descript: "James Selden Lay II, “Defense of Formosa”",
    source_citation: "",
};
pub const US_MEMO_1950_08_03_ACHESON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_03_acheson.pdf"),
    external_source: "https://catalog.archives.gov/id/183392241/",
    source_descript: "[US Secretary of State] Dean Gooderham Acheson: “Meeting with the President; Item 5; Formosa,” 3 Aug 1950",
    source_citation: "",
};
pub const US_MEMO_1950_08_03_STRONG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_03_strong.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0014",
    source_descript: "Robert Campbell Strong, “Telegram No. 180, August 3, 2.p.m.”",
    source_citation: "",
};
pub const US_MEMO_1950_08_03_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_03_nsc.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS054_03_10C0100",
    source_descript: "National Security Council, “NSC 37/10: Immediate United States Courses of Action with Respect to Formosa”",
    source_citation: "",
};
pub const US_MEMO_1950_08_23_ALLISON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_23_allison.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0015",
    source_descript: "John Moore Allison, “Memorandum of Conversation”",
    source_citation: "",
};
pub const US_MEMO_1950_08_25_AUSTIN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_25_austin.pdf"),
    external_source: "https://catalog.archives.gov/id/406141629/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1950_08_27_ACHESON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_27_acheson.pdf"),
    external_source: "https://catalog.archives.gov/id/183392371/",
    source_descript: "Dean Gooderham Acheson, “Letter to James Edwin Webb”",
    source_citation: "",
};
pub const US_MEMO_1950_08_30_RANNEY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_30_ranney.pdf"),
    external_source: "https://catalog.archives.gov/id/475792834/",
    source_descript: "Frederic Ranney, “Preliminary Conversations for September Foreign Ministers Meeting”",
    source_citation: "",
};
pub const US_MEMO_1950_08_31_STUART: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_08_31_stuart.pdf"),
    external_source: "https://catalog.archives.gov/id/475792823/",
    source_descript: "Wallace Stuart, “Principle Objectives of Discussions with the British and French Respecting Formosa”",
    source_citation: "",
};
pub const US_MEMO_1950_09_01_DOS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_09_01_dos.pdf"),
    external_source: "https://catalog.archives.gov/id/475792825/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1950_09_11_JOHNSON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_09_11_johnson.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0015",
    source_descript: "Louis Arthur Johnson, “Letter to Secretary of State Dean Gooderham Acheson”",
    source_citation: "",
};
pub const US_MEMO_1950_09_14_WEBB: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_09_13_webb.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS129_01_04C0015",
    source_descript: "James Edwin Webb, “Letter to Secretary of Defense Louis Arthur Johnson”",
    source_citation: "",
};
pub const US_MEMO_1950_09_22_CLUBB: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_09_22_clubb.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_24_06C0048_032",
    source_descript: "Oliver Clubb, “Comments on the British Draft Resolution Respecting Formosa for the General Assembly”",
    source_citation: "",
};
pub const US_MEMO_1950_10_23_ALLISON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_10_23_allison.pdf"),
    external_source: "https://catalog.archives.gov/id/183392648/",
    source_descript: "[Deputy Director of Far Eastern Affairs] John Moore Allison: “Memorandum of Conversation: Formosa,” 23 Oct 1950",
    source_citation: "",
};
pub const US_MEMO_1950_11_28_STUART: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_11_28_stuart.pdf"),
    external_source: "https://catalog.archives.gov/id/424040245/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1950_12_12_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_12_12_nsc.pdf"),
    external_source: "https://catalog.archives.gov/id/750722/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1950_12_05_OIR: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_12_05_oir.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/cia-rdp79r01012a000300050003-9",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1950_12_08_MCWILLIAMS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1950_12_08_mcwilliams.pdf"),
    external_source: "https://catalog.archives.gov/id/183392892/",
    source_descript: "[US Director of the Executive Secretariat] William J. McWilliams, “Meeting in the Secretary's Office, December 5, 1950, 10:00 a.m.”",
    source_citation: "",
};
pub const US_MEMO_1951_01_02_OMAR: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_01_02_omar.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_48_00C0001_091",
    source_descript: "Omar Nelson Bradley, “Strategic Importance of Formosa”",
    source_citation: "",
};
pub const US_MEMO_1951_05_03_NSC: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_05_03_nsc.pdf"),
    external_source: "https://catalog.archives.gov/id/352800762/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1951_05_22_KENNEDY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_05_22_kennedy.pdf"),
    external_source: "https://www.jfklibrary.org/asset-viewer/archives/dfppp-021-024",
    source_descript: "[Congressman] John Fitzgerald Kennedy: Unaddressed Letter, 22 May 1951",
    source_citation: "",
};
pub const US_MEMO_1951_06_26_BRADLEY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_06_26_bradley.pdf"),
    external_source: "https://catalog.archives.gov/id/269703344/",
    source_descript: "[Chairman of the Joint Chiefs of Staff] Omar Nelson Bradley: “Memorandum for the Secretary of Defense: Japanese Peace Treaty”, 26 Jun 1951",
    source_citation: "",
};
pub const US_MEMO_1951_06_27_MARSHALL: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_06_27_marshall.pdf"),
    external_source: "https://catalog.archives.gov/id/269703344/",
    source_descript: "[Secretary of Defense] George Catlett Marshall: Letter to [Secretary of State] Dean Gooderham Acheson, 27 Jun 1951",
    source_citation: "",
};
pub const US_MEMO_1951_06_28_ACHESON_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_06_28_acheson_1.pdf"),
    external_source: "https://archive.org/details/dos-report_s-11949-52-supp/",
    source_descript: "[Secretary of State] Dean Gooderham Acheson: Letter to [Secretary of Defense] George Catlett Marshall, 28 Jun 1951",
    source_citation: "",
};
pub const US_MEMO_1951_06_28_ACHESON_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_06_28_acheson_2.pdf"),
    external_source: "https://catalog.archives.gov/id/269703344/",
    source_descript: "[Secretary of State] Dean Gooderham Acheson: Letter to [President] Harry S Truman, 28 Jun 1951",
    source_citation: "",
};
pub const US_MEMO_1951_09_14_TRUMAN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_09_14_truman.pdf"),
    external_source: "https://catalog.archives.gov/id/269702117/",
    source_descript: "(President, US) Harry S Truman, Memorandum for (Secretary of State, US) David Dean Rusk, September 14, 1951",
    source_citation: "",
};
pub const US_MEMO_1951_11_12_RINGWALT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_11_12_ringwalt.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0086",
    source_descript: "Arthur Rumney Ringwalt, “Letter to Robert Heatlie Scott”",
    source_citation: "",
};
pub const US_MEMO_1951_11_22_ACHESON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1951_11_22_acheson.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_02C0087",
    source_descript: "[Secretary of State] Dean Gooderham Acheson: Letter to Robert Anthony Eden”",
    source_citation: "",
};
pub const US_MEMO_1952_01_05_ACHESON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1952_01_05_acheson.pdf"),
    external_source: "https://catalog.archives.gov/id/193224415/",
    source_descript: "[Secretary of State] Dean Gooderham Acheson: “Memorandum of Dinner Meeting on S.S. Williamsburg, January 5, 1952 Between President Truman and Prime Minister Churchill”",
    source_citation: "",
};
pub const US_MEMO_1952_01_09_DULLES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1952_01_09_dulles.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0016",
    source_descript: "John Foster Dulles, “Japan and China”",
    source_citation: "",
};
pub const US_MEMO_1952_01_17_DULLES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1952_01_17_dulles.pdf"),
    external_source: "https://archive.history.go.kr/id/AGB002_03_03C0017",
    source_descript: "John Foster Dulles, “Letter to Winston Churchill”",
    source_citation: "",
};
pub const US_MEMO_1952_03_22_FOSTER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1952_03_22_foster.pdf"),
    external_source: "https://catalog.archives.gov/id/348696226",
    source_descript: "(Deputy Secretary of Defense) William Chapman Foster, “Memorandum for the Executive Secretary National Security Council,” March 22, 1952",
    source_citation: "",
};
pub const US_MEMO_1952_10_21_MCCONAUGHY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1952_10_21_mcconaughy.pdf"),
    external_source: "https://catalog.archives.gov/id/193225804",
    source_descript: "(Director of Office of Chinese Affairs) Walter Patrick McConaughy II, “Call of Chinese Foreign Minister George Yeh on the Secretary, at the United States Mission to the United Nations, New York October 20, 1952, at 4:30 P.M.,” October 21, 1952",
    source_citation: "",
};
pub const US_MEMO_1953_02_06_DAVIS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1953_02_06_davis.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_80_00C0011",
    source_descript: "Richard Davis, “”",
    source_citation: "",
};
pub const US_MEMO_1954_05_22_CUTLER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_05_22_cutler.pdf"),
    external_source: "https://archive.org/details/dos-report_s-11952-54-supp/",
    source_descript: "[Special Assistant to the President for National Security Affairs] Robert Cutler: “Conference with the President,” 22 May 1954",
    source_citation: "",
};
pub const US_MEMO_1954_09_21_DRUMRIGHT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_09_21_drumright.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_90_00C0036",
    source_descript: "Everett Francis Drumright, “Transmittal of Record of the Conversation Which Took Place at Taipei between Generalissimo Chiang Kai-shek and Secretary of State Dulles”",
    source_citation: "",
};
pub const US_MEMO_1954_10_08_BROWN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_10_08_brown.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_90_00C0037",
    source_descript: "Ben Hill Brown II, “Letter to Leverett Atholville Saltonstall”",
    source_citation: "",
};
pub const US_MEMO_1954_10_18_DULLES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_10_18_dulles.pdf"),
    external_source: "https://archive.org/details/dos-report_s-11952-54-supp/",
    source_descript: "[Secretary of State] John Foster Dulles: “Memorandum of Conference with the President, the White House, 8:00 to 9:00 a.m.,” 18 Oct 1954",
    source_citation: "",
};
pub const US_MEMO_1954_10_25_WYMAN: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_10_25_wyman.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_100_00C0029",
    source_descript: "Willard Gordon Wyman, “Letter to Arthur Gilbert Trudeau”",
    source_citation: "",
};
pub const US_MEMO_1954_11_23_DULLES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_11_23_dulles.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS071_04_00C0015",
    source_descript: "John Foster Dulles, “Letter to Dwight David Eisenhower”",
    source_citation: "",
};
pub const US_MEMO_1954_12_13_MCCONAUGHY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_12_13_mcconaughy.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_90_00C0037",
    source_descript: "Walter Patrick McConaughy II, “Ratification of Mutual Defense Treaty”",
    source_citation: "",
};
pub const US_MEMO_1954_12_22_DULLES_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_12_22_dulles_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_90_00C0037",
    source_descript: "John Foster Dulles, “Letter to President Dwight David Eisenhower”",
    source_citation: "",
};
pub const US_MEMO_1954_12_22_DULLES_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_12_22_dulles_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_90_00C0037",
    source_descript: "John Foster Dulles, “Letter to Secretary of Defense Charles Erwin Wilson”",
    source_citation: "",
};
pub const US_MEMO_1954_12_22_EISENHOWER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1954_12_22_eisenhower.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_90_00C0037",
    source_descript: "Dwight David Eisenhower, “Letter to the Senate”",
    source_citation: "",
};
pub const US_MEMO_1955_02_01_EISENHOWER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1955_02_01_eisenhower.pdf"),
    external_source: "https://catalog.archives.gov/id/186536",
    source_descript: "Dwight David Eisenhower, Letter to (Supreme Allied Commander Europe) Alfred Maximilian Gruenther, February 1, 1955",
    source_citation: "",
};
pub const US_MEMO_1955_02_16_ARMSTRONG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1955_02_16_armstrong.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_100_00C0029",
    source_descript: "William Park Armstrong III, “Chinese Communist Views on the Status of Formosa”",
    source_citation: "",
};
pub const US_MEMO_1955_02_21_MCAULIFFE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1955_02_21_mcauliffe.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_100_00C0029",
    source_descript: "Eugene Vincent McAuliffe, “Excerpts from the February 21 ‘Summary of Decisions’ for the information of Mr. Armstrong”",
    source_citation: "",
};
pub const US_MEMO_1955_03_04_ARMSTRONG_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1955_03_04_armstrong_1.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_100_00C0029",
    source_descript: "William Park Armstrong III, “Procedure to Adopt the terms Taiwan and Penghu for Formosa and Pescadores”",
    source_citation: "",
};
pub const US_MEMO_1955_03_04_ARMSTRONG_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1955_03_04_armstrong_2.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_100_00C0029",
    source_descript: "William Park Armstrong III, “Formosa: The Origin of the Name”",
    source_citation: "",
};
pub const US_MEMO_1955_03_09_SHELBY: File =  File {
    internal_source: asset!("assets/docs/us/memo_1955_03_09_shelby.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_100_00C0029",
    source_descript: "Richard Roy Selby II, “Excerpts from the March 9 ‘Summary of Decisions’ for the information of Mr. Armstrong”",
    source_citation: "",
};
pub const US_MEMO_1955_04_05_EISENHOWER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1955_04_05_eisenhower.pdf"),
    external_source: "https://catalog.archives.gov/id/186500",
    source_descript: "Dwight David Eisenhower, “Memorandum for the Secretary of State,” April 5, 1955",
    source_citation: "",
};
pub const US_MEMO_1955_04_23_DULLES: File =  File {
    internal_source: asset!("assets/docs/us/memo_1955_04_23_dulles.pdf"),
    external_source: "https://archive.history.go.kr/id/AUS002_92_00C0002",
    source_descript: "John Foster Dulles, “Enlisting Support for US Policies Toward Taiwan”",
    source_citation: "",
};
pub const US_MEMO_1971_05_10_KISSINGER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1971_05_10_kissinger.pdf"),
    external_source: "https://catalog.archives.gov/id/559236017",
    source_descript: "[US Assistant to the President for National Security Affairs] Henry Alfred Kissinger: Letter from [President of the US] Richard Milhous Nixon to [Premier of the PRC] Zhou Enlai, 10 May 1971",
    source_citation: "",
};
pub const US_MEMO_1971_07_14_KISSINGER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1971_07_14_kissinger.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/loc-hak-467-5-1-7",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1971_10_21_KISSINGER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1971_10_21_kissinger.pdf"),
    external_source: "https://archive.org/details/detente-or-razryadka-the-kissinger-dobrynin-telephone-transcrip/72915/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1971_10_23_KISSINGER: File =  File {
    internal_source: asset!("assets/docs/us/memo_1971_10_23_kissinger.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d47",
    // https://archive.org/details/detente-or-razryadka-the-kissinger-dobrynin-telephone-transcrip/72927/
    // https://2001-2009.state.gov/r/pa/ho/frus/nixon/e13/c18861.htm
    // https://2001-2009.state.gov/documents/organization/72927.pdf
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1971_11_11_KISSINGER_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1971_11_11_kissinger_1.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/loc-hak-467-5-1-7",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1971_11_11_KISSINGER_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1971_11_11_kissinger_2.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/loc-hak-467-5-1-7",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_01_03_HAIG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_01_03_haig.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/loc-hak-467-5-1-7",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_01_06_HAIG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_01_06_haig.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/loc-hak-467-5-1-7",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_01_07_HAIG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_01_06_haig.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/loc-hak-467-5-1-7",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_21_LORD_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_21_lord_1.pdf"),
    external_source: "https://catalog.archives.gov/id/574801982",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_21_LORD_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_21_lord_2.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d89",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_21_LORD_3: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_21_lord_3.pdf"),
    external_source: "https://catalog.archives.gov/id/574801982",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_22_LORD_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_22_lord_1.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d90",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_22_LORD_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_22_lord_2.pdf"),
    external_source: "https://catalog.archives.gov/id/574801982",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_22_PLATT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_22_platt.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d91",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_23_LORD_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_23_lord_1.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d92",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_23_LORD_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_23_lord_2.pdf"),
    external_source: "https://catalog.archives.gov/id/574802050",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_24_LORD_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_24_lord_1.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d93",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_24_LORD_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_24_lord_2.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d95",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_24_LORD_3: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_24_lord_3.pdf"),
    external_source: "https://catalog.archives.gov/id/574802050",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_24_MEHLERT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_24_mehlert.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d94",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_25_LORD_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_25_lord_1.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d96",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_25_LORD_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_25_lord_2.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d97",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_25_LORD_3: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_25_lord_3.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d98",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_25_LORD_4: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_25_lord_4.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d99",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_25_LORD_5: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_25_lord_5.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d100",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_25_LORD_6: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_25_lord_6.pdf"),
    external_source: "https://catalog.archives.gov/id/574802129",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_25_LORD_7: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_25_lord_7.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d102",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_25_PLATT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_25_platt.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d101",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_26_LORD_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_26_lord_1.pdf"),
    external_source: "https://catalog.archives.gov/id/574802129",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_26_LORD_2: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_26_lord_2.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d103",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_26_LORD_3: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_26_lord_3.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d104",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_27_LORD_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_27_lord_1.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d106",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_28_PLATT: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_28_platt.pdf"),
    external_source: "https://history.state.gov/historicaldocuments/frus1969-76ve13/d107",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_02_28_LORD_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_02_28_lord_1.pdf"),
    external_source: "https://catalog.archives.gov/id/574802129",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_03_08_HALDEMAN_1: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_03_08_haldeman_1.pdf"),
    external_source: "https://catalog.archives.gov/id/574802129",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_03_10_CIA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_03_10_cia.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/cia-rdp85t00875r001100140006-5",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1975_10_21_BUSH: File =  File {
    internal_source: asset!("assets/docs/us/memo_1975_10_21_bush.pdf"),
    external_source: "https://catalog.archives.gov/id/446394273/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1981_03_27_OPA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1981_03_27_opa.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/cia-rdp84b00049r000701970010-2",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1982_07_08_GREGG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1982_07_08_gregg.pdf"),
    external_source: "https://www.reaganlibrary.gov/sites/default/files/2025-04/40-221-R10-095-2025.pdf",
    source_descript: "Don Gregg, “Memorandum for William P. Clark; Re: Taiwan Arms Sales”",
    source_citation: "",
};
pub const US_MEMO_1988_07_23_HELMS: File =  File {
    internal_source: asset!("assets/docs/us/memo_1988_07_23_helms.pdf"),
    external_source: "https://catalog.archives.gov/id/209204872/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1988_09_16_CLINTON: File =  File {
    internal_source: asset!("assets/docs/us/memo_1988_09_16_clinton.pdf"),
    external_source: "https://catalog.archives.gov/id/209204872/",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1970_01_24_KREISBERG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1970_01_24_kreisberg.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/loc-hak-467-6-1-6",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1970_02_21_KREISBERG: File =  File {
    internal_source: asset!("assets/docs/us/memo_1970_02_21_kreisberg.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/loc-hak-467-6-1-6",
    source_descript: "",
    source_citation: "",
};
pub const US_MEMO_1972_05_26_CIA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_05_26_cia.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/cia-rdp79t00975a022000020001-4",
    source_descript: "Central Intelligence Agency, “Central Intelligence Bulletin,” May 26, 1972",
    source_citation: "",
};
pub const US_MEMO_1972_09_25_CIA: File =  File {
    internal_source: asset!("assets/docs/us/memo_1972_09_25_cia.pdf"),
    external_source: "https://www.cia.gov/readingroom/document/cia-rdp79t00975a022800090002-8",
    source_descript: "Central Intelligence Agency, “Central Intelligence Bulletin,” September 25, 1972",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1864_1864_P03: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1864_1864_p03.pdf"),
    source_descript: "Foreign Relations of the United States: 1864, Part 3",
    external_source: "https://www.google.com/books/edition/us/NOrH_zS_gE4C/",
    source_citation: "Foreign Relations of the United States. 1864. pt. 3.                                    \
    United States Government Printing Office, 1865. Accessed September 19, 2024.",
};
pub const US_SERIAL_FRUS_1865_1865_P02: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1864_1864_p03.pdf"),
    source_descript: "Foreign Relations of the United States: 1865, Part 2",
    external_source: "https://www.google.com/books/edition/us/OqE6AAAAcAAJ/",
    source_citation: "Foreign Relations of the United States. 1865. pt. 2.                                    \
    United States Government Printing Office, 1866. Accessed February 8, 2025.",
};
pub const US_SERIAL_FRUS_1867_1867_P01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1867_1867_p01.pdf"),
    source_descript: "Foreign Relations of the United States: 1867, Part 1",
    external_source: "https://www.google.com/books/edition/us/GAcyAQAAMAAJ/",
    source_citation: "Foreign Relations of the United States. 1867. pt. 1.                                    \
    United States Government Printing Office, 1868. Accessed February 8, 2025.",
};
pub const US_SERIAL_FRUS_1868_1868_P01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1868_1868_p01.pdf"),
    source_descript: "Foreign Relations of the United States: 1868, Part 1",
    external_source: "https://www.google.com/books/edition/us/FXB1JIUtGzoC/",
    source_citation: "Foreign Relations of the United States. 1868. pt. 1.                                    \
    United States Government Printing Office, 1964. Accessed February 8, 2025.",
};
pub const US_SERIAL_FRUS_1873_1873_P01_V01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1873_1873_p01_v01.pdf"),
    source_descript: "Foreign Relations of the United States: 1873, Part 1, Volume 1",
    external_source: "https://www.google.com/books/edition/us/6i8R9gB9NWAC/",
    source_citation: "Foreign Relations of the United States. 1873. P.1, Vol.1.                             \
    United States Government Printing Office, 1873. Accessed September 19, 2024.",
};
pub const US_SERIAL_FRUS_1874_1874: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1874_1874.pdf"),
    source_descript: "Foreign Relations of the United States: 1874",
    external_source: "https://www.google.com/books/edition/us/vBSJ8X0mDpcC/",
    source_citation: "Foreign Relations of the United States. 1874.                                         \
    United States Government Printing Office, 1874. Accessed September 19, 2024.",
};
pub const US_SERIAL_FRUS_1875_1875_V01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1875_1875_v01.pdf"),
    source_descript: "Foreign Relations of the United States: 1875",
    external_source: "https://www.google.com/books/edition/us/GPjFoWJHJe0C/",
    source_citation: "Foreign Relations of the United States. 1875. vol. 1                                    \
    United States Government Printing Office, 1874. Accessed September 19, 2024.",
};
pub const US_SERIAL_FRUS_1875_1875_V02: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1875_1875_v02.pdf"),
    source_descript: "Foreign Relations of the United States: 1875",
    external_source: "https://www.google.com/books/edition/us/01kLAAAAYAAJ/",
    source_citation: "Foreign Relations of the United States. 1875. vol. 2                                    \
    United States Government Printing Office, 1874. Accessed September 19, 2024.",
};
pub const US_SERIAL_FRUS_1895_1895_P01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1895_1895_p01.pdf"),
    external_source: "https://www.google.com/books/edition/us/eGpIAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1942_1942_V01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1942_1942_v01.pdf"),
    external_source: "https://www.google.com/books/edition/us/9iI5WSCjLmMC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1943_1943_SUP: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1943_1943_supplemental.pdf"),
    source_descript: "Foreign Relations of the United States: 1943, The Conferences at Cairo and Tehran",
    external_source: "https://www.google.com/books/edition/us/1cxmAAAAMAAJ/",
    source_citation: "Foreign Relations of the United States. 1961.                                         \
    United States Government Printing Office, 1874. Accessed September 19, 2024.",
};
pub const US_SERIAL_FRUS_1945_1950: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1945_1950.pdf"),
    external_source: "https://www.google.com/books/edition/us/hIi7ulOnU1QC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1946_1946_V08: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1946_1946_v08.pdf"),
    external_source: "https://www.google.com/books/edition/us/rSS0a-dKAfQC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1945_1945_SUP_BERLIN_V01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1945_1945_sup_berlin_v01.pdf"),
    external_source: "https://www.google.com/books/edition/us/3fKlgfsRZW8C/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1947_1947_V06: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1947_1947_v06.pdf"),
    external_source: "https://www.google.com/books/edition/us/bcd8fb3xa0oC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1948_1948_V07: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1948_1948_v07.pdf"),
    external_source: "https://www.google.com/books/edition/us/Ls7yqBrfKscC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1948_1948_V08: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1948_1948_v08.pdf"),
    external_source: "https://www.google.com/books/edition/us/J6nRF4KT3vIC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1949_1949_V07_P02: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1949_1949_v07_p02.pdf"),
    external_source: "https://www.google.com/books/edition/us/zWERV8k8JfsC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1949_1949_V08: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1949_1949_v08.pdf"),
    external_source: "https://www.google.com/books/edition/us/OzN5PMxPrp4C/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1949_1949_V09: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1949_1949_v09.pdf"),
    external_source: "https://www.google.com/books/edition/us/4WNNAQAAMAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1950_1950_V06: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1950_1950_v06.pdf"),
    external_source: "https://www.google.com/books/edition/us/DeUtAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1952_1954_V01_P01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1952_1954_v01_p01.pdf"),
    external_source: "https://www.google.com/books/edition/us/q9ktAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1952_1954_V02_P02: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1952_1954_v02_p02.pdf"),
    external_source: "https://www.google.com/books/edition/us/8NotAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1952_1954_V03: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1952_1954_v03.pdf"),
    external_source: "https://www.google.com/books/edition/us/ftstAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1952_1954_V05: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1952_1954_v05.pdf"),
    external_source: "https://www.google.com/books/edition/us/6L4DRtAEZEEC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1952_1954_V06_P01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1952_1954_v06_p01.pdf"),
    external_source: "https://www.google.com/books/edition/us/NZJcHkEpBSYC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1952_1954_V12_P01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1952_1954_v12_p01.pdf"),
    external_source: "https://www.google.com/books/edition/us/82ZHAQAAMAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1952_1954_V14_P01: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1952_1954_v14_p01.pdf"),
    external_source: "https://www.google.com/books/edition/us/3nsCuGKAC6wC/",
    source_descript: "",
    source_citation: "",
};
pub const US_SERIAL_FRUS_1952_1954_V14_P02: File = File {
    internal_source: asset!("assets/docs/us/serial_frus_1952_1954_v14_p02.pdf"),
    external_source: "https://www.google.com/books/edition/us/QdwtAAAAYAAJ/",
    source_descript: "",
    source_citation: "",
};
pub const US_RELATIONS_WITH_CHINA: File = File {
    internal_source: asset!("assets/docs/us/book_relations_with_china.pdf"),
    source_descript: "United States Relations with China",
    external_source: "https://www.google.com/books/edition/us/0a3PAAAAIAAJ/",
    source_citation: "",
};
pub const SERIAL_PPOTPUS_1944: File =  File {
    internal_source: asset!("assets/docs/us/serial_ppotpus_1944.pdf"),
    external_source: "https://archive.org/details/publicpapersaddr0000samu/",
    source_descript: "Samuel Irving Rosenman: “Public Papers of the President of the United States: Franklin D. Roosevelt; 1944-1945”",
    source_citation: "",
};
pub const SERIAL_PPOTPUS_1945: File =  File {
    internal_source: asset!("assets/docs/us/serial_ppotpus_1945.pdf"),
    external_source: "https://www.google.com/books/edition/us/SODh0AaW18UC/",
    source_descript: "National Archives and Records Service, “Public Papers of the President of the United States: Harry S. Truman; 1945”",
    source_citation: "",
};
pub const SERIAL_PPOTPUS_1949: File =  File {
    internal_source: asset!("assets/docs/us/serial_ppotpus_1949.pdf"),
    external_source: "https://www.google.com/books/edition/us/o0bQQxYZoSsC/",
    source_descript: "National Archives and Records Service, “Public Papers of the President of the United States: Harry S. Truman; 1949”",
    source_citation: "",
};
pub const SERIAL_PPOTPUS_1953: File =  File {
    internal_source: asset!("assets/docs/us/serial_ppotpus_1953.pdf"),
    external_source: "https://www.google.com/books/edition/us/7gnOZHeItO0C/",
    source_descript: "National Archives and Records Service, “Public Papers of the President of the United States: Dwight D. Eisenhower; 1953”",
    source_citation: "",
};
pub const SERIAL_PPOTPUS_1955: File =  File {
    internal_source: asset!("assets/docs/us/serial_ppotpus_1955.pdf"),
    external_source: "https://www.google.com/books/edition/us/faWTh31pRsoC-pUC/",
    source_descript: "National Archives and Records Service, “Public Papers of the President of the United States: Dwight D. Eisenhower; 1955”",
    source_citation: "",
};
pub const SERIAL_PPOTPUS_1958: File =  File {
    internal_source: asset!("assets/docs/us/serial_ppotpus_1958.pdf"),
    external_source: "https://www.google.com/books/edition/us/OGoTTqSXe2oC/",
    source_descript: "National Archives and Records Service, “Public Papers of the President of the United States: Dwight D. Eisenhower; 1958”",
    source_citation: "",
};
pub const SERIAL_PPOTPUS_1998_V01: File =  File {
    internal_source: asset!("assets/docs/us/serial_ppotpus_1998_v01.pdf"),
    external_source: "https://www.google.com/books/edition/us/L59HuLVT-pUC/",
    source_descript: "National Archives and Records Administration, “Public Papers of the President of the United States: William J. Clinton; 1998; Book 1”",
    source_citation: "",
};
pub const BOOK_USSL_FCI_1952_09_26: File =  File {
    internal_source: asset!("assets/docs/us/book_ussl_fci_1952_09_26.pdf"),
    external_source: "https://www.google.com/books/edition/us/ZmWQ9dZvDfMC/",
    source_descript: "United States Senate Library, “Factual Campaign Information,” September 26, 1952",
    source_citation: "",
};
pub const SERIAL_LIFE_V23_N15: File =  File {
    internal_source: asset!("assets/docs/us/serial_life_v23_n15.pdf"),
    external_source: "https://archive.org/details/Life-1947-10-13-Vol-23-No-15/",
    source_descript: "William C. Bullitt, “A Report to the American People on China",
    source_citation: "",
};
pub const SERIAL_LIFE_V26_N08: File =  File {
    internal_source: asset!("assets/docs/us/serial_life_v26_n08.pdf"),
    external_source: "https://archive.org/details/Life-1949-02-21-Vol-26-No-8/",
    source_descript: "Life, “ECA'S Lapham Pays A Call on Formosa's Tribal Princesses",
    source_citation: "",
};
pub const US_CRUS_1887_1887: File = File {
    internal_source: asset!("assets/docs/us/crus_1887_1887.pdf"),
    source_descript: "Commercial Relations of the United States: January–March, 1887",
    external_source: "https://www.google.com/books/edition/us/FLhBAQAAMAAJ/",
    source_citation: "Foreign Relations of the United States. 1887.                                         \
    United States Government Printing Office, 1874. Accessed September 19, 2024.",
};
pub const US_C033_S02_REPORT_N0097: File =  File {
    internal_source: asset!("assets/docs/us/c033_s02_report_n0097.pdf"),
    external_source: "https://www.loc.gov/item/2024763533/",
    source_descript: "Remarks of Commodore Perry upon the Expediency of Extending Further Encouragement to American Commerce in the East",
    source_citation: "US Congress, <mark>32nd Cong., 2nd Sess., Rep. No. 97</mark> (Washington: A. O. P. Nicholson, 1856), vol. 2 p. 173-182",
};
pub const US_C035_S02_REPORT_N0022: File = File {
    internal_source: asset!("assets/docs/us/c035_s02_report_n0022.pdf"),
    external_source: "https://www.google.com/books/edition/us/S15KAAAAYAAJ/",
    source_descript: "35th Congress, 2nd Session, Report No. 22",
    source_citation: "",
};
pub const US_C035_S02_REPORT_N0068: File = File {
    internal_source: asset!("assets/docs/us/c035_s02_report_n0068.pdf"),
    external_source: "https://www.loc.gov/item/2022699338/",
    source_descript: "35th Congress, 2nd Session, Report No. 68",
    source_citation: "",
};
pub const US_C040_S02_REPORT_N0052: File =  File {
    internal_source: asset!("assets/docs/us/c040_s02_report_n0052.pdf"),
    external_source: "https://www.govinfo.gov/app/details/SERIALSET-01317_00_00-013-0052-0000",
    source_descript: "40th Congress, 2nd Session, Report No. 52, “Correspondence upon the Subject of the Murder, by the Inhabitants of the Island of Formosa, of the Ship's Company of the American Bark Rover”",
    source_citation: "US Congress, <mark>40th Cong., 2nd Sess., Rep. No. 52</mark> (Washington: Government Printing Office, 1868)",
};
pub const US_C080_S02_HEARING_1948_05_11_H_APPROPRIATIONS: File = File {
    internal_source: asset!("assets/docs/us/c080_s02_hearing_1948_05_11_h_appropriations.pdf"),
    external_source: "https://www.google.com/books/edition/us/y907AQAAMAAJ/",
    source_descript: "House Appropriations Committee: “Hearings Before the Subcommittee of the Committee on Appropriations on the Foreign Aid Appropriation Bill for 1949”",
    source_citation: "",
};
pub const US_C081_S02_RECORD_1950_01_09: File = File {
    internal_source: asset!("assets/docs/us/c081_s02_record_1950_01_09.pdf"),
    external_source: "https://www.congress.gov/bound-congressional-record/1950/01/09/",
    source_descript: "81st Congress, 2nd Session, Congressional Record for January 09, 1950",
    source_citation: "",
};
pub const US_C081_S02_RECORD_1950_01_11: File = File {
    internal_source: asset!("assets/docs/us/c081_s02_record_1950_01_11.pdf"),
    external_source: "https://www.congress.gov/bound-congressional-record/1950/01/11/",
    source_descript: "81st Congress, 2nd Session, Congressional Record for January 11, 1950",
    source_citation: "",
};
pub const US_C081_S02_RECORD_1950_01_17: File = File {
    internal_source: asset!("assets/docs/us/c081_s02_record_1950_01_17.pdf"),
    external_source: "https://www.congress.gov/bound-congressional-record/1950/01/17/",
    source_descript: "81st Congress, 2nd Session, Congressional Record for January 17, 1950",
    source_citation: "",
};
pub const US_C082_S01_REPORT_N0091: File = File {
    internal_source: asset!("assets/docs/us/c082_s01_report_n0091.pdf"),
    source_descript: "82nd Congress, 1st Session, Report No. 91",
    external_source: "https://www.loc.gov/item/2022700776/",
    source_citation: "",
};
pub const US_C082_S01_RECORD_1951_08_24: File = File {
    internal_source: asset!("assets/docs/us/c082_s01_record_1951_08_24.pdf"),
    external_source: "https://www.congress.gov/bound-congressional-record/1951/08/24/",
    source_descript: "82nd Congress, 1st Session, Congressional Record for August 24, 1951",
    source_citation: "",
};
pub const US_C082_S02_RECORD_1952_01_16: File = File {
    internal_source: asset!("assets/docs/us/c082_s02_record_1952_01_16.pdf"),
    external_source: "https://www.congress.gov/bound-congressional-record/1952/01/16/",
    source_descript: "82nd Congress, 2nd Session, Congressional Record for January 16, 1952",
    source_citation: "",
};
pub const US_C082_S02_RECORD_1952_01_17: File = File {
    internal_source: asset!("assets/docs/us/c082_s02_record_1952_01_17.pdf"),
    external_source: "https://www.congress.gov/bound-congressional-record/1952/01/17/",
    source_descript: "82nd Congress, 2nd Session, Congressional Record for January 17, 1952",
    source_citation: "",
};
pub const US_C083_S01_REPORT_N0093: File = File {
    internal_source: asset!("assets/docs/us/c083_s01_report_n0093.pdf"),
    source_descript: "83rd Congress, 1st Session, Document No. 93",
    external_source: "https://www.google.com/books/edition/us/xMsjAQAAMAAJ/",
    source_citation: "",
};
pub const US_C084_S01_HEARING_N0752: File =  File {
    internal_source: asset!("assets/docs/us/c084_s01_hearing_n0752.pdf"),
    external_source: "https://www.govinfo.gov/app/details/CPRT-84SPRT84752vVII/CPRT-84SPRT84752vVII/",
    source_descript: "",
    source_citation: "",
};


pub const GAZETTE_1951_08_15_PEOPLES_CHINA: File = File {
    internal_source: asset!("assets/docs/cn/gazette_1951_08_15_peoples_china.pdf"),
    external_source: "https://www.marxists.org/subject/china/peoples-china/1951/PC1951-V4-05-Sup1-OCR-sm.pdf",
    source_descript: "(Minister of Foreign Affairs, PRC) Zhou Enlai, “Statement on the US–British Draft Peace Treaty with Japan and the San Francisco Conference,” August 15, 1951",
    source_citation: "",
};
pub const CN_THDS_V01_P002: File = File {
    internal_source: asset!("assets/docs/cn/thds_v01_p002.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 1, Part 2",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10059869_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC1%E8%BC%AF_2_%E5%8F%B0%E7%81%A3%E5%BA%9C%E5%BF%97-%E5%8F%B0%E7%81%A3%E5%BA%9C%E8%B3%A6%E5%BD%B9%E5%86%8A_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Selected Memorials and Policy Discussions from Four Reigns: Daoguang, Xianfeng, Tongzhi, and Guangxu</i> Taiwan Historical Documents Series. Vol.4, No. 70. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V04_P070: File = File {
    internal_source: asset!("assets/docs/cn/thds_v04_p070.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 4, Part 70",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10059950_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC4%E8%BC%AF_70_%E9%81%93%E5%92%B8%E5%90%8C%E5%85%89%E5%9B%9B%E6%9C%9D%E5%A5%8F%E8%AD%B0%E9%81%B8%E8%BC%AF_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Selected Memorials and Policy Discussions from Four Reigns: Daoguang, Xianfeng, Tongzhi, and Guangxu</i> Taiwan Historical Documents Series. Vol.4, No. 70. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P116: File = File {
    internal_source: asset!("assets/docs/cn/thds_v06_p116.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 6, Part 116",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10059993_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC6%E8%BC%AF_116_%E9%84%AD%E6%B0%8F%E9%97%9C%E4%BF%82%E6%96%87%E6%9B%B8_%E6%B8%85%E4%BB%A3%E5%AE%98%E6%9B%B8%E8%A8%98%E6%98%8E%E5%8F%B0%E7%81%A3%E9%84%AD%E6%B0%8F%E4%BA%A1%E4%BA%8B_%E9%9D%96%E6%B5%B7%E5%BF%97_%E9%9D%96%E6%B5%B7%E7%B4%80%E7%95%A5_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P117: File = File {
    internal_source: asset!("assets/docs/cn/thds_v06_p117.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 6, Part 117",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-12843514_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E6%B5%B7%E7%B4%80%E8%BC%AF%E8%A6%81%E3%80%81%E9%96%A9%E6%B5%B7%E7%B4%80%E7%95%A5%E3%80%81%E6%B5%B7%E4%B8%8A%E8%A6%8B%E8%81%9E%E9%8C%84%E3%80%81%E9%96%A9%E6%B5%B7%E7%B4%80%E8%A6%81_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P118: File = File {
    internal_source: asset!("assets/docs/cn/thds_v06_p118.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 6, Part 118",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10059995_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC6%E8%BC%AF_118_%E5%8F%B0%E7%81%A3%E5%A4%96%E8%A8%98_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P120: File = File {
    internal_source: asset!("assets/docs/cn/thds_v06_p120.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 6, Part 120",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-12843511_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E5%BE%9E%E5%BE%B5%E5%AF%A6%E9%8C%84%E3%80%81%E9%9D%96%E6%B5%B7%E7%B4%80%E4%BA%8B_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P121: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p121.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 121",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10059998_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_121_%E6%9D%B1%E7%80%9B%E8%AD%98%E7%95%A5-%E6%9D%B1%E7%80%9B%E7%B4%80%E4%BA%8B-%E5%8F%B0%E7%81%A3%E7%B4%80%E4%BA%8B-%E5%8F%B0%E6%B5%B7%E8%A6%8B%E8%81%9E%E9%8C%84_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P122: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p122.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 122",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10059999_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_122_%E6%B5%B7%E6%9D%B1%E6%9C%AD%E8%A8%98-%E5%8F%B0%E9%99%BD%E8%A6%8B%E8%81%9E%E9%8C%84_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P123: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p123.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 123",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060000_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_123_%E6%B5%B7%E6%BF%B1%E5%A4%A7%E4%BA%8B%E8%A8%98-%E8%A3%A8%E6%B5%B7%E7%B4%80%E6%B8%B8-%E6%B5%B7%E5%9C%8B%E8%81%9E%E8%A6%8B%E9%8C%84_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P124: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p124.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 124",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-13283164_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_124_%E6%AC%BD%E5%AE%9A%E5%B9%B3%E5%AE%9A%E5%8F%B0%E7%81%A3%E7%B4%80%E7%95%A5_%E4%B8%8A.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P125: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p125.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 125",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-13283165_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_125_%E6%AC%BD%E5%AE%9A%E5%B9%B3%E5%AE%9A%E5%8F%B0%E7%81%A3%E7%B4%80%E7%95%A5_%E4%B8%8B.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P126: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p126.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 126",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060002_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_126_%E5%B9%B3%E5%8F%B0%E7%B4%80%E4%BA%8B%E6%9C%AC%E6%9C%AB-%E6%88%B4%E6%A1%88%E7%B4%80%E7%95%A5-%E5%B9%B3%E5%8F%B0%E7%B4%80%E7%95%A5-%E6%88%B4%E6%96%BD%E9%9B%A8%E6%A1%88%E7%B4%80%E7%95%A5-%E6%9D%B1%E5%BE%B5%E9%9B%86_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P127: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p127.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 127",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060003_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_127_%E5%8A%89%E9%8A%98%E5%82%B3%E6%92%AB%E5%8F%B0%E5%89%8D%E5%BE%8C%E6%AA%94%E6%A1%88_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P130: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p130.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 130",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060006_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_130_%E6%B8%85%E6%9C%9D%E6%9F%94%E9%81%A0%E8%A8%98%E9%81%B8%E9%8C%84-%E5%8F%B0%E7%81%A3%E5%B0%8D%E5%A4%96%E9%97%9C%E4%BF%82%E5%8F%B2%E6%96%99-%E6%B8%85%E5%AD%A3%E5%8F%B0%E7%81%A3%E6%B4%8B%E5%8B%99%E5%8F%B2%E6%96%99_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P131: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p131.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 131",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060007_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_131_%E7%94%B2%E6%88%8C%E5%85%AC%E7%89%98%E9%88%94%E5%AD%98-%E5%90%8C%E6%B2%BB%E7%94%B2%E6%88%8C%E6%97%A5%E5%85%B5%E4%BE%B5%E5%8F%B0%E5%A7%8B%E6%9C%AB_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P132: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p132.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 132",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060008_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_132_%E9%AD%82%E5%8D%97%E8%A8%98-%E5%89%B2%E5%8F%B0%E4%B8%89%E8%A8%98-%E5%8F%B0%E6%B5%B7%E6%80%9D%E6%85%9F%E9%8C%84-%E7%80%9B%E6%B5%B7%E5%81%95%E4%BA%A1%E8%A8%98-%E5%8F%B0%E6%88%B0%E6%BC%94%E7%BE%A9-%E9%A6%AC%E9%97%9C%E8%AD%B0%E5%92%8C%E4%B8%AD%E4%B9%8B%E4%BC%8A%E6%9D%8E%E5%95%8F%E7%AD%94.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P133: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p133.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 133",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060009_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_133_%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E7%94%B2%E9%9B%86_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P134: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p134.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 134",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060010_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_134_%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E4%B9%99%E9%9B%86_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P135: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p135.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 135",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060011_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_135_%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E4%B8%99%E9%9B%86-%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E4%B8%81%E9%9B%86_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P137: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p137.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 137",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060013_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_137_%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E5%B7%B1%E9%9B%86_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P138: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p138.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 138",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-13288024_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E5%BA%9A%E9%9B%86_%E4%B8%8A.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P139: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p139.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 139",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-12534873_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_139_%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E5%BA%9A%E9%9B%86_%E4%B8%8B.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V07_P140: File = File {
    internal_source: asset!("assets/docs/cn/thds_v07_p140.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 7, Part 140",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060015_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC7%E8%BC%AF_140_%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E8%BE%9B%E9%9B%86-%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E5%A3%AC%E9%9B%86-%E5%8F%B0%E6%A1%88%E5%8C%AF%E9%8C%84%E7%99%B8%E9%9B%86_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V08_P153: File = File {
    internal_source: asset!("assets/docs/cn/thds_v08_p153.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 8, Part 153",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060034_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC8%E8%BC%AF_153_%E5%85%AD%E4%BA%AD%E6%96%87%E9%81%B8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V08_P156: File = File {
    internal_source: asset!("assets/docs/cn/thds_v08_p156.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 8, Part 156",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-12534876_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC8%E8%BC%AF_156_%E7%AA%BA%E5%9C%92%E7%95%99%E8%8D%89-%E4%BD%BF%E7%BD%B2%E9%96%92%E6%83%85_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V08_P159: File = File {
    internal_source: asset!("assets/docs/cn/thds_v08_p159.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 8, Part 159",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060048_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC8%E8%BC%AF_159%E3%80%81160_%E6%9D%8E%E6%96%87%E5%BF%A0%E5%85%AC%E9%81%B8%E9%9B%86_%E4%B8%8A.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V08_P161: File = File {
    internal_source: asset!("assets/docs/cn/thds_v08_p161.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 8, Part 161",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060049_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC8%E8%BC%AF_161_%E6%B8%85%E7%A8%97%E9%A1%9E%E9%88%94%E9%81%B8%E9%8C%84_%E5%BC%B5%E6%96%87%E8%A5%84%E5%85%AC%E9%81%B8%E9%8C%84_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P173: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p173.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 173",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060050_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_173_%E5%8F%B0%E7%81%A3%E5%8D%97%E9%83%A8%E7%A2%91%E6%96%87%E9%9B%86%E6%88%90_%E4%B8%8A.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P177: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p177.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 177",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060061_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_177_%E5%8F%B0%E6%B8%B8%E6%97%A5%E8%A8%98_%E5%8F%B0%E7%81%A3%E9%81%8A%E8%A8%98_%E5%8F%B0%E7%81%A3%E9%81%8A%E8%A1%8C%E8%A8%98_%E9%A2%B1%E9%A2%A8%E9%9B%9C%E8%A8%98_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P178: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p178.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 178",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060062_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_178_%E4%B8%AD%E5%B1%B1%E5%82%B3%E4%BF%A1%E9%8C%84_%E6%B8%85%E5%88%9D%E6%B5%B7%E7%96%86%E5%9C%96%E8%AA%AA_%E5%8F%B0%E7%81%A3%E8%BC%BF%E5%9C%96_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P179: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p179.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 179",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060063_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_179_%E5%B7%A1%E5%8F%B0%E9%80%80%E6%80%9D%E9%8C%84.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P180: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p180.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 180",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060064_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_180_%E5%B7%A6%E6%96%87%E8%A5%84%E5%85%AC%E5%A5%8F%E7%89%98_%E5%90%B3%E5%85%89%E9%8C%84%E4%BD%BF%E9%96%A9%E5%A5%8F%E7%A8%BF%E9%81%B8%E9%8C%84_%E6%A5%8A%E5%8B%87%E6%84%A8%E5%85%AC%E5%A5%8F%E8%AD%B0_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P181: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p181.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 181",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060065_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_181_%E5%8F%B0%E7%81%A3%E6%97%A5%E8%A8%98%E8%88%87%E7%A8%9F%E5%95%9F_%E7%A6%8F%E5%BB%BA%E5%8F%B0%E7%81%A3%E5%A5%8F%E6%91%BA_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P182: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p182.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 182",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060066_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_182_%E5%8A%89%E5%A3%AF%E8%82%85%E5%85%AC%E5%A5%8F%E8%AD%B0_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P187: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p187.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 187",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060069_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_187_%E7%A6%8F%E5%BB%BA%E9%80%9A%E5%BF%97%E5%88%97%E5%82%B3%E9%81%B8_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P188: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p188.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 188",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060070_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_188_%E9%99%B3%E6%B8%85%E7%AB%AF%E5%85%AC%E5%B9%B4%E8%AD%9C_%E5%B0%8F%E9%85%89%E8%85%B4%E5%B1%B1%E9%A4%A8%E4%B8%BB%E4%BA%BA%E8%87%AA%E8%91%97%E5%B9%B4%E8%AD%9C_%E5%8F%B0%E7%81%A3%E9%9C%A7%E5%B3%B0%E6%9E%97%E6%B0%8F%E6%97%8F%E8%AD%9C_%E5%90%88%E8%A8%82%E6%9C%AC.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_THDS_V09_P189: File = File {
    internal_source: asset!("assets/docs/cn/thds_v09_p189.pdf"),
    source_descript: "Taiwan Historical Documents Series, Volume 9, Part 189",
    external_source: "https://commons.wikimedia.org/wiki/File:SSID-10060071_%E5%8F%B0%E7%81%A3%E6%96%87%E7%8D%BB%E5%8F%B2%E6%96%99%E5%8F%A2%E5%88%8A_%E7%AC%AC9%E8%BC%AF_189_%E9%87%8D%E4%BF%AE%E5%8F%B0%E9%83%A1%E5%90%84%E5%BB%BA%E7%AF%89%E5%9C%96%E8%AA%AA_%E5%85%A8.pdf",
    source_citation: "Kong Zhaoming. <i>Taiwan Diaries and Petitions, Fujian-Taiwan Memorials to the Throne</i> Taiwan Historical Documents Series. Vol.9, No. 181. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_SJXB_1872_07_01: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1872_07_01.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 53, 1st July 1872",
    external_source: "https://archive.org/details/shenbao-1872.07.01/",
    source_citation: "",
};
pub const CN_SJXB_1873_04_02: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_04_02.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 283, 2nd April 1873",
    external_source: "https://archive.org/details/shenbao-1873.04.02/",
    source_citation: "",
};
pub const CN_SJXB_1873_04_05: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_04_05.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 286, 5th April 1873",
    external_source: "https://archive.org/details/shenbao-1873.04.05/",
    source_citation: "",
};
pub const CN_SJXB_1873_04_09: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_04_09.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 289, 9th April 1873",
    external_source: "https://archive.org/details/shenbao-1873.04.09/",
    source_citation: "",
};
pub const CN_SJXB_1873_04_15: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_04_15.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 294, 15th April 1873",
    external_source: "https://archive.org/details/shenbao-1873.04.15/",
    source_citation: "",
};
pub const CN_SJXB_1873_05_06: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_05_06.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 312, 6th May 1873",
    external_source: "https://archive.org/details/shenbao-1873.05.06/",
    source_citation: "",
};
pub const CN_SJXB_1873_07_24: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_07_24.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 380, 7th July 1873",
    external_source: "https://archive.org/details/shenbao-1873.07.24/",
    source_citation: "",
};
pub const CN_SJXB_1873_10_11: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_10_11.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 458, 11th October 1873",
    external_source: "https://archive.org/details/shenbao-1873.10.11/",
    source_citation: "",
};
pub const CN_SJXB_1873_11_10: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_11_10.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 473, 10th November 1873",
    external_source: "https://archive.org/details/shenbao-1873.11.10/",
    source_citation: "",
};
pub const CN_SJXB_1873_11_11: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1873_11_11.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 474, 11th November 1873",
    external_source: "https://archive.org/details/shenbao-1873.11.11/",
    source_citation: "",
};
pub const CN_SJXB_1874_04_14: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_04_14.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 599, 14th April 1874",
    external_source: "https://archive.org/details/shenbao-1874.04.14/",
    source_citation: "",
};
pub const CN_SJXB_1874_04_16: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_04_16.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 601, 16th April 1874",
    external_source: "https://archive.org/details/shenbao-1874.04.16/",
    source_citation: "",
};
pub const CN_SJXB_1874_04_17: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_04_17.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 602, 17th April 1874",
    external_source: "https://archive.org/details/shenbao-1874.04.17/",
    source_citation: "",
};
pub const CN_SJXB_1874_04_22: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_04_22.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 606, 22th April 1874",
    external_source: "https://archive.org/details/shenbao-1874.04.22/",
    source_citation: "",
};
pub const CN_SJXB_1874_04_23: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_04_23.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 607, 23th April 1874",
    external_source: "https://archive.org/details/shenbao-1874.04.23/",
    source_citation: "",
};
pub const CN_SJXB_1874_06_08: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_06_08.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 646, 8th June 1874",
    external_source: "https://archive.org/details/shenbao-1874.06.08/",
    source_citation: "",
};
pub const CN_SJXB_1874_06_14: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_06_14.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 652, 14th June 1874",
    external_source: "https://archive.org/details/shenbao-1874.06.14/",
    source_citation: "",
};
pub const CN_SJXB_1874_06_27: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_06_27.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 663, 27th June 1874",
    external_source: "https://archive.org/details/shenbao-1874.06.27/",
    source_citation: "",
};
pub const CN_SJXB_1874_07_03: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_07_03.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 668, 3rd July 1874",
    external_source: "https://archive.org/details/shenbao-1874.07.03/",
    source_citation: "",
};
pub const CN_SJXB_1874_07_09: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_07_09.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 673, 9th July 1874",
    external_source: "https://archive.org/details/shenbao-1874.07.09/",
    source_citation: "",
};
pub const CN_SJXB_1874_07_27: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_07_27.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 688, 27th July 1874",
    external_source: "https://archive.org/details/shenbao-1874.07.27/",
    source_citation: "",
};
pub const CN_SJXB_1874_07_29: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_07_29.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 690, 29th July 1874",
    external_source: "https://archive.org/details/shenbao-1874.07.09/",
    source_citation: "",
};
pub const CN_SJXB_1874_08_03: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_08_03.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 694, 3rd August 1874",
    external_source: "https://archive.org/details/shenbao-1874.08.03/",
    source_citation: "",
};
pub const CN_SJXB_1874_08_08: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_08_08.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 699, 8th August 1874",
    external_source: "https://archive.org/details/shenbao-1874.08.08/",
    source_citation: "",
};
pub const CN_SJXB_1874_08_10: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_08_10.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 700, 10th August 1874",
    external_source: "https://archive.org/details/shenbao-1874.08.10/",
    source_citation: "",
};
pub const CN_SJXB_1874_08_18: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_08_18.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 707, 18th August 1874",
    external_source: "https://archive.org/details/shenbao-1874.08.18/",
    source_citation: "",
};
pub const CN_SJXB_1874_08_19: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_08_19.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 708, 19th August 1874",
    external_source: "https://archive.org/details/shenbao-1874.08.19/",
    source_citation: "",
};
pub const CN_SJXB_1874_08_21: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_08_21.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 710, 21st August 1874",
    external_source: "https://archive.org/details/shenbao-1874.08.21/",
    source_citation: "",
};
pub const CN_SJXB_1874_08_24: File = File {
    internal_source: asset!("assets/docs/cn/sjxb_1874_08_24.pdf"),
    source_descript: "Shanghai News (申江新報), Issue 710, 24th August 1874",
    external_source: "https://archive.org/details/shenbao-1874.08.21/",
    source_citation: "",
};
pub const CN_CANTON_REGISTER_V04: File = File {
    internal_source: asset!("assets/docs/cn/canton_register_v04.pdf"),
    source_descript: "The Canton Register, Volume 4",
    external_source: "https://archive.org/details/canton-register-4/",
    source_citation: "Kong Zhaoming. <i>Selected Memorials and Policy Discussions from Four Reigns: Daoguang, Xianfeng, Tongzhi, and Guangxu</i> Taiwan Historical Documents Series. Vol.4, No. 70. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_CANTON_REGISTER_V05: File = File {
    internal_source: asset!("assets/docs/cn/canton_register_v05.pdf"),
    source_descript: "The Canton Register, Volume 5",
    external_source: "https://archive.org/details/canton-register-5/",
    source_citation: "Kong Zhaoming. <i>Selected Memorials and Policy Discussions from Four Reigns: Daoguang, Xianfeng, Tongzhi, and Guangxu</i> Taiwan Historical Documents Series. Vol.4, No. 70. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_CANTON_REGISTER_V06: File = File {
    internal_source: asset!("assets/docs/cn/canton_register_v06.pdf"),
    source_descript: "The Canton Register, Volume 6",
    external_source: "https://archive.org/details/canton-register-6/",
    source_citation: "Kong Zhaoming. <i>Selected Memorials and Policy Discussions from Four Reigns: Daoguang, Xianfeng, Tongzhi, and Guangxu</i> Taiwan Historical Documents Series. Vol.4, No. 70. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};
pub const CN_CANTON_REGISTER_V10: File = File {
    internal_source: asset!("assets/docs/cn/canton_register_v06.pdf"),
    source_descript: "The Canton Register, Volume 10",
    external_source: "https://archive.org/details/canton-register-10/",
    source_citation: "Kong Zhaoming. <i>Selected Memorials and Policy Discussions from Four Reigns: Daoguang, Xianfeng, Tongzhi, and Guangxu</i> Taiwan Historical Documents Series. Vol.4, No. 70. \
    Ta Tong Book Company, 1987. Accessed February 9, 2025.",
};



pub const BOOK_EULENBURG: File =  File {
    internal_source: asset!("assets/docs/de/book_eulenburg.pdf"),
    external_source: "https://www.google.com/books/edition/de/oM4oAAAAYAAJ/",
    source_descript: "East Asia, 1860-1862, in letters from Count Fritz zu Eulenburg",
    source_citation: "Eulenburg, Friedrich zu, <mark>Ost-Asien, 1860-1862</mark> (Berlin: E.S. Mittler, 1900)",
};
pub const BOOK_BRANDT_1: File =  File {
    internal_source: asset!("assets/docs/de/book_von_brandt_1.pdf"),
    external_source: "https://www.google.com/books/edition/de/Ss9l0V1n_HMC/",
    source_descript: "East Asian Issues: China, Japan, Korea; Old and New",
    source_citation: "Brandt, Max von, <mark>Ostasiatische Fragen</mark> (Berlin: Gebrüder Paetel, 1897)",
};
pub const BOOK_BRANDT_2: File =  File {
    internal_source: asset!("assets/docs/de/book_von_brandt_2.pdf"),
    external_source: "https://www.google.com/books/edition/de/cW1DAQAAMAAJ/",
    source_descript: "Thirty-Three Years in East Asia: Memories of a German Diplomat; Volume 2",
    source_citation: "Brandt, Max von, <mark>Dreiunddreissig Jahre in Ost-Asien</mark> (Leipzig: Georg Wigand, 1901)",
};


pub const BOOK_WIERBŁOWSKI: File = File {
    internal_source: asset!("assets/docs/pl/book_wierbłowski.pdf"),
    external_source: "https://www.google.com/books/edition/pl/oxiMSeSPvPgC/",
    source_descript: "(Deputy Minister of Foreign Affairs, Poland) Stefan Wierbłowski, “Poland and the San Francisco Conference; September 4-8, 1951”",
    source_citation: "",
};


pub const SERIAL_EECL_MOC_V10: File = File {
    internal_source: asset!("assets/docs/fr/serial_eecl_moc_v10.pdf"),
    external_source: "https://gallica.bnf.fr/ark:/12148/bpt6k9763708t/",
    source_descript: "Society of Jesus, “Edifying and Curious Letters: Memories of China, Volume 10”",
    source_citation: "",
};
pub const SERIAL_EECL_MOC_V12: File = File {
    internal_source: asset!("assets/docs/fr/serial_eecl_moc_v12.pdf"),
    external_source: "https://gallica.bnf.fr/ark:/12148/bpt6k9763715z/",
    source_descript: "Society of Jesus, “Edifying and Curious Letters: Memories of China, Volume 12”",
    source_citation: "",
};
// Written by Edme-François Jomard regarding map submitted by Consul-General in Shanghai Louis-Charles-Nicolas-Maximilien de Montigny
pub const SERIAL_RDG_V14: File = File {
    internal_source: asset!("assets/docs/fr/serial_rdg_v14.pdf"),
    external_source: "https://www.google.com/books/edition/fr/N2dgAAAAIAAJ/",
    source_descript: "Ludovic Drapeyron, “Review of Geography, Volume 14”",
    source_citation: "",
};
pub const FR_MEMO_1784_12_20_VIEILLARD: File =  File {
    internal_source: asset!("assets/docs/fr/memo_1784_12_20_vieillard.pdf"),
    external_source: "https://archivesdiplomatiques.diplomatie.gouv.fr/ark:/14366/0wjh4bkl368s/",
    source_descript: "[Vice Consul of French Consulate in Canton] Philippe Vieillard: Letter to [Minister of the Navy] Charles-Eugène-Gabriel de La Croix de Castries, 20 Dec 1784",
    source_citation: "",
};
pub const FR_MEMO_1874_06_06_KRANTZ: File =  File {
    internal_source: asset!("assets/docs/fr/memo_1874_06_06_krantz.pdf"),
    external_source: "https://archivesdiplomatiques.diplomatie.gouv.fr/ark:/14366/97ztg2shvrj5/",
    source_descript: "[Commander in Chief of the Naval Division of the Seas of China and Japan] Jules-François-Émile Krantz: Letter to [Minister of the Navy and Colonies] Louis-Raymond de Montaignac de Chauvance, 6 Jun 1874",
    source_citation: "",
};
pub const FR_MEMO_1874_07_17_GRESLEY: File =  File {
    internal_source: asset!("assets/docs/fr/memo_1874_07_17_gresley.pdf"),
    external_source: "https://archivesdiplomatiques.diplomatie.gouv.fr/ark:/14366/97ztg2shvrj5/",
    source_descript: "[Chief of Staff of the Ministry of War] Henri-François-Xavier Gresley: Letter to [Minister of the Foreign Affairs] Louis-Charles-Élie-Amanien Decazes de Glücksbierg, 17 Jul 1874",
    source_citation: "",
};
pub const FR_MEMO_1884_10_10_COURBET: File =  File {
    internal_source: asset!("assets/docs/fr/memo_1884_10_10_courbet.pdf"),
    external_source: "https://archivesdiplomatiques.diplomatie.gouv.fr/ark:/14366/513f6vxlb80m/",
    source_descript: "[Commander of the Far East Squadron] Anatole-Amédée-Prosper Courbet: Letter to [Minister of the Navy and Colonies] Alexandre-Louis-François Peyron, 10 Oct 1884",
    source_citation: "",
};
pub const FR_MEMO_1884_10_11_PEYRON: File =  File {
    internal_source: asset!("assets/docs/fr/memo_1884_10_11_peyron.pdf"),
    external_source: "https://archivesdiplomatiques.diplomatie.gouv.fr/ark:/14366/513f6vxlb80m/",
    source_descript: "[Minister of the Navy and Colonies] Alexandre-Louis-François Peyron: Letter to [Commander of the Far East Squadron] Anatole-Amédée-Prosper Courbet, 11 Oct 1884",
    source_citation: "",
};
pub const FR_MEMO_1884_10_19_COURBET: File =  File {
    internal_source: asset!("assets/docs/fr/memo_1884_10_19_courbet.pdf"),
    external_source: "https://archivesdiplomatiques.diplomatie.gouv.fr/ark:/14366/513f6vxlb80m/",
    source_descript: "[Commander of the Far East Squadron] Anatole-Amédée-Prosper Courbet: Letter to [Minister of the Navy and Colonies] Alexandre-Louis-François Peyron, 19 Oct 1884",
    source_citation: "",
};
pub const FR_MEMO_1884_10_20_COURBET: File =  File {
    internal_source: asset!("assets/docs/fr/memo_1884_10_20_courbet.pdf"),
    external_source: "https://archivesdiplomatiques.diplomatie.gouv.fr/ark:/14366/513f6vxlb80m/",
    source_descript: "[Commander of the Far East Squadron] Anatole-Amédée-Prosper Courbet: “Notification of the Blockade of the Coast of the Island of Formosa,” 20 Oct 1884",
    source_citation: "",
};
pub const FR_MEMO_1884_12_05_PATENÔTRE: File =  File {
    internal_source: asset!("assets/docs/fr/memo_1884_12_05_patenôtre.pdf"),
    external_source: "https://archivesdiplomatiques.diplomatie.gouv.fr/ark:/14366/513f6vxlb80m/",
    source_descript: "[Minister Plenipotentiary of French Legation in Shanghai] Jules Patenôtre des Noyers: “Notification of the Blockade of the Coast of the Island of Formosa,” 5 Dec 1884",
    source_citation: "",
};


pub const BOOK_MFA: File =  File {
    internal_source: asset!("assets/docs/tw/book_mfa.pdf"),
    external_source: "https://www.google.com/books/edition/Taiwan/2DgnI0vvpVgC",
    source_descript: "Treaties between the Republic of China and Foreign States (1927-1957)",
    source_citation: "",
};
pub const GAZETTE_1942_11_04_CDN: File =  File {
    internal_source: asset!("assets/docs/tw/gazette_1942_11_04_cdn.pdf"),
    external_source: "https://archive.org/details/zhongyang-ribao-1942.11.04/",
    source_descript: "Central Daily News",
    source_citation: ""
};
pub const GAZETTE_1949_01_06_TPVD: File =  File {
    internal_source: asset!("assets/docs/tw/gazette_1949_01_06_tpvd.pdf"),
    external_source: "https://das.nlpi.edu.tw/handle/eax43",
    source_descript: "Taiwan People's Voice Daily",
    source_citation: ""
};
pub const TW_MEMO_1941_12_12_SOONG: File =  File {
    internal_source: asset!("assets/docs/tw/memo_1941_12_12_soong.pdf"),
    external_source: "https://catalog.archives.gov/id/28276867/",
    source_descript: "[Personal Representative of Chiang Kai-shek] Soong Tse-vung, Letter to [Secretary of the Treasury] Henry Morgenthau II, 12 Dec 1941",
    source_citation: "",
};
pub const MEMO_1943_11_25_WANG: File =  File {
    internal_source: asset!("assets/docs/tw/memo_1943_11_25_wang.pdf"),
    external_source: "https://ahonline.drnh.gov.tw/index.php?act=Display/image/2380617gzg=_ge",
    source_descript: "[Secretary General of the Supreme National Defense Council] Wang Chonghui, “Summary of Matters Handled During the Cairo Conference,” 25 Nov 1943",
    // Supreme National Defense Council = 国防最高委员会
    source_citation: "",
};
pub const TW_MEMO_1943_12_05_SOONG: File =  File {
    internal_source: asset!("assets/docs/tw/memo_1943_12_05_soong.pdf"),
    external_source: "https://catalog.archives.gov/id/16618380/",
    source_descript: "[First Lady of the Republic of China] Soong Mei-ling, Letter to [President of the United States] Franklin Delano Roosevelt, 5 Dec 1943",
    source_citation: "",
};
pub const TW_MEMO_1944_12_21_CHIANG: File =  File {
    internal_source: asset!("assets/docs/tw/memo_1944_12_21_chiang.pdf"),
    external_source: "https://ahonline.drnh.gov.tw/index.php?act=Display/image/7467177fEm9bmq",
    source_descript: "[Chairman of the Republic of China] Chiang Kai-shek, Diary Entry, 21 Dec 1944",
    source_citation: "",
};
pub const TW_MEMO_1949_01_12_CHIANG: File =  File {
    internal_source: asset!("assets/docs/tw/memo_1949_01_12_chiang.pdf"),
    external_source: "https://ahonline.drnh.gov.tw/index.php?act=Display/image/7462808=kqhpZd",
    source_descript: "[President of the Republic of China] Chiang Kai-shek, Telegram to [Chairman of Taiwan Provincial Government] Chen Cheng, 12 Jan 1949",
    source_citation: "",
};
pub const TW_MEMO_1961_06_09_CHIANG: File =  File {
    internal_source: asset!("assets/docs/tw/memo_1961_06_09_chiang.pdf"),
    external_source: "https://catalog.archives.gov/id/564377942/",
    source_descript: "[President of the Republic of China] Chiang Kai-shek, Letter to [Vice President of the United States] Lyndon Baines Johnson, 9 Jun 1961",
    source_citation: "",
};



pub const UN_S006_A_2119: File =  File {
    internal_source: asset!("assets/docs/un/un_s006_a_2119.pdf"),
    external_source: "https://digitallibrary.un.org/record/228954",
    source_descript: "United Nations, “Resolutions adopted by the General Assembly during its 6th session, 6 November 1951 to 5 February 1952”",
    source_citation: "",
};
pub const MEMO_1948_11_12_PAL: File =  File {
    internal_source: asset!("assets/docs/un/memo_1948_11_12_pal.pdf"),
    external_source: "https://unwcc.org/wp-content/uploads/2023/06/Reel-170-Transcript-of-the-IMTFE-USA-and-others-v-Araki-and-others-2.pdf",
    source_descript: "[Judge of the International Military Tribunal for the Far East] Radhabinod Pal: “Dissentient Judgment of Justice Pal,” 12 Nov 1948",
    source_citation: "",
};




pub const JP_MEMO_1874_03_13_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_03_13_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/655779",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Memorandum No. 22: Prospective Aboriginal Operation,” 13 Mar 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_03_31_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_03_31_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/655904",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Memorandum No. 23: Prospective Aboriginal Operation,” 31 Mar 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_04_20_SANJO: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_04_20_sanjo.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/660869",
    source_descript: "[Japanese Chancellor of the Realm] Sanjō Sanetomi: Letter to [Japanese Minister of Colonization] Okuma Shigenobu, 20 Apr 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_04_27_OKUMA: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_04_27_okuma.pdf"),
    external_source: "https://www.digital.archives.go.jp/DAS/meta/listPhoto?LANG=default&BID=F0000000000000012399&ID=&NO=12",
    source_descript: "[Japanese Minister of Colonization] Okuma Shigenobu: Letter to Charles-Guillaum-Joseph-Émile le Gendre, 27 Apr 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_04_28_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_04_28_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/DAS/meta/listPhoto?LANG=default&BID==F0000000000000012399&ID=&NO=13",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: Letter to [Japanese Minister of Colonization] Okuma Shigenobu, 28 Apr 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_05_04_SHINAGAWA: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_05_04_shinagawa.pdf"),
    external_source: "https://www.digital.archives.go.jp/DAS/meta/listPhoto?LANG=default&BID==F0000000000000012399&ID=&NO=21",
    source_descript: "[Consul of Japanese Consulate in Shanghai] Shinagawa Tadamichi: “Telegram No. 109”, 4 May 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_05_05_OKUMA: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_05_05_okuma.pdf"),
    external_source: "https://www.digital.archives.go.jp/DAS/meta/listPhoto?LANG=default&BID=F0000000000000012399&ID=&NO=22",
    source_descript: "[Japanese Minister of Colonization] Okuma Shigenobu: Letter to [Consul of Japanese Consulate in Shanghai] Shinagawa Tadamichi, 5 May 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_05_19_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_05_19_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/660622",
    //alt: https://www.digital.archives.go.jp/img/en/661085
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: Letter to [Japanese Legal Advisor to the Ministry of Foreign Affairs] Erasmus Peshine Smith, 19 May 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_05_19_SANJO: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_05_19_sanjo.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/667056",
    //alt: https://www.digital.archives.go.jp/img/en/661085
    source_descript: "[Japanese Chancellor of the Realm] Sanjō Sanetomi: “Notification to the Chambers, Ministries, Colonization Commission, Prefectures, and Cities regarding the departure of Commander-in-Chief Saigō to demand justice in the Aboriginal Territory,” 19 May 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_06_17_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_06_17_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/DAS/meta/listPhoto?LANG=default&BID=F0000000000000012399&ID=&NO=34",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: Letter to [Commander of the Taiwan Expedition] Saigō Jūdō, 17 Jun 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_06_18_SMITH: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_06_26_smith.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/660641",
    source_descript: "[Japanese Legal Advisor to the Ministry of Foreign Affairs] Erasmus Peshine Smith: Letter to [Minister of Foreign Affairs] Terashima Munenori, 18 Jun 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_06_26_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_06_26_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/660638",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: Letter to [Japanese Minister of Colonization] Okuma Shigenobu, 26 Jun 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_06_30_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_06_30_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/DAS/meta/listPhoto?LANG=default&BID=F0000000000000012397&ID=&NO=23",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Memorandum No. 32,” 30 Jun 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_07_05_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_07_05_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/DAS/meta/listPhoto?LANG=default&BID=F0000000000000012397&ID=&NO=20",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Memorandum No. 33,” 5 Jul 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_07_15_UNKNOWN: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_07_15_unknown.pdf"),
    external_source: "https://www.digital.archives.go.jp/DAS/meta/listPhoto?LANG=default&BID=F0000000000000012400&ID=&NO=2",
    source_descript: "Unknown: “Memorandum on Negotiations with Qing China,” 15 Jul 1874",
    source_citation: "",
};
pub const US_MEMO_1874_08_XX_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/us/memo_1874_08_xx_legendre.pdf"),
    external_source: "https://archive.org/details/IsAboriginalFormosaAPartOfTheChineseEmpire/",
    source_descript: "[Advisor to the Japanese Bureau of Taiwan Aborigine Affairs] Charles-Guillaum-Joseph-Émile le Gendre: “Is Aboriginal Formosa a part of the Chinese Empire,” Aug 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_08_08_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_08_08_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 2,” 8 Aug 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_08_18_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_08_18_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 4,” 18 Aug 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_08_28_LEGENDRE_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_08_28_legendre_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 5,” 28 Aug 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_08_28_LEGENDRE_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_08_28_legendre_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 6,” 28 Aug 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_08_12_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_08_12_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/664010",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 1: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 12 Aug 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_08_12_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_08_12_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/655909",
    //alt: https://www.digital.archives.go.jp/img/en/655528
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 1: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 12 Aug 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_09_18_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_09_18_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/665330",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 2: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 18 Sep 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_09_18_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_09_18_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/655961",
    //alt: https://www.digital.archives.go.jp/img/en/661356
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 2: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 18 Sep 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_09_23_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_09_23_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/665429",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 3: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 23 Sep 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_09_23_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_09_23_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/656017",
    //alt: https://www.digital.archives.go.jp/img/en/655478
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 3: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 23 Sep 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_10_01_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_10_01_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 8,” 1 Oct 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_10_05_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_10_05_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 9,” 5 Oct 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_10_11_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_10_11_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 10,” 11 Oct 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_10_17_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_10_17_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 11,” 17 Oct 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_10_23_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_10_23_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 12,” 23 Oct 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_10_28_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_10_28_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/666200",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 4: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 28 Oct 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_10_28_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_10_28_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/655963",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 4: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 28 Oct 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_10_30_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_10_30_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 13,” 30 Oct 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_11_04_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_11_04_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/665615",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 5: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 4 Nov 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_11_04_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_11_04_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/656018",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 5: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 4 Nov 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_11_05_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_11_05_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/665615",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 6: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 5 Nov 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_11_05_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_11_05_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/655962",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 6: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 5 Nov 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_11_11_LEGENDRE: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_11_11_legendre.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/3084785",
    source_descript: "Charles-Guillaum-Joseph-Émile le Gendre: “Dispatch No. 14,” 11 Nov 1874",
    source_citation: "",
};
pub const JP_MEMO_1874_11_15_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_11_15_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/665615",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum Nos. 7 and 8: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 15 Nov 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_11_15_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_11_15_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/656019",
    // continued: https://www.digital.archives.go.jp/img/en/660621
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 7: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 15 Nov 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_11_15_NOGUCHI_3: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_11_15_noguchi_3.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/660622",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 8: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 15 Nov 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_12_12_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_12_12_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/666147",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 9: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 12 Dec 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_12_12_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_12_12_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/660771",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 9: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 12 Dec 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1875_12_13_NOGUCHI_1: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_12_13_noguchi_1.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/663090",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 10: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 13 Dec 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};
pub const JP_MEMO_1874_12_13_NOGUCHI_2: File =  File {
    internal_source: asset!("assets/docs/jp/memo_1874_12_13_noguchi_2.pdf"),
    external_source: "https://www.digital.archives.go.jp/img/en/660658",
    source_descript: "[Secretary of the JP Bureau of Taiwan Aborigine Affairs] Noguchi Tsunetomo: “Memorandum No. 10: The Principle of the Course of Actions Taken towards the Aboriginal Territory of Formosa”, 13 Dec 1874",
    // Noguchi Tsunetomo = 野口常共, Secretary = Gonshonaishi = 権少内史
    source_citation: "",
};


pub const PK_MEMO_1971_04_27_HILALY: File =  File {
    internal_source: asset!("assets/docs/pk/memo_1971_04_27_hilaly.pdf"),
    external_source: "https://catalog.archives.gov/id/559236017",
    source_descript: "[Pakistani Ambassador to the US] Agha Hilaly: Letter from [Premier of the PRC] Zhou Enlai to [President of the US] Richard Milhous Nixon, 27 Apr 1971",
    source_citation: "",
};
pub const PK_MEMO_1971_06_02_HILALY: File =  File {
    internal_source: asset!("assets/docs/pk/memo_1971_06_02_hilaly.pdf"),
    external_source: "https://catalog.archives.gov/id/559236017",
    source_descript: "[Pakistani Ambassador to the US] Agha Hilaly: Letter from [Premier of the PRC] Zhou Enlai to [President of the US] Richard Milhous Nixon, 02 Jun 1971",
    source_citation: "",
};
pub const PK_MEMO_1971_06_21_HILALY: File =  File {
    internal_source: asset!("assets/docs/pk/memo_1971_06_21_hilaly.pdf"),
    external_source: "https://catalog.archives.gov/id/559236017",
    source_descript: "[Pakistani Ambassador to the US] Agha Hilaly: Letter from [Premier of the PRC] Zhou Enlai to [President of the US] Richard Milhous Nixon, 21 Jun 1971",
    source_citation: "",
};
*/