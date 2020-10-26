# chipr8 
----
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
Chipr-8 is a rust project for reversing a chip-8 rom into an assembly listing. It will soon be converted to a module for other projects to implement.

### Usage
    ./chipr8 [rom_path]

### Sample Output - [Pong from Netpro2k](https://github.com/netpro2k/Chip8/blob/master/games/Pong.ch8)
![Example Assembly Listing](https://i.imgur.com/SroikuC.png)

### To Do
- Usage flags
- Heavy code refactoring into cleaner structures
- Indentify and display sprites from data sections
- Write unit testing

### Dissassembly Design
The recursive traversal dissassembly algorithm it uses attempts to follow all code paths rather than linearly trying to interpret every byte as instructions. 

![Recursive Traversal Algorithm, figure (b)](https://www.researchgate.net/profile/Rafael_Costa5/publication/259176378/figure/fig1/AS:286310834814976@1445273230010/Disassembly-Algorithms-a-linear-sweep-and-b-recursive-traversal.png)

 Costa, Rafael & Pirmez, Luci & Boccardo, Davidson & Carmo, Luiz & Machado, Raphael. (2012). [TinyObf: Code Obfuscation Framework for Wireless Sensor Networks](https://www.researchgate.net/publication/259176378_TinyObf_Code_Obfuscation_Framework_for_Wireless_Sensor_Networks). 10.13140/2.1.1721.4408.
 
 This currently does not support the <b>Bnnn</b> instruction yet as it cannot be easily analysed since it uses the v0 register to jump to a location in memory.
