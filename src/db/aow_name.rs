pub mod weapon_name {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;

    pub static AOW_NAME: Lazy<Mutex<HashMap<u32, &str>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            (10,"test gem 1"),
            (20,"test gem 2"),
            (30,"test gem 3"),
            (40,"Ash of War: Lion's Claw"),
            (41,"Ash of War: Lion's Claw"),
            (80,"Ash of War: Lion's Claw"),
            (81,"Ash of War: Lion's Claw"),
            (82,"Ash of War: Lion's Claw"),
            (83,"Ash of War: Lion's Claw"),
            (84,"Ash of War: Lion's Claw"),
            (85,"Ash of War: Lion's Claw"),
            (86,"Ash of War: Lion's Claw"),
            (87,"Ash of War: Lion's Claw"),
            (88,"Ash of War: Lion's Claw"),
            (89,"Ash of War: Lion's Claw"),
            (90,"Ash of War: Lion's Claw"),
            (100,"Ash of War: Lion's Claw"),
            (101,"Ash of War: Impaling Thrust"),
            (102,"Ash of War: Piercing Fang"),
            (103,"Ash of War: Spinning Slash"),
            (104,"Ash of War:"),
            (105,"Ash of War: Charge Forth"),
            (106,"Ash of War: Stamp (Upward Cut)"),
            (107,"Ash of War: Stamp (Sweep)"),
            (108,"Ash of War: Blood Tax"),
            (109,"Ash of War: Repeating Thrust"),
            (110,"Ash of War: Wild Strikes"),
            (111,"Ash of War: Spinning Strikes"),
            (112,"Ash of War: Double Slash"),
            (113,"Ash of War: Prelate's Charge"),
            (114,"Ash of War: Unsheathe"),
            (115,"Ash of War: Square Off"),
            (116,"Ash of War: Giant Hunt"),
            (117,"Ash of War: Torch Attack"),
            (118,"Ash of War: Loretta's Slash"),
            (119,"Ash of War: Poison Moth Flight"),
            (120,"Ash of War: Spinning Weapon"),
            (121,"Ash of War:"),
            (122,"Ash of War: Storm Assault"),
            (123,"Ash of War: Stormcaller"),
            (124,"Ash of War: Sword Dance"),
            (125,"Ash of War: Sacred Blade"),
            (126,"Ash of War: Ice Spear"),
            (127,"Ash of War: Glintstone Pebble"),
            (128,"Ash of War: Bloody Slash"),
            (129,"Ash of War: Lifesteal Fist"),
            (130,"Ash of War: Eruption"),
            (131,"Ash of War: Prayerful Strike"),
            (132,"Ash of War: Gravitas"),
            (133,"Ash of War: Storm Blade"),
            (134,"Ash of War: Phantom Slash"),
            (135,"Ash of War: Earthshaker"),
            (136,"Ash of War: Golden Land"),
            (137,"Ash of War: Flaming Strike"),
            (138,"Ash of War: Thunderbolt"),
            (139,"Ash of War: Lightning Slash"),
            (140,"Ash of War: Carian Grandeur"),
            (141,"Ash of War: Carian Greatsword"),
            (142,"Ash of War: Vacuum Slice"),
            (143,"Ash of War: Black Flame Tornado"),
            (144,"Ash of War: Sacred Ring of Light"),
            (146,"Ash of War: Shield Bash"),
            (147,"Ash of War: Barricade Shield"),
            (148,"Ash of War: Parry"),
            (149,"Ash of War: Buckler Parry"),
            (151,"Ash of War: Carian Retaliation"),
            (152,"Ash of War: Storm Wall"),
            (153,"Ash of War: Golden Parry"),
            (154,"Ash of War: Shield Crash"),
            (155,"Ash of War: Through and Through"),
            (156,"Ash of War: Barrage"),
            (157,"Ash of War: Mighty Shot"),
            (158,"Ash of War: Enchanted Shot"),
            (159,"Ash of War: Sky Shot"),
            (160,"Ash of War: Rain of Arrows"),
            (161,"Ash of War:"),
            (163,"Ash of War:"),
            (164,"Ash of War: Hoarfrost Stomp"),
            (165,"Ash of War: Storm Stomp"),
            (166,"Ash of War: Kick"),
            (167,"Ash of War: Lightning Ram"),
            (168,"Ash of War: Flame of the Redmanes"),
            (169,"Ash of War: Ground Slam"),
            (170,"Ash of War: Golden Slam"),
            (171,"Ash of War: Waves of Darkness"),
            (173,"Ash of War: Determination"),
            (174,"Ash of War: Assassin's Gambit"),
            (175,"Ash of War: Golden Vow"),
            (176,"Ash of War: Sacred Order"),
            (177,"Ash of War: Seppuku"),
            (178,"Ash of War: Cragblade"),
            (179,"Ash of War: Shared Order"),
            (180,"Ash of War: Barbaric Roar"),
            (181,"Ash of War: Beast's Roar"),
            (182,"Ash of War:"),
            (183,"Ash of War: Braggart's Roar"),
            (184,"Ash of War: Troll's Roar"),
            (185,"Ash of War: Endure"),
            (186,"Ash of War: Vow of the Indomitable"),
            (187,"Ash of War: Holy Ground"),
            (190,"Ash of War: Quickstep"),
            (191,"Ash of War: Bloodhound's Step"),
            (192,"Ash of War: Raptor of the Mists"),
            (194,"Ash of War: White Shadow's Lure"),
            (195,"Ash of War: Sword Dance"),
            (196,"Ash of War: Firebreather"),
            (197,"Ash of War: Blood Blade"),
            (198,"Ash of War: Thops's Barrier"),
            (1000,"Ash of War: Lion's Claw"),
            (1001,"Ash of War: Impaling Thrust"),
            (1002,"Ash of War: Spinning Slash"),
            (1003,"Ash of War:"),
            (1004,"Ash of War: Glintblade Phalanx"),
            (1005,"Ash of War:"),
            (1006,"Ash of War: Eruption"),
            (1007,"Ash of War: Gravitas"),
            (1008,"Ash of War: Storm Blade"),
            (1009,"Ash of War: Earthshaker"),
            (1010,"Ash of War: Flaming Strike"),
            (1011,"Ash of War: Thunderbolt"),
            (1012,"Ash of War: Carian Greatsword"),
            (1013,"Ash of War: Storm Stomp"),
            (1014,"Ash of War: Determination"),
            (1015,"Ash of War: Holy Ground"),
            (1016,"Ash of War: Quickstep"),
            (1017,"Ash of War: Raptor of the Mists"),
            (10000,"Ash of War: Lion's Claw"),
            (10100,"Ash of War: Impaling Thrust"),
            (10200,"Ash of War: Piercing Fang"),
            (10300,"Ash of War: Spinning Slash"),
            (10500,"Ash of War: Charge Forth"),
            (10600,"Ash of War: Stamp (Upward Cut)"),
            (10700,"Ash of War: Stamp (Sweep)"),
            (10800,"Ash of War: Blood Tax"),
            (10900,"Ash of War: Repeating Thrust"),
            (11000,"Ash of War: Wild Strikes"),
            (11100,"Ash of War: Spinning Strikes"),
            (11200,"Ash of War: Double Slash"),
            (11300,"Ash of War: Prelate's Charge"),
            (11400,"Ash of War: Unsheathe"),
            (11500,"Ash of War: Square Off"),
            (11600,"Ash of War: Giant Hunt"),
            (11800,"Ash of War: Loretta's Slash"),
            (11900,"Ash of War: Poison Moth Flight"),
            (12000,"Ash of War: Spinning Weapon"),
            (12200,"Ash of War: Storm Assault"),
            (12300,"Ash of War: Stormcaller"),
            (12400,"Ash of War: Sword Dance"),
            (20000,"Ash of War: Glintblade Phalanx"),
            (20100,"Ash of War: Sacred Blade"),
            (20200,"Ash of War: Ice Spear"),
            (20300,"Ash of War: Glintstone Pebble"),
            (20400,"Ash of War: Bloody Slash"),
            (20500,"Ash of War: Lifesteal Fist"),
            (20700,"Ash of War: Eruption"),
            (20800,"Ash of War: Prayerful Strike"),
            (20900,"Ash of War: Gravitas"),
            (21000,"Ash of War: Storm Blade"),
            (21200,"Ash of War: Earthshaker"),
            (21300,"Ash of War: Golden Land"),
            (21400,"Ash of War: Flaming Strike"),
            (21600,"Ash of War: Thunderbolt"),
            (21700,"Ash of War: Lightning Slash"),
            (21800,"Ash of War: Carian Grandeur"),
            (21900,"Ash of War: Carian Greatsword"),
            (22000,"Ash of War: Vacuum Slice"),
            (22100,"Ash of War: Black Flame Tornado"),
            (22200,"Ash of War: Sacred Ring of Light"),
            (22400,"Ash of War: Blood Blade"),
            (22500,"Ash of War: Phantom Slash"),
            (22600,"Ash of War: Spectral Lance"),
            (22700,"Ash of War: Chilling Mist"),
            (22800,"Ash of War: Poisonous Mist"),
            (30000,"Ash of War: Shield Bash"),
            (30100,"Ash of War: Barricade Shield"),
            (30200,"Ash of War: Parry"),
            (30500,"Ash of War: Carian Retaliation"),
            (30600,"Ash of War: Storm Wall"),
            (30700,"Ash of War: Golden Parry"),
            (30800,"Ash of War: Shield Crash"),
            (30900,"Ash of War: No Skill"),
            (31000,"Ash of War: Thops's Barrier"),
            (40000,"Ash of War: Through and Through"),
            (40100,"Ash of War: Barrage"),
            (40200,"Ash of War: Mighty Shot"),
            (40400,"Ash of War: Enchanted Shot"),
            (40500,"Ash of War: Sky Shot"),
            (40600,"Ash of War: Rain of Arrows"),
            (50100,"Ash of War: Hoarfrost Stomp"),
            (50200,"Ash of War: Storm Stomp"),
            (50300,"Ash of War: Kick"),
            (50400,"Ash of War: Lightning Ram"),
            (50500,"Ash of War: Flame of the Redmanes"),
            (50600,"Ash of War: Ground Slam"),
            (50700,"Ash of War: Golden Slam"),
            (50800,"Ash of War: Waves of Darkness"),
            (50900,"Ash of War: Hoarah Loux's Earthshaker"),
            (60000,"Ash of War: Determination"),
            (60100,"Ash of War: Royal Knight's Resolve"),
            (60200,"Ash of War: Assassin's Gambit"),
            (60300,"Ash of War: Golden Vow"),
            (60400,"Ash of War: Sacred Order"),
            (60500,"Ash of War: Shared Order"),
            (60600,"Ash of War: Seppuku"),
            (60700,"Ash of War: Cragblade"),
            (65000,"Ash of War: Barbaric Roar"),
            (65100,"Ash of War: War Cry"),
            (65200,"Ash of War: Beast's Roar"),
            (65300,"Ash of War: Troll's Roar"),
            (65400,"Ash of War: Braggart's Roar"),
            (70000,"Ash of War: Endure"),
            (70100,"Ash of War: Vow of the Indomitable"),
            (70200,"Ash of War: Holy Ground"),
            (80000,"Ash of War: Quickstep"),
            (80100,"Ash of War: Bloodhound's Step"),
            (80200,"Ash of War: Raptor of the Mists"),
            (85000,"Ash of War: White Shadow's Lure"),
        ]))
    });
}