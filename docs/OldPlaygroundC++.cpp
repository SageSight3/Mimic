#include <string>
#include <iostream>
#include <cstdlib>
#include "Map.h"
using namespace std;

Map::Map(int mapSize, int waterPer, int baseH) {
	size = mapSize;
	baseHeight = baseH;
	map = new Tile * *[size];
	for (int row = 0; row < size; ++row) {
		map[row] = new Tile * [size];
		for (int col = 0; col < size; ++col) {
			map[row][col] = new Tile(baseHeight);
		}
	}

	percentWater = waterPer;
}

Map::~Map() {
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			delete map[row][col];
		}
	}
}

void Map::resetMap() {
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			map[row][col]->setHeight(INF);
		}
	}
}

void Map::print() {
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			cout << map[row][col]->getHeight() << ' ';
		}
		cout << endl;
	}

	cout << "\r\n\r\n";
}

void Map::setMapBase() {
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			map[row][col]->setHeight(baseHeight);
		}
	}
}

void Map::printCraterMap() {
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			cout << map[row][col]->getCraterRing() << ' ';
		}
		cout << endl;
	}

	cout << "\r\n\r\n";
}

void Map::printFaultMap() {
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			if (map[row][col]->isFaultRidge()) {
				cout << '1';
			}
			else {
				cout << '0';
			}
			cout << ' ';
		}
		cout << endl;
	}

	cout << "\r\n\r\n";
}

void Map::printWaterMap() {
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			if (map[row][col]->isWater()) {
				cout << '0';
			}
			else {
				cout << '1';
			}
			cout << ' ';
		}
		cout << endl;
	}

	cout << "\r\n\r\n";
}

void Map::genWaterLevel() { //rethink how to set water?
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			if (map[row][col]->getHeight() <= waterLevel) {
				map[row][col]->setWater(true);
			}
		}
	}
}

void Map::craterGen(int rowCoord, int colCoord, int minDepth) {
	srand(time(nullptr));

	int center[2] = { rowCoord, colCoord };
	double craterDepth = rand() % baseHeight + minDepth + .01;
	double radMulti = 1;
	if (craterDepth > 1) {
		radMulti = (1 / (log(craterDepth))) * 5;
	}
	int radius = craterDepth * radMulti + (rand() % 2);

	//draw edge of crater WIP
	int centerRowOff = rowCoord - radius;
	int centerColOff = colCoord - radius;
	for (int row = 0; row < (2 * (radius + 1)); ++row) {
		if (((centerRowOff + row) >= 0) && ((centerRowOff + row) < size)) {
			for (int col = 0; col < (2 * (radius + 1)); ++col) {
				if (((centerColOff + col) >= 0) && ((centerColOff + col) < size)) {
					double dist = sqrt(pow(centerRowOff + row - rowCoord, 2) + pow(centerColOff + col - colCoord, 2));
					if (((radius - round(dist)) == 0)) {
						if (map[centerRowOff + row][centerColOff + col]->getHeight() == baseHeight) {
							map[centerRowOff + row][centerColOff + col]->setHeight(baseHeight + 1);
						}
						map[centerRowOff + row][centerColOff + col]->setCraterRing('X');
					}
				}
			}
		}
	}

	//fill in crater
	radius = radius - 1;
	for (int row = 0; row < (2 * (radius + 1)); ++row) {
		if (((centerRowOff + row) >= 0) && ((centerRowOff + row) < size)) {
			for (int col = 0; col < (2 * (radius + 1)); ++col) {
				if (((centerColOff + col) >= 0) && ((centerColOff + col) < size)) {
					double dist = sqrt(pow(centerRowOff + row - rowCoord, 2) + pow(centerColOff + col - colCoord, 2));
					if (((radius - round(dist)) >= 0)) {
						int tileDepth = baseHeight - round(sqrt(pow(radius - dist, 2) / radMulti));
						if (tileDepth < map[centerRowOff + row][centerColOff + col]->getHeight()) {
							if (tileDepth < 0) { tileDepth = 0; }
							map[centerRowOff + row][centerColOff + col]->setHeight(tileDepth);
						}
						if (map[centerRowOff + row][centerColOff + col]->getCraterRing() == 'U') {
							map[centerRowOff + row][centerColOff + col]->setCraterRing(' ');
						}
					}
				}
			}
		}
	}
	cout << craterDepth << endl << endl;
}

void Map::faultGen(int minHeight, int maxHeight) {
	//will randomly draw fault lines through the map, ranging in length from minLen to maxLen
		//rules for drawing line, new tile in fault can't be closer to the tile at the midpoint
		//of it and maybe can't be adjacent to the previous
	//the height of each tile in a line (peaks) will vary from a relative min and max based off minHeight and maxHeight
		//actual peak tile height be a shifted based off their one
	//from each peak, each side of the fault will randomly level out over 0 to 3 tiles, based on rng

	//for testing
	baseHeight = 5;
	setMapBase();
	srand(time(nullptr));

	/*steps
		draw faults
			pick startingpoint, expand from there, assign new boundaries and ridge if divfault forks, stop when fault
			plateIDs hashed from their divergent boundary's id, depending on what side of it their on
			ridge, boundary, and plate are tracker classes <--rethink
				intead of ridge having an id and two hash values for each side of it, hash values act as ridge ids?
				use a bool in tile to know if at plate's div boundary
				store plates (make plates as structs?) temporarily in a list to be deleted at end of faultGen to access and update their
				lengths, tiles will only know their ids separately in an int that corresonds to the id of a plate in the list
				tiles have a boundary object in them that holds their left and right ids? <-- used for hashing plate ids
		faults claim territory
			tiles adjeacent to divergeent oundaries will be assigned their respective plateIDs on first pass through
			then second pass through will mass claim the remaining tiles in the map
			claim prio goes to the plate with the larger divergent boundary
		plate movement defined by plateID
			movement rule brainstorm
				tiles will move relative to their plate's nearest div boundary coordinate?
	*/

	//draw fault

	int adjs[][2] = { {1,0},{1,1},{0,1},{-1,1},{-1,0},{-1,-1},{0,-1},{1,-1} };
	int rowCoord = rand() % size;
	int colCoord = rand() % size;
	map[rowCoord][colCoord]->setFaultRidge(true);

	int startPos = rand() % size;
}

void Map::mapGenReal(int minDepthC, int minHeightF, int maxHeightF) {
	heightMapGen(minDepthC);
	//generate biome map, will begin with watermapgen
}

void Map::heightMapGen(int minDepth) {

	//faults generate
	//craters generate
		//each tile has a chance to generate just use use rng rolls for number thrown then throw, will be way faster
	//plates move
	//hotspots generate

	//avg height calculation
	int tileHeightSum = 0;
	for (int row = 0; row < size; ++row) {
		for (int col = 0; col < size; ++col) {
			tileHeightSum = tileHeightSum + map[row][col]->getHeight();
		}
	}
	avgHeight = tileHeightSum / pow(size, 2);
}

void Map::createImg() {

}