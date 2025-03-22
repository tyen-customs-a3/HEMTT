/**
	Adds a map marker for an object scaled to its size.
	
	Arguments:
	0: the object to have the map marker <OBJECT>
	
	Example:
	[bob] call pca_fnc_drawObjectMapMarker;
	
	Returns: 
	NONE
*/

//Comment out the line below when testing in SP/self-hosted MP server
if (hasInterface) exitWith {};

params ["_object"];

_pos = getPosATL _object;
_boundary = boundingBoxReal _object;
_boundaryMin = _boundary select 0;
_dir = getDir _object;

_markerName = format ["m_bnd%1", netId _object];

_marker = createMarker [_markerName, _pos];
_marker setMarkerShape "RECTANGLE";
_marker setMarkerColor "ColorGREY";
_marker setMarkerBrush "SolidFull";
_marker setMarkerSize [_boundaryMin select 0, _boundaryMin select 1];
_marker setMarkerDir _dir;