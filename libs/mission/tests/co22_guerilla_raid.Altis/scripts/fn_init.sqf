/*
 * Mission initialization function
 * This is a test script for HEMTT benchmarks
 */

// Global variables
g_missionStarted = false;
g_playerReady = false;

// Functions
fn_setupMission = {
    params [["_delay", 0, [0]]];
    
    // Initial setup
    setTimeMultiplier 1;
    setViewDistance 2000;
    
    // Delayed setup
    if (_delay > 0) then {
        [{
            // Set weather conditions
            0 setOvercast 0.3;
            0 setRain 0;
            0 setFog 0;
            forceWeatherChange;
            
            // Set mission status
            g_missionStarted = true;
            
            // Broadcast message to all players
            ["Mission initialized", "SUCCEEDED"] remoteExec ["BIS_fnc_showNotification", 0];
        }, [], _delay] call CBA_fnc_waitAndExecute;
    } else {
        g_missionStarted = true;
    };
    
    true
};

// Initialize players
[] spawn {
    waitUntil {!isNull player};
    
    player setVariable ["playerInit", true, true];
    g_playerReady = true;
    
    // Add event handlers
    player addEventHandler ["Killed", {
        params ["_unit", "_killer"];
        
        if (isPlayer _killer && _killer != _unit) then {
            systemChat format ["%1 was killed by %2", name _unit, name _killer];
        };
    }];
};

// Return success
true 