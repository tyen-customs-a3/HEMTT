/**
	Retrieves data and spawns the unit 
	
	Parameters: 
	0: unit data <ARRAY>
	1: spawned group <GROUP>
	2: spawn position <ARRAY>
	3: unit is stationary <BOOL>
	
	Example: 
	[<unitData>, <group>, <spawnPos>] call pca_fnc_spawnUnit;
	
	Returns: 
	the spawned units
*/

if (!isServer) exitWith {};

params ["_unitData", "_newGrp", "_spawnPos", ["_isStationary", false]];

_unitData params ["_unitType", "_unitPos", ["_unitLoadout", ""], ["_unitFaction", ""], ["_unitVarName", ""]];

if (_spawnPos select 2 < 0) then 
{
	_spawnPos set [2, 0.1];
};

if (surfaceIsWater _spawnPos) then 
{
	_spawnPos = ATLToASL _spawnPos;
};

//use CAN_COLLIDE for more precise spawning position
private _newUnit = _newGrp createUnit [_unitType, _spawnPos, [], 0, "NONE"];

waitUntil {alive _newUnit};

if (getPos _newUnit # 2 > 2) then 
{
	_onGround = [getPos _newUnit # 0, getPos _newUnit # 1, 0.1];
	_newUnit setPos _onGround;
};

_newUnit setVariable ["tmf_assignGear_role", _unitLoadout];
[_newUnit, _unitFaction, _unitLoadout] call tmf_assigngear_fnc_assigngear;

if (!(_unitVarName isEqualTo "")) then 
{
	[_newUnit, _unitVarName] remoteExec ["setVehicleVarName", 0, _newUnit];
	missionNamespace setVariable [_unitVarName, _newUnit, true];
};

if (_isStationary) then 
{
	_newUnit disableAI "PATH";
	_newUnit setPosATL _unitPos;
};

//return
_newUnit;