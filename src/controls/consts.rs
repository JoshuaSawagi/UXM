use smash::lib::LuaConst;

//COMMON
        //Ledge_const
pub const FIGHTER_STATUS_WORK_FLAG_DISABLE_CLIFF_CHECK: i32 = 0x20000000;

// Add this to your list of fighter instance work IDs
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_PIVOT_STICK_X_PREV: i32 = 0x2000;

pub const FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID: smash::lib::LuaConst = smash::lib::LuaConst::new(0x100000EE);

pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK: i32 = 0x20000116; //Indicates all fighters are on their last stock
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED: i32 = 0x20000117; //Tracks if the ball has bounced at least once since being thrown
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START: i32 = 0x20000118;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER: i32 = 496;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK: i32 = 497; //Tracks if a fighter used a certain special move in the air
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD: i32 = 498;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED: i32 = 499;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT: i32 = 500;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT: i32 = 501;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE: i32 = 502;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK: i32 = 503;
pub const FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT: i32 = 504; //Tracks if a player got hit during One-Hit mode
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW: i32 = 505;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH: LuaConst = LuaConst::new(506);
pub const FIGHTER_INSTANCE_WORK_ID_INT_MASHING: i32 = 507;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRIED: i32 = 508;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER: i32 = 509;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO: i32 = 510; //Returns false for exactly one frame after is_ready_go becomes true, used to initiate certain events exactly once at the start of a match
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER: i32 = 511;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_TIMER: i32 = 512;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL: i32 = 513;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_EFFECT: i32 = 514;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS: i32 = 515; //Flags when you just used a Final Smash in Special Smash
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE: i32 = 516;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_XLU_FRAME: i32 = 517;
pub const FIGHTER_INSTANCE_WORK_ID_INT_LANDING_FRAME: i32 = 518;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CHARGE: LuaConst = LuaConst::new(0x100000BD);
pub const FIGHTER_INSTANCE_WORK_ID_INT_FRAME_COUNTER: LuaConst = LuaConst::new(0x100000BE);
pub const FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER: LuaConst = LuaConst::new(0x100000BF);
pub const FIGHTER_INSTANCE_WORK_ID_INT_SOUND_COUNTER: LuaConst = LuaConst::new(0x100000C0);
pub const FIGHTER_INSTANCE_WORK_ID_INT_CRITICAL_COUNTER: LuaConst = LuaConst::new(0x100000C1);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_EFFECT_SCALE_MUL: LuaConst = LuaConst::new(0x4D);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_BUTTON_ON_FRAME: LuaConst = LuaConst::new(0x4E);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_STICK_DIRECTION: LuaConst = LuaConst::new(0x4F);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_CSTICK_DIRECTION: LuaConst = LuaConst::new(0x50);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_CRITICAL_ON_FRAME: LuaConst = LuaConst::new(0x51);
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FRAME: LuaConst = LuaConst::new(0x642);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI: LuaConst = LuaConst::new(0x2000012C);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW: LuaConst = LuaConst::new(0x2000012D);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N: LuaConst = LuaConst::new(0x2000012E);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S: LuaConst = LuaConst::new(0x2000012F);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_FLOAT: LuaConst = LuaConst::new(0x20000130);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CRITICAL: LuaConst = LuaConst::new(0x20000131);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_OMNI_FLOAT: LuaConst = LuaConst::new(0x20000132);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_S4_IS_CHARGED: LuaConst = LuaConst::new(0x20000133);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_HI4_IS_CHARGED: LuaConst = LuaConst::new(0x20000134);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_LW4_IS_CHARGED: LuaConst = LuaConst::new(0x20000135);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES: LuaConst = LuaConst::new(0x20000136);
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ULTRA_ARMOR: LuaConst = LuaConst::new(0x20000137);
pub const FIGHTER_STATUS_WORK_FLAG_ULTRA_ARMOR: LuaConst = LuaConst::new(0x21000030);
pub const FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL: LuaConst = LuaConst::new(0x2100002A);
pub const FIGHTER_STATUS_GLIDE_WORK_FLOAT_GLIDE_FRAME: LuaConst = LuaConst::new(0x100000B);
pub const FIGHTER_STATUS_GLIDE_WORK_INT_PITCH_SE: LuaConst = LuaConst::new(0x11000007);
pub const FIGHTER_STATUS_GLIDE_WORK_INT_STOP_SE: LuaConst = LuaConst::new(0x11000008);
pub const FIGHTER_STATUS_JUMP_FLAG_ASCENSION_REVALI_ENABLE: LuaConst = LuaConst::new(0x21000018);
pub const FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_N: LuaConst = LuaConst::new(0x21000023);
pub const FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_S: LuaConst = LuaConst::new(0x21000024);
pub const FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_HI: LuaConst = LuaConst::new(0x21000025);
pub const FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_LW: LuaConst = LuaConst::new(0x21000026);
pub const FIGHTER_STATUS_FLOAT_WORK_FLAG_INHERIT_AERIAL: LuaConst = LuaConst::new(0x2100000D);
pub const FIGHTER_STATUS_FLOAT_WORK_FLAG_IS_FLOAT: LuaConst = LuaConst::new(0x2100000E);
pub const FIGHTER_STATUS_FLOAT_WORK_FLAG_TURN: LuaConst = LuaConst::new(0x2100000F);
pub const FIGHTER_STATUS_FLOAT_WORK_INT_TIME: LuaConst = LuaConst::new(0x11000006);
pub const FIGHTER_STATUS_FLOAT_WORK_INT_ENABLE_UNIQ: LuaConst = LuaConst::new(0x11000007);
pub const FIGHTER_STATUS_FLOAT_WORK_INT_MTRANS: LuaConst = LuaConst::new(0x11000008);
pub const FIGHTER_STATUS_FLOAT_WORK_FLOAT_ROT_Y: LuaConst = LuaConst::new(0x1000009);
pub const FIGHTER_STATUS_FLOAT_WORK_FLOAT_TURN_DIR: LuaConst = LuaConst::new(0x100000A);
pub const FIGHTER_STATUS_KIND_SPECIAL_GUARD: LuaConst = LuaConst::new(0x643);
pub const FIGHTER_STATUS_KIND_FLOAT: LuaConst = LuaConst::new(0x644);
pub const COLLISION_KIND_MASK_PARRY: LuaConst = LuaConst::new(0x80);



//DEDEDE
pub const FIGHTER_DEDEDE_STATUS_KIND_APPEAL_SPECIAL: LuaConst = LuaConst::new(0x110);
pub const FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_BGM_ID: LuaConst = LuaConst::new(0x100000C4);

//DEMON
pub const FIGHTER_DEMON_STATUS_ATTACK_WORK_FLAG_CRITICAL: LuaConst = LuaConst::new(0x2100002A);

//DIDDY
pub const FIGHTER_DIDDY_INSTANCE_WORK_ID_FLAG_BANANA_EXIST: LuaConst = LuaConst::new(0x200000E2);
pub const FIGHTER_DIDDY_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LAUGH_TRIGGER: LuaConst = LuaConst::new(0x200000E4);

//DONKEY
pub const FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_APPEAL_SPECIAL: LuaConst = LuaConst::new(0x200000E1);
pub const FIGHTER_DONKEY_STATUS_KIND_SPECIAL_HI2 : LuaConst = LuaConst::new(0x1D3);


//LINK
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY: LuaConst = LuaConst::new(0x200000E9);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL: LuaConst = LuaConst::new(0x200000EA);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM: LuaConst = LuaConst::new(0x200000EB);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED: LuaConst = LuaConst::new(0x200000EC);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND: LuaConst = LuaConst::new(0x200000ED);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID: LuaConst = LuaConst::new(0x100000C6);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO: LuaConst = LuaConst::new(0x100000C7);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE: LuaConst = LuaConst::new(0x100000C8);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE: LuaConst = LuaConst::new(0x100000C9);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID: LuaConst = LuaConst::new(0x100000CA);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME: LuaConst = LuaConst::new(0x100000CB);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_INT_JUMP_BUTTON_ON_FRAME: LuaConst = LuaConst::new(0x100000CC);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y: LuaConst = LuaConst::new(0x100000CB);
pub const FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y: LuaConst = LuaConst::new(0x100000CC);
pub const FIGHTER_LINK_STATUS_KIND_ATTACK_DASH_BOUND: LuaConst = LuaConst::new(0x1E9);
pub const FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE: LuaConst = LuaConst::new(0x1EA);
pub const FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_TURN: LuaConst = LuaConst::new(0x1EB);
pub const FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_DROP: LuaConst = LuaConst::new(0x1EC);
pub const FIGHTER_LINK_STATUS_KIND_REVALI_GLIDE_LANDING: LuaConst = LuaConst::new(0x1ED);
pub const FIGHTER_LINK_STATUS_KIND_ASCEND_START: LuaConst = LuaConst::new(0x1EE);
pub const FIGHTER_LINK_STATUS_KIND_ASCEND: LuaConst = LuaConst::new(0x1EF);
pub const FIGHTER_LINK_STATUS_KIND_ASCEND_END: LuaConst = LuaConst::new(0x1F0);
pub const FIGHTER_LINK_STATUS_KIND_ASCEND_JUMP_GROUND: LuaConst = LuaConst::new(0x1F1);
pub const FIGHTER_LINK_STATUS_KIND_ASCEND_JUMP_GROUND_END: LuaConst = LuaConst::new(0x1F2);
pub const FIGHTER_LINK_GENERATE_ARTICLE_CLAWSHOT: LuaConst = LuaConst::new(0x9);
pub const FIGHTER_LINK_GENERATE_ARTICLE_CLAWSHOT_HAND: LuaConst = LuaConst::new(0xA);
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND: LuaConst = LuaConst::new(0x1000000E);
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG: LuaConst = LuaConst::new(0x1000000F);
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS: LuaConst = LuaConst::new(0x10000010);
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID: LuaConst = LuaConst::new(0x10000011);
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED: LuaConst = LuaConst::new(0x20000009);
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT: LuaConst = LuaConst::new(0x2000000A);
pub const WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW: LuaConst = LuaConst::new(0x2000000B);
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND: LuaConst = LuaConst::new(0x1000000C);
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS: LuaConst = LuaConst::new(0x1000000D);
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID: LuaConst = LuaConst::new(0x1000000E);
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED: LuaConst = LuaConst::new(0x2000000E);
pub const WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_FUSE_REFLECT: LuaConst = LuaConst::new(0x2000000F);

//KIRBY
pub const FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI_H: LuaConst = LuaConst::new(0x3E6);
pub const FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE:  LuaConst = LuaConst::new(0x1000010B);
pub const FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO: LuaConst = LuaConst::new(0x1000010C);
pub const FIGHTER_KIRBY_STATUS_SPECIAL_HI_FLAG_CANCEL: LuaConst = LuaConst::new(0x2100001E);

//KNUCKLES
pub const THROW_HI_FRAME_FALL: f32 = 30.0;
pub const THROW_HI_FRAME_LAND: f32 = 39.0;

//MARIO
pub const FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL: LuaConst = LuaConst::new(0x2100000E);
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_BUTTON_ON_FRAME: LuaConst = LuaConst::new(0x100000BF);

//METAKNIGHT
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_N: LuaConst = LuaConst::new(0x200000E6);
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S: LuaConst = LuaConst::new(0x200000E7);
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI: LuaConst = LuaConst::new(0x200000E8);
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW: LuaConst = LuaConst::new(0x200000E9);
pub const FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_GUARD: LuaConst = LuaConst::new(0x1F0);
pub const FIGHTER_METAKNIGHT_GENERATE_ARTICLE_GALAXIA_BEAM: LuaConst = LuaConst::new(0x3);
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_GUARD: LuaConst = LuaConst::new(0x200000E5);
pub const FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_META_POWER: LuaConst = LuaConst::new(0x200000EA);  
pub const WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT: LuaConst = LuaConst::new(0x0);
pub const WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_END: LuaConst = LuaConst::new(0x1);
pub const WEAPON_METAKNIGHT_GALAXIA_BEAM_INSTANCE_WORK_ID_FLOAT_ANGLE: LuaConst = LuaConst::new(0x1);
pub const WEAPON_METAKNIGHT_GALAXIA_BEAM_INSTANCE_WORK_ID_FLOAT_ROT: LuaConst = LuaConst::new(0x2);

//PALUTENA
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_DIVINE_POWER: LuaConst = LuaConst::new(0x200000E9); 

//PIT
pub const FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY: LuaConst = LuaConst::new(0x1D3);
pub const FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END: LuaConst = LuaConst::new(0x1F0);
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME: LuaConst = LuaConst::new(0x1000011);
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_LAND_TIME: LuaConst = LuaConst::new(0x1000012);
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE: LuaConst = LuaConst::new(0x1000013);
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_START_SE_COUNTER: LuaConst = LuaConst::new(0x1000014);
pub const FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLAG_BURN: LuaConst = LuaConst::new(0x1000015);
pub const FIGHTER_PIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FLY_POWER: LuaConst = LuaConst::new(0x4C);

//PLIZARDON
pub const FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2: LuaConst = LuaConst::new(0x1DB);
pub const FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2_LANDING: LuaConst = LuaConst::new(0x1D3);
pub const FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_GUARD: LuaConst = LuaConst::new(0x1D2);
pub const FIGHTER_PLIZARDON_STATUS_ROCKSTONE_WORK_INT_INTERVAL_FRAME_COUNT: LuaConst = LuaConst::new(0x11000011);
pub const FIGHTER_PLIZARDON_STATUS_ROCKSTONE_FLAG_GENE_ROCK: LuaConst = LuaConst::new(0x21000010);
pub const FIGHTER_PLIZARDON_STATUS_ATTACK_WORK_FLAG_CRITICAL: LuaConst = LuaConst::new(0x200000E7);
pub const FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCK: LuaConst = LuaConst::new(0x3);
pub const FIGHTER_PLIZARDON_GENERATE_ARTICLE_ROCKSTONE: LuaConst = LuaConst::new(0x4);
pub const WEAPON_PLIZARDON_ROCK_STATUS_KIND_START: LuaConst = LuaConst::new(0x0);
pub const WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_FLAG_BREAK: LuaConst = LuaConst::new(0x2000000B);
pub const WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_SPAWN_COOLDOWN: LuaConst = LuaConst::new(0x1000000A);
pub const WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_STONES_MAX: LuaConst = LuaConst::new(0x10000004);
pub const WEAPON_PLIZARDON_ROCK_INSTANCE_WORK_ID_INT_STONES_REMAIN: LuaConst = LuaConst::new(0x10000005);
pub const WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_START: LuaConst = LuaConst::new(0x0);
pub const WEAPON_PLIZARDON_ROCKSTONE_STATUS_KIND_MOVE: LuaConst = LuaConst::new(0x2);
pub const WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_FLOAT_ANGLE: LuaConst = LuaConst::new(0x10000009);
pub const WEAPON_PLIZARDON_ROCKSTONE_INSTANCE_WORK_ID_FLOAT_ROT: LuaConst = LuaConst::new(0x1000000B);

//PZENIGAME
pub const FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_GUARD: i32 = 0x1D3;

//PFUSHIGISOU
pub const FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_SHOOT: LuaConst = LuaConst::new(0x1D3);
pub const FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_END: LuaConst = LuaConst::new(0x1DA);

//LUCARIO
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_ENABLE_SPECIAL_HI: LuaConst = LuaConst::new(0x200000E3);
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI: LuaConst = LuaConst::new(0x200000E4);

//LUCAS & CLAUS
pub const FIGHTER_LUCAS_STATUS_ATTACK_WORK_FLAG_CRITICAL: LuaConst = LuaConst::new(0x21000010);

//LUCINA
pub const FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2: LuaConst = LuaConst::new(0x1EF);
pub const FIGHTER_LUCINA_STATUS_SPECIAL_LW_FLAG_ACTIVATE_SPECIAL_LW_HIT2: LuaConst = LuaConst::new(0x2100001F);

//LUIGI
pub const FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_PULSE_EFFECT_HANDLE: LuaConst = LuaConst::new(0x100000C3);
pub const FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_SMOKE_EFFECT_HANDLE: LuaConst = LuaConst::new(0x100000C4);

//RYU
pub const FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_START: i32 = 0x202;
pub const FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_CHARGE: i32 = 0x203;
pub const FIGHTER_RYU_STATUS_KIND_KAMEHAMEHA_FIRE: i32 = 0x204;
pub const FIGHTER_RYU_STATUS_WORK_ID_KAMEHAMEHA_FLOAT_CHARGE_TIME: i32 = 0x2100001C;

//SHIZUE
pub const FIGHTER_SHIZUE_INSTANCE_WORK_ID_INT_SHOT_KIND: LuaConst = LuaConst::new(0x100000D2); 
pub const FIGHTER_SHIZUE_INSTANCE_WORK_ID_FLAG_RANDOM_SHOT: LuaConst = LuaConst::new(0x200000F1);

//SIMON & RICHTER
pub const FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI: LuaConst = LuaConst::new(0x200000F7);

//TRAIL
pub const FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S: LuaConst = LuaConst::new(0x200000E6);
pub const FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI: LuaConst = LuaConst::new(0x200000E7);

//WOLF
pub const FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH: LuaConst = LuaConst::new(0x1EA);
pub const FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_END: LuaConst = LuaConst::new(0x1EB);


pub mod globals {
    // 0x1
    pub const FIGHTER_KIND:          i32 = 0x2;
    pub const OBJECT_ID:             i32 = 0x3;
    // 0x4
    pub const MODULE_ACCESSOR:       i32 = 0x5;
    // 0x6
    pub const INIT_STATUS_FUNC:      i32 = 0x7;
    pub const IS_STOPPING:           i32 = 0x8;
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9;
    pub const PREV_STATUS_KIND:      i32 = 0xA;
    pub const STATUS_KIND:           i32 = 0xB;
    pub const STATUS_COUNT:          i32 = 0xC;
    // 0xD
    pub const CURRENT_FRAME:         i32 = 0xE;
    pub const CURRENT_FRAME2:        i32 = 0xF;
    // 0x10
    // 0x11 func ptr
    // 0x12
    pub const SUB_STATUS3:           i32 = 0x13;
    pub const SUB_STATUS2:           i32 = 0x14;
    pub const SUB_STATUS:            i32 = 0x15;
    pub const SITUATION_KIND:        i32 = 0x16;
    pub const PREV_SITUATION_KIND:   i32 = 0x17;
    pub const PREV_STATUS_FRAME:     i32 = 0x18;
    // 0x19
    pub const STICK_X:               i32 = 0x1A;
    pub const STICK_Y:               i32 = 0x1B;
    pub const FLICK_X:               i32 = 0x1C;
    pub const FLICK_Y:               i32 = 0x1D;
    pub const FLICK_Y_DIR:           i32 = 0x1E;
    pub const PAD_FLAG:              i32 = 0x1F;
    pub const CMD_CAT1:              i32 = 0x20;
    pub const CMD_CAT2:              i32 = 0x21;
    pub const CMD_CAT3:              i32 = 0x22;
    pub const CMD_CAT4:              i32 = 0x23;
    // 0x24
    // 0x25
    // 0x26
    // 0x27
    // 0x28 some substatus
    pub const DASH_CALLBACK:         i32 = 0x29;
    // 0x2A
    pub const CUSTOM_ROUTINE:        i32 = 0x2B;
    // 0x2C
    // 0x2D
    // 0x2E
    // 0x2F
    // 0x30
    // 0x31
    // 0x32 some substatus
    pub const USE_SPECIAL_N_CALLBACK: i32 = 0x38;
    pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
    pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
    pub const USE_SPECIAL_LW_CALLBACK: i32 = 0x3B;
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C;
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D;
    pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
    

}



pub mod vars {
    pub mod common {
        pub mod instance {
            pub const HITSTUN_START: i32 = 0x0000;
            pub const IS_IN_HITSTUN: i32 = 0x0001;
            pub const CSTICK_OVERRIDE: i32 = 0x0002;
            pub const CSTICK_OVERRIDE_SECOND: i32 = 0x0003;

            // pub const IS_TAP_JUMP: i32 = 0x0004;
            pub const OMNI_FLOAT: i32 = 0x0005;
            pub const AERIAL_NO_FLOAT: i32 = 0x0006;
            pub const FLOAT_PAUSE_AERIAL: i32 = 0x0007;
            pub const SIDE_SPECIAL_CANCEL: i32 = 0x0008;
            pub const UP_SPECIAL_CANCEL: i32 = 0x0009;
            pub const JAB_DA_CHECKS: i32 = 0x000A;
            pub const TILT_CHECKS: i32 = 0x000B;
            pub const AERIAL_CHECKS: i32 = 0x000C;
            pub const SMASH_CHECKS: i32 = 0x000D;
            pub const SPECIAL_STALL: i32 = 0x000E;
            pub const SPECIAL_STALL_USED: i32 = 0x000F;
            pub const ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x0010;
            pub const DITCIT_SLIDING: i32 = 0x0011;
            pub const FOOTSTOOL_AIRDODGE_LOCKOUT: i32 = 0x0012;
            pub const CAN_ESCAPE_TUMBLE: i32 = 0x0013;
            pub const SPECIAL_WALL_JUMP: i32 = 0x0014;
            pub const TETHER_HOGGED: i32 = 0x0015;
            pub const B_REVERSED: i32 = 0x0016; // Converted for now, but will likely get removed when B Reverse Reimplementation happens
            pub const TUMBLE_KB: i32 = 0x0017;
            pub const CAN_GLIDE_TOSS: i32 = 0x0019;
            pub const IS_MOTION_BASED_ATTACK: i32 = 0x001A;
            pub const PREV_FLAG_DISABLE_ESCAPE_AIR: i32 = 0x001B;
            pub const ENABLE_WAVELAND_PLATDROP: i32 = 0x001C;
            pub const IS_DACUS: i32 = 0x001D;
            pub const IS_STICKY_WALK: i32 = 0x001E;
            pub const ENABLE_BOOST_RUN: i32 = 0x001F;
            pub const PERFECT_WAVEDASH: i32 = 0x0020;
            pub const JUMP_NEXT: i32 = 0x0021;
            pub const SHOULD_TRUMP_TETHER: i32 = 0x0022;
            pub const UP_SPECIAL_INTERRUPT: i32 = 0x0023; // Ness and Lucas use this
            pub const UP_SPECIAL_INTERRUPT_AIRTIME: i32 = 0x0024; // Ness and Lucas use this
            pub const SPECIAL_PROJECTILE_SPAWNED: i32 = 0x0025; // Luigi, Ivysaur, and Young Link use this
            pub const STALL_PREVENTION: i32 = 0x0027; //Ness and Lucas down b stall prevention
            pub const SPIN_ATTACK_LAND_CANCEL: i32 = 0x003E; // Link and Mii Sword use this
            pub const SIDE_SPECIAL_CANCEL_NO_HIT: i32 = 0x004D; // Used by Kazuya and Sora
            pub const IS_LATE_PIVOT: i32 = 0x004E;
            pub const CAN_PERFECT_PIVOT: i32 = 0x004F;
            pub const IS_SMASH_TURN: i32 = 0x0050;
            pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 0x0051;
            pub const IS_KNOCKDOWN_THROW: i32 = 0x0052;
            pub const IS_HEAVY_ATTACK: i32 = 0x0053;
            pub const IS_CC_NON_TUMBLE: i32 = 0x0054;
            pub const IS_GETTING_POSITION_FOR_ECB: i32 = 0x0055;
            pub const CHECK_CHANGE_MOTION_ONLY: i32 = 0x0056;
            pub const BEFORE_GROUND_COLLISION: i32 = 0x0057;
            pub const IS_IGNORED_STATUS_FRAME_0: i32 = 0x0058;
            pub const FLUSH_EFFECT_ACMD: i32 = 0x0059;
            pub const IS_PARRY_FOR_GUARD_OFF: i32 = 0x0060;
            pub const TEMPORARY_CLIFF_STOP: i32 = 0x0061;
            pub const ENABLE_FRAME_DATA_DEBUG: i32 = 0x0062;
            pub const IS_ATTACK_CANCEL: i32 = 0x0063;
            pub const DISABLE_CSTICK_BUFFER_ROLL_OOS: i32 = 0x0064;
            pub const IS_INIT: i32 = 0x0065;
            pub const IS_FLOAT: i32 = 0x0066;
            pub const WEIRD_ASS_TURN_RUN_ANIMATION: i32 = 0x0067;
            pub const ACMD_EFFECT: i32 = 0x0068;
            pub const WAS_PREV_STATUS_CANCELABLE: i32 = 0x0069;
            pub const IS_ENTER_DASH_CANCEL: i32 = 0x006A;
            pub const DOWN_DISABLE_PASSIVE: i32 = 0x006B;
            pub const DOWN_DISABLE_A_LAND: i32 = 0x006C;
            pub const IS_KILLING_BLOW: i32 = 0x006D;

            // ints
            pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0000;
            pub const COSTUME_SLOT_NUMBER: i32 = 0x0001; // Unironically why does this need to exist? We have WorkModule.
            pub const FLOAT_DURATION: i32 = 0x0002;
            pub const FLOAT_STATUS_KIND: i32 = 0x0003;
            pub const HITFALL_BUFFER: i32 = 0x0004;
            pub const FLY_NEXT_FRAME: i32 = 0x0005;

            // pub const JUMP_SQUAT_FRAME: i32 = 0x0005;
            pub const GIMMICK_TIMER: i32 = 0x0006;
            pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0x0007;
            pub const CSTICK_LIFE: i32 = 0x0008;
            pub const AGT_USED_COUNTER: i32 = 0x0009;
            pub const CLIFF_XLU_FRAME: i32 = 0x000A;
            pub const LAST_ATTACK_HITBOX_ID: i32 = 0x000B;
            pub const SHIELD_EFFECT_HANDLE: i32 = 0x000C;
            pub const FRAME_COUNTER: i32 = 0x000D;
            pub const LEFT_STICK_FLICK_X: i32 = 0x000E;
            pub const LEFT_STICK_FLICK_Y: i32 = 0x000F;
            pub const LEDGE_ID: i32 = 0x0010;
            pub const RIGHT_STICK_FLICK_X: i32 = 0x0011;
            pub const RIGHT_STICK_FLICK_Y: i32 = 0x0012;
            pub const PREV_STATUS_TRANSITION_FRAME: i32 = 0x0013;

            // floats
            pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0000;
            pub const CURRENT_MOMENTUM: i32 = 0x0001;
            pub const JUMPSQUAT_VELOCITY: i32 = 0x0002;
            /// This const is set in a fighter reset because the params used to calculate change depending on situation
            pub const JUMP_SPEED_RATIO: i32 = 0x0003;
            pub const DOUBLE_JUMP_FRAME: i32 = 0x0004;
            pub const GROUND_VEL: i32 = 0x0005; // Only ever gets set, goes effectively unused.
            pub const RAR_LENIENCY: i32 = 0x0006; // Only ever gets set, goes effectively unused.
            pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x0007;
            pub const DOUBLE_JUMP_TIMER: i32 = 0x0008; // Only used by Lucas, and it's commented out, goes unused.
            pub const ROLL_SPEED: i32 = 0x0009;
            pub const LAST_GROUNDED_POS: i32 = 0x000A;
            // pub const LAST_GROUNDED_POS_X: i32 = 0x000A;
            // pub const LAST_GROUNDED_POS_y: i32 = 0x000B;
            // pub const LAST_GROUNDED_POS_Z: i32 = 0x000C;
            pub const GET_DIST_TO_FLOOR: i32 = 0x000D;
            pub const ECB_BOTTOM_Y_OFFSET: i32 = 0x000E;
            pub const CURR_DASH_SPEED: i32 = 0x000F;
            pub const MOONWALK_SPEED: i32 = 0x0010;
            pub const ESCAPE_AIR_SLIDE_SPEED_X: i32 = 0x0011;
            pub const ESCAPE_AIR_SLIDE_SPEED_Y: i32 = 0x0012;
            pub const Y_POS: i32 = 0x0013;
            /// this multiplier can be set to a value between 0.1 and 3.0 to increase
            /// a character's jump speed max for momentum transfer (for meta quick, etc)
            pub const JUMP_SPEED_MAX_MUL: i32 = 0x0014;
            pub const ECB_TOP_Y_OFFSET: i32 = 0x0015;
            pub const LAST_ATTACK_HIT_LOCATION: i32 = 0x0016;
            pub const LAST_ATTACK_HIT_LOCATION_X: i32 = 0x0016;
            pub const LAST_ATTACK_HIT_LOCATION_Y: i32 = 0x0017;
            pub const LAST_ATTACK_HIT_LOCATION_Z: i32 = 0x0018;
            pub const ECB_CENTER_Y_OFFSET: i32 = 0x0019;
            pub const DASH_HIP_OFFSET_X: i32 = 0x0020;
            pub const RUN_HIP_OFFSET_X: i32 = 0x0021;
            pub const DACUS_TRANSITION_SPEED: i32 = 0x0022;
            pub const ATTACK_S3_CSTICK_X: i32 = 0x0023;
        }
        pub mod status {

            // flags
            pub const FAF_REACHED: i32 = 0x10FD;
            pub const PREV_AUTOCANCEL_FLAG: i32 = 0x10FE;
            pub const DISABLE_ECB_SHIFT: i32 = 0x10FF;
            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1000;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1001;
            pub const APPLY_DASH_END_SPEED_MUL: i32 = 0x1002;
            pub const SHOULD_WAVELAND: i32 = 0x1000;
            pub const IS_JAB_LOCK_ROLL: i32 = 0x1000;
            pub const IS_SPIKE: i32 = 0x1001;
            pub const DAMAGE_FLY_RESET_TRIGGER: i32 = 0x1002;
            pub const SUICIDE_THROW_CAN_CLATTER: i32 = 0x1000;
            pub const ENABLE_UCF: i32 = 0x1000;
            pub const PUMMEL_OVERRIDE_GLOBAL_STATS: i32 = 0x1000;
            pub const CSTICK_IRAR: i32 = 0x1000;
            pub const FLOAT_INHERIT_AERIAL: i32 = 0x1000;
            pub const IS_TELEPORT_WALL_RIDE: i32 = 0x1000; // Mewtwo, Palutena, Sheik, and Zelda use this
            pub const ENABLE_SPECIAL_WALLJUMP: i32 = 0x1050;
            pub const HIT_EFFECT_DROP_ITEM: i32 = 0x1051;
            pub const SHOULD_HITFALL: i32 = 0x1006;
            pub const NO_POCKET: i32 = 0x1052;
            pub const IS_DASH_CANCEL: i32 = 0x1055;

            // ints
            pub const DOWN_STAND_FB_KIND: i32 = 0x1000;
            pub const FLOAT_FRAME: i32 = 0x1000;
            pub const FLOAT_ENABLE_UNIQ: i32 = 0x1001;
            pub const FLOAT_MTRANS: i32 = 0x1002;
            pub const WARP_EFF_HANDLER: i32 = 0x1000;

            // floats
            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1000;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1001;
            pub const RESTING_HIP_OFFSET_Y: i32 = 0x1000;
            pub const TELEPORT_INITIAL_SPEED_X: i32 = 0x1000;
            pub const TELEPORT_INITIAL_SPEED_Y: i32 = 0x1001;
        }
    }
}