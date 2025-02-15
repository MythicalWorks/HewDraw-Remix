
use super::*;


#[acmd_script( agent = "jack", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 112, 0, 40, 6.0, 0.0, 9.5, 7.5, Some(0.0), Some(8.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 112, 0, 40, 4.0, 0.0, 10.0, 13.0, Some(0.0), Some(6.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 361, 112, 0, 40, 5.0, 0.0, 6.0, 4.5, Some(0.0), Some(6.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 120, 0, 40, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(8.0), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 120, 0, 40, 5.0, 0.0, 8.5, 10.0, Some(0.0), Some(11.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 1, Hash40::new("top"), 3.0, 361, 325, 0, 50, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(8.0), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
            ATTACK(fighter, 3, 1, Hash40::new("top"), 3.0, 361, 325, 0, 50, 5.0, 0.0, 8.5, 10.0, Some(0.0), Some(11.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.25);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 112, 0, 40, 6.0, 0.0, 9.5, 7.5, Some(0.0), Some(6.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 112, 0, 40, 4.0, 0.0, 8.0, 13.0, Some(0.0), Some(6.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 120, 0, 40, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(7.5), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 120, 0, 40, 5.0, 0.0, 7.0, 10.0, Some(0.0), Some(11.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 1, Hash40::new("top"), 3.0, 361, 325, 0, 50, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(7.5), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
            ATTACK(fighter, 3, 1, Hash40::new("top"), 3.0, 361, 325, 0, 50, 5.0, 0.0, 7.0, 10.0, Some(0.0), Some(11.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.25);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("legr"), 0, 0, 0);
        attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("legr"), 0, 0, 0);
        AttackModule::set_size(boma, 0, 0.0);
        AttackModule::set_size(boma, 1, 0.0);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0, 14, 15.5);
            attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0, 7, 10);
            AttackModule::set_size(boma, 0, 3.0);
            AttackModule::set_size(boma, 1, 5.0);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            FT_MOTION_RATE(fighter, 1.111);
        }
    }
    
}

#[acmd_script( agent = "jack", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.667);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.667);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        // ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 83, 116, 0, 43, 5.5, 0.0, 7.0, 3.0, Some(0.0), Some(12.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 83, 116, 0, 43, 5.5, 0.0, 7.0, 6.0, Some(0.0), Some(12.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 83, 116, 0, 43, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("knife"), 12.0, 83, 116, 0, 43, 5.5, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 83, 122, 0, 43, 5.5, 0.0, 7.0, 3.0, Some(0.0), Some(12.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("knife"), 12.0, 83, 122, 0, 43, 5.0, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 83, 122, 0, 43, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 1, Hash40::new("knife"), 3.0, 83, 277, 0, 59, 5.0, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 4, 1, Hash40::new("armr"), 3.0, 83, 277, 0, 59, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 5, 1, Hash40::new("top"), 3.0, 83, 277, 0, 59, 5.0, 0.0, 7.0, 3.0, Some(0.0), Some(12.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.25);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 1.25);
            HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 5, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 83, 122, 0, 43, 5.0, 0.0, 28.0, 1.0, Some(0.0), Some(16.0), Some(9.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 5, 1, Hash40::new("top"), 3.0, 83, 277, 0, 59, 5.0, 0.0, 28.0, 1.0, Some(0.0), Some(16.0), Some(9.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 83, 116, 0, 43, 5.0, 0.0, 28.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 5, 1, Hash40::new("top"), 3.0, 83, 191, 0, 59, 5.0, 0.0, 28.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            FT_MOTION_RATE(fighter, 1.111);
        }
    }
    
}

#[acmd_script( agent = "jack", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn jack_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.0, 32, 106, 0, 30, 4.0, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 32, 106, 0, 30, 4.0, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("knife"), 13.0, 32, 106, 0, 30, 4.0, 0.0, 1.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("knife"), 13.0, 32, 106, 0, 30, 4.0, 1.0, 4.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 0, 0, Hash40::new("knife"), 13.0, 32, 108, 0, 30, 4.0, 0.0, 1.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("knife"), 13.0, 32, 108, 0, 30, 4.0, 1.0, 4.0, 1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 32, 108, 0, 30, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("armr"), 13.0, 32, 108, 0, 30, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 32, 108, 0, 30, 4.0, 0.0, 6.5, 6.5, Some(0.0), Some(4.0), Some(15.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 1, Hash40::new("top"), 2.0, 38, 311, 0, 44, 4.0, 0.0, 6.5, 6.5, Some(0.0), Some(4.0), Some(15.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 30, 106, 0, 35, 4.0, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 30, 106, 0, 35, 4.0, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("knife"), 13.0, 30, 106, 0, 35, 4.0, 0.0, 1.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("knife"), 13.0, 30, 106, 0, 35, 4.0, 1.0, 4.5, 0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            ATTACK(fighter, 0, 0, Hash40::new("knife"), 13.0, 30, 108, 0, 35, 4.0, 0.0, 1.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("knife"), 13.0, 30, 108, 0, 35, 4.0, 1.0, 4.5, 1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 30, 108, 0, 35, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 3, 0, Hash40::new("armr"), 13.0, 30, 108, 0, 35, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 30, 108, 0, 35, 4.0, 0.0, 6.0, -8.0, Some(0.0), Some(3.0), Some(-15.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 1, Hash40::new("top"), 2.0, 30, 311, 0, 44, 4.0, 0.0, 6.0, -8.0, Some(0.0), Some(3.0), Some(-15.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.33);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.33);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.33);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.33);
            ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.33);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if WorkModule::is_flag(boma,  *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            FT_MOTION_RATE(fighter, 1.111);
        }
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        jack_attack_s4_s_game,
        jack_attack_hi4_game,
        jack_attack_lw4_game,
    );
}

