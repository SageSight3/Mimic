# Tasks

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

## In Progress
* Design Map Specification IN PROGRESS - **MEDIUM**
* Build crater generator

## Incomplete

### High Priority
* Write Map Processor unit tests for map generation - TEMP VERSION DONE
* Build water map generation
* Digitize written documentation

### Medium Priority
* Once placeholder functionality's implemented, compile and benchmark a release build
* Commment code
* Implement basic eons
* review transient crater depth height calculation, may need to do the same for rim radius

### Low Priority

* Refactor GUI for MVC and multithreading
    * look into enums
    * at bare minimum have map_processor.process_map() fork onto a different thread than the main app
    * GUI should be update over the different phases of process_map()
* Write App Unit tests
* Write unit tests for mark_gui_dirty and mark_gui_clean
* Look into multithreading
* fix gui image path print
* refactor crater_tiles_coords() in ImpactGenerator into multiple methods