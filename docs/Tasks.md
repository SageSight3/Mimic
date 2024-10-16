# Tasks

## Completed

* Build base GUI - DONE
* Temporary map generation - DONE
    * create base map data structure, to outline base map data interpreter and image gernerator
* Refactor map generation to be in Map Processor - DONE
* Write Map Processor unit tests for map generation - TEMP VERSION DONE
* Build MVC for GUI - MINIMUM DONE

## In Progress
* Design Map Specification IN PROGRESS - **MEDIUM**
* Refactor GUI MVC and multithreading to be cleaner - **LOW**
    * look into enums
    * remove if left unused

## Incomplete
* Write Map Processor unit tests for map generation - TEMP VERSION DONE - **HIGH**
* Build MVC for GUI - MINIMUM DONE - **LOW**
* Write App Unit tests - **LOW**
* Write unit tests for mark_gui_dirty and mark_gui_clean - **LOW**
* Temporary Image Interpreter - **HIGH**
    * implement temporary image interpreter so a image generator can be implemented
* Image Generator - **HIGH**
    * take interpreted map data and output an image
* Look into multithreading - **MEDIUM**
    * at bare minimum have map_processor.process_map() fork onto a different thread than the main app
    * GUI should be update over the different phases of process_map()
* Generate map image using placeholder generator - **HIGH**
* Once placeholder functionality's implemented, compile and benchmark a release build - **MEDIUM** 
* Build crater generator - **HIGH**
* Build water map generator - **HIGH**
* Digitize written documentation - **HIGH**
* Commment code - **MEDIUM**