/*
 * Mission initialization function
 * This is a complex test script for HEMTT benchmarks
 */

// Global variables
MISSION_VERSION = "1.2.0";
g_missionStarted = false;
g_playerReady = false;
g_enemyCount = 0;
g_scoreData = createHashMap;
g_vehicles = [];
g_spawnPoints = [];
g_weatherEnabled = true;
g_debugMode = false;

// Configuration
#define MAX_ENEMIES 50
#define RESPAWN_DELAY 30
#define CLEANUP_INTERVAL 300
#define DEFAULT_VIEW_DISTANCE 2500

// Type definitions
#include "\a3\ui_f\hpp\defineResincl.inc"
#include "\a3\ui_f\hpp\defineDIKCodes.inc"

// Function definitions
fn_setupMission = {
    params [["_delay", 0, [0]], ["_weatherEnabled", true, [true]]];
    
    // Store parameters
    g_weatherEnabled = _weatherEnabled;
    
    // Log initialization
    diag_log format ["[MISSION] Initializing mission version %1", MISSION_VERSION];
    
    // Initial setup
    setTimeMultiplier 1;
    setViewDistance DEFAULT_VIEW_DISTANCE;
    
    // Find spawn points
    g_spawnPoints = entities "Logic" select {
        _x getVariable ["isSpawnPoint", false];
    };
    
    if (count g_spawnPoints == 0) then {
        g_spawnPoints = [
            [1000, 1000, 0],
            [1200, 1200, 0],
            [1400, 1000, 0],
            [1000, 1400, 0]
        ] apply {
            private _logic = createVehicle ["Land_HelipadEmpty_F", _x, [], 0, "NONE"];
            _logic setVariable ["isSpawnPoint", true, true];
            _logic
        };
    };
    
    // Register mission event handlers
    addMissionEventHandler ["EntityKilled", {
        params ["_victim", "_killer"];
        
        if (isPlayer _killer && {_victim isKindOf "Man"} && {side _victim != side _killer}) then {
            private _score = g_scoreData getOrDefault [getPlayerUID _killer, 0];
            g_scoreData set [getPlayerUID _killer, _score + 1];
            
            // Update scoreboard
            [] call Mission_fnc_handleScore;
            
            // Notify player
            [format ["Enemy killed. Total score: %1", _score + 1]] remoteExec ["hint", _killer];
        };
    }];
    
    // Setup weather controller
    if (_weatherEnabled) then {
        // Dynamic weather system
        [{
            if (!g_weatherEnabled) exitWith {};
            
            private _currentOvercast = overcast;
            private _targetOvercast = random 1;
            private _transitionTime = 1800 + random 1800;
            
            _targetOvercast setOvercast _targetOvercast;
            _transitionTime setRain (if (_targetOvercast > 0.7) then {random 1} else {0});
            
            // Add some fog during early morning
            if (daytime > 4 && daytime < 9) then {
                _transitionTime setFog [0.1 + random 0.2, 0.05, 10 + random 50];
            } else {
                _transitionTime setFog 0;
            };
            
            diag_log format ["[MISSION] Weather update: Overcast %1->%2, Rain %3, Fog %4", 
                _currentOvercast, _targetOvercast, rain, fog];
        }, 1800, 0] call CBA_fnc_addPerFrameHandler;
    };
    
    // Setup cleanup routine
    [{
        // Clean up dead bodies
        {
            if (_x isKindOf "Man" && !alive _x && !(_x isKindOf "Animal")) then {
                deleteVehicle _x;
            };
        } forEach allDead;
        
        // Clean up empty vehicles that haven't been used
        {
            if (_x isKindOf "LandVehicle" || _x isKindOf "Air") then {
                if (crew _x isEqualTo [] && damage _x > 0.8) then {
                    deleteVehicle _x;
                };
            };
        } forEach vehicles - g_vehicles;
        
    }, CLEANUP_INTERVAL, 0] call CBA_fnc_addPerFrameHandler;
    
    // Delayed setup
    if (_delay > 0) then {
        [{
            // Set mission status
            g_missionStarted = true;
            
            // Start enemy spawner
            [] spawn Mission_fnc_spawnEnemies;
            
            // Broadcast message to all players
            ["Mission initialized", "SUCCEEDED"] remoteExec ["BIS_fnc_showNotification", 0];
            playSound "MissionStart";
        }, [], _delay] call CBA_fnc_waitAndExecute;
    } else {
        g_missionStarted = true;
        [] spawn Mission_fnc_spawnEnemies;
    };
    
    // Return success
    true
};

// Initialize players
[] spawn {
    waitUntil {!isNull player};
    
    player setVariable ["playerInit", true, true];
    g_playerReady = true;
    
    // Setup player loadout
    if (player getVariable ["customLoadout", false]) then {
        private _loadout = player getVariable ["savedLoadout", []];
        if (count _loadout > 0) then {
            player setUnitLoadout _loadout;
        };
    };
    
    // Add event handlers
    player addEventHandler ["Killed", {
        params ["_unit", "_killer"];
        
        if (isPlayer _killer && _killer != _unit) then {
            systemChat format ["%1 was killed by %2", name _unit, name _killer];
        };
        
        // Respawn after delay
        [{
            if (!alive player) then {
                forceRespawn player;
            };
        }, [], RESPAWN_DELAY] call CBA_fnc_waitAndExecute;
    }];
    
    // Add key handlers
    waitUntil {!isNull (findDisplay 46)};
    (findDisplay 46) displayAddEventHandler ["KeyDown", {
        params ["_display", "_key", "_shift", "_ctrl", "_alt"];
        
        // Debug menu - Ctrl+D
        if (_key == DIK_D && _ctrl) then {
            g_debugMode = !g_debugMode;
            systemChat format ["Debug mode: %1", ["OFF", "ON"] select g_debugMode];
            true
        } else {
            false
        };
    }];
    
    // Add debug overlay
    if (isServer || g_debugMode) then {
        [{
            if (!g_debugMode) exitWith {};
            
            private _text = format [
                "Mission Status: %1\nPlayers: %2\nEnemies: %3\nVehicles: %4\nFPS: %5",
                ["WAITING", "RUNNING"] select g_missionStarted,
                count allPlayers,
                g_enemyCount,
                count g_vehicles,
                round diag_fps
            ];
            
            hintSilent _text;
        }, 1, 0] call CBA_fnc_addPerFrameHandler;
    };
};

// Return success
true 