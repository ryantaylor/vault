<?php

class CoH2Player {

	private $name;		// player name
	private $faction; 	// player faction
	private $bulletins;	// equipped intel bulletins
	private $commands;	// counter for commands issued
	
	public function __construct() {
		$this->name = null;
		$this->faction = null;
		$this->bulletins = array();
		$this->commands = 0;
	}
	
	public static function createWithName($name) {
		$player = new CoH2Player();
		$player->setName($name);
		return $player;
	}
	
	// get/set
	
	public function getName() 						{ return $this->name; }
	public function setName($name) 					{ $this->name = $name; }
	
	public function getFaction() 					{ return $this->faction; }
	public function setFaction($faction) 			{ $this->faction = $faction; }
	
	public function getBulletins() 					{ return $this->bulletins; }
	public function setBulletins(array $bulletins) 	{ $this->bulletins = $bulletins; }
	
	public function getCommands() 					{ return $this->commands; }
	public function setCommands($commands) 			{ $this->commands = $commands; }
}