/*
 * Enemy spawning function
 * Spawns enemies at random intervals
 */

#define SPAWN_INTERVAL 60
#define MAX_ENEMIES 50
#define ENEMY_GROUPS ["OPF_F", "OPF_T_F", "OPF_R_F"]
#define ENEMY_TYPES ["O_Soldier_F", "O_Soldier_GL_F", "O_Soldier_AR_F", "O_Soldier_LAT_F", "O_medic_F", "O_Soldier_TL_F"]

private _enemySpawnHandler = -1;
private _enemyTypes = ENEMY_TYPES;
private _enemyFactions = ENEMY_GROUPS;
private _maxEnemies = MAX_ENEMIES;

// Check if custom enemy types are defined
if (!isNil "MISSION_ENEMY_TYPES") then {
    _enemyTypes = MISSION_ENEMY_TYPES;
};

// Check if custom factions are defined
if (!isNil "MISSION_ENEMY_FACTIONS") then {
    _enemyFactions = MISSION_ENEMY_FACTIONS;
};

// Check if custom max enemies are defined
if (!isNil "MISSION_MAX_ENEMIES") then {
    _maxEnemies = MISSION_MAX_ENEMIES;
};

// Initialize enemy counter
g_enemyCount = 0;

// Start enemy spawning
_enemySpawnHandler = [{
    // Exit if we've reached max enemies
    if (g_enemyCount >= _this select 0) exitWith {};
    
    // Get a random spawn point
    private _spawnPoint = selectRandom g_spawnPoints;
    if (isNil "_spawnPoint") exitWith {
        diag_log "[MISSION] No spawn points available for enemies";
    };
    
    // Get a random enemy faction
    private _faction = selectRandom (_this select 1);
    
    // Get random enemy types
    private _types = _this select 2;
    
    // Determine group size
    private _groupSize = floor (3 + random 3);
    
    // Create group
    private _group = createGroup [east, true];
    
    // Spawn enemies
    for "_i" from 1 to _groupSize do {
        private _type = selectRandom _types;
        private _pos = [getPos _spawnPoint, 10, 50, 3, 0, 0.5, 0] call BIS_fnc_findSafePos;
        
        private _unit = _group createUnit [_type, _pos, [], 0, "NONE"];
        _unit setSkill (0.3 + random 0.4);
        
        // Add to enemy count
        g_enemyCount = g_enemyCount + 1;
    };
    
    // Set group behavior
    _group setBehaviour "AWARE";
    _group setCombatMode "RED";
    
    // Send group on patrol
    [_group, getPos _spawnPoint, 300] call CBA_fnc_taskPatrol;
    
    // Add event handler to track when enemies are killed
    {
        _x addEventHandler ["Killed", {
            g_enemyCount = g_enemyCount - 1;
        }];
    } forEach units _group;
    
    // Log spawn
    diag_log format ["[MISSION] Spawned enemy group with %1 units. Total enemies: %2", _groupSize, g_enemyCount];
    
}, SPAWN_INTERVAL, [_maxEnemies, _enemyFactions, _enemyTypes]] call CBA_fnc_addPerFrameHandler;

// Return the spawn handler ID
_enemySpawnHandler 