### GUI
Has:
* basic title and heading
* button to generate maps
* has observer that will be looking at map processor to update GUI during different phases of generation

### Map Specification

### Map Generator
* temporary implementation: randomly sets the height for each of the map's tiles to a value in a range from 1 to 256 
* temporary pass implementation for execution time testing

### Impact Generator
* customizeable generator parameters
* generates craters at random coordinates
* craters wrap around map
* digs tranisent crater
* builds crater rim
* for now, all craters are simple

### Water Map Generator
* covers a specified percentage of the map's total area in water

### Map Data/Image Interpreter
* interprets tile data and converts it into pixel color data, where one pixel = one tile
* height map interpretation is grayscale
* water map interpretation is blue

### Image Generator
* takes pixels from interpreted image data and builds an rgb image from them, then writes the image to a file
