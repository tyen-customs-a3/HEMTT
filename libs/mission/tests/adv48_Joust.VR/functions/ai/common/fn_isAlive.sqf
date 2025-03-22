/**
	Check if unit is alive and conscious
	
	Parameters:
	0: unit to be checked <OBJECT>
	
	Example:
	bob call pca_fnc_isAlive;
	
	Returns: 
	true if unit is alive and conscious
*/

alive _this && {(lifeState _this) isNotEqualTo "INCAPACITATED"};