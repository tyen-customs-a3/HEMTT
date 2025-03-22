/**
	Set the skill level of the unit;
	
	Parameters:
	0: the unit <OBJECT>
	1: preset skill levels, default 5 <NUMBER>
	
	Example:
	[bob, 0] call pca_fnc_setUnitSkills;
	
	Returns: 
	NONE
*/

params ["_unit", ["_preset", 5]];

if !(_unit call pca_fnc_isAlive) exitWith {};

switch (_preset) do 
{
	case 1 : 
	{
		_unit setSkill ["general", 1];
		_unit setSkill ["commanding", 		(random [0.9, 0.95, 1])];
		_unit setSkill ["courage", 			(random [0.9, 0.95, 1])];
		_unit setSkill ["spotDistance", 	(random [0.9, 0.95, 1])];
		_unit setSkill ["spotTime", 		(random [0.9, 0.95, 1])];
		_unit setSkill ["reloadSpeed", 		(random [0.6, 0.8, 1])];
		_unit setSkill ["aimingAccuracy", 	(random [0.65, 0.75, 0.9])];
		_unit setSkill ["aimingShake", 		(random [0.65, 0.75, 0.9])];
		_unit setSkill ["aimingSpeed", 		(random [0.8, 0.9, 1])];
	};
	case 2 : 
	{
		_unit setSkill ["general", 1];
		_unit setSkill ["commanding", 		(random [0.85, 0.95, 1])];
		_unit setSkill ["courage", 			(random [0.9, 0.95, 1])];
		_unit setSkill ["spotDistance", 	(random [0.8, 0.9, 1])];
		_unit setSkill ["spotTime", 		(random [0.8, 0.9, 1])];
		_unit setSkill ["reloadSpeed", 		(random [0.6, 0.75, 0.9])];
		_unit setSkill ["aimingAccuracy", 	(random [0.6, 0.7, 0.8])];
		_unit setSkill ["aimingShake", 		(random [0.6, 0.7, 0.8])];
		_unit setSkill ["aimingSpeed", 		(random [0.7, 0.8, 0.9])];
	};
	case 3 : 
	{
		_unit setSkill ["general", 1];
		_unit setSkill ["commanding", 		(random [0.8, 0.9, 1])];
		_unit setSkill ["courage", 			(random [0.9, 0.9, 1])];
		_unit setSkill ["spotDistance", 	(random [0.8, 0.85, 1])];
		_unit setSkill ["spotTime", 		(random [0.8, 0.85, 1])];
		_unit setSkill ["reloadSpeed", 		(random [0.6, 0.7, 0.8])];
		_unit setSkill ["aimingAccuracy", 	(random [0.55, 0.65, 0.75])];
		_unit setSkill ["aimingShake", 		(random [0.55, 0.65, 0.75])];
		_unit setSkill ["aimingSpeed", 		(random [0.65, 0.75, 0.85])];
	};
	case 4 : 
	{
		_unit setSkill ["general", 1];
		_unit setSkill ["commanding", 		(random [0.8, 0.9, 1])];
		_unit setSkill ["courage", 			(random [0.85, 0.9, 1])];
		_unit setSkill ["spotDistance", 	(random [0.75, 0.85, 1])];
		_unit setSkill ["spotTime", 		(random [0.75, 0.85, 1])];
		_unit setSkill ["reloadSpeed", 		(random [0.5, 0.65, 0.8])];
		_unit setSkill ["aimingAccuracy", 	(random [0.5, 0.6, 0.75])];
		_unit setSkill ["aimingShake", 		(random [0.5, 0.6, 0.75])];
		_unit setSkill ["aimingSpeed", 		(random [0.6, 0.7, 0.8])];
	};
	case 5 : 
	{
		_unit setSkill ["general", 1];
		_unit setSkill ["commanding", 		(random [0.7, 0.8, 1])];
		_unit setSkill ["courage", 			(random [0.8, 0.85, 1])];
		_unit setSkill ["spotDistance", 	(random [0.7, 0.8, 1])];
		_unit setSkill ["spotTime", 		(random [0.7, 0.8, 1])];
		_unit setSkill ["reloadSpeed", 		(random [0.5, 0.6, 0.7])];
		_unit setSkill ["aimingAccuracy", 	(random [0.4, 0.5, 0.6])];
		_unit setSkill ["aimingShake", 		(random [0.4, 0.5, 0.6])];
		_unit setSkill ["aimingSpeed", 		(random [0.5, 0.6, 0.7])];
	};
	case 6 : 
	{
		_unit setSkill ["general", 1];
		_unit setSkill ["commanding", 		(random [0.6, 0.7, 1])];
		_unit setSkill ["courage", 			(random [0.7, 0.8, 1])];
		_unit setSkill ["spotDistance", 	(random [0.5, 0.6, 1])];
		_unit setSkill ["spotTime", 		(random [0.5, 0.6, 1])];
		_unit setSkill ["reloadSpeed", 		(random [0.4, 0.5, 0.6])];
		_unit setSkill ["aimingAccuracy", 	(random [0.3, 0.45, 0.55])];
		_unit setSkill ["aimingShake", 		(random [0.3, 0.45, 0.55])];
		_unit setSkill ["aimingSpeed", 		(random [0.4, 0.5, 0.6])];
	};
	default
	{
		_unit setSkill ["general", 1];
		_unit setSkill ["commanding", 0.5];
		_unit setSkill ["courage", 0.5];
		_unit setSkill ["spotDistance", 0.5];
		_unit setSkill ["spotTime", 0.5];
		_unit setSkill ["reloadSpeed", 0.5];
		_unit setSkill ["aimingAccuracy", 0.5];
		_unit setSkill ["aimingShake", 0.5];
		_unit setSkill ["aimingSpeed", 0.5];
	};
};
