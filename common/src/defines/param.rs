pub trait CritterParam {
    fn param(&self, p: Param) -> i32 {
        self.params_all()[p as usize]
    }
    fn uparam(&self, p: Param) -> u32 {
        self.param(p) as u32
    }
    fn bparam(&self, p: Param) -> bool {
        self.param(p) != 0
    }
    fn params_range(&self, range: std::ops::Range<Param>) -> &[i32] {
        &self.params_all()[range.start as usize..range.end as usize]
    }
    fn params_range_inc(&self, range: std::ops::RangeInclusive<Param>) -> &[i32] {
        &self.params_all()[*range.start() as usize..=*range.end() as usize]
    }
    fn params_all(&self) -> &[i32];
}

#[cfg(feature = "server")]
pub trait CritterParamMut {
    fn param_mut(&mut self, p: Param) -> &mut i32 {
        &mut self.params_all_mut()[p as usize]
    }
    fn set_uparam(&mut self, p: Param, val: u32) {
        *self.param_mut(p) = val as i32;
    }
    fn params_all_mut(&mut self) -> &mut [i32];
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Param {
    //STAT_BEGIN = 0,
    //STAT_END = 199,
    //STAT_COUNT = 200,
    //STAT_EXT_BEGIN = 32,
    //STAT_EXT_END = 63,
    ST_STRENGTH = 0,
    ST_PERCEPTION = 1,
    ST_ENDURANCE = 2,
    ST_CHARISMA = 3,
    ST_INTELLECT = 4,
    ST_AGILITY = 5,
    ST_LUCK = 6,
    ST_MAX_LIFE = 7,
    ST_ACTION_POINTS = 8,
    ST_ARMOR_CLASS = 9,
    ST_MELEE_DAMAGE = 10,
    ST_CARRY_WEIGHT = 11,
    ST_SEQUENCE = 12,
    ST_HEALING_RATE = 13,
    ST_CRITICAL_CHANCE = 14,
    ST_MAX_CRITICAL = 15,
    ST_NORMAL_ABSORB = 16,
    ST_LASER_ABSORB = 17,
    ST_FIRE_ABSORB = 18,
    ST_PLASMA_ABSORB = 19,
    ST_ELECTRO_ABSORB = 20,
    ST_EMP_ABSORB = 21,
    ST_EXPLODE_ABSORB = 22,
    ST_NORMAL_RESIST = 23,
    ST_LASER_RESIST = 24,
    ST_FIRE_RESIST = 25,
    ST_PLASMA_RESIST = 26,
    ST_ELECTRO_RESIST = 27,
    ST_EMP_RESIST = 28,
    ST_EXPLODE_RESIST = 29,
    ST_RADIATION_RESISTANCE = 30,
    ST_POISON_RESISTANCE = 31,
    ST_STRENGTH_EXT = 32,
    ST_PERCEPTION_EXT = 33,
    ST_ENDURANCE_EXT = 34,
    ST_CHARISMA_EXT = 35,
    ST_INTELLECT_EXT = 36,
    ST_AGILITY_EXT = 37,
    ST_LUCK_EXT = 38,
    ST_MAX_LIFE_EXT = 39,
    ST_ACTION_POINTS_EXT = 40,
    ST_ARMOR_CLASS_EXT = 41,
    ST_MELEE_DAMAGE_EXT = 42,
    ST_CARRY_WEIGHT_EXT = 43,
    ST_SEQUENCE_EXT = 44,
    ST_HEALING_RATE_EXT = 45,
    ST_CRITICAL_CHANCE_EXT = 46,
    ST_MAX_CRITICAL_EXT = 47,
    ST_NORMAL_ABSORB_EXT = 48,
    ST_LASER_ABSORB_EXT = 49,
    ST_FIRE_ABSORB_EXT = 50,
    ST_PLASMA_ABSORB_EXT = 51,
    ST_ELECTRO_ABSORB_EXT = 52,
    ST_EMP_ABSORB_EXT = 53,
    ST_EXPLODE_ABSORB_EXT = 54,
    ST_NORMAL_RESIST_EXT = 55,
    ST_LASER_RESIST_EXT = 56,
    ST_FIRE_RESIST_EXT = 57,
    ST_PLASMA_RESIST_EXT = 58,
    ST_ELECTRO_RESIST_EXT = 59,
    ST_EMP_RESIST_EXT = 60,
    ST_EXPLODE_RESIST_EXT = 61,
    ST_RADIATION_RESISTANCE_EXT = 62,
    ST_POISON_RESISTANCE_EXT = 63,
    ST_TOXIC = 64,
    ST_RADIOACTIVE = 65,
    ST_KILL_EXPERIENCE = 66,
    ST_BODY_TYPE = 67,
    ST_LOCOMOTION_TYPE = 68,
    ST_DAMAGE_TYPE = 69,
    ST_AGE = 70,
    ST_GENDER = 71,
    ST_CURRENT_HP = 72,
    ST_POISONING_LEVEL = 73,
    ST_RADIATION_LEVEL = 74,
    ST_CURRENT_AP = 75,
    ST_EXPERIENCE = 76,
    ST_LEVEL = 77,
    ST_UNSPENT_SKILL_POINTS = 78,
    ST_UNSPENT_PERKS = 79,
    ST_KARMA = 80,
    ST_FOLLOW_CRIT = 81,
    ST_REPLICATION_MONEY = 82,
    ST_REPLICATION_COUNT = 83,
    ST_REPLICATION_TIME = 84,
    ST_REPLICATION_COST = 85,
    ST_TURN_BASED_AC = 86,
    ST_MAX_MOVE_AP = 87,
    ST_MOVE_AP = 88,
    ST_NPC_ROLE = 89,
    ST_VAR0 = 90,
    ST_VAR1 = 91,
    ST_VAR2 = 92,
    ST_VAR3 = 93,
    ST_VAR4 = 94,
    ST_VAR5 = 95,
    ST_VAR6 = 96,
    ST_VAR7 = 97,
    ST_VAR8 = 98,
    ST_VAR9 = 99,
    ST_PLAYER_KARMA = 100,
    ST_BONUS_LOOK = 101,
    ST_HANDS_ITEM_AND_MODE = 102,
    ST_FREE_BARTER_PLAYER = 103,
    ST_DIALOG_ID = 104,
    ST_AI_ID = 105,
    ST_TEAM_ID = 106,
    ST_BAG_ID = 107,
    ST_LAST_STEAL_CR_ID = 108,
    ST_STEAL_COUNT = 109,
    ST_LAST_WEAPON_ID = 110,
    ST_LAST_WEAPON_USE = 111,
    ST_BASE_CRTYPE = 112,
    ST_DEAD_BLOCKER_ID = 113,
    ST_CURRENT_ARMOR_PERK = 114,
    ST_TALK_DISTANCE = 115,
    ST_SCALE_FACTOR = 116,
    ST_WALK_TIME = 117,
    ST_RUN_TIME = 118,
    ST_MAX_TALKERS = 119,
    ST_SPEED_MOD = 120,
    ST_ARCADE_DEAD = 121,
    ST_LOCALMAP_DIR = 122,
    ST_LOCALMAP_PID = 123,
    ST_EXPERIENCE_MULTIPLIER = 124,
    ST_EXPERIENCE_MULTIPLIER_TIME = 125,
    ST_DYSPNEA = 126,
    ST_THRIST = 127,
    ST_HUNGER = 128,
    ST_WAYPOINT_GROUP = 129,
    ST_LAST_CONT_ID = 130,
    ST_LAST_DOOR_ID = 131,
    ST_LAST_CRIT_ID = 132,
    ST_OBJECT_TYPE = 133,
    ST_ITEM_HEXX = 134,
    ST_ITEM_HEXY = 135,
    ST_ITEM_PID = 136,
    ST_ACCESS_LEVEL = 137,
    ST_STUDY_EXP = 138,
    ST_STUDY_EXP_MAX = 139,
    ST_LTP_TYPE_DIR_HEX = 140,
    ST_LTP_MAP = 141,
    ST_LTP_TIME = 142,
    ST_LTP_PARAM0 = 143,
    ST_LTP_PARAM1 = 144,
    ST_LTP_PARAM2 = 145,
    ST_CRSTATUS = 146,
    ST_BLOOD_TOXIC = 147,
    ST_OVERDOSE = 148,
    ST_BLEED = 149,
    //ST_ORG_DMG = 150,
    ST_ANIM3D_LAYERS = 150,
    ST_PARALYSIS_LEVEL = 180,
    ST_WARNINGS = 181,
    HANDCUFFS = 182,
    ST_MUTATION = 183,
    ST_ARCADE_DANGER = 184,
    LAST_ATTACKED = 185,
    ST_BLOCK = 186,
    ST_DESEASE = 187,
    ST_KILLER_PERSON = 188,
    ST_QMAP = 189,
    ST_STAMINA = 190,
    ST_APREGEN = 192,
    SK_SMALL_GUNS = 200,
    SK_BIG_GUNS = 201,
    SK_ENERGY_WEAPONS = 202,
    SK_UNARMED = 203,
    SK_MELEE_WEAPONS = 204,
    SK_THROWING = 205,
    SK_FIRST_AID = 206,
    SK_DOCTOR = 207,
    SK_SNEAK = 208,
    SK_LOCKPICK = 209,
    SK_STEAL = 210,
    SK_TRAPS = 211,
    SK_SCIENCE = 212,
    SK_REPAIR = 213,
    SK_SPEECH = 214,
    SK_BARTER = 215,
    SK_GAMBLING = 216,
    SK_OUTDOORSMAN = 217,
    TAG_SKILL1 = 226,
    TAG_SKILL2 = 227,
    TAG_SKILL3 = 228,
    TAG_SKILL4 = 229,
    TO_SK_FIRST_AID = 230,
    TO_SK_DOCTOR = 231,
    TO_SK_REPAIR = 232,
    TO_SK_SCIENCE = 233,
    TO_SK_LOCKPICK = 234,
    TO_SK_STEAL = 235,
    TO_SK_OUTDOORSMAN = 236,
    TO_DEATH = 237,
    TO_BATTLE = 238,
    TO_TRANSFER = 239,
    TO_REMOVE_FROM_GAME = 240,
    TO_REPLICATION = 241,
    TO_TIREDNESS = 242,
    TO_SNEAK = 243,
    TO_HEALING = 244,
    TO_STEALING = 248,
    TO_AGGRESSOR = 249,
    TO_HAIR_GROW = 250,
    TO_SAY = 251,
    TO_DEAD = 252,
    PE_BOOKWORM = 300,
    PE_AWARENESS = 301,
    PE_BONUS_HTH_ATTACKS = 302,
    PE_BONUS_HTH_DAMAGE = 303,
    PE_BONUS_MOVE = 304,
    PE_BONUS_RANGED_DAMAGE = 305,
    PE_BONUS_RATE_OF_FIRE = 306,
    PE_EARLIER_SEQUENCE = 307,
    PE_FASTER_HEALING = 308,
    PE_MORE_CRITICALS = 309,
    PE_NIGHT_VISION = 310,
    PE_PRESENCE = 311,
    PE_RAD_RESISTANCE = 312,
    PE_TOUGHNESS = 313,
    PE_STRONG_BACK = 314,
    PE_SHARPSHOOTER = 315,
    PE_SILENT_RUNNING = 316,
    PE_SURVIVALIST = 317,
    PE_MASTER_TRADER = 318,
    PE_EDUCATED = 319,
    PE_HEALER = 320,
    PE_FORTUNE_FINDER = 321,
    PE_BETTER_CRITICALS = 322,
    PE_EMPATHY = 323,
    PE_SLAYER = 324,
    PE_SNIPER = 325,
    PE_SILENT_DEATH = 326,
    PE_ACTION_BOY = 327,
    PE_MENTAL_BLOCK = 328,
    PE_LIFEGIVER = 329,
    PE_DODGER = 330,
    PE_SNAKEATER = 331,
    PE_MR_FIXIT = 332,
    PE_MEDIC = 333,
    PE_MASTER_THIEF = 334,
    PE_SPEAKER = 335,
    PE_HEAVE_HO = 336,
    PE_FRIENDLY_FOE = 337,
    PE_PICKPOCKET = 338,
    PE_GHOST = 339,
    PE_CULT_OF_PERSONALITY = 340,
    PE_SCROUNGER = 341,
    PE_EXPLORER = 342,
    PE_FLOWER_CHILD = 343,
    PE_PATHFINDER = 344,
    PE_ANIMAL_FRIEND = 345,
    PE_SCOUT = 346,
    PE_MYSTERIOUS_STRANGER = 347,
    PE_RANGER = 348,
    PE_QUICK_POCKETS = 349,
    PE_SMOOTH_TALKER = 350,
    PE_SWIFT_LEARNER = 351,
    PE_TAG = 352,
    PE_MUTATE = 353,
    PE_ADRENALINE_RUSH = 380,
    PE_CAUTIOUS_NATURE = 381,
    PE_COMPREHENSION = 382,
    PE_DEMOLITION_EXPERT = 383,
    PE_GAMBLER = 384,
    PE_GAIN_STRENGTH = 385,
    PE_GAIN_PERCEPTION = 386,
    PE_GAIN_ENDURANCE = 387,
    PE_GAIN_CHARISMA = 388,
    PE_GAIN_INTELLIGENCE = 389,
    PE_GAIN_AGILITY = 390,
    PE_GAIN_LUCK = 391,
    PE_HARMLESS = 392,
    PE_HERE_AND_NOW = 393,
    PE_HTH_EVADE = 394,
    PE_KAMA_SUTRA_MASTER = 395,
    PE_KARMA_BEACON = 396,
    PE_LIGHT_STEP = 397,
    PE_LIVING_ANATOMY = 398,
    PE_MAGNETIC_PERSONALITY = 399,
    PE_NEGOTIATOR = 400,
    PE_PACK_RAT = 401,
    PE_PYROMANIAC = 402,
    PE_QUICK_RECOVERY = 403,
    PE_SALESMAN = 404,
    PE_STONEWALL = 405,
    PE_THIEF = 406,
    PE_WEAPON_HANDLING = 407,
    PE_VAULT_CITY_TRAINING = 408,
    PE_ALCOHOL_RAISED_HP = 409,
    PE_ALCOHOL_RAISED_HP_II = 410,
    PE_ALCOHOL_LOWERED_HP = 411,
    PE_ALCOHOL_LOWERED_HP_II = 412,
    PE_AUTODOC_RAISED_HP = 413,
    PE_AUTODOC_RAISED_HP_II = 414,
    PE_AUTODOC_LOWERED_HP = 415,
    PE_AUTODOC_LOWERED_HP_II = 416,
    PE_EXPERT_EXCREMENT = 417,
    PE_JINXED_II = 419,
    PE_TERMINATOR = 420,
    PE_GECKO_SKINNING = 430,
    PE_VAULT_CITY_INOCULATIONS = 431,
    PE_DERMAL_IMPACT = 432,
    PE_DERMAL_IMPACT_ENH = 433,
    PE_PHOENIX_IMPLANTS = 434,
    PE_PHOENIX_IMPLANTS_ENH = 435,
    PE_NCR_PERCEPTION = 436,
    PE_NCR_ENDURANCE = 437,
    PE_NCR_BARTER = 438,
    PE_NCR_REPAIR = 439,
    PE_COWBOY = 440,
    PE_FORT_LID_1 = 442,
    PE_FORT_LID_2 = 443,
    PE_FORT_LID_3 = 444,
    PE_ZLO_LID_1 = 445,
    PE_ZLO_LID_2 = 446,
    PE_ZLO_LID_3 = 447,
    PE_GM_CLON = 448,
    ADDICTION_NUKA_COLA = 470,
    ADDICTION_BUFFOUT = 471,
    ADDICTION_MENTATS = 472,
    ADDICTION_PSYCHO = 473,
    ADDICTION_RADAWAY = 474,
    ADDICTION_JET = 475,
    ADDICTION_TRAGIC = 476,
    KARMA_BERSERKER = 480,
    KARMA_CHAMPION = 481,
    KARMA_CHILDKILLER = 482,
    KARMA_SEXPERT = 483,
    KARMA_PRIZEFIGHTER = 484,
    KARMA_GIGOLO = 485,
    KARMA_GRAVE_DIGGER = 486,
    KARMA_MARRIED = 487,
    KARMA_PORN_STAR = 488,
    KARMA_SLAVER = 489,
    KARMA_VIRGIN_WASTES = 490,
    KARMA_MAN_SALVATORE = 491,
    KARMA_MAN_BISHOP = 492,
    KARMA_MAN_MORDINO = 493,
    KARMA_MAN_WRIGHT = 494,
    KARMA_SEPARATED = 495,
    KARMA_PEDOBEAR = 496,
    KARMA_VC_GUARDSMAN = 497,
    DAMAGE_POISONED = 500,
    DAMAGE_RADIATED = 501,
    DAMAGE_EYE = 502,
    DAMAGE_RIGHT_ARM = 503,
    DAMAGE_LEFT_ARM = 504,
    DAMAGE_RIGHT_LEG = 505,
    DAMAGE_LEFT_LEG = 506,
    //MODE_BEGIN = 510,
    //MODE_END = 549,
    //MODE_COUNT = 40,
    MODE_HIDE = 510,
    MODE_NO_STEAL = 511,
    MODE_NO_BARTER = 512,
    MODE_NO_ENEMY_STACK = 513,
    MODE_NO_PVP = 514,
    MODE_END_COMBAT = 515,
    MODE_DEFAULT_COMBAT = 516,
    MODE_NO_HOME = 517,
    MODE_GECK = 518,
    MODE_NO_FAVORITE_ITEM = 519,
    MODE_NO_ITEM_GARBAGER = 520,
    MODE_DLG_SCRIPT_BARTER = 521,
    MODE_UNLIMITED_AMMO = 522,
    MODE_NO_DROP = 523,
    MODE_NO_LOOSE_LIMBS = 524,
    MODE_DEAD_AGES = 525,
    MODE_NO_HEAL = 526,
    MODE_INVULNERABLE = 527,
    MODE_NO_FLATTEN = 528,
    MODE_SPECIAL_DEAD = 529,
    MODE_RANGE_HTH = 530,
    MODE_NO_KNOCK = 531,
    MODE_NO_LOOT = 532,
    MODE_NO_SUPPLY = 533,
    MODE_NO_KARMA_ON_KILL = 534,
    MODE_BARTER_ONLY_CASH = 535,
    MODE_NO_PUSH = 536,
    MODE_NO_UNARMED = 537,
    MODE_NO_AIM = 538,
    MODE_NO_WALK = 539,
    MODE_NO_RUN = 540,
    MODE_NO_TALK = 541,
    TRAIT_FAST_METABOLISM = 550,
    TRAIT_BRUISER = 551,
    TRAIT_SMALL_FRAME = 552,
    TRAIT_ONE_HANDER = 553,
    TRAIT_FINESSE = 554,
    TRAIT_KAMIKAZE = 555,
    TRAIT_HEAVY_HANDED = 556,
    TRAIT_FAST_SHOT = 557,
    TRAIT_BLOODY_MESS = 558,
    TRAIT_JINXED = 559,
    TRAIT_GOOD_NATURED = 560,
    TRAIT_CHEM_RELIANT = 561,
    TRAIT_CHEM_RESISTANT = 562,
    TRAIT_SEX_APPEAL = 563,
    TRAIT_SKILLED = 564,
    TRAIT_NIGHT_PERSON = 565,
    REPUTATION_DEN = 570,
    REPUTATION_KLAMATH = 571,
    REPUTATION_MODOC = 572,
    REPUTATION_VAULT_CITY = 573,
    REPUTATION_GECKO = 574,
    REPUTATION_BROKEN_HILLS = 575,
    REPUTATION_NEW_RENO = 576,
    REPUTATION_SIERRA = 577,
    REPUTATION_VAULT15 = 578,
    REPUTATION_NCR = 579,
    REPUTATION_CATHEDRAL = 580,
    REPUTATION_SAD = 581,
    REPUTATION_REDDING = 582,
    REPUTATION_SF = 583,
    REPUTATION_NAVARRO = 584,
    REPUTATION_ARROYO = 585,
    REPUTATION_PRIMAL_TRIBE = 586,
    REPUTATION_RANGERS = 587,
    REPUTATION_VAULT13 = 588,
    REPUTATION_SACRAMENTO = 589,
    QST_MEDIUM = 700,
    QST_INVIS = 701,
    QST_VISION = 702,
    QST_GAMEMODE = 703,
    ST_CELL_X = 704,
    ST_CELL_Y = 705,
    CR_HEXX = 706,
    CR_HEXY = 707,
    CR_VAL0 = 708,
    CR_VAL1 = 709,
    CR_VAL2 = 710,
    CR_VAL3 = 711,
    CR_VAL4 = 712,
    CR_VAL5 = 713,
    CR_VAL6 = 714,
    CR_VAL7 = 715,
    CR_VAL8 = 716,
    CR_VAL9 = 717,
    CR_ROULETTE_START_MONEY = 718,
    BP_ROBOT_SELF_REPAIR = 720,
    BP_BLOOD_LOSS = 721,
    CR_STRENGTH = 722,
    CR_PERCEPTION = 723,
    CR_AGILITY = 724,
    CR_ENDURANCE = 725,
    CR_IS_AGGRESSIVE = 726,
    CR_SLEEPING_STATE = 727,
    CR_SINF_MODE = 728,
    CR_LAST_ROTTEN_CYCLE = 729,
    CR_FIXED_SPEED = 730,
    CR_IS_WINDUPED = 731,
    CR_AUTO_AIM = 732,
    P_CRITTER_ID = 760,
    P_BODYSWAPPED = 761,
    QST_CHAR_VER = 790,
    QST_CHAR_SECRET = 791,
    //MERC_BEGIN = 800,
    //MERC_END = 810,
    MERC_MASTER_ID = 800,
    MERC_ALWAYS_RUN = 801,
    MERC_CANCEL_ON_ATTACK = 802,
    MERC_LOSE_DIST = 803,
    MERC_MASTER_DIST = 804,
    MERC_TYPE = 805,
    MERC_DEFEND_MASTER = 806,
    MERC_ASSIST_MASTER = 807,
    MERC_CANCEL_TIME = 808,
    MERC_CANCEL_ON_GLOBAL = 809,
    MERC_WAIT_FOR_MASTER = 810,
    PARAMS_COUNT = 1000,
}
