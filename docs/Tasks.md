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

## In Progress
* Design Map Specification IN PROGRESS - **MEDIUM**

## Incomplete

### High Priority
* Write Map Processor unit tests for map generation - TEMP VERSION DONE - **HIGH**
* Image Generator - **HIGH**
    * take interpreted map data and output an image
    * at bare minimum have map_processor.process_map() fork onto a different thread than the main app
    * GUI should be update over the different phases of process_map()
* Generate map image using placeholder generator - **HIGH**
* Build crater generator - **HIGH**
* Build water map generator - **HIGH**
* Digitize written documentation - **HIGH**

### Medium Priority
* Once placeholder functionality's implemented, compile and benchmark a release build - **MEDIUM**
* Commment code - **MEDIUM**

### Low Priority

* Refactor GUI for MVC and multithreading - **LOW**
    * look into enums
* Write App Unit tests - **LOW**
* Write unit tests for mark_gui_dirty and mark_gui_clean - **LOW**
* Look into multithreading - **LOW**