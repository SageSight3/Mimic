# Tasks

## In Progress
* Design Map Specification IN PROGRESS - **MEDIUM**

## Incomplete

### High Priority
* Write Map Processor unit tests for map generation - TEMP VERSION DONE
* Digitize written documentation
* rim max added height adjustments for larger craters (if rad is to big, maybe multiplier should shrink by an order of magnitude)

### Medium Priority
* Once placeholder functionality's implemented, compile and benchmark a release build
* Commment code
* Implement basic eons
* refactor impact generator implementation an unit tests so that Crater::new() only needs crater depth and map to make a crater
    * will need to move crater_tile_coords()
* adjust setup's noise bounds/refactor to bring map height_range down to range closer to a desired range (maybe 100 - 300?)

### Low Priority

* investigate build_crater_rim() in Impact generator to see if can fix occasional crater where rim radius ends with height_to_add for decreasing slope = 2
* Refactor GUI for MVC and multithreading
    * look into enums
    * at bare minimum have map_processor.process_map() fork onto a different thread than the main app
    * GUI should be update over the different phases of process_map()
* Write App Unit tests
* Write unit tests for mark_gui_dirty and mark_gui_clean
* Look into multithreading
* fix gui image path print
* refactor crater_tiles_coords() in ImpactGenerator into multiple methods

## Completed

* Build base GUI
* Temporary map generation
    * create base map data structure, to outline base map data interpreter and image gernerator
* Refactor map generation to be in Map Processor
* Write Map Processor unit tests for map generation
* Refactor GUI MVC and multithreading to be cleaner
    * remove if left unused
* Temporary Image Interpreter
    * implement temporary image interpreter so a image generator can be implemented
* Image Generator
    * take interpreted map data and output an image
* Generate map image using placeholder generator
* review transient crater depth height calculation, may need to do the same for rim radius
* Build impact generator
* Build water map generation
* add overflow error impact_generator line 198, self.ejecta_volume += (new_height - old_height), could be causing more serious issues than initially realized
