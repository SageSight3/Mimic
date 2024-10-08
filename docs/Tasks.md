# Tasks

## Completed

* Build base GUI - DONE
* Temporary map generation - DONE
    * create base map data structure, to outline base map data interpreter and image gernerator
* efactor map generation to be in Map Processor - DONE
* Write Map Processor unit tests for map generation - TEMP VERSION DONE
* Build MVC for GUI - MINIMUM DONE

## In Progress
* Design Map Specification IN PROGRESS

## Incomplete
* Write Map Processor unit tests for map generation - TEMP VERSION DONE
* Temporary Image Interpreter
    * implement temporary image interpreter so a image generator can be implemented
* Image Generator
    * take interpreted map data and output an image
* Look into multithreading
    * at bare minimum have map_processor.process_map() fork onto a different thread than the main app
    * GUI should be update over the different phases of process_map()
* Generate map image using placeholder generator
* Once placeholder functionality's implemented, compile and benchmark a release build 
* Build crater generator
* Build water map generator
* Digitize written documentation
* Commment code
