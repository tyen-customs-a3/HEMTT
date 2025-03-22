/**
	choose JIP Target
	
	Parameters:
	0: NONE
	
	Example:
	[] call pca_fnc_jipChooseTarget;
	
	Returns: 
	the target
*/

private _teleportTarget = objNull;
private _partGroup = units group player;

if (player != leader group player && {alive leader group player}) then 
{
	_teleportTarget = leader group player;
};

if (player == leader group player || {!alive leader group player}) then 
{
	_partGroup = _partGroup - [(leader group player)];
	_teleportTarget = _partGroup select (_partGroup findIf {alive _x});
};

//return
_teleportTarget;