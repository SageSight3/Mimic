* Should map generation belong to the map module or be in it's own? **CLOSED**
* Should the feature generators be a part of the same module as the map generator, or should they each have their own module?
* How will the GUI interface with app? **CLOSED**
* Should eon be tracked by Strings or enums?
* Should the map specification be parsed by the map generator module or separately? 
* Should project be reorganized to be more in line with rust's reccommended/debugger implemented structute?
* Should map_attrs be moved back into the map module? 
* Should unit tests be refactored to be less thorough?
    * There's a lot of duplicate code from tests, largely/maybe fully from map_processor_test, that is maybe overly thorough, with a lot of it just serving to confirm that its own test cases' results matches the results of others.
* Should transient crater depths be shallower than they should be, or accurate to what they should be, even if they appear squished with the map's scales (1 height = 100 meters, tile = 5km x 5km)?
    * should the map's scale be changed to better accomodate craters, even if that means size increases significantly, or the map would only represent a section of a planet?
    * right now, full depth is implemented on current map scale, map may appear squished