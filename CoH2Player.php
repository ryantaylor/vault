<?php

class CoH2Player {

	private $name;			// player name
	private $id;			// internal player ID
	private $faction; 		// player faction
	private $team;			// player team
	private $position;		// starting position
	private $commanders;	// numeric IDs of commanders
	private $bulletins;		// equipped intel bulletins
	private $bulletinIds;	// numeric IDs of bulletins
	private $commands;		// counter for commands issued
	
	public function __construct() {
		$this->name = null;
		$this->faction = null;
		$this->team = 0;
		$this->position = 0;
		$this->commanders = array();
		$this->bulletins = array();
		$this->bulletinIds = array();
		$this->commands = 0;
	}
	
	public static function createWithName($name) {
		$player = new CoH2Player();
		$player->setName($name);
		return $player;
	}
	
	// get/set
	
	public function getName() 							{ return $this->name; }
	public function setName($name) 						{ $this->name = $name; }
	
	public function getId() 							{ return $this->id; }
	public function setId($id) 							{ $this->id = $id; }
	
	public function getFaction() 						{ return $this->faction; }
	public function setFaction($faction) 				{ $faction == 0 ? $this->faction = "Ostheer" : $this->faction = "Soviets"; }
	
	public function getTeam() 							{ return $this->team; }
	public function setTeam($team) 						{ $this->team = $team; }
	
	public function getPosition() 						{ return $this->position; }
	public function setPosition($position) 				{ $this->position = $position; }
	
	public function getCommanders() 					{ return $this->commanders; }
	public function getCommander($index)				{ return $this->commanders[$index]; }
	public function setCommanders(array $commanders) 	{ $this->commanders = $commanders; }
	public function addCommander($commander)			{ array_push($this->commanders, $commander); }
	
	public function getBulletins() 						{ return $this->bulletins; }
	public function getBulletin($index)					{ return $this->bulletins[$index]; }
	public function setBulletins(array $bulletins) 		{ $this->bulletins = $bulletins; }
	public function addBulletin($bulletin)				{ array_push($this->bulletins, $bulletin); }
	
	public function getBulletinIds() 					{ return $this->bulletinIds; }
	public function getBulletinId($index)				{ return $this->bulletinIds[$index]; }
	public function setBulletinIds(array $bulletinIds) 	{ $this->bulletinIds = $bulletinIds; }
	public function addBulletinId($bulletinId)			{ array_push($this->bulletinIds, $bulletinId); }
	
	public function getCommands() 						{ return $this->commands; }
	public function setCommands($commands) 				{ $this->commands = $commands; }
}