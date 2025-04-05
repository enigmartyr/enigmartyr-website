use std::any::TypeId;
use dioxus::document::Stylesheet;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use crate::generic::web::timeline::Timeline;
use crate::politics::data::person::*;
use crate::politics::data::file::*;
use web_sys::{HtmlCollection, window};
use crate::politics::tw::sovereignty::TipType::*;
use log::debug;

const BLOG_CSS: Asset = asset!("/assets/style/blog.css");

pub fn c_1886_11_17_denby_01() -> Element {
    rsx!(FileCitation {
        file: &US_CRUS_1887_1887,
        FigureAttribution {
            figure: &CHARLES_BRADLEY,
            locale: [
                rsx!(span { "US Legation" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { "Nov 17th, 1886" })
            ],
            h2 {
                "Importance of the Island of Formosa in Our Trade with China"
            },
            p {
                "The steady growth of the foreign trade of Formosa and the great military          \
                importance of the island, forming, as it does, to use the words of a recent        \
                memorial to the throne, “ a portal to the Southern China Sea and a bulwark for     \
                seven provinces,” authorize me to call your attention to the present condition of  \
                the island and its importance in our trade with China."
            },
            p {
                "Although having nominally possession of Formosa since the latter part of the      \
                seventeenth century, ", mark { "the Chinese have only been able to establish their \
                rule along the coast, the interior of the island being still inhabited by          \
                independent tribes of aborigines" }, " who live chiefly by hunting, and who have   \
                ever shown unconquerable aversion to the Chinese."
            },
            p {
                "In their fondness for head hunting they resemble the Dayaks of Borneo, and we     \
                learn from a recent memorial of the military administrator in Formosa that the     \
                murders committed by them number over a thousand yearly."
            },
            p {
                "Notwithstanding the animosity of the natives and the inability of the authorities \
                to afford adequate protection to persons and property, the natural resources of    \
                Formosa and t he wonderful fertility of its soil have for years past caused a      \
                stream of emigration to flow thither from the provinces on the mainland, and day   \
                by day the area of cultivation is spreading and the natural products becoming      \
                better known and more thoroughly worked."
            },
            p {
                "The present military administrator in Formosa, Lin-Ming-Chuan, whose energetic    \
                defense against the French, in 1884-'85 called forth the admiration of China,      \
                appears to be doing much in the way of establishing order and developing the       \
                resources of the island."
            },
            p {
                "Memorializing the throne under date of July 20 last, he states that he has been   \
                able this year to bring into subjection over 400 villages, that over 70,000 people \
                have embraced civilization, and that over 20,000 acres of land earlier brought     \
                under cultivation, but since abandoned, have been reclaimed."
            },
            p {
                "The defenses of Formosa have also attracted the serious attention of the military \
                administrator, and the ports on the west and north coasts, the only ones           \
                accessible, as well as the Pescadore Islands, are being fortified and armed with   \
                the most approved type of Krupp and Armstrong guns. It is also intended to         \
                establish telegraph lines and to connect the island with the mainland by a         \
                submarine cable. A short railway will in all probability be constructed between    \
                Kelung and Tamsui, and the name of an American firm, well known in China, is       \
                mentioned in connection with this last undertaking."
            },
            p {
                "Heretofore Formosa has been an integral part of the province of Fuhkien, but on   \
                the 16th of January, of this year, there appeared a decree ordering it to be       \
                constituted a separate province, but, joined to Fuhkien and the Pescadore Islands, \
                made the headquarters of the general of the Haitan division."
            },
            p {
                del { "..." }
            },
            p {
                style: "text-align-last: right;",
                "—Charles Denby"
            }
        }
    })
}

pub fn c_1858_11_20_bradley_01() -> Element {
    rsx!(FileCitation {
        file: &US_C035_S02_REPORT_N0068,
        FigureAttribution {
            figure: &CHARLES_BRADLEY,
            locale: [
                rsx!(span { "US Consulate" }),
                rsx!(span { "Ningbo, China" }),
                rsx!(span { "Nov 20th, 1858" })
            ],
            h2 {
                "Notes on the Additional Chinese Ports Opened by Conventions with the Four Treaty  \
                Powers"
            },
            p {
                "Niuchwang, in the Manchu province of Shinking, is the most northern port opened   \
                by any of the treaties. It is situated at the mouth of the Liau river, which       \
                empties into the northeastern extremity of the Gulf of Liautung, in about latitude \
                40° 45' N., longitude 122° 50' E., some thirteen leagues from Moukden, the         \
                capital. The limited productions of the province are wheat, barley, pulse, millet, \
                ginseng, rhubarb, timber, camels and horses, some of which are now exchanged at Kí-\
                iu-wan, (a market town on and within the frontiers of Corea,) and others are       \
                bartered with the Euhkien junkmen, who mostly monopolize the trade of the place by \
                sea. A few of the native craft also ply between this and Tientsin, laden with      \
                grain for the “Great Northern Capital.” The harbor is artificial, formed by        \
                strongly built stone piers, between which an opening is left capable of admitting  \
                the largest junks, which, however, can only enter and depart at high water; at low \
                tide the larger vessels remaining within it always ground. Tire winters in this    \
                region are represented as long and intensely severe. I imagine that no American    \
                consul will be required here, and that the opening of the port was obtained by     \
                Great Britain rather as a convenient place for watching the movements of Russia    \
                towards the south and west than for the inconsiderable commercial advantages which \
                it affords. Count Poutiatine, the Russian minister, was opposed to this            \
                concession, and I think it was solely on political grounds."
            },
            p {
                del { "..." }
            },
            p {
                "Taiwan-fu, the departmental city in the island of Taiwan, (Formosa,) is situated  \
                on the southwest coast of the island, in latitude 23° N., longitude 120° 7' 50″ E. \
                It carries on a large and thrifty commerce with many places on the main land and   \
                in the Eastern seas, exporting rice, maize, fruits, timber, camphor, indigo, salt, \
                sulphur, &c. A considerable and increasing European trade has been permitted at    \
                this port for the past three or four years. I deem it highly important that it     \
                should have a consular officer, not only for the protection of commerce, which     \
                cannot fail to be considerable, but also for the sake of our seamen, ", mark {
                "many of whom are from time to time wrecked on the Formosan coast, where they are  \
                either put to death or held in captivity" }, " or otherwise suffer extreme         \
                hardships."
            },
            p {
                del { "..." }
            },
            p {
                "Mr. Interpreter Martin very sensibly remarks that 11 the effect of giving to      \
                foreign ships the range of the coast will probably be to throw the whole coasting  \
                trade into their hands, to the detriment of the native junks.” Already the Chinese \
                prefer foreign vessels, not only because they are swift, insurable, and secure     \
                from pirates, but because they sail more cheaply than junks, the latter being      \
                subject to restrictions, impressments, and dangers, which oblige them to charge    \
                high freights. All that was needed to complete the ascend ency of the foreign ship \
                has now been acquired in the reduction of tonnage dues and exemption from more     \
                than one payment in four months. Small American vessels, (say from 300 to 450 or   \
                500 tons,) well officered, will doubtless secure a good part of the coast trade,   \
                and are likely to sail more safely than heretofore, since, with the displacement   \
                of the native craft, the pirates who preyed upon them may be expected to disappear."
            },
            p {
                "Until, however, our tonnage measurement shall have been altered, and made to      \
                conform with that of Great Britain and of maritime Europe in general, we shall     \
                probably be under chartered by the vessels of those countries; paying, as they do, \
                four mace for fifty feet, as tonnage dues, whilst we pay the same for forty feet."
            },
            p {
                style: "text-align-last: right;",
                "—C. W. Bradley"
            }
        }
    })
}

pub fn c_1855_02_14_perry_01() -> Element {
    rsx!(FileCitation {
        file: &US_C033_S02_REPORT_N0097,
        FigureAttribution {
            figure: &MATTHEW_PERRY,
            locale: [
                rsx!(span { "14 Wall street" }),
                rsx!(span { "New York, NY" }),
                rsx!(span { "Feb 14th, 1855" })
            ],
            p {
                "But, after all, these events in the history and fate of nations are doubtless     \
                directed by an overruling Providence, and probably we could not, if we would,      \
                change their course, or avert our ultimate destiny. It only belongs to us to       \
                endeavor to act justly and honorably in all our foreign relations, and I cannot    \
                but believe that we should be just to ourselves and to the world, to en", ins {
                "c" }, "ourage whatever practical measures might be suggested to change for the    \
                better the political and civil condition of China and Japan, and the countries     \
                more to the south; and especially with respect to Formosa. The United States alone \
                should assume the initiative. ", mark { "This magnificent island, though nominally \
                a province of China, is practically independent. The imperial authorities          \
                maintaining a feeble and precarious footing only in isolated parts of the island;"
                }, " a large portion being in possession of independent tribes, and yet such is    \
                its productiveness in minerals, drugs, and the more valuable products of those     \
                genial regions, that at this time a revenue, estimated at a million of dollars, is \
                collected, though little or none of it goes into the imperial treasury."
            },
            p {
                "The inhabitants of the island may be divided into two classes: the first composed \
                of those at present submitting to the authority of China, whether of native or     \
                Chinese blood; and the other, and ", mark { "probably the more numerous portion" },
                ", of natives, yet in their unconquered and primitive state. The whole population  \
                has been estimated at two, and by some, as high as three millions; and looking to  \
                the peculiar abstemiousness of the people of the east, who rarely indulge in any   \
                other than vegetable food, it may reasonably be supposed that an island of the     \
                extent of Formosa, and of such fertility, could subsist even a larger number."
            }
        }
    })
}

pub fn c_1851_10_10_perry_01() -> Element {
    rsx!(FileCitation {
        file: &NYDH_1851_10_12,
        FigurelessAttribution {
            locale: [
                rsx!(span { "14 Wall street" }),
                rsx!(span { "New York, NY" }),
                rsx!(span { "Oct 8th, 1851" })
            ],
            p {
                "Hon. W. A. Graham, Secretary of the U. S. Navy:"
            },
            p {
                "Sir: Prompted by feelings of humanity, combined with those of duty, I             \
                respectfully solicit your attention to the following circumstances of the loss of  \
                the bark Coquette, commanded by my brother, Captain Prescott."
            },
            p {
                "He left Shanghae with his vessel, in September, 1849, for Canton, in company with \
                two brigs. A few days after their departure a severe typhoon arose, and the        \
                Coquette, with the two brigs, never reached the port of their destination.         \
                Recently two seamen were taken by an English vessel of war from the island of      \
                Formosa. They state that the bark Coquette, with a brig, had been wrecked on that  \
                island, and that the crews were either murdered or made prisoners."
            },
            p {
                "I would inquire if the department over which you preside can take any steps by    \
                which the above statements may be tested, and if the missing crews are imprisoned? \
                Whether or not some measures can be adopted to secure their liberation?            \
                Respectfully yours,"
            },
            p {
                style: "text-align-last: right;",
                "—H. W. Prescott"
            }
        },
        FigureAttribution {
            figure: &CHARLES_CONRAD,
            locale: [
                rsx!(span { "Navy Department" }),
                rsx!(span { "Washington, DC" }),
                rsx!(span { "Oct 10th, 1874" })
            ],
            p {
                "H. W. Prescott, 14 Wall street, New York:"
            },
            p {
                "Sir: Your letter of the 8th instant, in relation to the wreck of the bark         \
                Coquette, under the command of your brother, Captain Prescott, has been received."
            },
            p {
                "A copy of your communication will be sent to Commodore Aulick, commanding the     \
                East India Squadron, with instructions to send one of the vessels of his command   \
                to the island of Formosa, for the purpose of ascertaining the facts in the case of \
                the wreck of the bark Coquette, and the treatment of the crew."
            },
            p {
                "I am, Sir, respectfully, your obedient servant,"
            },
            p {
                style: "text-align-last: right;",
                "—C. M. Conrad, Acting Secretary of the Navy"
            }
        }
    })
}

pub fn c_1851_10_10_conrad_01() -> Element {
    rsx!(FileCitation {
        file: &NYDH_1851_10_12,
        FigurelessAttribution {
            locale: [
                rsx!(span { "14 Wall street" }),
                rsx!(span { "New York, NY" }),
                rsx!(span { "Oct 8th, 1851" })
            ],
            p {
                "Hon. W. A. Graham, Secretary of the U. S. Navy:"
            },
            p {
                "Sir: Prompted by feelings of humanity, combined with those of duty, I             \
                respectfully solicit your attention to the following circumstances of the loss of  \
                the bark Coquette, commanded by my brother, Captain Prescott."
            },
            p {
                "He left Shanghae with his vessel, in September, 1849, for Canton, in company with \
                two brigs. A few days after their departure a severe typhoon arose, and the        \
                Coquette, with the two brigs, never reached the port of their destination.         \
                Recently two seamen were taken by an English vessel of war from the island of      \
                Formosa. They state that the bark Coquette, with a brig, had been wrecked on that  \
                island, and that the crews were either murdered or made prisoners."
            },
            p {
                "I would inquire if the department over which you preside can take any steps by    \
                which the above statements may be tested, and if the missing crews are imprisoned? \
                Whether or not some measures can be adopted to secure their liberation?            \
                Respectfully yours,"
            },
            p {
                style: "text-align-last: right;",
                "—H. W. Prescott"
            }
        },
        FigureAttribution {
            figure: &CHARLES_CONRAD,
            locale: [
                rsx!(span { "Navy Department" }),
                rsx!(span { "Washington, DC" }),
                rsx!(span { "Oct 10th, 1874" })
            ],
            p {
                "H. W. Prescott, 14 Wall street, New York:"
            },
            p {
                "Sir: Your letter of the 8th instant, in relation to the wreck of the bark         \
                Coquette, under the command of your brother, Captain Prescott, has been received."
            },
            p {
                "A copy of your communication will be sent to Commodore Aulick, commanding the     \
                East India Squadron, with instructions to send one of the vessels of his command   \
                to the island of Formosa, for the purpose of ascertaining the facts in the case of \
                the wreck of the bark Coquette, and the treatment of the crew."
            },
            p {
                "I am, Sir, respectfully, your obedient servant,"
            },
            p {
                style: "text-align-last: right;",
                "—C. M. Conrad, Acting Secretary of the Navy"
            }
        }
    })
}

pub fn c_1874_05_22_house_01() -> Element {
    let t01 = rsx!(Tip { tip: S("Hengchun"), "Liangkiao" });
    rsx!(FileCitation {
        file: &BOOK_CHINESE_REPO_20,
        FigureAttribution {
            figure: &EDWARD_HOUSE,
            locale: [
                rsx!(span { "SS ", i { ruby { "Yūkō Maru", rt { "有功丸" }}}}),
                rsx!(span { ruby { "Hengchun", rt { "恆春" }}, " Bay" }),
                rsx!(span { "22 May 1874" })
            ],
            p {
                "While the skirmish of the 22d was going on, several large ships entered and       \
                anchored in ", { t01 }, " Bay. The earliest to arrive was the “Takasagu Maru”      \
                which brought General Saigo, the Commander in Chief, with his staff, and fifteen   \
                hundred soldiers and laborers. She was followed by a second transport, the         \
                passengers in which increased the entire available force of fighting men to about  \
                thirteen hundred. Before the disembarkation commenced, two other vessels           \
                approached from a different direction, the nationality of which was at first       \
                doubtful, but which presently proved to be a Chinese frigate and gunboat."
            },
            p {
                "The arrival of these ships of war excited much curiosity, for, up to this time,   \
                we, in Formosa, had received no definite or trustworthy intelligence of the views  \
                of the Chinese government, since the commencement of the efforts of foreign        \
                Ministers to divert them from the true issue, and great uncertainty was felt as to \
                the course of action they might adopt."
            },
            p {
                "It was a relief to find, on visiting these newcomers, that their visit was in no  \
                respect unfriendly, for not only were all anxieties as to Chinese interference     \
                thus allayed, but conclusive proofs were afforded of the falseness of the grounds  \
                upon which the mischief-makers at Tokio and Peking had based their irritating      \
                proceedings. According to these messengers, the government of China had never yet  \
                objected in any degree to the Japanese movement, and one of the special errands of \
                the ships was to communicate with the natives of the Liangkiao valley and to       \
                assure the chiefs personally, and the people by posted proclamations, that the     \
                Japanese were here to do a good work, that the Chinese authorities were in         \
                sympathy with them, and that it should be the duty of all the inhabitants to       \
                assist them in every way that lay in their power."
            },
            p {
                "Making due allowance for the high-flown extravagance of some of their             \
                declarations, it was abundantly clear that, up to this time, no hostile sentiments \
                had been generally diffused among the Chinese officials; and indeed, evidence was  \
                afterward supplied that the Admiral in command at Amoy had reiterated the          \
                familiar assertions that his government was not responsible for and had no         \
                jurisdiction over the savage population, some days after the first party of        \
                Japanese had actually reached Formosa."
            }
        }
    })
}

pub fn c_1875_xx_xx_house_01() -> Element {
    rsx!(FileCitation {
        file: &BOOK_CHINESE_REPO_20,
        FigureAttribution {
            figure: &EDWARD_HOUSE,
            locale: [
                rsx!(span { ""}),
                rsx!(span { "Tokyo, Japan" }),
                rsx!(span { "circa 1875" })
            ],
            p {
                "the office of Foreign Minister, he went as Ambassador to Peking chiefly to lay    \
                his designs before the Chinese government, and to obtain their views upon that and \
                other proposed Japanese projects. His public diplomatic successes during this      \
                mission are matters of common notoriety. It was, in fact, through him that the     \
                long unsettled question of Imperial audiences was brought to a prompt solution."
            },
            p {
                "His success in the more private negotiations, hitherto unrevealed, was not less   \
                complete, from his own point of view, but it was afterward generally admitted that \
                although he obtained a distinct declaration from the Chinese of their              \
                irresponsibility for the acts of the savages, and of their acquiescence in the     \
                right of Japan to send a mission to regulate the affair independently, he was      \
                unfortunate in not requiring from the evasive and crafty officials a formal        \
                expression of this avowal, in writing."
            },
            p {
                "The absence of documentary evidence in these particulars was at a later date      \
                treacherously turned to the disadvantage of Japan, and it was only by the exercise \
                of great firmness and spirit that the Chinese were ultimately compelled to abide   \
                by the language they had used in these early discussions. The explanation of the   \
                omission to secure a permanent record of their declarations is simple. The fact of \
                their neither exercising nor claiming control over the savage region was so        \
                commonly recognized that nothing beyond a verbal allusion to it was regarded as    \
                essential."
            },
            p {
                "The official chart published by the Chinese government defines the district under \
                their jurisdiction as “bounded by mou", ins { "n" }, "tains in the rear”—the       \
                territory of the aborigines being thus excluded. All inquiries by persons          \
                interested in an explanation of the question had led to the same conclusions. Mr.  \
                Burlingame, while investigating the “Rover” affair, had discovered that the        \
                “savages were not Chinese, but outlaws of another race, who from time immemorial   \
                had been a sort of wrecking banditti.” To demand a written acknowledgment of what  \
                was accepted as an established truth appeared both unnecessary and injudicious;    \
                and it has since been placed beyond reasonable doubt that the introduction of this \
                issue as a disturbing element in later nego", ins { "t" }, "iations was a foreign  \
                inspiration, and was suggested by disingenuous, and, as the event proved,          \
                unskilful advisers of the Chinese councillors."
            },
            p {
                del { "..." }
            },
            p {
                "It was already evident that, by those who blindly opposed the movements of the    \
                Japanese, a strong point would be made of the assumption of Chinese authority,     \
                tardy as it was, over the whole island and people of Formosa. That this            \
                declaration was an after-thought, and a very late after-thought, there could be no \
                question. ", del { "... " }, "What more than this is needed to show the            \
                worthlessness of the sudden assumption of universal authority in Formosa, or to    \
                shatter the pretensions of those who endeavored to hold up the Japanese to obloquy \
                as the invaders of established and acknowledged Chinese rights?"
            },
            p {
                del { "..." }
            },
            p {
                "In illustration of the absence of necessity for written declarations of so patent \
                a fact as China's lack of authority over aboriginal Formosa, one of the members of \
                the first Japanese delegation subsequently employed this comparison:"
            },
            p {
                "“Until a recent time, the sovereigns of England called themselves kings of        \
                France, and included the lilies of that country in their arms. That was a far more \
                direct nominal claim than China ever assumed over Formosa. Yet an ambassador who   \
                should have asked for a written declaration that England had no jurisdiction over  \
                France would have been derided. A fact so universally known might indeed have been \
                touched upon in verbal discussions, but its documentary acknowledgement would      \
                never have been required.”"
            }
        }
    })
}

pub fn c_1851_05_xx_alcock_01() -> Element {
    let t01 = rsx!(Tip { tip: S("Keelung"), "Keilung" });
    rsx!(FileCitation {
        file: &BOOK_CHINESE_REPO_20,
        FigureAttribution {
            figure: &RUTHERFORD_ALCOCK,
            locale: [
                rsx!(span { "UK Consulate"}),
                rsx!(span { "Shanghai, China" }),
                rsx!(span { "circa May 1851" })
            ],
            p {
                "The Larpent, belonging to Mr. Thomas Ripley, left Liverpool for Shanghai on the   \
                18th May last year, in command of Captain Gibson."
            },
            p {
                "On the 12th September, (116 days out) at about 5 P.M., she was off Botel Tobago,  \
                a small island sixty miles east from the south end of Formosa, when she was put    \
                about and stood across to Formosa with a N.E. wind. The ship held on this tack     \
                until 20 minutes past 9 P.M., when she struck on the mainland of Formosa stem on,  \
                so close to land that the men could have got on shore from the flying jib-boom.    \
                When she struck she was going at the rate of 4 or 5 knots. The fourth mate, Mr.    \
                Bland, had the watch at the time; and he afterwards informed the men in the boat   \
                that he went aft to tell the Captain there was land ahead."
            },
            p {
                "From the survivors, who were in their hammocks, we learn that they were awoke by  \
                the striking of the ship, and on rushing on deck found everything in confusion.    \
                The watch ran to the braces, and backed the foreyard which sent her right off. It  \
                was however soon seen that she had experienced great damage, and was making water  \
                fast, and the crew was sent to the pumps. She had at this time ran a mile and      \
                a-half from the shore; the water however gained so fast on them, that leaving the  \
                pumps they commenced getting the boats out. The first got out was the jolly-boat,  \
                but she was immediately stove alongside. The launch and starboard quarter-boat (a  \
                life-boat) were afterwards got out, and into them were put provisions, a few       \
                cutlasses, and some powder, but no shot."
            },
            p {
                "The crew got into boats about 2 1/2 A.M., the Captain, first mate, and six men in \
                the life-boat; the second, third, and fourth mates, and twenty men in the launch.  \
                There was no sea, and they lay off to see the ship go down, which she did about    \
                3.20 A.M. by Captain Gibson's watch. At daybreak both boats made for the shore,    \
                and all hands landed. Shortly afterwards four of the inhabitants came down to the  \
                beach; they were not Chinese, but belonged to one of the aboriginal tribes. They   \
                tried to pilfer but were driven away with the cutlasses. The Captain, fearing      \
                hostility on the part of the natives, ordered the boats to be launched, and they   \
                then stood down the coast together until about 3 P.M., when the people in the      \
                launch hailed the Captain, and told him they could go no further, as the boat was  \
                making a great deal of water, and that it required eight men to bail her."
            },
            p {
                "He replied that they must do the best they could, that if they liked they might   \
                try and reach a Spanish settlement that lay eighty or 90 miles to the westward, or \
                Hongkong. They told him they could not venture in the state the boat was in. He    \
                then promised to stay by them until the boat was repaired; night came on, and the  \
                launch hove to, having, according to the mate's calculation, run about 94 miles;   \
                next morning the life-boat was not visible. The launch was then rowed ashore, and  \
                the crew landed near Sugar-loaf point, where they hauled the boat up, and set      \
                about repairing her and cooking provisions; while thus engaged they were fired     \
                upon with matchlocks from a neighboring wood—several were killed and wounded, 9    \
                took to the water, who were pursued by the natives in catamarans."
            },
            p {
                "The 2d mate, Mr. Griffits, not being being a good swimmer, made back for the      \
                land, but was attacked and his head cut off. Alexander Berries and George Harrison \
                kept together, and escaped to a rock where they remained two days without food or  \
                water. William Blake (carpenter) and James Hill (apprentice) escaped together in   \
                another direction. The two first, driven by hunger, and shortly afterwards         \
                encountered about fifty of their natives, who at first presented their matchlocks  \
                at them, but did not fire. Two women then gave them clothes to wrap round their    \
                loins, as they were naked; and an old man took them to his house. Three days       \
                afterwards, George Armstrong escaped on a catamaran to a Chinese sampan lying off  \
                the coast, but the men in her put him to death."
            },
            p {
                "Berries remained with his protector about four months, when a Chinaman who lived  \
                about 5 miles off bought him for six dollars. With this man, whose name was Kenah, \
                he remained until he was taken on board the Antelope. While with this man, Berries \
                learned that Blake and Hill had escaped to some Chinese village, and that some     \
                time after they were sent 8 miles to the interior, where Berries saw them while    \
                going with his master to a village called San Sianah. The master of Berries was    \
                willing to give him his liberty; but as the other men's master would not part with \
                them, they agreed to run with Berries to San Sianah, where they were hospitably    \
                received by the Mandarin.  Their master's wife followed to reclaim them, and the   \
                Mandarin paid her £14, the ransom she asked."
            },
            p {
                "Shortly afterwards the Antelope was off the coast, when the Mandarin sent his son \
                and four men in a boat to put them on board. Berries during his captivity made     \
                four or five attempts to get on board English ships; and once nearly succeeded in  \
                reaching the Flying Dutchman, but the wind getting up prevented him. Armstrong and \
                Hill learned that the master in the life-boat had put into the village where they  \
                were first captured, for the purpose of obtaining water, but none of them have     \
                ever heard of him since. None of the three men state that they saw all their       \
                comrades murdered, but they are the only survivors of the crew of the launch, as   \
                during their residence they picked up a sufficient knowledge of the language to    \
                understand what the natives said, and they never mentioned that there were any     \
                more saved. There seems to be no hope for the life-boat and her crew."
            }
        }
    })
}

pub fn c_1855_12_xx_habersham_01() -> Element {
    let t01 = rsx!(Tip { tip: S("Keelung"), "Keilung" });
    rsx!(FileCitation {
        file: &BOOK_HABERSHAM,
        FigureAttribution {
            figure: &ALEXANDER_HABERSHAM,
            locale: [
                rsx!(span { "USS ", i { "John Hancock" }}),
                rsx!(span { "East China Sea" }),
                rsx!(span { "circa Dec 1855" })
            ],
            p {
                "I will say nothing more about Formosa for the present. We left its shores about   \
                as wise as we were upon our arrival, and it was not until our second visit that    \
                we picked up what little information now exists upon the files of the Expedition   \
                in regard to it. Upon leaving ", { t01 }, " for Hong-Kong we kept along the east   \
                coast of the island, in the vain search for a reported harbour. There was nothing  \
                to be seen but an iron-bound coast with range after range of lofty mountains       \
                lifting themselves above the heavy surf that broke along the entire beach. One day \
                we thought we had discovered it: we saw ahead the smoke of distant villages rising \
                back of a bight in the coast which looked very much like a harbour; but, upon      \
                approaching it, we found ourselves mistaken. We, however, lowered a boat and       \
                attempted to land, but the surf was breaking so furiously that it would have been  \
                madness to have entered it. Besides, the beach was crowded by naked and excited    \
                savages, whom it was generally reported were cannibals, and into whose company we  \
                should consequently have preferred being thrown with reliable arms in our hands.   \
                The two convicts, whom the captain had taken in the boat to interpret in case of   \
                his being able to land, became so frightened at the savage appearance of those     \
                reported man-eaters, that they went on their knees to him, protesting, through     \
                the steward, that the islanders had eaten many of their countrymen, and that if he \
                went any nearer they would do the same by him and the boat's crew."
            },
            p {
              del { "..." }
            },
            p {
                "The setting sun looked upon us as we returned on board, and before he had again   \
                shone on those sloping greens we were well on our way around the south point of    \
                the island, in search of a landing among the savages in their own country. This, I \
                regret to say, we never found, the whole east coast being one continued line of    \
                foaming breakers, that carried death upon their rolling crests to every thing like \
                a boat. Where were the fine harbours of the Count de Benyowsky? The roaring of the \
                surf was our only answer. More than once, however, impelled by our excessive       \
                curiosity to learn more of these unknown people, did we attempt to land; and more  \
                exciting attempts at shore-going I never participated in. Upon one of these        \
                occasions we entered upon the dangerous trial with two of our best boats; but,     \
                upon nearly losing the inner one, with all who were in her, we wisely returned on  \
                board. We got more than one near view of the savages, however, heard their         \
                voices, and answered their signs; but all this only increased our desire to know   \
                more of them, for now we saw that they were veritable red men; and what were red   \
                men doing on the island of Formosa?"
            },
            p {
                "From what I could see over the distance which separated our boat from the crowded \
                beach, I found the previous description of our “innocent sportsman” substantiated  \
                by my own eyes and those of others. We saw an excited crowd of fine-looking men    \
                and women, copper-coloured, and possessed of the slightest possible amount of      \
                clothing,—the former boasting only a cloth tied around the head, while the latter  \
                had but a thin loose garment that seemed to gather around the throat and extended  \
                no farther than the knee. Some of the men were armed with bow and arrow, others    \
                with very serviceable-looking matchlocks; the women held various articles in their \
                hands, probably for barter, and, as we pulled away after our narrow escape, they   \
                evinced their sorrow and desire to trade by loud cries and the most violent        \
                gestures. Our Chinese boy had almost fainted from fright as the inner boat backed  \
                into the surf in the attempt to land: he could only tremble and cry out, “Dey eat  \
                man! dey eat man!” His friends on the other side had evidently impressed him with  \
                that unpleasant national characteristic, and hence his fright when apparently      \
                about to be rolled helplessly to their feet by a boiling surf."
            },
            p {
              del { "..." }
            },
            p {
                "We were surprised at this air of comfort among half-naked savages, and could not  \
                but wonder how they could have built such nice-looking houses, until we finally    \
                concluded that their prisoners had been made to turn their hands to masonry as     \
                well as gardening. Thus ended our second and last visit to Formosa, and all that   \
                we learned in regard to it may be condensed into a few words, viz:"
            },
            p {
                "We found it two hundred and five miles long by about sixty average width. It runs \
                N. by E. and S. by W., has a range of mountains running along its entire east      \
                coast, and is peopled by two different races of men—Chinese and red men. ",
                mark { "The former possess the north and west side of the island, the latter the   \
                east and south, and they exist in a state of constant hostility." }, "The country  \
                in the possession of the former is undulating or low, that of the latter rugged    \
                and mountainous. There are harbours on the north and west side, and none on the    \
                east. All else is conjecture. So much for Formosa and its mysterious red men."
            }
        }
    })
}

pub fn c_1898_05_12_pickering_01() -> Element {
    let t01 = rsx!(Tip { tip: S("Governor-General of Min-Zhe"), "viceroy" });
    let t02 = rsx!(Tip { tip: S("Fujian"), "Fuh-kien" });
    let t03 = rsx!(Tip { tip: S("Zhejiang"), "Cheh-kiang" });
    let t04 = rsx!(Tip { tip: S("Beijing"), "Pekin" });
    let t05 = rsx!(Tip { tip: S("Taiwanfoo"), "Tainan" });
    rsx!(FileCitation {
        file: &BOOK_PICKERING,
        FigureAttribution {
            figure: &WILLIAM_PICKERING,
            locale: [
                rsx!(span { ""}),
                rsx!(span { "" }),
                rsx!(span { "12th May 1898" })
            ],
            p {
                "The Chinese governed ", mark { "the portion of the island under their control" },
                " merely for the benefit of the officials, and in many parts of Formosa within the \
                Chinese pale anarchy prevailed for generations."
            },
            p {
              del { "..." }
            },
            p {
                "We now come to the Ch'i-hoan, or ‘raw savages’ of the high mountains. ", mark {
                "These people remained untouched by the semi-civilisation of the Chinese           \
                occupiers"}, " and they had never been visited in their haunts until 1865, when I  \
                succeeded in gaining access to those near Mount Morrison, whilst Mr. Dodd, a       \
                merchant of Tamsui, visited the tribes in the north of the island. Of my           \
                excursions into their territories, and of what befell me there, I will tell later."
            },
            p {
              del { "..." }
            },
            p {
                "Until a few years before ", del { "..." }, ", Formosa, together with the          \
                Pescadores Islands, was governed by a Tao-tai, or intendant of circuit, who        \
                resided at the capital of the island, Taiwanfoo. The Tao-tai was directly          \
                subordinate to the ", { t01 }, " of the two Chinese provinces ", { t02 }, " and ",
                { t03 }, ", but he had the power of life and death, and also possessed the special \
                privilege of communicating directly with the Emperor at ", { t04 }, ". He was the  \
                highest magistrate, and was bound to make an annual inspection of the departments. \
                It was believed that he was paid only 1,600 taels (not £6oo) by his imperial       \
                master, but his emoluments from many sources were very large; those drawn from    \
                the taxes on camphor especially reaching a fabulous amount."
            },
            p {
                "The next in civil authority was the Tai-wan-Fu, or prefect, and then the Hiens,   \
                or district magistrates."
            },
            p {
                "The Hiens had charge of the seven or eight hien, or districts, into which ", mark {
                "the part of the island under Chinese jurisdiction was divided" }, ". As I have    \
                before stated, ", mark { "the Chinese could only claim dominion over the plain on  \
                the west coast, and over one or two of the ranges of hills" }, ": the mountains in \
                the interior, and the South Cape, being still in the possession of tribes of       \
                aborigines and savages, all of whom were of totally different races and languages  \
                to the Chinese."
            },
            p {
              del { "..." }
            },
            p {
                "At rare intervals some general, desirous of gaining a reputation at Pekin, and of \
                testing the fighting quality of his troops, armed with European rifles, would      \
                organise an expedition into the mountains to subdue a savage tribe, and to extend  \
                the emperor’s dominion."
            },
            p {
                "Usually, however, these valiant onslaughts were fruitless and disastrous, the     \
                Chinese soldiers being either defeated from ambush or decimated by the deadly      \
                fever of the jungle."
            },
            p {
                "On one or two occasions a Chinese commander did indeed induce a savage tribe, by  \
                liberal gifts, to consent to ‘the tonsure,’ thus owning themselves vassals of the  \
                ‘Son of Heaven’ at Pekin."
            },
            p {
                "The general upon this sent out for a supply of razors, and proudly announced his  \
                great victory over the barbarians. When the shaving had been completed, he         \
                returned with all speed to a life of ease in the capital, ", { t05 },
                ", accompanied by the remains of his army."
            },
            p {
                "Alas for the dominion of the illustrious ‘Son of Heaven’! As soon as the gifts    \
                were consumed and their hair had grown, the savages, in almost every instance,     \
                repented them of their bargain and of their new civilisation; and, throwing off    \
                their allegiance, they returned to their familiar wild habits and customs."
            },
            p {
              del { "..." }
            },
            p {
                "A portion of this day's journey was through a wild and barren tract of country,   \
                but in the afternoon we arrived at a Hak-ka town, called Lam-tsng, situated at the \
                western foot of the last chain of mountains, which rise to a height of about 4,000 \
                feet, and which separate, as the Chinese express it, ", mark { "‘the dominion of   \
                men from the mountains of barbarism.’" }
            }
        }
    })
}

pub fn c_1857_02_10_nye_01() -> Element {
    rsx!(FileCitation {
        file: &US_C035_S02_REPORT_N0022,
        FigureAttribution {
            figure: &GIDEON_NYE_2,
            locale: [
                rsx!(span { "US Legation"}),
                rsx!(span { "Macao, China" }),
                rsx!(span { "10 Feb 1857" })
            ],
            p {
                "Sir: I hasten to beg your excellency's attention to the report of the loss of     \
                the New York clipper ship “Highflyer,” upon the southern part of the neighboring   \
                island of Formosa, with a hope that you may consider it in the light of the        \
                information already in the archives of the government, of sufficient importance to \
                justify your bringing it to the special notice of his excellency the naval         \
                commander-in-chief."
            },
            p {
                "It will be in the recollection of your excellency that I brought the subject of   \
                the state of that portion of Formosa, and the circumstances of the disappearance   \
                of several vessels when in the neighborhood thereof, to the notice of the          \
                government several years ago, with reference more especially to the circumstance   \
                of the disappearance of the clipper “Kelpie,” in 1848-9, with my brother,          \
                Thomas S. H. Nye, on board as a passenger."
            },
            p {
                "Subsequent occurences, and the presence of a large force at his excellency's      \
                command, induced me to bring the subject to the notice of Commodore Perry three    \
                years ago, who caused some search and inquiry to be made at Formosa; but the point \
                of real interest in this question, viz: the southeastern part of the island, was   \
                not examined."
            },
            p {
                "More recently I have been interested in a commercial enterprise with Mr. Robinet  \
                and Messrs. Williams, Anthon & Co., at Ape's Hill and other stations in Formosa,   \
                and the invariable account that I have received has been that ", mark { "the       \
                southeastern part of the island is in the possession of a mongrel race of great    \
                ferocity, with whom and the Chinese who inhabit only the western side of the       \
                island, there is constant hostility." }
            },
            p {
                "The investigations conducted by the orders of your excellency when chargé of the  \
                United States, and those more recently made by Commodore Perry's order, with such  \
                information as I am in a condition to afford Commodore Armstrong, will suffice, I  \
                am sure, to convince his excellency, in the first place, that, in the matter of    \
                these shipwrecks, he has only to deal with the residents of ", mark { "that part   \
                of the coast where the Chinese have never exercised jurisdiction" }, "; in the     \
                second place, that these residents or inhabitants are simply cruel, bloodthirsty   \
                savages, as little regardful of mercy as they are (from sheer brutal ignorance) of \
                the power of civilized governments; and hence, in the third place, that it is a    \
                duty to humanity and civilization to make an example of such of them as he can     \
                gain access to, after making fully known to them the reason for so doing. I would  \
                be glad if he took po", ins { "ss" }, "ession of that part of the island and held  \
                it, in the interests of humanity and commerce, for the benefit as well of China,",
                mark { "in respect to the only portion ever subject to her" }, ", as of all other  \
                nations having intercourse with this part of the world."
            }
        }
    })
}

pub fn c_1856_02_12_nye_01() -> Element {
    let t01 = rsx!(Tip { tip: S("Pingtan Island"), "Hainan" });
    let t02 = rsx!(Tip { tip: S("Shantou"), "Swatow" });
    rsx!(FileCitation {
        file: &NYDH_1857_04_18,
        FigureAttribution {
            figure: &GIDEON_NYE_2,
            locale: [
                rsx!(span { "Nye Bros. & Co."}),
                rsx!(span { "Canton, China" }),
                rsx!(span { "Feb 12th, 1856" })
            ],
            p {
                "David Ogden, Esq., New York:"
            },
            p {
                "Since writing the above we have heard, through Chinese, of a vessel at ", { t01 },
                ", dismasted and refitting, bound from California to Hong Kong, and we have strong \
                hopes that it proves to be the Highflyer. Yours, faithfully,"
            },
            p {
                style: "text-align-last: right;",
                "—Nye Bros. & Co."
            }
        },
        FigureAttribution {
            figure: &GIDEON_NYE_2,
            locale: [
                rsx!(span { "US Consulate"}),
                rsx!(span { "Macao, China" }),
                rsx!(span { "Apr 18th, 1857" })
            ],
            p {
                "David Ogden, Esq., New York:"
            },
            p {
                "Dear Sir: Referring to the former communications sent you upon the subject of     \
                your missing ship, the Highflyer, I now regret to add the report that a ship was   \
                lost upon the south part of Formosa in the autumn or winter of 1853, which I       \
                suppose we must accept as applicable to your vessel, as the period corresponds     \
                with her departure from California."
            },
            p {
                "I say I regret this inference, because I have reason, from previous researches at \
                that period, to believe that those of her passengers and crew who may have escaped \
                to the shore, will have been murdered by the savage clan—a race who inhabit the    \
                southeastern part of that island."
            },
            p {
                "I addressed H. E., the Plenipotentiary, upon the subject, with a view to bring it \
                to the notice of the naval commander-in-chief, on the day that I heard the report, \
                and I am hopeful that a vessel may soon be spared from China to proceed to the     \
                island to search for vestiges of the wreck, though the report received at Apes'    \
                Hill was that the ship was burnt. With much feeling of sympathy for those whose    \
                relations and friends have thus been lost, I remain, sir, yours, faithfully,"
            },
            p {
                style: "text-align-last: right;",
                "—Gideon Nye, Jr."
            }
        }
    })
}

pub fn c_1863_06_30_burlingame_01() -> Element {
    rsx!(FileCitation {
        file: &US_FRUS_1864_1864_P03,
        FigureAttribution {
            figure: &ANSON_BURLINGAME,
            locale: [
                rsx!(span { "US Legation" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { "Jun 30th, 1863" })
            ],
            p {
                "The captain reported the wreck to the prefect of Taiwan, and besought protection  \
                and help; but he moved so tardily, that before any officials reached the spot the  \
                villagers had carried off everything, to the loss of more than $20,000."
            }
        }
    })
}

pub fn c_1863_07_28_burlingame_01() -> Element {
    let t01 = rsx!(Tip { tip: P(&CHARLES_LEGENDRE), "The United States consul" });
    let t02 = rsx!(Tip { tip: S("modern-day Xiamen"), "Amoy" });
    let t03 = rsx!(Tip { tip: S("modern-day Taiwan"), "Formosa" });
    let t04 = rsx!(Tip { tip: S("modern-day Tainan"), ins { "the city of " }, "Taiwan" });
    let t05 = rsx!(Tip { tip: S("modern-day Budai"), "Pu-tai-tsin" });
    let t06 = rsx!(Tip { tip: S("Chiayi County"), "Kia-i" });

    rsx!(FileCitation {
        file: &US_FRUS_1864_1864_P03,
        FigureAttribution {
            figure: &ANSON_BURLINGAME,
            locale: [
                rsx!(span { "U.S. Legation" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { "Jul 28th, 1863" })
            ],
            p {
                { t01 }, " at ", { t02 }, " has informed me that the American ship Iskanderia was  \
                lost on ", { t03 }, ", December 27, 1861, on her way from ", { t04 }, " to Amoy,   \
                at a place called ", { t05 }, ", in the district of ", { t06 }, ", distant about   \
                ten miles from the city of Taiwan. The ship was half embedded in the sand when the \
                villagers robbed her of everything, but did not wound the crew."
            },
            p {
                "The captain reported the wreck to the prefect of Taiwan, and besought protection  \
                and help; but he moved so tardily, that before any officials reached the spot the  \
                villagers had carried off everything, to the loss of more than $20,000."
            }
        }
    })
}

pub fn c_1863_07_28_burlingame_02() -> Element {
    let t01 = rsx!(Tip { tip: S("modern-day Taiwan"), "Formosa" });
    let t02 = rsx!(Tip { tip: S("perhaps modern-day Tamsui"), "Tanshwin" });
    let t03 = rsx!(Tip { tip: S("Keelung"), "Kilung" });

    rsx!(FileCitation {
        file: &US_FRUS_1864_1864_P03,
        FigureAttribution {
            figure: &ANSON_BURLINGAME,
            locale: [
                rsx!(span { "U.S. Legation" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { "Jul 28th, 1863" })
            ],
            p {
                "I have also learned that the American ship Lucky Star, on her passage from        \
                Shanghai to Hong-Kong, went ashore on ", { t01 }, ", last November, between ",
                { t02 }, " and ", { t03 }, "."
            },
            p {
                "When she was seen, more than two thousand natives arrived with knives and spears, \
                assembled on the beach to watch her, and when the captain with his wife and son    \
                reached the shore with a boat’s crew, they were all robbed of their clothes, the   \
                female of her ornaments, and thus stripped, carried several miles into the         \
                interior and held for a ransom of a thousand dollars."
            },
            p {
                "The other sailors made their way to Tanshwai, and reported these proceedings to   \
                the officers, who declined to act on the instant. The foreigners there learning    \
                the state of the case, collected a party of men, and went to the place to bring    \
                away all the party, but they found that the ship and her cargo of cotton, valued   \
                at $80,000, had been entirely plundered."
            }
        }
    })
}

pub fn c_1874_12_23_shen_01() -> Element {
    rsx!(FileCitation {
        file: &CN_THDS_V09_P181,
        FigureAttribution {
            figure: &SHEN_BAOZHEN,
            locale: [
                rsx!(span { "Forbidden City" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { "Dec 23th, 1874" })
            ],
            h1 {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "請移駐巡撫摺",
                    rt {
                        lang: "en",
                        "Memorial Requesting the Transfer of the Provincial Governor"
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "奏為台地善後，勢當漸圖；",
                    rt {
                        lang: "en",
                        "This memorial concerns the reformation of Taiwan, which must be gradually planned;"
                    }
                },
                ruby {
                    lang: "cn",
                    "番境開荒，事關創始。",
                    rt {
                        lang: "en",
                        "the cultivation of aboriginal territories is a foundational task."
                    }
                },
                ruby {
                    lang: "cn",
                    "請旨移駐巡撫，以專責成，以經久遠事。",
                    rt {
                        lang: "en",
                        "I request an imperial edict to transfer the Governor here, to take full   \
                        charge and manage longterm affairs."
                    }
                },
                ruby {
                    lang: "cn",
                    "竊臣等於十月二十七日將收回草房營地各情形奏明在案；",
                    rt {
                        lang: "en",
                        "I humbly submitted previously a memorial on the twenty-seventh day of the \
                        tenth month regarding the recovery of the thatched cottage garrison area;"
                    }
                },
                ruby {
                    lang: "cn",
                    "因思洋務稍鬆，即善後不容稍緩。",
                    rt {
                        lang: "en",
                        "considering that foreign affairs have slightly relaxed, the reformation   \
                        cannot be delayed."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "惟此次之善後，與往時不同。",
                    rt {
                        lang: "en",
                        "However, this reformation is different from those of the past."
                    }
                },
                ruby {
                    lang: "cn",
                    "台地之所謂善後，即台地之所謂創始也；",
                    rt {
                        lang: "en",
                        "What is called “reformation” in Taiwan is actually what should be called  \
                        “formation” in Taiwan;"
                    }
                },
                ruby {
                    lang: "cn",
                    "善後難，以創始為善後則尤難。",
                    rt {
                        lang: "en",
                        "reformations are difficult, but beginning from scratch during a           \
                        reformation is even more difficult."
                    }
                },
                ruby {
                    lang: "cn",
                    "臣等曩為海防孔亟，一面撫番，一面開路，以絕彼族覬覦之心，以消目前肘腋之患；",
                    rt {
                        lang: "en",
                        "Previously, I humbly and urgently focused on coastal defense, while       \
                        simultaneously pacifying the aborigines and building roads, in order to    \
                        eliminate covetous intentions and to resolve the immediate threats;"
                    }
                },
                ruby {
                    lang: "cn",
                    "固未遑為經久之謀。",
                    rt {
                        lang: "en",
                        "indeed we had no time to make long-term plans."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "數月以來，南北諸路，縋幽鑿險，斬棘披荊，雖各著成效；",
                    rt {
                        lang: "en",
                        "Over these past months, on all roads north and south, we have descended   \
                        into remote areas and carved through dangerous terrain, clearing thorns    \
                        and thickets, and though each has shown results;"
                    }
                },
                ruby {
                    lang: "cn",
                    "卑南、歧萊各處，雖分列軍屯，祗有端倪，尚無綱紀。",
                    rt {
                        lang: "en",
                        "at places like Beinan and Qilai, although military settlements have been  \
                        established separately, there are only beginnings, still no systematic order."
                    }
                },
                ruby {
                    lang: "cn",
                    "若不從此悉心籌畫，詳定規模，路非不已開也，謂一開之不復塞，則不敢知；",
                    rt {
                        lang: "en",
                        "If we do not carefully plan from here and establish detailed frameworks,  \
                        it is not that roads have not been opened, but we dare not know if once    \
                        opened they will not become blocked again;"
                    }
                },
                ruby {
                    lang: "cn",
                    "番非不已撫也，謂一撫之不復疑，則不敢必。",
                    rt {
                        lang: "en",
                        "it is not that the aborigines have not been pacified, but we dare not be  \
                        certain that once pacified they will not become suspicious again."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "何則？",
                    rt {
                        lang: "en",
                        "Why is this so?"
                    }
                },
                mark {
                    ruby {
                        lang: "cn",
                        "台地延袤千有餘里，官吏所治祗濱海平原三分之一，余皆番社耳。",
                        rt {
                            lang: "en",
                            "Taiwan extends for over a thousand li, and the officials only govern  \
                            one-third of the coastal plains, while the rest are all aboriginal     \
                            villages."
                        }
                    }
                },
                ruby {
                    lang: "cn",
                    "國家並育番黎，但令薄輸土貢，永禁侵陵，意至厚也。",
                    rt {
                        lang: "en",
                        "The state nurtures both aborigines and common people, only requiring      \
                        light tribute of local products and permanently prohibiting encroachment:  \
                        these intentions are most generous."
                    }
                },
                ruby {
                    lang: "cn",
                    "而奸民積匪，久已越界潛蹤，驅番佔地，而成窟穴，則有官未開而民先開者；",
                    rt {
                        lang: "en",
                        "But dishonest people and accumulated bandits have long crossed boundaries \
                        in secret, driving away aborigines and occupying land, making hideouts:    \
                        these are cases where officials have not yet broken ground but civilians    \
                        have done so prior;"
                    }
                },
                ruby {
                    lang: "cn",
                    "入山既深，入跡罕到，野番穴處，涵育孳生，則有番已開而民未開者；",
                    rt {
                        lang: "en",
                        "and where the mountains go deep and human traces rarely reach, wild       \
                        aborigines dwell in caves and multiply: these are cases where aborigines   \
                        have developed areas but civilians have not yet done so;"
                    }
                },
                ruby {
                    lang: "cn",
                    "疊巘外包，平埔中擴，鹿豕游竄，草木蒙茸，地廣番稀，棄而弗處，則有民未開而番亦未開者。",
                    rt {
                        lang: "en",
                        "and layered peaks encircle the outside, the plains and foothills expand   \
                        in the middle, deer and boars roam freely, vegetation grows thick, the     \
                        land is vast but aborigines are sparse, abandoned and uninhabited: these   \
                        are cases where neither civilians nor aborigines have developed the areas."
                    }
                },
                ruby {
                    lang: "cn",
                    "是但言開山，而山之不同已若此。",
                    rt {
                        lang: "en",
                        "So just speaking of opening up the mountains, the mountains' conditions   \
                        already differ this much."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "雖然此第言後山耳，其繁重已若此。",
                    rt {
                        lang: "en",
                        "However, this only speaks of the area behind the mountains, yet the       \
                        complexity is already this severe."
                    }
                },
                mark {
                    ruby {
                        lang: "cn",
                        "山前之入版圖也，百有餘年，一切規制何嘗具備。",
                        rt {
                            lang: "en",
                            "The area before the mountains have been part of our territory for     \
                            over a hundred years, yet when have all the regulations ever been implemented?"
                        }
                    }
                },
                ruby {
                    lang: "cn",
                    "就目前之積弊而論，",
                    rt {
                        lang: "en",
                        "Speaking of current accumulated problems: "
                    }
                },
                ruby {
                    lang: "cn",
                    "班兵之惰窳也、",
                    rt {
                        lang: "en",
                        "the garrison soldiers are lazy and corrupt,"
                    }
                },
                ruby {
                    lang: "cn",
                    "蠹役之盤役也、",
                    rt {
                        lang: "en",
                        "the petty officials are exploitative,"
                    }
                },
                ruby {
                    lang: "cn",
                    "土匪之橫恣也、",
                    rt {
                        lang: "en",
                        "the local bandits are unrestrained,"
                    }
                },
                ruby {
                    lang: "cn",
                    "民俗之慆淫也、",
                    rt {
                        lang: "en",
                        "the folk customs are depraved,"
                    }
                },
                ruby {
                    lang: "cn",
                    "海防陸守之俱虛也、",
                    rt {
                        lang: "en",
                        "both sea and land defenses are empty,"
                    }
                },
                ruby {
                    lang: "cn",
                    "械鬥扎厝之迭見也。",
                    rt {
                        lang: "en",
                        "and the frequent occurrence of armed conflicts and house sieges."
                    }
                },
                ruby {
                    lang: "cn",
                    "學術之不明，庠序以容豪猾；",
                    rt {
                        lang: "en",
                        " Learning is not promoted, and schools have become havens for the crafty  \
                        and powerful;"
                    }
                },
                ruby {
                    lang: "cn",
                    "禁令之不守，煙賭以為饔飧。",
                    rt {
                        lang: "en",
                        "prohibitions are not enforced, and opium smoking and gambling have become \
                        as common as eating meals."
                    }
                },
                ruby {
                    lang: "cn",
                    "官斯土者，",
                    rt {
                        lang: "en",
                        "Among the officials of this land,"
                    }
                },
                ruby {
                    lang: "cn",
                    "非無振作有為、",
                    rt {
                        lang: "en",
                        "it iss not that there are not those who are enterprising and capable,"
                    }
                },
                ruby {
                    lang: "cn",
                    "正己率屬之員，",
                    rt {
                        lang: "en",
                        "who rectify themselves and lead their subordinates,"
                    }
                },
                ruby {
                    lang: "cn",
                    "始苦於事權之牽制，",
                    rt {
                        lang: "en",
                        "but they first suffer from restrictions on their authority,"
                    }
                },
                ruby {
                    lang: "cn",
                    "繼苦於毀譽之混淆，",
                    rt {
                        lang: "en",
                        "then suffer from confused mix of praise and criticism,"
                    }
                },
                ruby {
                    lang: "cn",
                    "救過不遑，",
                    rt {
                        lang: "en",
                        "have have no time to correct mistakes…"
                    }
                },
                ruby {
                    lang: "cn",
                    "計功何自？",
                    rt {
                        lang: "en",
                        "so how can they even begin to accomplish anything?"
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } }
            }
        }
    })
}

pub fn c_1875_01_12_shen_01() -> Element {
    let t01 = rsx!(Tip {
        tip: S("1st year of Emperor Guangxu (1875)\n12th lunar month (beg. 28 Dec 1875)\n1st day of the month (28 Dec 1875)"),
        ruby {
            lang: "cn",
            "竊臣等於十二月初一日，",
            rt {
                lang: "en",
                "On the 1st day of the 12th month,"
            }
        }
    });
    let t02 = rsx!(Tip {
        tip: S("1st year of Emperor Guangxu (1875)\n10th lunar month (beg. 29 Oct 1875)\n23rd day of the month (21 Nov 1875)"),
        ruby {
            lang: "cn",
            "本年十月二十三日上諭：",
            rt {
                lang: "en",
                "the Imperial Edict dated the 23rd day of the 10th month of this year."
            }
        }
    });
    rsx!(FileCitation {
        file: &CN_THDS_V09_P181,
        FigureAttribution {
            figure: &SHEN_BAOZHEN,
            locale: [
                rsx!(span { "Forbidden City" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { "Jan 12th, 1875" })
            ],
            h1 {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "臺地後山請開舊禁摺",
                    rt {
                        lang: "en",
                        "Memorial Requesting That the Old Restrictions Be Lifted from Taiwan's   \
                        Back Mountain"
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "奏為臺地後山急須耕墾，請開舊禁，以杜訛索而廣招徠；",
                    rt {
                        lang: "en",
                        "This memorial concerns the urgent need to cultivate Taiwan's eastern      \
                        territory, hereby requesting the lifting of old prohibitions to prevent    \
                        extortion and broadly attract settlers;"
                    }
                },
                ruby {
                    lang: "cn",
                    "恭摺馳陳，仰祈聖鑑事。",
                    rt {
                        lang: "en",
                        "I humbly submit this memorial for Your Majesty's sage consideration."
                    }
                },
                { t01 },
                ruby {
                    lang: "cn",
                    "業將南北路開通及擬將琅𤩝、旗後等處佈置各情形奏明在案。",
                    rt {
                        lang: "en",
                        "I had already submitted a memorial regarding the opening of north-south   \
                        roads and the proposed arrangements for Hengchun, Cihou, and other areas."
                    }
                },
                ruby {
                    lang: "cn","是日，奉到",
                    rt {
                        lang: "en",
                        "On that day, I received"
                    }
                },
                { t02 },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "十二月初四日，復奉到本年十一月十三日上諭：",
                    rt {
                        lang: "en",
                        "On the fourth day of the twelfth month, we again received the Imperial    \
                        Edict dated the thirteenth day of the eleventh month of this year."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "臣等伏讀之下，仰見聖謨遠大，欽感莫名。",
                    rt {
                        lang: "en",
                        "Upon reading these edicts, I see Your Majesty's far-reaching wisdom and   \
                        feel boundless gratitude."
                    }
                },
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "因思全臺後山除番社外，無非曠土。",
                    rt {
                        lang: "en",
                        "I consider that all of Taiwan's eastern territory, except for         \
                        indigenous villages, is nothing but wilderness."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "蓋臺灣地廣人稀，山前一帶雖經蕃息百有餘年，戶口尚未充牣。",
                    rt {
                        lang: "en",
                        "For Taiwan's territory is vast but sparsely populated; although the       \
                        western plains have been settled for over a hundred years, the population  \
                        is still not sufficient."
                    }
                },
                mark {
                    ruby {
                        lang: "cn",
                        "內地人民向來不准偷渡，",
                        rt {
                            lang: "en",
                            "Mainland people have long been prohibited from illegal crossing,"
                        }
                    }
                },
                ruby {
                    lang: "cn",
                    "近雖文法稍弛，",
                    rt {
                        lang: "en",
                        "and although regulations have recently loosened somewhat, "
                    }
                },
                mark {
                    ruby {
                        lang: "cn",
                        "而開禁未有明文，",
                        rt {
                            lang: "en",
                            "there is no explicit policy allowing entry,"
                        }
                    }
                },
                ruby {
                    lang: "cn",
                    "地方官思設法招徠，每恐與例不合。",
                    rt {
                        lang: "en",
                        "so local officials who wish to recruit settlers fear it may violate       \
                        regulations."
                    }
                },
                ruby {
                    lang: "cn",
                    "今欲開山不先招墾，則路雖通而仍塞；",
                    rt {
                        lang: "en",
                        "If we wish to develop the mountains without first recruiting settlers,    \
                        then even though roads are open they remain blocked;"
                    }
                },
                ruby {
                    lang: "cn",
                    "欲招墾不先開禁，則民裹足而不前。",
                    rt {
                        lang: "en",
                        "if we wish to recruit settlers without first lifting prohibitions, then   \
                        people will hold back and not advance."
                    }
                },
                ruby {
                    lang: "cn",
                    "臣等查舊例稱：",
                    rt {
                        lang: "en",
                        "We have examined the old regulations which state:"
                    }
                }
            },
            p {
                class: "chinese-text",
                ol {
                    li {
                        mark {
                            ruby {
                                lang: "cn",
                                "臺灣不准內地人民偷渡；",
                                rt {
                                    lang: "en",
                                    "Taiwan does not allow mainland people to cross illegally;"
                                }
                            }
                        },
                        ruby {
                            lang: "cn",
                            "拏獲偷渡船隻，將船戶等分別治罪，文武官議處兵役治罪。",
                            rt {
                                lang: "en",
                                "when illegal crossing vessels are caught, the vessel owners are   \
                                to be punished accordingly, and civil and military officials are   \
                                to deliberate on punishments for soldiers involved."
                            }
                        }
                    },
                    li {
                        ruby {
                            lang: "cn",
                            "又",
                            rt {
                                lang: "en",
                                "Furthermore,"
                            }
                        },
                        ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                        ruby {
                            lang: "cn",
                            "如有充作客頭，在沿海地方引誘偷渡之人，為首者充軍，為從者杖一百、徒三年；",
                            rt {
                                lang: "en",
                                "If there are those who act as migrant leaders and entice people   \
                                along the coast to cross illegally, the ringleaders shall be       \
                                exiled to serve in the army, accomplices shall receive 100 strokes \
                                and three years imprisonment;"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "互保之船戶及歇寓知情容隱者杖一百、枷一個月；",
                            rt {
                                lang: "en",
                                "The vessel owners who provide mutual guarantees and those who     \
                                knowingly harbor illegal crossers shall receive 100 strokes and    \
                                wear the cangue for one month;"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "偷渡之人杖八十，遞回原籍；",
                            rt {
                                lang: "en",
                                "Those who cross illegally shall receive 80 strokes and be sent    \
                                back to their original residence;"
                            },
                        },
                        ruby {
                            lang: "cn",
                            "文武失察者，分別議處。",
                            rt {
                                lang: "en",
                                "civil and military officials who fail to detect such cases shall  \
                                be dealt with accordingly."
                            },
                        }
                    },
                    li {
                        ruby {
                            lang: "cn",
                            "又內地商人置貨過臺，由原籍給照；",
                            rt {
                                lang: "en",
                                "Furthermore, mainland merchants transporting goods to Taiwan must \
                                obtain permits from their place of origin;"
                            },
                        },
                        ruby {
                            lang: "cn",
                            "如不及回籍，則由廈防廳查明取保給照；",
                            rt {
                                lang: "en",
                                "if unable to return home, they may obtain permits from the Xiamen \
                                Defense Office after investigation and guarantee;"
                            },
                        },
                        ruby {
                            lang: "cn",
                            "該廳濫給，降三級調用。",
                            rt {
                                lang: "en",
                                "if that office issues permits liberally, they shall be demoted    \
                                three ranks."
                            },
                        },
                    },
                    li {
                        ruby {
                        lang: "cn",
                            "又沿海村鎮有引誘客民過臺數至三十人以上者，壯者新疆為奴，老者煙瘴充軍。",
                            rt {
                                lang: "en",
                                "Furthermore, if coastal villages and towns entice more than       \
                                thirty migrants to cross to Taiwan, the able-bodied shall be made  \
                                slaves in Xinjiang, while the elderly shall be exiled to serve in  \
                                malarial regions."
                            }
                        }
                    },
                    li {
                        ruby {
                            lang: "cn",
                            "又內地人民往臺者，地方官給照盤驗出口；",
                            rt {
                                lang: "en",
                                "Furthermore, mainland people going to Taiwan must obtain permits  \
                                from local officials who inspect exits;"
                            },
                        },
                        ruby {
                            lang: "cn",
                            "濫給者，分別次數罰俸降調。",
                            rt {
                                lang: "en",
                                "officials who issue permits liberally shall be fined and demoted  \
                                according to the number of occurrences."
                            }
                        }
                    },
                    li {
                        ruby {
                            lang: "cn",
                            "又無照民人過臺，失察之口岸官照人數分別降調；",
                            rt {
                                lang: "en",
                                "Furthermore, for people crossing to Taiwan without permits, port  \
                                officials who fail to detect them shall be demoted according to    \
                                the number of people;"
                            },
                        },
                        ruby {
                            lang: "cn",
                            "隱匿者革職。",
                            rt {
                                lang: "en",
                                "those who conceal such cases shall be dismissed from office."
                            }
                        }
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "以上六條，皆嚴禁內地民人渡臺之舊例也。",
                    rt {
                        lang: "en",
                        "The above six articles are all old regulations strictly prohibiting       \
                        mainland people from crossing to Taiwan."
                    }
                },
                ruby {
                    lang: "cn",
                    "又稱：",
                    rt {
                        lang: "en",
                        "Furthermore, it states: "
                    }
                }
            },
            p {
                class: "chinese-text",
                ol {
                    li {
                        ruby {
                            lang: "cn",
                            "凡民人私入番境者杖一百；",
                            rt {
                                lang: "en",
                                "Any civilian who privately enters indigenous territory shall  \
                                receive 100 strokes;"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "拏獲如在近番處所抽藤、",
                            rt {
                                lang: "en",
                                "those who are apprehended near indigenous areas collecting rattan,"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "釣鹿、",
                            rt {
                                lang: "en",
                                "hunt deer,"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "伐木、",
                            rt {
                                lang: "en",
                                "cut timber,"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "採棕者杖一百、徒三年。",
                            rt {
                                lang: "en",
                                "or harvest palm near shall receive 100 strokes and three years    \
                                imprisonment."
                            }
                        }
                    },
                    li {
                        ruby {
                            lang: "cn",
                            "又臺灣南勢、北勢一帶，山口勒石分為番界；",
                            rt {
                                lang: "en",
                                "Furthermore, along Taiwan's southern and northern regions, stone  \
                                markers at mountain passes mark the indigenous boundaries;"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "如有偷越運貨者，失察之專管官降調，該管上司罰俸一年。",
                            rt {
                                lang: "en",
                                "if anyone smuggles goods across, the officials who fail to detect \
                                it shall be demoted, and their superiors shall be fined one year's \
                                salary."
                            }
                        }
                    },
                    li {
                        ruby {
                            lang: "cn",
                            "又臺地民人不得與番民結親，",
                            rt {
                                lang: "en",
                                "Furthermore, Taiwan's civilians are not permitted to marry        \
                                indigenous people,"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "違者離異、治罪，地方官參處；",
                            rt {
                                lang: "en",
                                "and violators shall be separated and punished, with local         \
                                officials held responsible;"
                            }
                        },
                        ruby {
                            lang: "cn",
                            "從前已娶者，毋許往來番社，違者治罪。",
                            rt {
                                lang: "en",
                                "those previously married are not allowed to visit indigenous      \
                                villages, violators shall be punished."
                            }
                        }
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "以上三條，皆嚴禁臺民私入番界之舊例也。",
                    rt {
                        lang: "en",
                        "The above three articles are all old regulations strictly prohibiting     \
                        Taiwanese civilians from entering indigenous territories."
                    }
                },
                ruby {
                    lang: "cn",
                    "際此開山伊始、招墾方興，臣等揆度時勢，合無仰懇天恩，將一切舊禁盡與開豁，以廣招徠，俾無瞻顧。",
                    rt {
                        lang: "en",
                        "As we break ground in the developing of this island, I, considering the   \
                        circumstances of the times, humbly request Your Majesty's grace to         \
                        completely lift all old prohibitions to broadly attract settlers and leave \
                        them without hesitation."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "臣等思當茲開闢後山，凡百以便民為急，不得不因時變通；",
                    rt {
                        lang: "en",
                        "We consider that in developing the eastern territory, facilitating        \
                        people's lives is urgent and we must adapt to the times;"
                    }
                },
                ruby {
                    lang: "cn",
                    "合無再懇天恩，飭地方官將鐵、竹兩項悉弛舊禁，以斷胥役勒索之路，以濟閭閻日用之需。",
                    rt {
                        lang: "en",
                        "I humbly request Your Majesty's grace again to order local officials to   \
                        completely lift the old prohibitions on iron and bamboo, to cut off the    \
                        path of clerks' and runners' extortion, and to aid common people's daily   \
                        needs."
                    }
                }
            }
        }
    })
}

pub fn c_1875_08_28_shen_01() -> Element {
    rsx!(FileCitation {
        file: &CN_THDS_V04_P070,
        FigureAttribution {
            figure: &SHEN_BAOZHEN,
            locale: [
                rsx!(span { "Forbidden City" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { "Aug 28th, 1875" })
            ],
            h1 {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "會籌全臺大局疏",
                    rt {
                        lang: "en",
                        "Memorial on Planning the Overall Situation of Taiwan"
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "福州將軍臣文煜、",
                    rt {
                        lang: "en",
                        "General of Fuzhou Chen Wenyu,"
                    }
                },
                ruby {
                    lang: "cn",
                    "閩浙總督臣李鶴年、",
                    rt {
                        lang: "en",
                        "Governor-General of Min-Zhe Li Henian,"
                    }
                },
                ruby {
                    lang: "cn",
                    "福建巡撫臣王凱泰、",
                    rt {
                        lang: "en",
                        "Provincial Governor of Fujian Wang Kaitai,"
                    }
                },
                ruby {
                    lang: "cn",
                    "辦理臺灣等處海防兼理各國事務臣沈葆楨跪奏，為會籌全臺大局，",
                    rt {
                        lang: "en",
                        "and Taiwan's Maritime Defense and Foreign Affairs Commissioner Shen       \
                        Baozhen humbly submit this memorial regarding the overall planning for     \
                        Taiwan, "
                    }
                },
                ruby {
                    lang: "cn",
                    "撫番開路，",
                    rt {
                        lang: "en",
                        "such as pacifying aboriginals and opening roads,"
                    }
                },
                ruby {
                    lang: "cn",
                    "勢難中止，",
                    rt {
                        lang: "en",
                        "matters which cannot be halted,"
                    }
                },
                ruby {
                    lang: "cn",
                    "並巡撫兼顧省臺情形，恭折馳陳，仰祈聖鑒事。",
                    rt {
                        lang: "en",
                        "and the situation of the Provincial Governor overseeing both the province \
                        of Fujian and Taiwan, respectfully presenting this for Imperial            \
                        consideration."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "惟臺地自去年倭人啟釁，",
                    rt {
                        lang: "en",
                        "Since last year when the Japanese initiated conflict,"
                    }
                },
                ruby {
                    lang: "cn",
                    "外假復仇，內圖占地，",
                    rt {
                        lang: "en",
                        "outwardly claiming revenge while inwardly plotting to occupy territory,"
                    }
                },
                ruby {
                    lang: "cn",
                    "狡謀已露，逆焰方張，",
                    rt {
                        lang: "en",
                        "their devious schemes have been exposed and their rebellious intentions   \
                        are expanding,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不得已而有撫番開路之舉。",
                    rt {
                        lang: "en",
                        "which has made the pacification of aboriginals and road construction      \
                        unavoidable."
                    }
                },
                ruby {
                    lang: "cn",
                    "當時固謂海防未固，則外侮難消，",
                    rt {
                        lang: "en",
                        "At that time, it was said that without secure coastal defenses, external  \
                        threats cannot be eliminated;"
                    }
                },
                ruby {
                    lang: "cn",
                    "山險未通，則海防先無從下手。",
                    rt {
                        lang: "en",
                        "but without access through the mountains, coastal defense cannot even     \
                        begin."
                    }
                },
                ruby {
                    lang: "cn",
                    "前山各口消息，尚能探悉，島岸尚可周知，",
                    rt {
                        lang: "en",
                        "while we can gather intelligence about the entrances in the western       \
                        mountains and know about the coastal conditions,"
                    }
                },
                ruby {
                    lang: "cn",
                    "後山則途徑不通，人跡罕到。",
                    rt {
                        lang: "en",
                        "the eastern mountains remain inaccessible with rarely any human presence."
                    }
                },
                ruby {
                    lang: "cn",
                    "但謀前山拒虎，一任後門進狼，",
                    rt {
                        lang: "en",
                        "If we only plan to resist the tiger at the front mountains while allowing \
                        wolves to enter through the back door,"
                    }
                },
                ruby {
                    lang: "cn",
                    "雖日事籌防，而防務究無把握。",
                    rt {
                        lang: "en",
                        "even though we plan defenses daily, our defensive measures will never be  \
                        reliable."
                    }
                },
                ruby {
                    lang: "cn",
                    "雖日事籌防，而防務究無把握。",
                    rt {
                        lang: "en",
                        "even though we plan defenses daily, our defensive measures will never be  \
                        reliable."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "人第知今日開山之為撫番，固不知今日撫番之實以防海也。",
                    rt {
                        lang: "en",
                        "People only know that today's mountain development is for pacifying       \
                        aboriginals, but they do n0t realize that today's aboriginal pacification  \
                        is actually for coastal defense."
                    }
                },
                ruby {
                    lang: "cn",
                    "第知豫籌防海之關係臺灣安危，而不知豫籌防海之關係南北洋全局也。",
                    rt {
                        lang: "en",
                        "They only know that planning coastal defense relates to Taiwan's          \
                        security, but do not realize that planning coastal defense relates to the  \
                        overall situation of both the Northern and Southern seas."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "倭事雖已順平，而各路之師，至今不可撤。",
                    rt {
                        lang: "en",
                        "Although the Japanese incident has been settled, the troops stationed     \
                        along various routes still cannot be withdrawn."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "不知臣等之經營後山者為防患計，非為興利計；",
                    rt {
                        lang: "en",
                        "They don't understand that our development of the eastern    \
                        mountains is a plan to prevent future troubles, not a plan for profit;"
                    }
                },
                ruby {
                    lang: "cn",
                    "為興利盡可緩圖，為防患必難中止。",
                    rt {
                        lang: "en",
                        "while profit-making can be postponed, preventing future troubles cannot   \
                        be halted."
                    }
                },
                ruby {
                    lang: "cn",
                    "外人之垂涎臺地，非一日亦非一國也。",
                    rt {
                        lang: "en",
                        "Foreigners' covetousness of Taiwan's territory is neither recent nor      \
                        limited to one country."
                    }
                },
                ruby {
                    lang: "cn",
                    "去歲倭事，特嚆矢耳。",
                    rt {
                        lang: "en",
                        "Last year's Japanese incident was merely the beginning."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "自法郎西據安南，",
                    rt {
                        lang: "en",
                        "Since France has occupied Annam,"
                    }
                },
                ruby {
                    lang: "cn",
                    "英吉利據印度、新加坡等處，",
                    rt {
                        lang: "en",
                        "and England has occupied India, Singapore and other places,"
                    }
                },
                ruby {
                    lang: "cn",
                    "南洋各國漸為所收，",
                    rt {
                        lang: "en",
                        "various countries in the South Seas have gradually been taken,"
                    }
                },
                ruby {
                    lang: "cn",
                    "遂使遠隔數萬里之豺狼，得以近吾臥榻。",
                    rt {
                        lang: "en",
                        "allowing wolves from tens of thousands of miles away to approach our      \
                        bedside."
                    }
                },
                ruby {
                    lang: "cn",
                    "以臺地閩左屏藩，七省門戶，天氣和暖，年榖易成",
                    rt {
                        lang: "en",
                        "Taiwan serves as Fujian's left flank defense and gateway to seven         \
                        provinces, with warm weather and easily grown annual crops."
                    }
                },
                mark {
                    ruby {
                        lang: "cn",
                        "後山一帶，我不盡收版圖，",
                        rt {
                            lang: "en",
                            "If we don't fully incorporate the eastern mountain region into our    \
                            territory,"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "彼必陰謀侵占。",
                        rt {
                            lang: "en",
                            "they will surely plot to occupy it."
                        }
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "後山一去，前山何可復守！",
                    rt {
                        lang: "en",
                        "If we lose the eastern mountains, how can we defend the western mountains!"
                    }
                },
                ruby {
                    lang: "cn",
                    "臺地者，中土之藩籬也。",
                    rt {
                        lang: "en",
                        "Taiwan is the fence protecting China proper."
                    }
                },
                ruby {
                    lang: "cn",
                    "藩籬既撤，",
                    rt {
                        lang: "en",
                        "If this fence is removed,"
                    }
                },
                ruby {
                    lang: "cn",
                    "則蛇蠍之毒，將由背膂而入我腹心。",
                    rt {
                        lang: "en",
                        "then the poison of snakes and scorpions will enter our core through our   \
                        back."
                    }
                }
            }
        }
    })
}

pub fn c_1877_01_xx_shen_01() -> Element {
    let t01 = rsx!(Tip {
        tip: S("2nd year of Emperor Guangxu (1876)\n11th lunar month (beg. 16 Dec 1876)\n25th day of the month (10 Jan 1877)"),
        ruby {
            lang: "cn",
            "竊於本年十一月二十五日，",
            rt {
                lang: "en",
                "On the 25th day of the 11th month of this year,"
            }
        }
    });
    rsx!(FileCitation {
        file: &CN_THDS_V04_P070,
        FigureAttribution {
            figure: &SHEN_BAOZHEN,
            locale: [
                rsx!(span { "Forbidden City" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { i {"circa"}, " Jan 1877" })
            ],
            h1 {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "籌商臺灣事宜疏",
                    rt {
                        lang: "en",
                        "A Memorial on Deliberating Taiwan Affairs"
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "頭品頂帶兩江總督臣沈葆楨跪奏，為遵旨籌商臺灣事宜，恭折具陳，仰祈聖鑒事。",
                    rt {
                        lang: "en",
                        "I, Shen Baozhen, First-Rank Official and Governor-General of Liangjiang,  \
                        respectfully submit this memorial in accordance with the imperial edict    \
                        regarding the deliberation of Taiwan affairs, humbly presenting this for   \
                        Your Majesty's consideration:"
                    }
                },
                { t01 },
                ruby {
                    lang: "cn",
                    "承准軍機大臣密寄，光緒二年十一月十九日奉上諭，",
                    rt {
                        lang: "en",
                        "I received a confidential dispatch from the Grand Council containing an   \
                        Imperial Edict dated the 19th day of the 11th month of the 2nd year of     \
                        Guangxu,"
                    }
                },
                ruby {
                    lang: "cn",
                    "丁日昌奏臺灣亟應統籌全局，省臺勢難兼顧情形，著該督等速議具奏等因。",
                    rt {
                        lang: "en",
                        "stating that Ding Richang had memorialized about the urgent need to plan  \
                        comprehensively for Taiwan, and the difficulty of managing both provincial \
                        and Taiwan affairs simultaneously, ordering governors to quickly           \
                        deliberate and report. "
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "欽此。",
                    rt {
                        lang: "en",
                        "By Imperial Command:"
                    }
                },
                ruby {
                    lang: "cn",
                    "查原奏所稱購船、練兵、炮臺、電線、開礦、招墾諸務，皆臣在臺時先後條奏，緣絀於經費，限於時日， \
                    或奏焉而未及舉，或舉焉而未及成者。",
                    rt {
                        lang: "en",
                        "Regarding the matters mentioned in the original memorial (purchasing      \
                        ships, training troops, building fortifications, installing telegraph      \
                        lines, opening mines, and recruiting settlers), I had previously submitted \
                        memorials about all of these during my time in Taiwan, although, due to    \
                        limited funds and time constraints, some matters were proposed but not     \
                        implemented, while others were started but not completed."
                    }
                },
                ruby {
                    lang: "cn",
                    "惟鐵路一端，當時未經議及；而實為臺地所宜行。",
                    rt {
                        lang: "en",
                        "However, the matter of railways was not discussed at that time—though it  \
                        is actually quite suitable for implementation in Taiwan."
                    }
                }
            },
            p {
                class: "chinese-text",
                mark {
                    ruby {
                        lang: "cn",
                        "其云不出數年，日本必出全力以圖規取者，誠洞見症結，綢繆未雨之苦衷，非故為危言聳聽者也。",
                        rt {
                            lang: "en",
                            "The warning that within a few years Japan will certainly exert all    \
                            its effort to attempt to take Taiwan shows true insight into the core  \
                            problem and represents prudent preparation before the storm; it is     \
                            not merely alarmist talk."
                        }
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "然則鐵甲水雷，宜盡北洋先辦，而臺灣眼前，不得不姑且從緩，惟招墾則必不可緩。",
                    rt {
                        lang: "en",
                        "Therefore, ironclads and torpedoes should be prioritized for the          \
                        Northern Fleet, while such matters in Taiwan must be temporarily           \
                        postponed. However, the recruitment of settlers cannot be delayed."
                    }
                },
                ruby {
                    lang: "cn",
                    "蓋以杜內地之大害，興全臺之大利，計無逾於此者。",
                    rt {
                        lang: "en",
                        "For preventing major problems in the interior and promoting the greatest  \
                        benefits for all of Taiwan, no plan surpasses this one."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "臺灣之吏治營政，若不認真整頓，則目前之利藪，皆後日之亂階。",
                    rt {
                        lang: "en",
                        "If Taiwan's administrative and military affairs are not seriously put in  \
                        order, then today's sources of profit will all become tomorrow's seeds of  \
                        chaos."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "愚昧之見，是否有當，理合恭折密陳，伏乞聖鑒訓示。",
                    rt {
                        lang: "en",
                        "Whether these humble views are appropriate or not, I respectfully submit  \
                        this confidential memorial and humbly await Your Majesty's sage review and \
                        instructions."
                    }
                }
            }
        }
    })
}

pub fn c_1875_02_24_ding_01() -> Element {
    let t01 = rsx!(Tip {
        tip: P(&LI_HONGZHANG),
        ruby {
            lang: "cn",
            "李鴻章",
            rt {
                lang: "en",
                "Li Hongzhang"
            }
        }
    });
    let t02 = rsx!(Tip {
        tip: P(&CHARLES_LEGENDRE),
        ruby {
            lang: "cn",
            "黎展達",
            rt {
                lang: "en",
                "Charles LeGendre"
            }
        }
    });
    rsx!(FileCitation {
        file: &CN_THDS_V04_P070,
        FigureAttribution {
            figure: &DING_RICHANG,
            locale: [
                rsx!(span { "Forbidden City" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { "Feb 24th, 1875" })
            ],
            h1 {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "代陳丁日昌議覆海防事宜疏",
                    rt {
                        lang: "en",
                        "Memorial Submitting Deliberations by Ding Richang on Coastal Defense      \
                        Affairs"
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "大學士直隸總督一等伯臣",
                    rt {
                        lang: "en",
                        "Grand Secretary, Governor-General of Zhili, First Class Earl, your        \
                        minister"
                    }
                },
                { t01 },
                ruby {
                    lang: "cn",
                    "跪奏，",
                    rt {
                        lang: "en",
                        "kneels to memorialize,"
                    }
                },
                ruby {
                    lang: "cn",
                    "為據情代陳，",
                    rt {
                        lang: "en",
                        "to present facts on behalf of another,"
                    }
                },
                ruby {
                    lang: "cn",
                    "仰祈聖鑒事。",
                    rt {
                        lang: "en",
                        "and humbly requests Imperial consideration."
                    }
                },
                ruby {
                    lang: "cn",
                    "竊上年九月間，欽奉上諭總理各國事務衙門奏，海防亟宜切籌，",
                    rt {
                        lang: "en",
                        "In the ninth month of last year, I humbly received an Imperial Edict in   \
                        response to the memorial from the Zongli Yamen, stating that coastal       \
                        defense must be urgently planned,"
                    }
                },
                ruby {
                    lang: "cn",
                    "所陳練兵、",
                    rt {
                        lang: "en",
                        "and the proposed items regarding training troops,"
                    }
                },
                ruby {
                    lang: "cn",
                    "簡器、",
                    rt {
                        lang: "en",
                        "selecting wares,"
                    }
                },
                ruby {
                    lang: "cn",
                    "造船、",
                    rt {
                        lang: "en",
                        "building ships,"
                    }
                },
                ruby {
                    lang: "cn",
                    "籌餉、",
                    rt {
                        lang: "en",
                        "raising funds,"
                    }
                },
                ruby {
                    lang: "cn",
                    "用人、",
                    rt {
                        lang: "en",
                        "employing personnel,"
                    }
                },
                ruby {
                    lang: "cn",
                    "持久各條，",
                    rt {
                        lang: "en",
                        "and maintaining sustainability,"
                    }
                },
                ruby {
                    lang: "cn",
                    "均係緊要機宜，",
                    rt {
                        lang: "en",
                        "each being equally crucial matters,"
                    }
                },
                ruby {
                    lang: "cn",
                    "著詳細籌議辦法覆奏。",
                    rt {
                        lang: "en",
                        "of which detailed implementation plans should be deliberated and reported \
                        back."
                    }
                },
                ruby {
                    lang: "cn",
                    "此外別有要計，亦即一並奏陳等因。",
                    rt {
                        lang: "en",
                        "If there are other important strategies, they should also be reported together."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "欽此。",
                    rt {
                        lang: "en",
                        "By Imperial Command:"
                    }
                },
                ruby {
                    lang: "cn",
                    "臣當即依限詳議覆奏在案。",
                    rt {
                        lang: "en",
                        "Your subject immediately submitted a detailed memorial in response within the prescribed time limit."
                    }
                },
                ruby {
                    lang: "cn",
                    "因思前江蘇巡撫臣丁日昌，",
                    rt {
                        lang: "en",
                        "Considering that the former Governor of Jiangsu, your subject Ding Richang,"
                    }
                },
                ruby {
                    lang: "cn",
                    "隨辦洋務有年，",
                    rt {
                        lang: "en",
                        "has handled foreign affairs for many years,"
                    }
                },
                ruby {
                    lang: "cn",
                    "熟悉機宜，",
                    rt {
                        lang: "en",
                        "is familiar with proper procedures,"
                    }
                },
                ruby {
                    lang: "cn",
                    "究心時事，",
                    rt {
                        lang: "en",
                        "and studies current affairs thoroughly,"
                    }
                },
                ruby {
                    lang: "cn",
                    "曾密鈔總理衙門原奏六條，函屬該前撫妥議切實辦法，",
                    rt {
                        lang: "en",
                        "I had privately copied the six articles from the Zongli Yamen's original  \
                        memorial and wrote to request that the former governor carefully           \
                        deliberate on practical methods,"
                    }
                },
                ruby {
                    lang: "cn",
                    "以為集思廣益之助。",
                    rt {
                        lang: "en",
                        "to gather our collective wisdom."
                    }
                },
                ruby {
                    lang: "cn",
                    "茲接丁日昌寄呈逐條議覆折稿，",
                    rt {
                        lang: "en",
                        "Now I have received Ding Richang's draft memorial discussing each article \
                        in detail,"
                    }
                },
                ruby {
                    lang: "cn",
                    "臣思整頓海防為軍國要政，",
                    rt {
                        lang: "en",
                        "I humbly believe that organizing coastal defense is a crucial military    \
                        and state matter, "
                    }
                },
                ruby {
                    lang: "cn",
                    "該前撫既有所見，未便壅於上聞，",
                    rt {
                        lang: "en",
                        "and since the former governor has insights, it would not be proper to     \
                        withhold them from Imperial knowledge,"
                    }
                },
                ruby {
                    lang: "cn",
                    "謹照繕丁日昌議覆六條清單，恭呈御覽。",
                    rt {
                        lang: "en",
                        "and so I respectfully present a clean copy of Ding Richang's responses to \
                        the six articles for Your Majesty's review:"
                    }
                }
            },
            p {
                class: "chinese-text",
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "日本彈丸小島，",
                    rt {
                        lang: "en",
                        "Japan, though merely a small island nation,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不過夜郎靡莫之倫，",
                    rt {
                        lang: "en",
                        "no different from the ancient kingdoms of Yelang and Mimo,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而年來發憤為雄，",
                    rt {
                        lang: "en",
                        "has in recent years become ambitious and powerful,"
                    }
                },
                ruby {
                    lang: "cn",
                    "變更峨冠博帶之舊習，",
                    rt {
                        lang: "en",
                        "abandoning their old customs of tall hats and broad sashes,"
                    }
                },
                ruby {
                    lang: "cn",
                    "師法輪船飛礮之新製，",
                    rt {
                        lang: "en",
                        "and learning to make new technologies like steamships and artillery."
                    }
                },
                ruby {
                    lang: "cn",
                    "其陰而有謀，固屬可慮，",
                    rt {
                        lang: "en",
                        "Their secretive scheming is certainly worrisome,"
                    }
                },
                ruby {
                    lang: "cn",
                    "其窮而無賴，則更可憂。",
                    rt {
                        lang: "en",
                        "and their desperate unscrupulousness is even more concerning."
                    }
                },
                ruby {
                    lang: "cn",
                    "以北境之塞希倫地予俄，",
                    rt {
                        lang: "en",
                        "They have given Russia the northern territory of Sakhalin,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而日俄之交固，",
                    rt {
                        lang: "en",
                        "thereby securing Japanese-Russian relations,"
                    }
                },
                ruby {
                    lang: "cn",
                    "用",
                    rt {
                        lang: "en",
                        "and employed"
                    }
                },
                ruby {
                    lang: "cn",
                    "李太國",
                    rt {
                        lang: "en",
                        "Horatio Lay"
                    }
                },
                ruby {
                    lang: "cn",
                    "開火車鐵路，",
                    rt {
                        lang: "en",
                        "to build railways,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而多借英國之債，",
                    rt {
                        lang: "en",
                        "while borrowing heavily from Britain,"
                    }
                },
                ruby {
                    lang: "cn",
                    "其國主常見英使",
                    rt {
                        lang: "en",
                        "and their emperor frequently meets with the British minister"
                    }
                },
                ruby {
                    lang: "cn",
                    "巴夏禮",
                    rt {
                        lang: "en",
                        "Harry Parkes"
                    }
                },
                ruby {
                    lang: "cn",
                    "與之潛謀密計，",
                    rt {
                        lang: "en",
                        "for secret consultations,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而日英之交固。",
                    rt {
                        lang: "en",
                        "thus securing Japanese-British relations."
                    }
                },
                ruby {
                    lang: "cn",
                    "用",
                    rt {
                        lang: "en",
                        "They employed"
                    }
                },
                { t02 },
                ruby {
                    lang: "cn",
                    "密查臺灣情形，",
                    rt {
                        lang: "en",
                        "to secretly investigate conditions in Taiwan,"
                    }
                },
                ruby {
                    lang: "cn",
                    "資為指臂腹心，",
                    rt {
                        lang: "en",
                        "using this as strategic intelligence,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而日美之交固。",
                    rt {
                        lang: "en",
                        "thus securing Japanese-American relations."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "彼其低首下心，伈伈睍睍，",
                    rt {
                        lang: "en",
                        "Their bowing and scraping, appearing meek and fearful,"
                    }
                },
                ruby {
                    lang: "cn",
                    "以求悅於各國者，",
                    rt {
                        lang: "en",
                        "appearing so humble and eager to please foreign powers,"
                    }
                },
                ruby {
                    lang: "cn",
                    "豈有他哉？",
                    rt {
                        lang: "en",
                        "what other purpose could this serve?"
                    }
                },
                ruby {
                    lang: "cn",
                    "蓋其覬覦臺灣，",
                    rt {
                        lang: "en",
                        "They conceal their lust for Taiwan,"
                    }
                },
                ruby {
                    lang: "cn",
                    "已寢食寤寐之不忘，",
                    rt {
                        lang: "en",
                        "while they think of it day and night;"
                    }
                },
                mark {
                    ruby {
                        lang: "cn",
                        "中國倘棄之如遺，",
                        rt {
                            lang: "en",
                            "should China abandon it carelessly,"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "固已從心所欲，",
                        rt {
                            lang: "en",
                            "then they would achieve their desire;"
                        }
                    }
                },
                ruby {
                    lang: "cn",
                    "萬一勢出於戰，",
                    rt {
                        lang: "en",
                        "and should it come to war,"
                    }
                },
                ruby {
                    lang: "cn",
                    "則有交慝各國，",
                    rt {
                        lang: "en",
                        "then their allies would mediate on their behalf;"
                    }
                },
                ruby {
                    lang: "cn",
                    "為之解鈴說合，",
                    rt {
                        lang: "en",
                        "as such we may initiate a conflict,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不致能發而不能收。",
                    rt {
                        lang: "en",
                        "but may not control its resolution."
                    }
                },
                ruby {
                    lang: "cn",
                    "此其所以肆然無忌，",
                    rt {
                        lang: "en",
                        "This is why they act so brazenly and unrestrained,"
                    }
                },
                ruby {
                    lang: "cn",
                    "快志於一逞也。",
                    rt {
                        lang: "en",
                        "eager to satisfy their ambitions."
                    }
                },
                ruby {
                    lang: "cn",
                    "臣任苏藩司时，",
                    rt {
                        lang: "en",
                        "When I served as Provincial Governor of Jiangsu,"
                    }
                },
                ruby {
                    lang: "cn",
                    "曾於議覆修約條內，",
                    rt {
                        lang: "en",
                        "I stated in my response then regarding treaty provisions:"
                    }
                },
                ruby {
                    lang: "cn",
                    "陳明日本陰柔而有遠志，",
                    rt {
                        lang: "en",
                        "Japan—though appearing gentle—harbors long-term ambitions;"
                    }
                },
                ruby {
                    lang: "cn",
                    "中國所買槍礮，",
                    rt {
                        lang: "en",
                        "the rifles and canons purchased by China,"
                    }
                },
                ruby {
                    lang: "cn",
                    "皆彼國選餘之物，",
                    rt {
                        lang: "en",
                        "Are all merely their discarded items;"
                    }
                },
                ruby {
                    lang: "cn",
                    "宜陽與之好，",
                    rt {
                        lang: "en",
                        "and so we should maintain positive relations,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而陰為之備。",
                    rt {
                        lang: "en",
                        "while secretly preparing against them."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "其時李鴻章深以臣言為然，",
                    rt {
                        lang: "en",
                        "At that time, Li Hongzhang seriously took my words for truth,"
                    }
                },
                ruby {
                    lang: "cn",
                    "當即代為密陳。",
                    rt {
                        lang: "en",
                        "and promptly submitted a secret memorial on my behalf."
                    }
                },
                ruby {
                    lang: "cn",
                    "今日本雖小有所償，",
                    rt {
                        lang: "en",
                        "Although Japan is now receiving some compensation,"
                    }
                },
                ruby {
                    lang: "cn",
                    "然彼之所費，",
                    rt {
                        lang: "en",
                        "compare their actual expenses,"
                    }
                },
                ruby {
                    lang: "cn",
                    "既不啻十倍此數，",
                    rt {
                        lang: "en",
                        "which is already no less than ten times the figure,"
                    }
                },
                ruby {
                    lang: "cn",
                    "况死于是役者，",
                    rt {
                        lang: "en",
                        "moreover, those who have died, by way of conscription,"
                    }
                },
                ruby {
                    lang: "cn",
                    "複五六百人，",
                    rt {
                        lang: "en",
                        "number five or six hundred men;"
                    }
                },
                ruby {
                    lang: "cn",
                    "萬一他日複藉端發難，",
                    rt {
                        lang: "en",
                        "thus they may one day create further trouble,"
                    }
                },
                ruby {
                    lang: "cn",
                    "以數舶橫亙於黃海黑水洋之間，",
                    rt {
                        lang: "en",
                        "such as positioning several ships across the length of the Yellow Sea,"
                    }
                },
                ruby {
                    lang: "cn",
                    "則津滬之氣不通，",
                    rt {
                        lang: "en",
                        "thereby blocking communication between Tianjin and Shanghai,"
                    }
                },
                ruby {
                    lang: "cn",
                    "事事為之棘手；",
                    rt {
                        lang: "en",
                        "and everything will become difficult to handle;"
                    }
                },
                ruby {
                    lang: "cn",
                    "而臺灣之患，",
                    rt {
                        lang: "en",
                        "The danger of Taiwan,"
                    }
                },
                ruby {
                    lang: "cn",
                    "猶其小而緩者也。",
                    rt {
                        lang: "en",
                        "in comparison, would be relatively minor."
                    }
                }
            },
            p {
                class: "chinese-text",
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "以上六條，皆就總理衙門原奏，",
                    rt {
                        lang: "en",
                        "The above six points all elaborate on and expand upon the original        \
                        memorial from the Zongli Yamen,"
                    }
                },
                ruby {
                    lang: "cn",
                    "略申餘蘊，附呈管見。",
                    rt {
                        lang: "en",
                        "to which I have humbly attached my personal views."
                    }
                },
                ruby {
                    lang: "cn",
                    "是否有當？",
                    rt {
                        lang: "en",
                        "Are they appropriate?"
                    }
                },
                ruby {
                    lang: "cn",
                    "伏乞聖鑒。",
                    rt {
                        lang: "en",
                        "I respectfully await Your Majesty's enlightened judgment."
                    }
                }
            }
        }
    })
}

pub fn c_1876_xx_xx_ding_01() -> Element {
    let t01 = rsx!(Tip {
        tip: P(&SHEN_BAOZHEN),
        ruby {
            lang: "cn",
            "沈葆楨",
            rt {
                lang: "en",
                "Shen Baozhen"
            }
        }
    });
    rsx!(FileCitation {
        file: &CN_THDS_V04_P070,
        FigureAttribution {
            figure: &DING_RICHANG,
            locale: [
                rsx!(span { "Forbidden City" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { i { "circa" }, " 1876" })
            ],
            h1 {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "請速籌臺事全局疏",
                    rt {
                        lang: "en",
                        "Memorial Requesting Swift Planning for the Overall Taiwan Situation"
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "福建巡撫臣丁日昌跪奏，",
                    rt {
                        lang: "en",
                        "Your humble servant Ding Richang, Governor of Fujian, kneels to submit    \
                        this memorial,"
                    }
                },
                ruby {
                    lang: "cn",
                    "為臺事速宜統籌全局，",
                    rt {
                        lang: "en",
                        "regarding the urgent need to plan comprehensively for Taiwan's affairs,"
                    }
                },
                ruby {
                    lang: "cn",
                    "恭折密陈，仰祈圣鉴事。",
                    rt {
                        lang: "en",
                        "which I humbly present in confidence for Your Majesty's consideration:"
                    }
                },
                ruby {
                    lang: "cn",
                    "竊查臺灣生番蠢動，",
                    rt {
                        lang: "en",
                        "Upon investigation, while the aboriginal peoples of Taiwan are provoking  \
                        unrest,"
                    }
                },
                ruby {
                    lang: "cn",
                    "尚是癬疥之疾，",
                    rt {
                        lang: "en",
                        "this is but a minor infliction akin to ringworm or scabies,"
                    }
                },
                ruby {
                    lang: "cn",
                    "惟日本處心積慮，",
                    rt {
                        lang: "en",
                        "however, Japan has been harboring ulterior motives,"
                    }
                },
                ruby {
                    lang: "cn",
                    "極意窺伺，",
                    rt {
                        lang: "en",
                        "while lying in wait and anticipation,"
                    }
                },
                ruby {
                    lang: "cn",
                    "傳聞近日有屯兵琉球之說，",
                    rt {
                        lang: "en",
                        "and there is rumor of Japan stationing troops in the Ryukyus,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而德國亦常密遣兵船，",
                    rt {
                        lang: "en",
                        "as well as the secret dispatch of a German vessel,"
                    }
                },
                ruby {
                    lang: "cn",
                    "前往臺北，",
                    rt {
                        lang: "en",
                        "bound for Taipei,"
                    }
                },
                ruby {
                    lang: "cn",
                    "測繪地圖。",
                    rt {
                        lang: "en",
                        "for the purpose of survey."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "查琉球距臺北雞籠，",
                    rt {
                        lang: "en",
                        "Investigating further, from Ryukyu to Keelung in northern Taiwan,"
                    }
                },
                ruby {
                    lang: "cn",
                    "水程不過千里，",
                    rt {
                        lang: "en",
                        "the sea journey is no more than a thousand li,"
                    }
                },
                ruby {
                    lang: "cn",
                    "朝發可以夕至，",
                    rt {
                        lang: "en",
                        "one can depart in the morning and arrive by evening,"
                    }
                },
                ruby {
                    lang: "cn",
                    "該國弱小而貧，",
                    rt {
                        lang: "en",
                        "the Ryukyus are weak and poor,"
                    }
                },
                ruby {
                    lang: "cn",
                    "數百年來，為中國不侵不叛之臣。",
                    rt {
                        lang: "en",
                        "for hundreds of years it has been a loyal vassal to China, neither        \
                        invading nor rebelling."
                    }
                },
                ruby {
                    lang: "cn",
                    "其入貢也，",
                    rt {
                        lang: "en",
                        "Regarding their tribute missions,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不惟表其恭謹，",
                    rt {
                        lang: "en",
                        "not only do they demonstrate their respect and carefulness,"
                    }
                },
                ruby {
                    lang: "cn",
                    "即販賣土貨，",
                    rt {
                        lang: "en",
                        "even in selling local products,"
                    }
                },
                ruby {
                    lang: "cn",
                    "亦藉以稍得微利。",
                    rt {
                        lang: "en",
                        "but they also only seek modest profits."
                    }
                },
                ruby {
                    lang: "cn",
                    "聞今年貢物已具，",
                    rt {
                        lang: "en",
                        "I have heard that this year's tribute goods are already prepared,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而日本強之不令賷行。",
                    rt {
                        lang: "en",
                        "but Japan has forcefully prevented them from proceeding with the mission."
                    }
                },
                ruby {
                    lang: "cn",
                    "外則以示桀驁，",
                    rt {
                        lang: "en",
                        "Outwardly, this shows their arrogance,"
                    }
                },
                ruby {
                    lang: "cn",
                    "實則懼琉球密以情偽相告，",
                    rt {
                        lang: "en",
                        "but in reality, they fear Ryukyu might secretly inform us of their true   \
                        intentions,"
                    }
                },
                ruby {
                    lang: "cn",
                    "居心叵測，",
                    rt {
                        lang: "en",
                        "their motives are suspicious,"
                    }
                },
                ruby {
                    lang: "cn",
                    "可恨亦復可憂。",
                    rt {
                        lang: "en",
                        "this is both hateful and worrisome."
                    }
                }
            },
            p {
                class: "chinese-text",
                { t01 },
                ruby {
                    lang: "cn",
                    "前因倭兵屯紮琅嶠，",
                    rt {
                        lang: "en",
                        "previously dealt with Japanese troops stationed in Hengchun,"
                    }
                },
                ruby {
                    lang: "cn",
                    "是以經營僅在臺南一帶。",
                    rt {
                        lang: "en",
                        "and his administrative focus was limited to the Tainan area."
                    }
                },
                ruby {
                    lang: "cn",
                    "其實臺灣精華所聚，",
                    rt {
                        lang: "en",
                        "In truth, the essence of Taiwan's value,"
                    }
                },
                ruby {
                    lang: "cn",
                    "全局在臺北、淡水、雞籠等處，",
                    rt {
                        lang: "en",
                        "the strategic entirety lies in Taipei, Tamsui, Keelung and other such     \
                        places,"
                    }
                },
                ruby {
                    lang: "cn",
                    "而外人心目所注，亦在臺北、淡水、雞籠。",
                    rt {
                        lang: "en",
                        "and foreigners also have their eyes set on Taipei, Tamsui, and Keelung."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "從前榛狉未闢，",
                    rt {
                        lang: "en",
                        "In the past, when the wilderness was yet untamed,"
                    }
                },
                ruby {
                    lang: "cn",
                    "習與相忘，",
                    rt {
                        lang: "en",
                        "and we were accustomed to ignoring each other,"
                    }
                },
                ruby {
                    lang: "cn",
                    "近則天主、耶穌等教，訌入內山，",
                    rt {
                        lang: "en",
                        "but recently, Catholic and Protestant religions have penetrated into the  \
                        mountains,"
                    }
                },
                ruby {
                    lang: "cn",
                    "一切利源以及險阻，無不深知。",
                    rt {
                        lang: "en",
                        "and all the resources and strategic positions are known to them."
                    }
                },
                mark {
                    ruby {
                        lang: "cn",
                        "是以彼族所繪臺地圖說，",
                        rt {
                            lang: "en",
                            "Thus the maps of Taiwan drawn by these foreigners,"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "較之官繪者尤為詳盡，",
                        rt {
                            lang: "en",
                            "are even more detailed than those drawn by our officials,"
                        }
                    }
                },
                ruby {
                    lang: "cn",
                    "而臺屬各口，兵船林立，",
                    rt {
                        lang: "en",
                        "and warships stand like forests at all ports of Taiwan,"
                    }
                },
                ruby {
                    lang: "cn",
                    "潮來汐往，",
                    rt {
                        lang: "en",
                        "coming and going with the tides,"
                    }
                },
                ruby {
                    lang: "cn",
                    "無日無之。",
                    rt {
                        lang: "en",
                        "no day gone without them."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "年來彼族，",
                    rt {
                        lang: "en",
                        "In recent years, these foreigners,"
                    }
                },
                ruby {
                    lang: "cn",
                    "無論要求何事，",
                    rt {
                        lang: "en",
                        "no matter what they demand,"
                    }
                },
                ruby {
                    lang: "cn",
                    "動輒以兵船相恫喝，",
                    rt {
                        lang: "en",
                        "frequently threaten us with their warships,"
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "惟臺灣勢同孤注，",
                    rt {
                        lang: "en",
                        "and Taiwan stands as our single outpost,"
                    }
                },
                ruby {
                    lang: "cn",
                    "如果兵力有餘，",
                    rt {
                        lang: "en",
                        "if we have surplus military force,"
                    }
                },
                ruby {
                    lang: "cn",
                    "則遇彼族用武挾制之時，",
                    rt {
                        lang: "en",
                        "then when these foreigners use military force to coerce us,"
                    }
                },
                ruby {
                    lang: "cn",
                    "自可由臺出奇兵，",
                    rt {
                        lang: "en",
                        "we can deploy surprise forces from Taiwan,"
                    }
                },
                ruby {
                    lang: "cn",
                    "斷其後路，",
                    rt {
                        lang: "en",
                        "thereby cutting off their retreat,"
                    }
                },
                ruby {
                    lang: "cn",
                    "為擊首應尾之計，",
                    rt {
                        lang: "en",
                        "implementing a strategy of striking both head and tail,"
                    }
                },
                ruby {
                    lang: "cn",
                    "令彼族多所瞻顧，",
                    rt {
                        lang: "en",
                        "making them look over their shoulders in multiple directions,"
                    }
                },
                ruby {
                    lang: "cn",
                    "似更諸事易於轉圜。",
                    rt {
                        lang: "en",
                        "which would make various matters easier to negotiate."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "同治十三年冬，",
                    rt {
                        lang: "en",
                        "In the winter of 1874,"
                    }
                },
                ruby {
                    lang: "cn",
                    "總理衙門原議練兵制器，",
                    rt {
                        lang: "en",
                        "the Zongli Yamen originally discussed training troops and manufacturing   \
                        weapons,"
                    }
                },
                ruby {
                    lang: "cn",
                    "以備海防之用。",
                    rt {
                        lang: "en",
                        "for the purpose of coastal defense."
                    }
                },
                ruby {
                    lang: "cn",
                    "蓋亦深慮臺灣有關東南大局，",
                    rt {
                        lang: "en",
                        "This was indeed due to deep concern about Taiwan's importance to the      \
                        greater southeastern situation,"
                    }
                },
                ruby {
                    lang: "cn",
                    "因而為未雨綢繆之計。",
                    rt {
                        lang: "en",
                        "thus being prepared in advance."
                    }
                },
                ruby {
                    lang: "cn",
                    "以臣愚見，",
                    rt {
                        lang: "en",
                        "In my humble opinion,"
                    }
                },
                ruby {
                    lang: "cn",
                    "臺灣若不認真整頓，",
                    rt {
                        lang: "en",
                        "if Taiwan is not seriously put in order,"
                    }
                },
                ruby {
                    lang: "cn",
                    "速籌備御之方，",
                    rt {
                        lang: "en",
                        "and defensive measures are not quickly prepared,"
                    }
                },
                mark {
                    ruby {
                        lang: "cn",
                        "不出數年，日本必出全力以圖規取，",
                        rt {
                            lang: "en",
                            "within but a few years Japan will certainly use all its power to      \
                            attempt to take it,"
                        }
                    }
                },
                ruby {
                    lang: "cn",
                    "其時恐不止如前時尚能以言語退敵也。",
                    rt {
                        lang: "en",
                        "and at that time, I fear we will no longer be able to repel enemies with  \
                        mere words as we did before."
                    }
                },
                ruby {
                    lang: "cn",
                    "臺中琅嶠之役，",
                    rt {
                        lang: "en",
                        "During the Hengchun incident in central Taiwan,"
                    }
                },
                ruby {
                    lang: "cn",
                    "沿海各省，",
                    rt {
                        lang: "en",
                        "the coastal provinces,"
                    }
                },
                ruby {
                    lang: "cn",
                    "舉辦海防，",
                    rt {
                        lang: "en",
                        "in establishing coastal defenses,"
                    }
                },
                ruby {
                    lang: "cn",
                    "用費殆將千萬，",
                    rt {
                        lang: "en",
                        "spent nearly ten million,"
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
                ruby {
                    lang: "cn",
                    "惟臺灣有備，",
                    rt {
                        lang: "en",
                        "However, if Taiwan is prepared,"
                    }
                },
                ruby {
                    lang: "cn",
                    "沿海可以無憂；",
                    rt {
                        lang: "en",
                        "then the coast can be free from worry;"
                    }
                },
                ruby {
                    lang: "cn",
                    "臺灣不安，",
                    rt {
                        lang: "en",
                        "but if Taiwan is not secure,"
                    }
                },
                ruby {
                    lang: "cn",
                    "則全局殆為震動。",
                    rt {
                        lang: "en",
                        "then the entire situation would be shaken."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } }
            }
        }
    })
}

pub fn c_1877_xx_xx_ding_01() -> Element {
    let t01 = rsx!(Tip {
        tip: S("2nd year of Emperor Guangxu (1876)\n11th lunar month (beg. 16 Dec 1876)\n15th day of the month (31 Dec 1876)"),
        ruby {
            lang: "cn",
            "竊臣於十一月十五日",
            rt {
                lang: "en",
                "On the 15th day of the 11th month,"
            }
        }
    });
    rsx!(FileCitation {
        file: &CN_THDS_V04_P070,
        FigureAttribution {
            figure: &DING_RICHANG,
            locale: [
                rsx!(span { "Forbidden City" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { i { "circa" }, " 1877" })
            ],
            h1 {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "籌商大員移紮臺灣後山疏",
                    rt {
                        lang: "en",
                        "Memorial on Planning the Relocation of the Official to Taiwan's Back      \
                        Mountains"
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "福建巡撫臣丁日昌跪奏，",
                    rt {
                        lang: "en",
                        "The Provincial Governor of Fujian, Official Ding Richang, kneels to       \
                        present this memorial,"
                    }
                },
                ruby {
                    lang: "cn",
                    "為臺灣後山防務緊要，",
                    rt {
                        lang: "en",
                        "regarding the urgent military matters of Taiwan's eastern territory,"
                    }
                },
                ruby {
                    lang: "cn",
                    "擬請大員移紮，以靈呼應而求實濟，",
                    rt {
                        lang: "en",
                        "and the proposed relocation of this official in order to achieve          \
                        effective coordination and practical results;"
                    }
                },
                ruby {
                    lang: "cn",
                    "恭折仰祈聖鑒事。",
                    rt {
                        lang: "en",
                        "I humbly submit this memorial for Your Majesty's sage consideration:"
                    }
                }
            }
        }
    })
}

pub fn c_1877_01_xx_ding_01() -> Element {
    let t01 = rsx!(Tip {
        tip: S("2nd year of Emperor Guangxu (1876)\n11th lunar month (beg. 16 Dec 1876)\n15th day of the month (31 Dec 1876)"),
        ruby {
            lang: "cn",
            "竊臣於十一月十五日",
            rt {
                lang: "en",
                "On the 15th day of the 11th month,"
            }
        }
    });
    let t02 = rsx!(Tip {
        tip: S("2nd year of Emperor Guangxu (1876)\n11th lunar month (beg. 16 Dec 1876)\n18th day of the month (03 Jan 1877)"),
        ruby {
            lang: "cn",
            "旋於十八日",
            rt {
                lang: "en",
                "On the 18th day "
            }
        }
    });
    rsx!(FileCitation {
        file: &CN_THDS_V04_P070,
        FigureAttribution {
            figure: &DING_RICHANG,
            locale: [
                rsx!(span { "Forbidden City" }),
                rsx!(span { "Beijing, China" }),
                rsx!(span { i { "circa" }, " Jan 1877" })
            ],
            h1 {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "親勘臺灣北路後山大略情形疏",
                    rt {
                        lang: "en",
                        "Memorial Regarding Investigation into the General Condition of the Back   \
                        Mountains in Northern Taiwan"
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "福建巡撫臣丁日昌跪奏，",
                    rt {
                        lang: "en",
                        "The Provincial Governor of Fujian, Minister Ding Richang, respectfully    \
                        submits this memorial while kneeling,"
                    }
                },
                ruby {
                    lang: "cn",
                    "為微臣東渡親勘臺灣北路後山大略情形，",
                    rt {
                        lang: "en",
                        "regarding this humble minister's personal inspection of the general       \
                        conditions of the back mountains in Northern Taiwan after crossing the sea,"
                    }
                },
                ruby {
                    lang: "cn",
                    "恭折陳明，仰祈聖鑒事。",
                    rt {
                        lang: "en",
                        "on which I humbly present this detailed report for Your Majesty's         \
                        consideration:"
                    }
                },
                { t01 },
                ruby {
                    lang: "cn",
                    "由閩省起程，乘坐輪船渡臺，",
                    rt {
                        lang: "en",
                        " I departed from Fujian Province for Taiwan via steamship,"
                    }
                },
                ruby {
                    lang: "cn",
                    "曾經報明在案。",
                    rt {
                        lang: "en",
                        "as was previously reported and documented."
                    }
                },
                { t02 },
                ruby {
                    lang: "cn",
                    "到臺灣之北路雞籠，",
                    rt {
                        lang: "en",
                        "I subsequently arrived at Keelung in Northern Taiwan,"
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "行三日，",
                    rt {
                        lang: "en",
                        "After traveling for three days,"
                    }
                },
                ruby {
                    lang: "cn",
                    "抵蘇澳，",
                    rt {
                        lang: "en",
                        "I arrived at Su'ao,"
                    }
                },
                ruby {
                    lang: "cn",
                    "總兵張升階帶領各弁勇來見，",
                    rt {
                        lang: "en",
                        "where General Zhang Shengjie led various officers and brave soldiers to meet us,"
                    }
                },
                ruby {
                    lang: "cn",
                    "類皆病容滿面，",
                    rt {
                        lang: "en",
                        "all of whom exhibited signs of illness on their faces;"
                    }
                },
                ruby {
                    lang: "cn",
                    "據稱該鎮新帶兩營來此駐扎，",
                    rt {
                        lang: "en",
                        "according to them the garrison had recently brought two camps to station  \
                        here,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不及月餘，病者巳二百餘人，死者復十餘人。",
                    rt {
                        lang: "en",
                        "of whom, in less than a month, over two hundred had fallen ill and more   \
                        than ten had died."
                    }
                },
                ruby {
                    lang: "cn",
                    "半月前，",
                    rt {
                        lang: "en",
                        "half a month ago,"
                    }
                },
                ruby {
                    lang: "cn",
                    "福靖新到各營勇丁，來市買米，",
                    rt {
                        lang: "en",
                        "the newly arrived brave soldiers from the Fujing camp came to the market  \
                        to buy rice,"
                    }
                },
                ruby {
                    lang: "cn",
                    "回至蘇澳五里亭，",
                    rt {
                        lang: "en",
                        "and upon returning to the Five Li Pavilion in Su'ao,"
                    }
                },
                ruby {
                    lang: "cn",
                    "被生番狙殺九名。",
                    rt {
                        lang: "en",
                        "nine were ambushed and killed by the raw aboriginals."
                    }
                },
                ruby {
                    lang: "cn",
                    "該營官副將朱寶隆疏於防範，",
                    rt {
                        lang: "en",
                        "The camp officer Lieutenant General Zhu Baolong was negligent in          \
                        prevention,"
                    }
                },
                ruby {
                    lang: "cn",
                    "本應重辦，",
                    rt {
                        lang: "en",
                        "and should have been severely punished,"
                    }
                },
                ruby {
                    lang: "cn",
                    "姑念全營病者過半，",
                    rt {
                        lang: "en",
                        "but considering more than half the camp was ill,"
                    }
                },
                ruby {
                    lang: "cn",
                    "情有可原，",
                    rt {
                        lang: "en",
                        "and there were extenuating circumstances,"
                    }
                },
                ruby {
                    lang: "cn",
                    "相應請旨將候補副將朱寶隆即行革職，",
                    rt {
                        lang: "en",
                        "Lieutenant General Zhu Baolong was accordingly dismissed from his post by \
                        imperial edict,"
                    }
                },
                ruby {
                    lang: "cn",
                    "以示懲儆。",
                    rt {
                        lang: "en",
                        "as a warning to others."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "該生番向例俟秋冬間，",
                    rt {
                        lang: "en",
                        "It has been the custom of these raw aboriginals that, during autumn and   \
                        winter,"
                    }
                },
                ruby {
                    lang: "cn",
                    "即須出草殺人，",
                    rt {
                        lang: "en",
                        "they must go out to hunt heads,"
                    }
                },
                ruby {
                    lang: "cn",
                    "能割取首級者，眾人稱為英雄，",
                    rt {
                        lang: "en",
                        "and those who can take heads are then heroes by their people,"
                    }
                },
                ruby {
                    lang: "cn",
                    "即敲折一齒以為號，",
                    rt {
                        lang: "en",
                        "they then break off one tooth as a mark,"
                    }
                },
                ruby {
                    lang: "cn",
                    "番俗方肯以女妻之。",
                    rt {
                        lang: "en",
                        "and only then will their customs allow them to take a wife."
                    }
                },
                ruby {
                    lang: "cn",
                    "該生番數年來，",
                    rt {
                        lang: "en",
                        "These raw aboriginals, over the years,"
                    }
                },
                ruby {
                    lang: "cn",
                    "依舊殺人，",
                    rt {
                        lang: "en",
                        "have continued to kill,"
                    }
                },
                ruby {
                    lang: "cn",
                    "並不知有所謂就撫之說。",
                    rt {
                        lang: "en",
                        "and know nothing of what we call submission and pacification."
                    }
                },
                ruby {
                    lang: "cn",
                    "臣審明後，",
                    rt {
                        lang: "en",
                        "After my humble investigation,"
                    }
                },
                ruby {
                    lang: "cn",
                    "當驗各生番有敲折一齒及二齒者，",
                    rt {
                        lang: "en",
                        "I examined the raw aboriginals who had one or two broken teeth,"
                    }
                },
                ruby {
                    lang: "cn",
                    "計共四名，據供均經殺人數次，",
                    rt {
                        lang: "en",
                        "counted four in total, all confessing to have killed multiple times,"
                    }
                },
                ruby {
                    lang: "cn",
                    "當即正法，",
                    rt {
                        lang: "en",
                        "and had them executed;"
                    }
                },
                ruby {
                    lang: "cn",
                    "以首級祭陣亡諸勇之靈。",
                    rt {
                        lang: "en",
                        "their heads used as sacrifice to appease the spirits of our fallen brave  \
                        soldiers."
                    }
                },
                ruby {
                    lang: "cn",
                    "其未經折齒諸番，飭令暫留營中，",
                    rt {
                        lang: "en",
                        "those aboriginals who had not yet broken their teeth were ordered to
                        temporarily remain in the camp,"
                    }
                },
                ruby {
                    lang: "cn",
                    "以備將來擒縱之用。",
                    rt {
                        lang: "en",
                        "to be used for future capture and release operations."
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "臣查後山生番計共數百社，",
                    rt {
                        lang: "en",
                        "I humbly enumerated that the raw aboriginals of the Eastern Mountains     \
                        total several hundred communities,"
                    }
                },
                ruby {
                    lang: "cn",
                    "穴居野處，",
                    rt {
                        lang: "en",
                        "living in caves and wild places,"
                    }
                },
                ruby {
                    lang: "cn",
                    "並無總目管轄，",
                    rt {
                        lang: "en",
                        "with no overall chief governing them,"
                    }
                },
                ruby {
                    lang: "cn",
                    "行同禽獸。",
                    rt {
                        lang: "en",
                        "while behaving like birds and beasts."
                    }
                },
                ruby {
                    lang: "cn",
                    "但以殺人為樂，",
                    rt {
                        lang: "en",
                        "They take pleasure only in killing,"
                    }
                },
                ruby {
                    lang: "cn",
                    "其居平原者，稍知人性，",
                    rt {
                        lang: "en",
                        "even those who live in the plains and know somewhat of human nature—"
                    }
                },
                ruby {
                    lang: "cn",
                    "名曰平埔番，",
                    rt {
                        lang: "en",
                        "so-called called Plains Aboriginals—"
                    }
                },
                ruby {
                    lang: "cn",
                    "性極詭詐，",
                    rt {
                        lang: "en",
                        "are extremely cunning in nature,"
                    }
                },
                ruby {
                    lang: "cn",
                    "每慫恿生番殺人，",
                    rt {
                        lang: "en",
                        "often incite the raw aboriginals to kill,"
                    }
                },
                ruby {
                    lang: "cn",
                    "居間取利。",
                    rt {
                        lang: "en",
                        "and profit from being intermediaries."
                    }
                },
                ins { ruby { lang: "cn", "⋯⋯", rt { lang: "en", "…" } } },
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "臣屬張升階先為確查該番良歹，",
                    rt {
                        lang: "en",
                        "I humbly instructed Zhang Shengjie to first thoroughly investigate the    \
                        good and bad among these aboriginals:"
                    }
                },
                ruby {
                    lang: "cn",
                    "其平埔近海各番易與洋人勾結者，",
                    rt {
                        lang: "en",
                        "those Plains Aboriginals near the sea who easily collude with foreigners,"
                    }
                },
                ruby {
                    lang: "cn",
                    "可撫則撫，",
                    rt {
                        lang: "en",
                        "pacify those who can be pacified,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不可撫則須擇尤痛加剿辦。",
                    rt {
                        lang: "en",
                        "and those who cannot be pacified must be selected for severe suppression."
                    }
                },
                ruby {
                    lang: "cn",
                    "然後另選頭目，",
                    rt {
                        lang: "en",
                        "Then select new chiefs,"
                    }
                },
                ruby {
                    lang: "cn",
                    "令之剃髮，",
                    rt {
                        lang: "en",
                        "order them to shave their heads,"
                    }
                },
                ruby {
                    lang: "cn",
                    "歸入版圖，",
                    rt {
                        lang: "en",
                        "incorporate them into our territory,"
                    }
                },
                ruby {
                    lang: "cn",
                    "嚴定界地，",
                    rt {
                        lang: "en",
                        "strictly define boundary lands,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不許他人侵占，",
                    rt {
                        lang: "en",
                        "disallow the occupation by others,"
                    }
                },
                ruby {
                    lang: "cn",
                    "俾得自安耕鑿，",
                    rt {
                        lang: "en",
                        "enable them to till the land,"
                    }
                },
                ruby {
                    lang: "cn",
                    "庶法立恩加，",
                    rt {
                        lang: "en",
                        "establish law and treat with grace,"
                    }
                },
                ruby {
                    lang: "cn",
                    "知懼而後知感，",
                    rt {
                        lang: "en",
                        "so they may know fear and then know kindness;"
                    }
                },
                ruby {
                    lang: "cn",
                    "方免彼此相持，永無息肩之日。",
                    rt {
                        lang: "en",
                        "only then can we avoid mutual conflict and and never-ending struggle."
                    }
                },
                    mark {
                    ruby {
                        lang: "cn",
                        "且我之所以撫番者，",
                        rt {
                            lang: "en",
                            "Moreover, as for our original purpose for pacifying the aboriginals,"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "原以杜洋人覬覦之謀，",
                        rt {
                            lang: "en",
                            "being primarily the prevention of schemes by foreigners who lust      \
                            after Taiwan:"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "若不大舉剿辦，收入版圖，",
                        rt {
                            lang: "en",
                            "if we do not launch a major campaign and incorporate this territory,"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "萬一洋人復重利餌番，",
                        rt {
                            lang: "en",
                            "and if foreigners again lure the aboriginals with bait,"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "曰吾取地於番也，",
                        rt {
                            lang: "en",
                            "accuse us of taking land that belongs to the aboriginals,"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "非取地於中國也，",
                        rt {
                            lang: "en",
                            "not land that belongs to China,"
                        }
                    },
                    ruby {
                        lang: "cn",
                        "我復何說之辭！",
                        rt {
                            lang: "en",
                            "what words would we have in response!"
                        }
                    }
                }
            },
            p {
                class: "chinese-text",
                ruby {
                    lang: "cn",
                    "故為目前計，",
                    rt {
                        lang: "en",
                        "Therefore for current considerations:"
                    }
                },
                ruby {
                    lang: "cn",
                    "得番地不足以為益，",
                    rt {
                        lang: "en",
                        "obtaining the aboriginal land may not be beneficial,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不得番地不足以為損，",
                    rt {
                        lang: "en",
                        "while forgoing the aboriginal land may not be harmful;"
                    }
                },
                ruby {
                    lang: "cn",
                    "為大局計，",
                    rt {
                        lang: "en",
                        "but for the bigger picture:"
                    }
                },
                ruby {
                    lang: "cn",
                    "得番地則可永斷葛藤，",
                    rt {
                        lang: "en",
                        "obtaining the aboriginal land may conclusively terminate entanglements,"
                    }
                },
                ruby {
                    lang: "cn",
                    "不得番地，則恐難息窺伺。",
                    rt {
                        lang: "en",
                        "while forgoing the aboriginal may be obstructive towards preventing plots."
                    }
                }
            }
        }
    })
}

#[component]
pub fn PoliticsTaiwanSovereignty() -> Element {
    rsx! {
        link { rel: "stylesheet", href: BLOG_CSS },
        Timeline {
            id: "primary-timeline",
            t0: 1845,
            tf: 2025
        },
        div {
            id: "text-area",
            BlogChapter {
                title: "Preface",
                cites: vec![],
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
            },
            BlogChapter {
                title: "Section 1.1: Shipwreck Hazard",
                cites: vec![
                    c_1851_05_xx_alcock_01(),
                    c_1851_10_10_conrad_01(),
                    c_1856_02_12_nye_01(),
                    c_1858_11_20_bradley_01()
                ],
                p {
                    "The Kelpie, The Rover, The Larpent. Also, The Porpoise, The Highflyer"
                }
            },
            BlogChapter {
                title: "Section 1.2: Layman's Observed Boundary",
                cites: vec![
                    c_1855_02_14_perry_01(),
                    c_1855_12_xx_habersham_01(),
                    c_1857_02_10_nye_01(),
                    c_1886_11_17_denby_01(),
                    c_1898_05_12_pickering_01()
                ],
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
            },
            BlogChapter {
                title: "Section 1.3: Rover Incident",
                cites: vec![],
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
            },
            BlogChapter {
                title: "Section 1.4: Taiwan Expedition",
                cites: vec![c_1874_05_22_house_01()],
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
            },
            BlogChapter {
                title: "Section 1.5: Is 1875's Taiwan A Part Of The Chinese Empire?",
                cites: vec![],
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
            },
            BlogChapter {
                title: "Section 1.6: International Law",
                cites: vec![],
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
            }
        }
    }
}

#[component]
fn BlogChapterCitations(cites: Vec<Element>) -> Element {
    let citation_count: u8 = cites.len() as u8;
    let mut citation_index = use_signal(|| 1u8);
    let mut bc_cc_children = use_signal(|| Option::<HtmlCollection>::None);

    use_effect(move || if let Some(bc_cc_children) = bc_cc_children() {
        let c_el = bc_cc_children.get_with_index((citation_index() - 1) as u32).unwrap();
        let p_el = c_el.parent_element().unwrap();
        let disp = c_el.get_bounding_client_rect().x() - p_el.get_bounding_client_rect().x();
        p_el.scroll_by_with_x_and_y(disp, 0f64);
        c_el.scroll_to_with_x_and_y(0f64, 0f64);
    });

    rsx! {
        div {
            class: "bc-cc-greater",
            div {
                class: "bc-cc-panel",
                span {
                    class: if citation_index() > 1 {
                        "bc-cc-button bc-cc-button-prev"
                    } else {
                        "bc-cc-button bc-cc-button-prev disabled"
                    },
                    onclick: move |_| citation_index.set((citation_index - 1).max(1u8)),
                    "<"
                },
                span {
                    class: "bc-cc-index",
                    "{ citation_index() } of { citation_count }"
                },
                span {
                    class: if citation_index() < citation_count {
                        "bc-cc-button bc-cc-button-next"
                    } else {
                        "bc-cc-button bc-cc-button-next disabled"
                    },
                    onclick: move |_| citation_index.set((citation_index + 1).min(citation_count)),
                    ">"
                }
            },
            div {
                class: "bc-cc-proper",
                onmounted: move |e| bc_cc_children.set(Some(e.as_web_event().children())),
                { cites.into_iter() }
            }
        }
    }
}

#[component]
fn BlogChapter(title: String, cites: Vec<Element>, children: Element) -> Element {
    rsx! {
        div {
            class: "bc",
            h2 {
                "{ title }"
            },
            if cites.len() > 0 {
                BlogChapterCitations { cites: cites }
            },
            { children }
        }
    }
}


const FILE_LOCAL_STILL: Asset = asset!("/assets/icon/file-local-still.svg");
const FILE_LOCAL_HOVER: Asset = asset!("/assets/icon/file-local-hover.svg");
const FILE_CLOUD_STILL: Asset = asset!("/assets/icon/file-cloud-still.svg");
const FILE_CLOUD_HOVER: Asset = asset!("/assets/icon/file-cloud-hover.svg");

#[component]
fn FileCitation(file: &'static File, children: Element) -> Element {
    rsx! {
        div {
            class: "file-citation",
            div {
                class: "file-citation-header",
                span {
                    class: "file-citation-banner",
                    { file.banner }
                },
                div {
                    class: "file-citation-link-button file-citation-internal-link",
                    style: "--file-citation-link-image-still: url({FILE_LOCAL_STILL}); --file-citation-link-image-hover: url({FILE_LOCAL_HOVER});",
                    onclick: move |_| drop(window().unwrap().open_with_url(&*file.record.to_string()))
                },
                div {
                    class: "file-citation-link-button file-citation-external-link",
                    style: "--file-citation-link-image-still: url({FILE_CLOUD_STILL}); --file-citation-link-image-hover: url({FILE_CLOUD_HOVER});",
                    onclick: move |_| drop(window().unwrap().open_with_url(file.source))
                }
            },
            {children}
        }
    }
}

#[component]
fn FigurePortrait(figure: &'static Person) -> Element {
    let portrait = figure.portraiture.clone().unwrap_or(Portraiture {
        sketch: asset!("assets/portrait/george_yeh.gif"),
        normal: asset!("assets/portrait/george_yeh.webp"),
        source: "<i>葉公超.</i> Photograph. Accessed September 14, 2024. https://k.sina.cn/article_5765389469_157a4dc9d001017eyq.html"
    });
    rsx! {
        div {
            class: "portrait-box",
            img {
                class: "portrait-sketch",
                src: "{portrait.sketch}"
            },
            img {
                class: "portrait-normal",
                src: "{portrait.normal}"
            },
            span {
                class: "portrait-banner",
                { figure.name.last }
            }
        }
    }
}

#[component]
fn FigureAttribution(figure: &'static Person, locale: [Element; 3], children: Element) -> Element {
    let [locale_line_1, locale_line_2, locale_line_3] = locale;
    let portrait = figure.portraiture.clone().unwrap_or(Portraiture {
        sketch: asset!("assets/portrait/george_yeh.gif"),
        normal: asset!("assets/portrait/george_yeh.webp"),
        source: "<i>葉公超.</i> Photograph. Accessed September 14, 2024. https://k.sina.cn/article_5765389469_157a4dc9d001017eyq.html"
    });
    rsx! {
        div {
            class: "attribution",
            div {
                class: "attribution-locale",
                FigurePortrait { figure: figure },
                p {
                    { locale_line_1 },
                    br { },
                    br { },
                    { locale_line_2 },
                    br { },
                    br { },
                    { locale_line_3 }
                }
            },
            { children }
        }
    }
}

#[component]
fn FigurelessAttribution(locale: [Element; 3], children: Element) -> Element {
    let [locale_line_1, locale_line_2, locale_line_3] = locale;
    rsx! {
        div {
            class: "attribution",
            div {
                class: "attribution-locale",
                p {
                    { locale_line_1 },
                    br { },
                    br { },
                    { locale_line_2 },
                    br { },
                    br { },
                    { locale_line_3 }
                }
            },
            { children }
        }
    }
}

#[derive(Clone, PartialEq)]
enum TipType {
    S(&'static str),
    P(&'static Person)
}

#[component]
fn Tip(tip: TipType, children: Element) -> Element {
    rsx! {
        span {
            class: "tooltip-indicator",
            { children },
            div {
                class: "tooltip-box",
                match tip.clone() {
                    S(detail) => {
                        let detail: Vec<&str> = detail.split('\n').collect();
                        rsx!(for x in 0..detail.len() {
                            if x > 0 { br{} },
                            { detail.get(x).unwrap() }
                        })
                    },
                    P(figure) => rsx!(FigurePortrait { figure: figure })
                }
            }
        }
    }
}