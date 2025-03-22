/**
	adds an ace action to player that allows them to JIP teleport 
*/

waitUntil {speed player > 1};

if !(count units group player > 1) exitWith {};
if !({alive _x} count units group player > 1) exitWith {};

private _response = 
[
	"
		Welcome to the Mission! 
		<br/><br/>
		You can use the 'TELEPORT TO GROUP' action in your ACE self interaction menu to teleport to your group. 
		<br/><br/>
		The action will disappear once 10 minutes has passed since you've joined or moving too far away from the spawn area.
	", "JIP TELEPORT AVAILABLE", true, false
] call BIS_fnc_guiMessage;

private _jipActionCode = 
{
	private _target = [] call pca_fnc_jipChooseTarget;
	private _vehSpot = [_target] call pca_fnc_jipEmptySeat;
	
	if (!_vehSpot && {speed _target > 19 || {(getPosATL _target) select 2 > 5}}) exitWith 
	{
		private _title1 = "<t color = '#FFBA26' size='1'>CAN NOT TELEPORT!</t><br/>";
		private _title2 = "<t color = '#FFFFFF' size='1'>Teleport target moving too fast or currently in air. Try again in a little bit!</t><br/>";
		[_title1 + _title2, 2, player, 14] call ace_common_fnc_displayTextStructured;
	};
	
	player allowDamage false;
	
	cutText ["", "BLACKOUT", 1, true];
	
	if (_vehSpot) then 
	{
		player moveInAny vehicle _target;
	}
	else
	{
		player setPosATL (_target getPos [3, getDir _target - 180]);
	};
	
	cutText ["", "BLACK IN", 1, true];
	player allowDamage true;
};

player setVariable ["_startPos", getPosASL player];

private _jipTeleAction = ["teleportToGroup", "TELEPORT TO GROUP", "\a3\ui_f\data\gui\cfg\communicationMenu\transport_ca.paa", _jipActionCode, {player distance2D (player getVariable ["_startPos", [0,0,0]]) < 200}] call ace_interact_menu_fnc_createAction;

[player, 1, ["ACE_SelfActions"], _jipTeleAction] call ace_interact_menu_fnc_addActionToObject;

[{[player, 1, ["ACE_SelfActions", "teleportToGroup"]] call ace_interact_menu_fnc_removeActionToObject;}, [], 1200] call CBA_fnc_waitAndExecute;
