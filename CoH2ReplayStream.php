<?php

class CoH2ReplayStream {

	private $stream;	// replay filestream
	
	public function __construct($file) {
		$this->stream = fopen($file, "r");
	}
	
	public function close() {
		fclose($this->stream);
	}
	
	public function getPosition() {
		return ftell($this->stream);
	}
	
	public function readUInt32() {
		$val = fread($this->stream, 4);
		$end = $val{3} . $val{2} . $val{1} . $val{0};
		//return print_r($this->bchexdec(bin2hex($end)), true);
		return $this->bchexdec(bin2hex($end));
	}
	
	public function readUInt16() {
		$val = fread($this->stream, 2);
		$end = $val{1} . $val{0};
		//return print_r($this->bchexdec(bin2hex($end)), true);
		return $this->bchexdec(bin2hex($end));
	}
	
	public function readText($length) {
		$val = fread($this->stream, $length);
		//return print_r($val, true);
		return $val;
	}
	
	public function skip($numBytes = 4) {
		fseek($this->stream, $numBytes, SEEK_CUR);
	}
	
	public function seek($position) {
		fseek($this->stream, $position, SEEK_SET);
	}
	
	private function bchexdec($hex) {
		if(strlen($hex) == 1) {
			return hexdec($hex);
		} else {
			$remain = substr($hex, 0, -1);
			$last = substr($hex, -1);
			return bcadd(bcmul(16, $this->bchexdec($remain)), hexdec($last));
		}
	}
}