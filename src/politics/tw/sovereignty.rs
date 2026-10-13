/*use dioxus::prelude::*;
use crate::{block, p, translated_p, parse_segments, subtitle};
use crate::politics::data::file::*;
use crate::politics::data::person::*;
use crate::politics::{PureTextLang, ParagraphType, TextSegment, Paragraph, Chapter, ChapterBlock, Politics, stable_hash, Section};


fn ch_migration_ban() -> Chapter {
    ("Migration Ban", vec![
        block!("TODO", [
            translated_p!(
                ZH,
                crafted: [EN "Memorial on the Investigation and Management of Village Clearing in Taiwan (5th Year of Daoguang [1825])"],
                foreign: [ZH "查辦臺灣清莊事宜疏（道光五年）"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "by ", fig &ZHAO_SHENZEN, EN " and ", fig &SUN_ERZHUN],
                foreign: [ZH "趙慎畛、孫爾準"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Governor-General of Fujian and Zhejiang, Zhao Shenzhen, and Governor of Fujian, Sun Erzhun, humbly kneel and memorialize", EN " regarding the various matters of conducting village clearings and suppressing smuggling in Taiwan."],
                foreign: [ZH "閩浙總督臣趙慎畛、福建巡撫臣孫爾準跪奏，", ZH "爲查辦臺灣清莊緝私各事宜事。"]
            ),
            translated_p!(
                ZH,
                crafted: [mark [EN "Taiwan is suspended in isolation across the sea,", EN " lacking permanent inhabitants and characterized by unstable customs.", EN " Mainland vagrants frequently cross the waters through illegal means."], EN " These idle wanderers lack specialized trades for support,", EN " possess no ancestral lands to cultivate,", EN " and are naturally defined by an indolent and slothful disposition.", EN " Even when hired for labor,", EN " they remain incapable of enduring toilsome hardship.", EN " If they are not hiding in gambling dens,", EN " then they are lurking about to commit acts of petty theft."],
                foreign: [mark [ZH "竊照臺灣孤懸海外，", ZH "民無土著，", ZH "俗本輕浮，"], ZH "素有內地遊民，", ZH "偷越私渡，", ZH "此等遊手好閒之人，", ZH "既無藝業可守，", ZH "又無田地可耕，", ZH "且懶惰性成，", ZH "即傭工力作，", ZH "亦復不耐辛勞，", ZH "非匿跡於賭場，", ZH "即潛蹤於鼠竊。"]
            ),
            translated_p!(
                ZH,
crafted: [EN "Drifting in a foreign land with no way to trace their origins,", EN "it is feared they will gradually gather in increasing numbers,", EN "forming gangs and factions.", EN "When the local area is at peace,", EN "they engage in reckless robbery,", EN "bringing harm to the neighborhoods.", EN "Furthermore, the population of Taiwan Prefecture consists mostly of people from Zhangzhou and Quanzhou,", EN "who are naturally inclined toward contention.", EN "At the slightest sign of a trivial dispute,", EN "these vagrants immediately seize the opportunity to join in,", EN "lending their strength to acts of violence.", EN "In minor cases, this incites armed clan feuds,", EN "while in major cases, they plot rebellious activities."],
foreign: [ZH "異鄉飄泊，", ZH "無可稽查，", ZH "恐漸聚漸多，", ZH "成羣結黨，", ZH "當地方無事之時，", ZH "肆行搶劫，", ZH "爲害閭閻，", ZH "而臺郡率皆漳泉民人，", ZH "素尚忿爭，", ZH "一遇雀角微嫌，", ZH "該遊民等即乘機附和，", ZH "助勢逞兇，", ZH "小則激成械鬥，", ZH "大則謀爲不軌。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Last year's southern route bandits, Xu Shang and Yang Liangbin,", EN " gathered crowds to incite unrest,", EN " likely instigated by such vagrant elements.", EN " Fortunately, high officials acted with speed,", EN " capturing leaders and accomplices,", EN " and eradicating the threat entirely.", EN " However, mainland vagrants still persist,", EN " and their expulsion is vital for long-term security.", EN " No method is superior to 'Village Clearing',", EN " which is simpler to organize than the full baojia system."],
                foreign: [ZH "上年南路匪徒許尚、楊良斌等，", ZH "糾衆滋事，", ZH "即恐有遊民從中煽誘。", ZH "幸該鎮道府等拏辦妥速，", ZH "當將首夥各犯，", ZH "殲除殆盡。", ZH "但內地遊民，", ZH "尚未絕跡，", ZH "亟須查逐清釐，", ZH "以爲久安之計。", ZH "然欲查逐遊民，", ZH "莫善於清莊一法，", ZH "其事較編查保甲，", ZH "尤爲簡而易行。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "During policing crises in Yanping and Jianning,", EN " we adapted the baojia regulations", EN " into the 'Lianjia' (united-armor) method,", EN " which yielded successful results.", EN " Village clearing is similar in its implementation.", EN " When your minister Sun Erzun traveled to Taiwan,", EN " he ordered strict village clearing,", EN " forbidding any harboring of bandits.", EN " This was previously reviewed by the Emperor on the record."],
                foreign: [ZH "前年臣等因延平、建寧等郡捕務緊要，", ZH "曾飭各屬於保甲之外，", ZH "小爲變通，", ZH "辦理連甲，", ZH "頗有成效。", ZH "清莊亦與聯甲相仿。", ZH "上年臣孫爾準赴臺時，", ZH "亦飭臺屬力行清莊之法，", ZH "毋許容留匪類，", ZH "均於稽察棚民複查保甲案內，", ZH "先後奏蒙聖鑑在案。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Taiwan is settled by Zhangzhou, Quanzhou, and Cantonese groups,", EN " residing within villages divided by regional origin.", EN " Each village has established headmen,", EN " and the villagers follow their management.", EN " We have ordered Taiwan's high officials", EN " to hold local magistrates accountable", EN " for ensuring these headmen", EN " lead elders and village chiefs", EN " in conducting rigorous inspections."],
                foreign: [ZH "緣臺灣向系漳、泉、粵三籍之人，", ZH "分莊居住，", ZH "各莊設有頭人，", ZH "莊民鹹聽頭人管束。", ZH "臣等見覆分飭臺灣鎮道府，", ZH "督飭廳縣，", ZH "責成該頭人，", ZH "率同客長鄉耆，", ZH "並莊正、保正人等，", ZH "實力查察。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Any suspicious strangers,", EN " or those wandering near temples and rest stops", EN " without kin to support them,", EN " are deemed to be foreign vagrants.", EN " Headmen must report them to officials,", EN " who shall determine their native origin.", EN " They shall be expelled across the water,", EN " tattooed with 'Cross Water' as a mark,", EN " and deported to their original home districts.", EN " Return to Taiwan is strictly prohibited."],
                foreign: [ZH "如有面生可疑，", ZH "或在古廟涼亭及飯歇處所飄流無定之人，", ZH "查無親屬相依者，", ZH "既屬外來遊民。", ZH "該頭人等立即稟報地方官，", ZH "訊明內地籍貫，", ZH "照例逐令過水，", ZH "面刺過水二字，", ZH "遞迴原籍安插，", ZH "毋許復行偷渡赴臺；"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Even regarding those seeking work as porters,", EN " porter headmen must verify their characters.", EN " If they are truly honest commoners,", EN " with signed guarantees,", EN " they may remain and serve in the trade.", EN " However, those of unknown origin or violent temper,", EN " prone to inciting trouble,", EN " must be reported,", EN " and expelled to the mainland."],
                foreign: [ZH "即在夫行投充小夫者，", ZH "亦令夫頭查明，", ZH "果系良民誠實安分者，", ZH "出具保結，", ZH "仍準在行充夫，", ZH "如來歷不明，", ZH "及好勇鬥很，", ZH "動輒生事之徒，", ZH "亦俱報明本管官，", ZH "一律驅逐回籍。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Headmen clearing their own villages", EN " ensures every household is informed.", EN " This avoids the harassment of runners", EN " and complex paperwork.", EN " Villagers, defending their homes, cooperate gladly.", EN " Magistrates, upon receiving reports,", EN " must immediately interrogate suspects.", EN " Vagrants are to be guarded strictly,", EN " and deported in groups.", EN " Given the sea's dangers,", EN " secure transport is essential.", EN " We have ordered the drafting of regulations,", EN " to prevent trouble or escapes mid-journey."],
                foreign: [ZH "該頭人等，", ZH "各清各莊，", ZH "家喻戶曉，", ZH "既無胥役紛查之擾，", ZH "又無門牌冊籍之繁。", ZH "各莊民自衛自家，", ZH "亦樂於從事。", ZH "臺地各廳縣，", ZH "一據頭人稟報，", ZH "立時訊明，", ZH "先將該遊民嚴加看管，", ZH "或數人或數十人，", ZH "分起押解回內，", ZH "海洋險阻，", ZH "必須妥爲管解，", ZH "亦令議立章程，", ZH "以免中途滋事疏脫。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Arriving at mainland ports,", EN " local officials will manage their return stage by stage.", EN " Quanzhou and Zhangzhou magistrates", EN " must receive these deportees,", EN " requiring clan elders to provide bonds", EN " and enforcing strict control over them.", EN " They must not wander out again.", EN " Meanwhile, mainland and Taiwan officials", EN " must instruct port guards", EN " to inspect all vessels with genuine vigor.", EN " Should any wicked people illegally cross,", EN " both people and vessels shall be prosecuted."],
                foreign: [ZH "俟解至內地入口，", ZH "即由地方官逐程遞解，", ZH "並通飭泉州、漳州各府廳縣，", ZH "如遇臺灣遞解遊民到境，", ZH "按照籍貫，", ZH "飭命該處鄉耆族房取保，", ZH "嚴行管束，", ZH "毋許私自外出，", ZH "一面分檄內地，", ZH "臺灣該管各文武，", ZH "嚴飭守口員弁，", ZH "凡遇船隻出入口者，", ZH "務須實力盤詰。", ZH "如有奸民越渡臺洋，", ZH "即將人船拏解究辦。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "With rigorous investigations,", EN " vagrants cannot sneak into Taiwan,", EN " and local peace is attained.", EN " Regarding Taiwan’s salt administration,", EN " merchant households traditionally hire guards.", EN " However, coastal criminals", EN " are building illegal pans", EN " to illicitly produce and sell salt,", EN " which is most severe in Chiayi County.", EN " These salt bandits are precisely such vagrants.", EN " If not arrested promptly,", EN " they will gather in force", EN " and harm the land."],
                foreign: [ZH "似此稽查嚴密，", ZH "庶遊民不能溷跡赴臺，", ZH "地方獲就安謐。", ZH "至臺灣府鹽務，", ZH "向由販戶承辦，", ZH "自行選僱哨捕巡緝，", ZH "見聞該處沿海奸民，", ZH "私設埕坎，", ZH "擅自曬鹽販賣，", ZH "惟嘉義縣境內爲尤甚。", ZH "此等曬私匪類，", ZH "即系遊民，", ZH "若不及早查拏，", ZH "日久嘯聚，", ZH "亦足爲土地之害。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "We have already dispatched urgent directives to intendants", EN " to supervise local magistrate agents.", EN " In coordination with merchants,", EN " they must conduct tight inspections.", EN " Producers everywhere must be captured,", EN " suppressing smuggling", EN " and ensuring official sales.", EN " This secures revenue and food supplies,", EN " while depriving bandits of a living.", EN " Mainland criminals", EN " will be deterred by the news.", EN " Taiwan will be cleared of such elements,", EN " benefiting this vital maritime region."],
                foreign: [ZH "業經飛飭臺灣道府，", ZH "督飭廳縣，", ZH "選派幹役，", ZH "協同販戶，", ZH "嚴緊查緝，", ZH "將各處私曬匪徒，", ZH "拏獲解究，", ZH "俾私販斂戢，", ZH "官引暢銷，", ZH "不特課賦民食，", ZH "無短絀匱乏之虞，", ZH "且令匪棍流民，", ZH "無托足謀生之所，", ZH "則內地奸徒，", ZH "聞風畏縮，", ZH "不敢再行前往，", ZH "而臺地得以一律肅清，", ZH "實於海外要區，", ZH "大有巔益。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Taiwan’s distance across the sea", EN " makes comprehensive surveillance difficult.", EN " We fear distant magistrates", EN " may become negligent or lax.", EN " We are conducting secret inquiries.", EN " Strict orders have been issued:", EN " officials clearing vagrants", EN " without disturbing civilians", EN " shall be rewarded", EN " as dedicated servants of the state."],
                foreign: [ZH "惟臺灣重洋間隔，", ZH "非如內地之耳目易周，", ZH "恐該廳縣以距省窵遠，", ZH "意存泄玩，", ZH "臣等見在密加訪察，", ZH "並嚴札通飭該廳縣等，", ZH "如將境內遊民人等，", ZH "認真細查，", ZH "押逐淨盡，", ZH "使民間不受擾累，", ZH "即系實心任事之員，", ZH "定當量予獎勵。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Failure in performance,", EN " through laziness or incompetence,", EN " by failing to expel vagrants,", EN " proves unfitness for defense.", EN " They shall be recalled,", EN " demoted in rank,", EN " or reassigned as assistants,", EN " per the severity of the failure.", EN " Those feigning compliance", EN " or seeking avoidance", EN " will face immediate impeachment", EN " as a warning to others.", EN " Incentives and penalties", EN " will motivate local officials.", EN " Taiwan authorities will investigate locally", EN " and report the truth with honesty.", EN " Favoritism shall incur heavy punishment."],
                foreign: [ZH "如奉行不力，", ZH "或由於辦事懈怠，", ZH "或限於才質迂疏，", ZH "雖經查逐，", ZH "未能悉數驅除，", ZH "即不加緊搜查，", ZH "有初鮮終，", ZH "亦難勝海疆之任，", ZH "應即撤回內地，", ZH "或改補簡缺，", ZH "或降補佐雜，", ZH "分別輕重，", ZH "奏明辦理。", ZH "若竟陽奉陰違，", ZH "或畏難苟安，", ZH "膜置不辦，", ZH "則自甘暴棄，", ZH "立即嚴參示儆。", ZH "如此酌定勸懲，", ZH "使各廳縣，", ZH "共知奮興警懼，", ZH "方可冀收實效，", ZH "仍令該鎮道府，", ZH "就近確查，", ZH "隨時據實稟報，", ZH "毋得稍事徇護，", ZH "並乾重咎。"]
            ),
            translated_p!(
                ZH,
                crafted: [EN "Regarding this vital maritime security issue,", EN " we respectfully submit this formal memorial,", EN " seeking Your Majesty's divine review and instructions."],
                foreign: [ZH "臣等爲海疆地方緊要起見，", ZH "謹恭折具奏，", ZH "伏乞皇上聖鑑訓示。"]
            )
        ])
    ])
}
fn ch_vicious_natives() -> Chapter {
    ("Vicious Natives", vec![
        block!("Perry Expedition (1852-1855)", [
            p!(@Initial:
                EN "The Perry Expedition was commanded by United States Commodore ",
                fig &PERRY_MATTHEW,
                EN ", under orders from President Millard Fillmore. Perry's primary goal was to force an end to Japan's 220-year-old policy of isolation and to open Japanese ports to American trade, through the use of gunboat diplomacy if necessary. By the time Commodore Perry's squadron of four ships sailed into Tokyo Bay on July 8, 1853, the Japanese Tokugawa government had already fended off similarly unwelcome intrusions by the French, the Russians, the Dutch, and the British."
            ),
            p!(EN "The expedition made multiple port calls before reaching Japan, including stops at Cape Town, Sri Lanka, Singapore, and Hong Kong. Finally, on March 31, 1854, Perry signed the Convention of Kanagawa which opened the ports of Shimoda and Hakodate to American ships, provided for care of shipwrecked sailors, and the establishment of an American consulate in Shimoda. After Perry returned to the United States in 1855, Congress voted to grant him a reward of $20,000 in appreciation of his work in Japan."),
            p!(
                EN "During their expedition, a visit was paid to the Island of Taiwan, with the goal of locating a rumored coal deposit. Record of this visit survives in the writing of one ",
                fig &BREESE_KIDDER,
                EN "."
            ),
            p!(EN "Breese was appointed a Navy Midshipman in 1847 during the Mexican–American War and served on the sea until 1852, when he was assigned to the United States Naval Academy to prepare for examination. Warranted a Passed Midshipman in June 1852, Breese took part in the Commodore Perry's expedition to Japan for the next three years."),
            p!(EN "Breese would attain the rank of Captain (NATO 0F-5) during his lifetime."),
            p!(EN "His retelling of the occasion at Taiwan is as follows", file &BOOK_SPROSTON, EN ":"),
            p!(@Excerpt: EN "In the morning when I turned out we were under topsails and courses running free. Keelung Island was close aboard, a small conical island four miles from the port, and looming up through the mist right ahead rose the high land of Formosa. Several heavy squalls obliged us to lay to until they cleared off, when, taking advantage of a lull, we stood in, taking on board a Chinese pilot; and before meridian were at anchor in the harbour of Keelung."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "Several of the officers, this day, visited a coal mine about a mile from the town. They found it of easy access and apparently good quality. The mandarins state that the junks load with this useful, and, in this part of the world, scarce article, at a port eighty miles to the southward of this place. We wanted to form a party, and, assisted by a Chinese guide, make a land excursion to the mines, but ", mark [EN "on inquiry we found that on account of the barbarous tribes—and, as they say here, also cannibals—which infest the country, none of the Chinese would show us the way; so here the expedition ends, no doubt."]),
            p!(@Excerpt: EN "We visited this port with two objects in view; namely, to obtain coal (which we have at least found) and to discover if there were any prisoners upon the islands—unfortunate seamen cast away on this inhospitable land and doomed either to death or slavery. But on this point ", mark [ EN "we have been unable to gain any information."]),
            p!(EN "Breese's account of the Perry Expedition's visit to Taiwan was a glimpse into the island's violent and lawless nature, as perceived by the expedition. The refusal of Chinese guides to lead the party to the coal mines, citing the presence of \"barbarous tribes\" and rumored cannibals, demonstrates the dangerous and untamed reputation of Taiwan's native inhabitants.")
        ]),
        block!("Eulenburg Expedition (1859-1862)", [
            p!(@Initial: EN "Both of these dudes say the same thing about the attack in 1860:", file &BOOK_EULENBURG, file &BOOK_BRANDT_1)
        ]),
        block!("Bax Expedition ()", [
            p!(@Initial: fig &BAX_BONHAM),
            p!(file &BOOK_BAX, EN ":"),
            p!(@Excerpt: EN "There were two villages of <ins>Pepohoan</ins> on the south side of <ins>Su'ao Bay</ins>, which were carefully protected by stockades of bamboo, and had a watchtower near the centre, as they also sometimes suffer from the attacks of the savages. In consequence of the crews of vessels that are wrecked on the east coast of Formosa being always ill-treated and often killed, I was desirous of making some arrangement with the tribes to be kind to foreigners who were thrown into their power, and to forward them on to the Chinese, who would send then in to the nearest consulate, according to the treaty. An English gentleman belonging to the Chinese custom service, and who was stationed at Kelung, and had come as my guest to this place, was also anxious to accomplish the same object. We therefore landed with his Chinese servant, and tried to get a guide at the <ins>Pepohoan</ins> villages, but unfortunately without success, as they said they had a feud at present with the nearest savage tribes, and any of their people would be killed if taken by them. The Chinese, of course, refused to go, as they were always at open war."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "On the 25th of May the \"Dwarf\" left Nagasaki to follow the Japanese expedition; after a favourable run we called in at Kelung and coaled, and obtained all the information we could about it<del>...</del>"),
            p!(@Excerpt: EN "A short time before the arrival of the \"Dwarf\" at Kelung, three canoes, containing twelve men belonging to the Pellew islands, had reached Kelung. They had been blown away from home while fishing, and had been sixty-four days at sea, and had travelled about 1,300 miles. A fourth canoe had been driven off with them, but had parted company a few days before the others had arrived at Kelung, and had not since been heard of. They had lived on the fish they caught, and the rain supplied them with water to drink, as their bamboo vessels for holding it were very small. They had coasted Formosa, but had not dared to land for fear of being killed by the natives; at last they were fortunately set close in to Kelung, and discovered by the Europeans at the custom-house, who had them brought in<del>...</del>"),
            p!(@Excerpt: EN "When they were first picked up they were in a very emaciated condition, but by the kind treatment they received they soon recovered. On finding their way on board the ship they appeared in good spirits and were much petted by the seamen. In consequence of the great difficulty of getting these people back to their homes again, there being no communication between Kelung and the Pellew islands, they were sent to Hongkong in an English steamer, and from that place an English man-of-war eventually took them back to the Pellew islands.")
        ])
    ])
}

fn ch_divided_island() -> Chapter {
    ("Divided Island", vec![
        block!("Perry Expedition, Revisited", [
            p!(@Initial:
                EN "Previously, we explored Commodore ",
                fig &PERRY_MATTHEW,
                EN "'s expedition to Japan in 1853, specifically their interim visit to Taiwan, as recounted by ",
                fig &BREESE_KIDDER,
                EN ". Their expedition's attempt to locate rumored coal deposits was unsuccessful, on account of the refusal by Chinese guides to venture inland due to their fear of the barbarous tribes and \"cannibals\"."
            ),
            p!(EN "Here, we look again to Breese's commentary on Taiwan, its isolation, and the dynamics between its Chinese settlers and native tribes, alongside Perry's thoughts on the matter. Breese's account of Taiwan continues", file &BOOK_SPROSTON, EN ":"),
            p!(@Excerpt: EN "The Chinese have but little intercourse with the savage tribes, that we can discover, holding but an insecure footing themselves and liable to attacks from their formidable enemies at any moment. This is in every sense an unknown island to civilized nations, and years will in all probability lapse before any determined steps will be taken to explore it to any great extent."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "Formosa is a large island off the Province of <ins>Fujian</ins> China, from which it is separated by a channel from 90 to 120 miles in breadth. Its length is about 250 miles by 50 or so in width. It was the last conquest made <ins>in</ins> the present Tartar dynasty, and even now, it is only a conquest by name, as the Chinese are restricted to a very small part of the Island. Vessels have now and then been wrecked upon its inhospitable shores, <ins>and</ins> the fates of all have been death or captivity."),
            p!(EN "Breese reveals that Chinese dominion over Taiwan was largely nominal and geographically limited, described as \"only a conquest by name\" with the Chinese \"restricted to a very small part of the Island.\" The Chinese presence was militarily insecure, facing constant threats from indigenous populations, and had failed to establish effective control or integration across the island. Essentially, Chinese authority existed more as a theoretical claim than as practical governance over Taiwan."),
            p!(EN "Perry also has written on the subject. In his report to Congress, Perry implored that the United States should make itself a greater presence in Asia, \"especially\" with regard to Taiwan", file &US_C033_S02_REPORT_N0097, EN ":"),
            p!(@Excerpt: EN "The day will however arrive, and at no distant period, when political events, and the unanimous and urgent appeals of our commercial men, will make it obligatory on the United States to look with greater solicitude to our eastern commerce, and to extend the advantages of our national friendship and protection, as well to Japan and <ins>Ryukyu</ins> as to other powers but little better known to western nations."),
            p!(@Excerpt: EN "I may refer to <ins>Thailand</ins>, Cambodia, <ins>Vietnam</ins>, parts of Borneo and Sumatra, and many of the islands of the eastern archipelago, and more especially to the island of Formosa."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "It only belongs to us to endeavor to act justly and honorably in all our foreign relations, and I cannot but believe that we should be just to ourselves and to the world, to <ins>encourage</ins> whatever practical measures might be suggested to change for the better the political and civil condition of China and Japan, and the countries more to the south; and especially with respect to Formosa. The United States alone should assume the initiative."),
            p!(EN "He then supplied his thoughts with relation to Taiwan:"),
            p!(@Excerpt: EN "The imperial authorities maintaining a feeble and precarious footing only in isolated parts of the island; a large portion being in possession of independent tribes, and yet such is its productiveness in minerals, drugs, and the more valuable products of those genial regions, that at this time a revenue, estimated at a million of dollars, is collected, though little or none of it goes into the imperial treasury."),
            p!(@Excerpt: EN "The inhabitants of the island may be divided into two classes: the first composed of those at present submitting to the authority of China, whether of native or Chinese blood; and the other, and probably the more numerous portion, of natives, yet in their unconquered and primitive state. The whole population has been estimated at two, and by some, as high as three millions; and looking to the peculiar abstemiousness of the people of the east, who rarely indulge in any other than vegetable food, it may reasonably be supposed that an island of the extent of Formosa, and of such fertility, could subsist even a larger number."),
            p!(EN "He even went so far as to advocate for American possession of the island:"),
            p!(@Excerpt: EN "It may, I think, be safely assumed that an American settlement at Kelung would be looked upon with favor by the Chinese<del>...</del>"),
            p!(@Excerpt: EN "Grants of land and important privileges, including the advantages of working the coal mines, could, doubtless, be obtained by purchase at nominal cost<del>...</del>"),
            p!(@Excerpt: EN "Kelung would become a port of general resort for vessels of all nations. The settlers might not only direct the working of the mines, already fully described in this volume, but the products of the land could be largely increased by the introduction of those improvements in agriculture and the mechanic arts which have been hitherto unknown to the Chinese<del>...</del>"),
            p!(@Excerpt: EN "An American settlement, once firmly established in Formosa, would gradually increase its social and political power, its area of landed possessions, and consequently its wealth and usefulness. Neither of the European governments could reasonably object to its advancement<del>...</del>"),
            p!(@Excerpt: EN "The geographical position of Formosa renders it eminently suited as an entrepôt for American trade, from which communications might be established with China, Japan, <ins>Ryukyu</ins>, <ins>Vietnam</ins>, Cambodia, <ins>Thailand</ins>, the Philippines, and all the islands situated in the adjacent seas; and it recommends itself more strongly from the fact of its capability of furnishing abundant supplies of coal, which, in the present and increasing use of steam for purposes of commerce, will prove of vast importance to the eastern trade.<del>...</del>"),
            p!(EN "Perry's report confirms that Chinese dominion over Taiwan remained extremely weak, describing Chinese authorities as maintaining \"a feeble and precarious footing only in isolated parts of the island\" with \"a large portion being in possession of independent tribes.\" Even more telling, while some revenue was collected from the island, \"little or none of it goes into the imperial treasury,\" indicating Chinese administrative control was so limited they couldn't effectively extract resources from their supposed territory. Perry saw this weakness as an opportunity for American expansion, believing the Chinese presence was so tenuous that an American settlement could gradually expand its influence across the island.")
        ]),
        block!("Bax Expedition, Revisited", [
            p!(@Initial: fig &BAX_BONHAM),
            p!(file &BOOK_BAX, EN ":"),
            p!(@Excerpt: EN "We remained here for a few days, and then went on to a large bay, about fifty miles to the southward, called <ins>Su'ao</ins> Bay, which is on the border between the Chinese and savage territory. It is a fine bay, but open to the east, and therefore unsafe in the typhoon season, except for junks and very small gun-vessels that can anchor behind the reef at the end of the bay. A Chinese village is built by a small river, and has a slightly cultivated valley around it. Everywhere else the mountains and hills come close down to the water. On the savage side they are densely wooded; but those hills on the Chinese side are bare of trees, not only on account of cultivation, but also to deprive the savages of the advantage of attacking them by crawling up under cover of the forest, which is their favourite mode of fighting. A few farms were scattered about the valley at the head of the bay, but the farmers had to go to their work in the fields with their weapons ready for use, and keeping one man on the look-out to give the alarm. Many of the fields nearest to the savages had evidently been uncultivated for some time on account of their savage neighbours."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "China <ins>has</ins> possession of a part of Formosa, and <ins>claims</ins> the whole of it."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "The savages held possession of the mountains, and were in a constant state of warfare with the Chinese settlers on the low country."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "All the villages near the S.W. point of Formosa were peopled by Chinese and <ins>Pepohoans</ins>, and owned allegiance to China. On the east side of the bay, the savage tribes held the country, and were constantly fighting with their more civilized neighbours, who have to keep their weapons by them ready for use while cultivating their fields.")
        ]),
        block!("Morrison on the Status of Taiwan", [
            p!(@Initial:
                fig &MORRISON_GABRIEL,
                EN " was a pioneering British civil engineer whose business ventures in China played a crucial role in the early development of the country's railway infrastructure during the late 19th century. Born in London and educated at Glasgow University under the renowned professor Lord Kelvin, Morrison brought technical expertise to the Far East, having previously participated in laying the first transatlantic telegraph cable."
            ),
            p!(EN "His most significant achievement came in 1876 when he served as engineer-in-charge of China's first railway project, successfully constructing and opening the line between Shanghai and Wusong under the consulting supervision of Sir G.B. Bruce, though the Chinese Government soon purchased and dismantled the railway."),
            p!(EN "Following this initial project, Morrison established himself permanently in Shanghai as a civil engineer and devoted considerable energy to promoting the development of trunk railway lines throughout China. His business approach was characterized by extensive travel through the Chinese interior."),
            p!(EN "In 1885, he formed the partnership of \"Morrison & Cratton\" with architect F.M. Cratton, and their firm became associated with many large engineering works undertaken in Shanghai and other Chinese cities. Morrison also served on the Shanghai Municipal Council and was the first President of the Society of Engineers and Architects in Shanghai, demonstrating his significant influence in the foreign business community."),
            p!(EN "In 1877, his \"business took him to the island\" of Taiwan, and he had his thoughts on the island published that very year", file &GAZETTE_GEO_MAG_V04, EN ":"),
            p!(@Excerpt: EN "It is only a very few years since the Island of Formosa, or Taiwan, as it is called by the Chinese, was practically an unknown land, and even now the number of Europeans who have penetrated into the interior is very small."),
            p!(@Excerpt: EN "In April 1877, business took me to the island, and during my stay I visited all the treaty ports, and journeyed overland from <ins>Tainan</ins> to Kelung, a distance by road of 218 miles."),
            p!(@Excerpt: EN "My business gave me the entree of the Governor's Yamen, and required that I should collect as much information as possible, from observation and from reports, as well as from interviews with those acquainted with the country. Although I did not penetrate into the savage district, and cannot therefore describe any previously unknown land, still I believe that much of the information contained in the following pages has never been published, and I have no doubt will be found useful by any one wishing to visit the island, or to make himself acquainted with its present condition or future prospects."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "Although nominally a part of China, the sovereignty of the Chinese only extends over about half the island; over certain other parts they exercise a nominal sway, but there are still some portions into which no Chinaman dares to enter."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "Even subsequent to the appointment of British consuls, the natives were in the habit of murdering the crews of any vessels wrecked on their coast. The crew of the U.S. vessel 'Rover' met with this fate, and the U.S. consul at Amoy, of whose district Formosa formed a part, seeing that it was useless at that time to negotiate with the Chinese authorities, who were powerless, determined to go direct to the savages."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "The island may be divided, either according to its physical character or according to its inhabitants; but, practically, it makes little difference which method is adopted, because, roughly speaking, the flat portion is civilised, while the mountainous portion is savage."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "The country which has just been described is the flat and civilised portion of the island. The hilly and uncivilised portion is of quite a different character. The west coast of the island is so low that it can hardly be seen at a distance of 3 or 4 miles, and houses and trees near the coast are liable to be mistaken for junks at a very slight distance. The east coast, on the other hand, is generally mountainous, with deep water very close to the shore. A ridge of mountains runs up the centre of the island. Mount Morrison, the highest of the range, is said to be 12,850 feet, and Mount Sylvia 11,300 feet. Mount Morrison is not named after the well known missionary and Chinese scholar, but after the captain of one of the early vessels trading to <ins>Tainan</ins>."),
            p!(@Excerpt: EN "All the land east of the mountains is supposed to be hilly, and inhabited by savages."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "For some time the Chinese have had military stationed on the borders of the savage country, trying to open up communication with the east coast. <ins>Su'ao</ins> Bay, on the east coast, and all to the north of it, have been in Chinese hands for many years. South of it, there are two Chinese settlements, called <ins>Xincheng</ins> and <ins>Hualien</ins>. The most northerly has been in existence twenty years, but has never numbered more than a hundred Chinese. In <ins>Hualien</ins>, on the other hand, there are about 700 Chinese and about 1000 semi-civilised savages."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "The people who inhabit the two parts of the island differ as much in their characteristics as do the two portions of country."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "In travelling through the civilised part of the island, it is a good thing to take a proper supply of provisions. Carrying coolies can easily be obtained, and no advantage is to be derived from going badly provided, except the advantage of being more than usually uncomfortable. <del>...</del> It must, of course, be understood that these remarks refer only to the civilised portion of the island. When the traveller makes up his mind to enter the borders of the savage country, he must cut down his baggage to the smallest possible dimensions, and, if he carries provisions, should only carry a small supply to guard against emergencies, and not for general use. And here it may be well to speak of carrying a revolver or other weapon."),
            p!(EN "Morrison's account once again reflects the limited nature of Chinese control over Taiwan during this period. His observation that Chinese sovereignty \"only extends over about half the island\" is  consistent with other contemporary assessments, with effective control confined largely to the western coastal plains while mountainous regions remained under indigenous control."),
            p!(EN "Morrison describes Chinese military forces \"stationed on the borders of the savage country, trying to open up communication with the east coast\"—language that suggests ongoing efforts at expansion. The minimal Chinese settlements on the east coast, with Xincheng numbering fewer than a hundred residents after twenty years, further illustrates the tenuous nature of Chinese territorial claims beyond the core administrative areas.")
        ])
    ])
}

fn ch_taiwan_expedition() -> Chapter {
    ("Taiwan Expedition", vec![
        block!("Chinese Absolve Responsibility", [
            p!(@Initial: fig &BAX_BONHAM),
            p!(file &BOOK_BAX, EN ":"),
            p!(@Excerpt: EN "Hitherto they had always replied to complaints of foreign nations, by stating that they had no control over the savage tribes on the east coast, and that the foreigners must punish them themselves.")
        ]),
        block!("Japanese Land Troops in Chinese Territory", [
            p!(@Initial: fig &BAX_BONHAM),
            p!(file &BOOK_BAX, EN ":"),
            p!(@Excerpt: EN "When the Chinese found what a formidable force had been landed at <ins>Hengchun</ins>, they denied that the Japanese had ever applied to them, or that they had ever given them permission to land an armed force in Formosa, as they now began to fear that the Japanese meditated the conquest of the whole island. From the size of the expedition, and from the reports of the Chinese officials of the number of Japanese, who, calling themselves merchants, had been travelling over and examining every part of the country for some time previously, their fears appeared to have some good foundation."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "When the Japanese had first landed, they made their camp among the low sand ridges near the beach; the camp was made of tents, and was clean and well kept. Afterwards, when the summer advanced and the weather became too hot for living under canvas, they moved to another camp that they had made of wooden huts with double roofs of thatch, about a mile farther off and more inland. The second camp had something the appearance of a Japanese village; the floors of the huts were raised off the ground to keep them dry, and a hospital was erected, and every precaution taken for the health of the troops. The first camp was broken up when the second one was finished. There were no warships at <ins>Hengchun</ins> when we first arrived, but afterwards the Japanese steam corvette and gunboat, and also transports, came in occasionally, bringing stores and munitions of war, and taking away any sick people that it was necessary to send to Japan. The vessels never remained longer than they could help at anchor in the bay."),
            p!(@Excerpt: EN "The Chinese town of <ins>Hengchun</ins> was situated about a mile and a half inland. The mountains commenced abruptly about two, or two and a half miles from the beach; between them and the sea the land was fairly cultivated with rice and sweet potatoes; several Chinese villages were scattered about. From this there could be no doubt that the place where the Japanese had made their camp was bonafide Chinese <ins>territory</ins>."),
            p!(@Excerpt: EN "<del>...</del>"),
            p!(@Excerpt: EN "I may as well here mention that, although I do not profess to form an opinion as to the course taken by the Japanese in avenging the death of their countrymen, whether they were right or wrong in landing such a large force in Chinese territory, the effect has been very beneficial to all nations that possess vessels trading in these seas, as it has caused the Chinese to send gun-boats and officials to several savage villages on the coast, and to endeavour to induce the chiefs to be kind to any shipwrecked people that should fall into their power in future. The Japanese quite ignored the old excuse of the Chinese that they had no control over those tribes, and therefore could not be responsible for their acts.")
        ]),
    ])
}

fn ch_western_sympathies() -> Chapter {
    ("Western \"Sympathies\"", vec![
        block!("Wade, of Britain", [

        ]),
        block!("Williams, of America", [

        ]),
        block!("de Geofroy, of France", [

        ]),
        block!("von Brandt, of Prussia", [
            p!(@Initial:
                fig &VONBRANDT_MAXIMILIAN,
                EN ", a career diplomat of Prussian origin, having served more-or-less a decade each in Japan and China, would later recount his understanding of events.",
                EN " He would only find office in China shortly <i>following</i> the events of 1874, therefor his perspective would have been from the vantage of his office in Japan."
            ),
            p!(EN "Some decades later, in 1897, he would publish a text detailing his work on the Asian continent."),
            p!(EN "He betrayed no bias to either Japan or China (having been involved with both by that point), and was quite critical of Japanese means and motivations, but none-the-less <i>does</i> place blame with the Chinese."),
            p!(EN "It became his view, after having been present in both Japan and China in official capacities, that", file &BOOK_BRANDT_1, EN ":"),
            p!(@Excerpt: EN "With characteristic clumsiness, China denied responsibility for these incidents, which gave rise to the Japanese expedition to South Formosa in 1874."),
            p!(EN "In 1901, another of his works, also recounting his thoughts and actions at the time, again places blame with the Chinese", file &BOOK_BRANDT_2, EN ":"),
            p!(@Excerpt: EN "The Mikado government <del>...</del> decided to take action against the savages on the southern tip of Formosa, since, based on the foolish declarations of the Tsungli Yamen, it believed it could assume that China would give Japan a free hand there. The new American ambassador, Mr. Bingham, seemed to share this view<del>...</del>"),
            p!(EN "He even recounts his efforts to dissuade the Japanese against their expedition", file &BOOK_BRANDT_2, EN ":"),
            p!(@Excerpt: EN "My English colleague and I, along with some of the other representatives, had done our best to dissuade the Japanese government from the venture, which would have required great sacrifices of money and men, promised no practical success, and could at least involve them in unpleasantness."),
            p!(EN "Notably, he does not list 'Chinese sovereignty' as a cause for concern.")
        ])
    ])
}

fn ch_wartime_agreements() -> Chapter {
    ("Wartime Agreements", vec![
        block!("Atlantic Charter", [
            subtitle!("Operation RIVIERA"),
            p!(@Initial: EN "",
            ),
        ]),
        block!("Declaration by the United Nations", [
            p!(@Initial: EN "",
                file &US_MEMO_1941_12_08_GAUSS,
                file &US_MEMO_1941_12_08_HULL,
                file &US_MEMO_1941_12_10_ROOSEVELT,
                file &US_MEMO_1941_12_12_WELLES,
                file &TW_MEMO_1941_12_12_SOONG,
                file &US_MEMO_1941_12_16_GAUSS,
                file &US_MEMO_1941_12_17_STIMSON,
                file &US_MEMO_1941_12_19_HULL,
                file &US_MEMO_1941_12_19_SAVAGE_1,
                file &US_MEMO_1941_12_19_SAVAGE_2,
                file &US_MEMO_1941_12_22_HULL,
                file &US_MEMO_1942_01_01_EARLY,
                file &US_MEMO_1942_01_01_ROOSEVELT,
                file &BOOK_HULL,
                file &BOOK_CHURCHILL_1,
                file &US_SERIAL_FRUS_1942_1942_V01,
                file &US_MEMO_1942_01_XX_CCS
            ),
            subtitle!("Operation ARCADIA"),
            p!(@Excerpt: EN "My associates and I had been preparing a comprehensive summary of our relations with Japan, culminating in Pearl Harbor, which the President might wish to send to Congress. We completed this about December 10, and Mr. Roosevelt sent it as a message to Congress on December 15. <del>...</del>"),
            subtitle!("Operation ARCADIA"),
            p!(@Excerpt: EN "My associates and I had been preparing a comprehensive summary of our relations with Japan, culminating in Pearl Harbor, which the President might wish to send to Congress. We completed this about December 10, and Mr. Roosevelt sent it as a message to Congress on December 15. <del>...</del>"),
            p!(@Excerpt: EN "I was then beginning to work, along with my associates at the State Department, on a far more important declaration—that which later was called the Declaration by United Nations. Almost immediately after Pearl Harbor I had begun to consider the form of unity the nations fighting against Hitler and Japan should take. <del>...</del>"),
            p!(@Excerpt: EN "On Saturday December 13 I asked Maxwell M. Hamilton, Chief of the Division of Far Eastern Affairs, to draw up a draft of a declaration to be made by the nations fighting the Axis, which would bind them together until victory and would commit them to the basic principles that we upheld. The following morning, Sunday, exactly one week after Pearl Harbor, I met with a group of my associates to discuss this declaration and another declaration setting up a Supreme War Council. <del>...</del>"),
            p!(@Excerpt: EN "From December 15 to December 19 my associates and I spent considerable time perfecting drafts of the document that became the United Nations Declaration. We also discussed another document that would have created a Supreme War Council composed of the heads or representatives of the Governments of the United Kingdom, the United States, China, and the U.S.S.R. The function of the War Council would be \"to supervise and coordinate the general conduct of the war and to provide for its successful prosecution.\""),
            p!(EN "TODO"),
            p!(@Excerpt: EN "I sent both papers to the President at the White House at one o'clock on December 19", file &US_MEMO_1941_12_19_HULL, file &US_SERIAL_FRUS_1942_1942_V01, EN ", along with a memorandum outlining the activities of the Supreme War Council of 1917-1918. <del>...</del>"),
            p!(@Excerpt: EN "In the Cabinet discussion that followed, there seemed to be general approval of both documents. The President raised only one point about which he had any hesitancy. This concerned the constitutionality of the Joint Declaration, which later became known officially as the Declaration by United Nations. He brought up especially the question whether he had the constitutional right to sign an agreement that pledged the signatories not to conclude a separate armistice or cease hostilities."),
            p!(@Excerpt: EN "When I returned to the State Department I called my advisers in for a discussion of this point. We generally felt that there was no real doubt as to the constitutionality of the provisions. I suggested, however, that the provision might well be taken from the Joint Declaration, which would be signed by many countries, and placed in the Supreme War Council agreement, which would be signed by four countries. We therefore prepared new drafts, also embodying two other minor changes, and I sent these to the White House that evening to be substituted for the two documents I had placed before the Cabinet.", file &US_MEMO_1941_12_19_SAVAGE_2),
            p!(@Excerpt: EN "December 22 I sent the President an eight-page opinion by Green H. Hackworth, Legal Adviser of the State Department, to the effect that there was not the slightest question about the constitutionality of the President's undertaking not to cease hostilities against or conclude a separate armistice with the common enemies or any of them except by common agreement.", file &US_MEMO_1941_12_22_HULL, EN " I wrote the President that, in my judgment, Hackworth's opinion \"makes clear the legal and constitutional validity of this provision.\" <del>...</del> The President thereupon agreed that this clause could well be reinserted in the Declaration."),
            p!(@Excerpt: EN "That day I gave the President a further memorandum on the Supreme War Council my associates and I proposed. <del>...</del> On the same day Prime Minister Churchill arrived in Washington. During the next few days the President went over with the Prime Minister our texts of the proposed Joint Declaration and of the proposal or the Supreme War Council. Mr. Churchill or Ambassador Halifax communicated these to London for the War Cabinet’s opinion."),
            p!(@Excerpt: EN "The President sent me on December 27 his written comments on the two texts. Churchill had received recommendations from the War Cabinet on the Joint Declaration and the President said he had taken these into account. <del>...</del> The President said he had been trying to think of a way to obviate the necessity of two different documents, the Joint Declaration and the Supreme War Council proposal. <del>...</del> To meet his thought that there should not be two separate documents, we prepared a draft combining the Joint Declaration and the Supreme War Council proposal."),
            p!(@Excerpt: EN "I now had ready three drafts to take to the White House—the new draft of the Joint Declaration, a copy of the December 19 draft on the Supreme War Council, and the amalgamated draft containing the Joint Declaration and the agreement on the Supreme War Council."),
            p!(@Excerpt: EN "Lord Halifax joined in the later stages of the conference in my office. <del>...</del> Halifax and I then went to the White House for our conference with the President and the Prime Minister. During the conference it was apparent that Mr. Roosevelt and Mr. Churchill were not yet in agreement that a Supreme War Council should be set up. Most of our discussion was devoted to this subject. We therefore agreed to adopt the new draft of the Joint Declaration, standing alone, leaving the Supreme War Council draft for further discussion. <del>...</del>"),
            p!(@Excerpt: EN "Two days later Litvinov came to my office with three amendments suggested by his Government.", file &US_MEMO_1941_12_29_HULL, EN " <del>...</del> The Ambassador seemed surprised when I said that the proposed declaration had not been delivered to any other country for signature pending Soviet Russia’s reaction. He had brought with him a separate declaration that Stalin planned to make upon the assumption that the Joint Declaration was already in circulation among other countries for signature. <del>...</del> I sent the Soviet amendments to the President later that day. <del>...</del> The President agreed to the incorporation of the Soviet amendments. <del>...</del>"),
            p!(@Excerpt: EN "The following day, December 31, I sent the Joint Declaration in its final form to the President for signature by himself and the Prime Minister. I suggested that the President might desire to ask Ambassador Litvinov and T. V. Soong, who had just been made Foreign Minister of China, to attach their signatures at the White House. <del>...</del> Later I telephoned the President and suggested that since there would not be enough signatures on the document by the next day, January 1, it would be preferable to wait a few days before releasing it. The President and I agreed that the Joint Declaration should be dated January 1 and made public on January 2 or 3. <del>...</del>"),
            p!(@Excerpt: EN "On the morning of December 31, while Prime Minister Churchill was having a bath in the White House, the President came to him and suggested that the Joint Declaration carry the title, \"Declaration by United Nations.\" The distinguished bather agreed, and thus the term \"United Nations\" came into being. The President had our final draft recopied at the White House so as to bear the new title."),
            p!(@Excerpt: EN "On the first day of the New Year, Roosevelt, Churchill, Litvinov, and Soong signed the historic document. <del>...</del> While these signatures were being affixed, I issued a public statement, saying: \"The Declaration by the United Nations joins together, in the greatest common war effort in history, the purpose and will of twenty-six free nations, representing the overwhelming majority of the inhabitants of all six continents. This is a living proof that law-abiding and peace-loving nations can unite in using the sword when necessary to preserve liberty and justice and the fundamental values of mankind. Against this host we can be sure that the forces of barbaric savagery and organized wickedness cannot and will not prevail.\""),
        ]),
        block!("Moscow Declarations", [
            p!(@Initial: EN "",

            )
        ]),
        block!("Cairo Declaration", [
            subtitle!("Operation SEXTANT"),
            p!(@Initial: EN "",
                file &US_MEMO_1942_03_07_GAUSS,
                file &US_MEMO_1942_04_08_GAUSS,
                file &US_MEMO_1942_04_16_GAUSS,
                file &US_MEMO_1942_05_26_GAUSS,
                file &US_MEMO_1942_07_14_GAUSS,
                file &GAZETTE_1942_11_04_CDN,
                file &US_MEMO_1943_03_09_DAVIES,
                file &US_MEMO_1943_05_18_CURRIE,
                file &US_MEMO_1943_10_18_GAUSS,
                file &US_MEMO_1943_11_30_STETTINIUS,
                file &US_MEMO_1943_12_02_HULL,
                file &US_MEMO_1943_12_09_SUMMERLIN,
                file &US_MEMO_1943_01_XX_CCS,
                file &US_MEMO_1943_12_XX_CCS
            )
        ]),
        block!("Tehran Declaration", [
            subtitle!("Operation EUREKA"),
            p!(@Initial: EN "",

            )
        ]),
        block!("Declaration on Liberated Europe", [
            subtitle!("Operation ARGONAUT"),
            p!(@Initial: EN "",

            )
        ]),
        block!("Potsdam Declaration", [
            subtitle!("Operation TERMINAL"),
            p!(@Initial: EN "",

            )
        ])
    ])
}

fn ch_post_surrender_policies() -> Chapter {
    ("Post Surrender Policies", vec![
        block!("SWNCC 68: National Composition of Forces to Occupy Formosa", [
            p!(@Initial: EN "", file &US_MEMO_1945_03_19_SWNCC, file &US_MEMO_1945_07_30_SWNCC_SFE, file &US_MEMO_1945_08_03_ROBERTS, file &US_MEMO_1945_08_06_SWNCC_SFE, file &US_MEMO_1945_08_10_MCFARLAND, file &US_MEMO_1945_08_23_SWNCC_SFE_1, file &US_MEMO_1945_08_23_SWNCC_SFE_2, file &US_MEMO_1945_09_19_SWNCC_SFE_1, file &US_MEMO_1945_09_21_SWNCC_1, file &US_MEMO_1945_09_25_SWNCC),
        ]),
        block!("SWNCC 150: Initial Post-Defeat Policy Relating to Japan", [
            p!(@Initial: EN "",
                file &US_MEMO_1944_05_04_BALLANTINE,
                file &US_MEMO_1945_06_11_SWNCC,
                file &US_MEMO_1945_08_11_SWNCC_1,
                file &US_MEMO_1945_08_11_SWNCC_2,
                file &US_MEMO_1945_08_12_SWNCC_1,
                file &US_MEMO_1945_08_12_SWNCC_2,
                file &US_MEMO_1945_08_22_SWNCC,
                file &US_MEMO_1945_08_23_SWNCC_SFE_3,
                file &US_MEMO_1945_08_30_SWNCC,
                file &US_MEMO_1945_08_31_SWNCC,
                file &US_MEMO_1945_09_06_SWNCC,
                file &US_MEMO_1945_09_17_SWNCC_SFE,
                file &US_MEMO_1945_09_19_SWNCC_SFE_2,
                file &US_MEMO_1945_09_19_SWNCC,
                file &US_MEMO_1945_09_21_SWNCC_1,
                file &US_MEMO_1945_09_21_SWNCC_2,
                file &US_MEMO_1945_09_24_SWNCC,
                file &US_MEMO_1945_11_03_MCFARLAND,
                file &US_MEMO_1945_11_27_SWNCC,
                file &US_MEMO_1945_12_07_SWNCC,
                file &US_MEMO_1945_12_14_SWNCC_SFE)
        ]),
        block!("SWNCC 181: Custody of Japanese Archives and Diplomatic Property", [
            p!(@Initial: EN "", file &US_MEMO_1945_08_29_SWNCC, file &US_MEMO_1945_08_31_SWNCC, file &US_MEMO_1945_09_01_SWNCC_SFE, file &US_MEMO_1945_09_03_SWNCC, file &US_MEMO_1945_09_04_SWNCC_SFE_1, file &US_MEMO_1945_09_04_SWNCC_SFE_2, file &US_MEMO_1945_09_10_SWNCC, file &US_MEMO_1945_09_12_SWNCC, file &US_MEMO_1945_09_13_ACHESON, file &US_MEMO_1945_09_21_SWNCC_1, file &US_MEMO_1945_09_21_SWNCC_3, file &US_MEMO_1945_09_24_ROSS, file &US_MEMO_1945_10_10_SWNCC),
        ]),
        block!("SWNCC 272: Sovereignty of Formosa", [
            p!(@Initial: EN ""),
        ])
    ])
}

fn ch_territorial_adjustments() -> Chapter {
    ("Territorial Adjustments", vec![
        block!("Poland", [
            p!(@Initial: EN "", file &US_MEMO_1945_07_21_THOMPSON, file &US_MEMO_1945_07_22_THOMPSON, file &US_MEMO_1945_07_27_THOMPSON, file &US_MEMO_1945_08_09_ROSS)
        ]),
        block!("Kuriles", [
            p!(@Initial: EN "", file &US_MEMO_1945_05_12_GREW_1, file &US_MEMO_1945_05_12_GREW_2)
        ])
    ])
}

fn ch_taiwan_status() -> Chapter {
    ("Taiwan Status: Awaiting Treaty", vec![
        block!("Hoover on Taiwan", [
            p!(@Initial:
                EN "In 1949, U.S. President ",
                fig &HOOVER_HERBERT,
                EN ", in letter to Senator ",
                fig &KNOWLAND_WILLIAM,
                EN ", provided \"an argument that could be made\" with respect to the legal status of Taiwan",
                file &BOOK_HOOVER,
                EN ":"
            ),
            p!(@Excerpt:
                EN "I do not think there is any purpose in my letter of today of introducing an argument in it as to the legal situation of Formosa. You might, however, think over an argument that could be made; namely, that Formosa and the Pescadores are still in ",
                fig &MACARTHUR_DOUGLAS,
                EN "'s jurisdiction. The Japanese acquired them from China in 1895. In subsequent years, we frequently ratified the <i>status quo</i> of their possessions."
            ),
            p!(@Excerpt: EN "The Cairo Declaration and the Potsdam surrender were executive acts which were never matters of treaties ratified by the Senate. Such a ratification would only come up in a treaty with Japan, and MacArthur might collaterally hold them pending a treaty.")
        ])
    ])
}

fn ch_on_the_nature_of_promises() -> Chapter {
    ("On the Nature of Promises", vec![
        block!("Declaration by the United Nations", [
            p!(@Initial: EN "",
                file &US_MEMO_1942_07_14_GAUSS,
                file &BOOK_STILWELL,
                file &US_MEMO_1943_12_09_GAUSS,
                file &US_MEMO_1942_02_11_GEROW,
                file &BOOK_MOUNTBATTEN
            ),
        ]),
        block!("Operation BUCCANEER", [
            p!(@Initial: EN "",
                file &US_MEMO_1944_XX_XX_MCCREA,
                file &US_MEMO_1944_06_22_VINCENT,
                file &BOOK_LEAHY,
                file &BOOK_KING,
                file &US_MEMO_1943_12_05_HOPKINS,
                file &US_MEMO_1943_12_06_ROOSEVELT,
                file &US_MEMO_1943_12_09_HEARN
            ),
        ]),
        block!("Equipped 90 Divisions", [
            p!(@Initial: EN "",
                file &US_MEMO_1945_08_11_HURLEY,
                file &US_MEMO_1945_09_03_BYRNES,
                file &US_MEMO_1945_09_03_ELSEY,
                file &US_MEMO_1945_09_03_HULL,
                file &US_MEMO_1945_09_05_LEAHY,
                file &US_MEMO_1945_09_07_ACHESON,
                file &US_MEMO_1945_09_12_SWNCC
            ),
        ])
    ])
}

fn ch_executive_authority() -> Chapter {
    ("Executive Authority", vec![
        block!("TODO", [
            p!(@Initial: EN "", file &US_MEMO_1945_08_09_ROSS),
        ])
    ])
}

fn sc_historical_status() -> Section {
    ("Historical Status: pre-1895", vec![
        ch_migration_ban(),
        ch_vicious_natives(),
        ch_divided_island(),
        ch_taiwan_expedition(),
        ch_western_sympathies()
    ])
}

fn sc_political_evolution() -> Section {
    ("Political Evolution: 1895-1995", vec![
        ch_wartime_agreements(),
        ch_post_surrender_policies(),
        ch_territorial_adjustments(),
        ch_taiwan_status(),
        ch_on_the_nature_of_promises(),
        ch_executive_authority()
    ])
}

fn sc_modern_developments() -> Section {
    ("Modern Developments: post-1995", vec![
    ])
}*/

use dioxus::prelude::*;
use reqwest;
use crate::politics::{Politics, stable_hash, IndexSection};

// --- ROUTE COMPONENTS ---

#[component]
pub fn PoliticsTaiwanSovereigntyIndex() -> Element {
    // You can hardcode your desired starting hash or fetch index.json here to grab the first one dynamically.
    use_effect(move || {
        use_navigator().replace(format!("/politics/tw/sovereignty/{}", stable_hash("Historical Status: pre-1895")));
    });

    rsx! { div { class: "loading-text", "Loading..." } }
}

#[component]
pub fn PoliticsTaiwanSovereigntyChapter(chapter_hash: String) -> Element {
    // Fetch the lightweight index file representing the navigation hierarchy.
    let sections_resource = use_resource(move || async move {
        let response = reqwest::get("/assets/data/politics/tw/sovereignty/index.json").await.ok()?;
        let data: Vec<IndexSection> = response.json().await.ok()?;
        Some(data)
    });

    match &*sections_resource.read() {
        Some(Some(sections)) => {
            rsx! {
                Politics {
                    sections: sections.clone(),
                    active_hash: chapter_hash,
                }
            }
        }
        Some(None) => rsx! { div { class: "loading-text", "Failed to parse index data." } },
        None => rsx! { div { class: "loading-text", "Loading navigation..." } },
    }
}