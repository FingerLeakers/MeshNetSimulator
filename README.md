# MeshNetSimulator

Community networks such as [Freifunk](https://freifunk.net) struggle with scaling issues of their [MANET](https://en.wikipedia.org/wiki/Mobile_ad_hoc_network)s. The cause is management traffic caused by hundreds of nodes.

This is a simple simulator for sketching mesh network routing strategies in the hopes to find better approaches to mesh routing. Please note that this simulator does not virtualize a TCP/IP stack nor all characteristics of wireless connections. The dynamic nature of MANETs is also not (yet) covered by this simulator.

This a command line editor and simulator, but the output can be viewed with this [graph viewer](https://github.com/mwarning/GraphViewer/).

The project was formally written in JavaScript. Use the commit history if you want to look at that version.

Available are [basic information](docs/about_mesh_networking.md) about mesh routing and a collection of [examples](docs/node_examples.md).

## Related Software

[OMNeT++](https://www.omnetpp.org/): OMNeT++ is an extensible, modular, component-based C++ simulation library and framework, primarily for building network simulators.

[ns-3](https://www.nsnam.org/): ns-3 is a discrete-event network simulator for Internet systems, targeted primarily for research and educational use. 

[EMANE](https://github.com/adjacentlink/emane): Distributed wireless network emulation framework.

[MeshViewer](https://github.com/ffrgb/meshviewer): A visualization tool for mesh networks. Primarily used by Freifunk communities. Some code was used by this project.

## Various Links

- Primer on wireless mesh routing algorithms [Review on Routing Algorithms in Wireless Mesh Networks](http://www.ijcst.org/Volume3/Issue5/p15_3_5.pdf)

- [Review of Simulators for Wireless Mesh Network](http://dlibra.itl.waw.pl/dlibra-webapp/Content/1800/ISSN_1509-4553_3_2014_82.pdf)

- Understanding Mesh Networking ([Part I](https://inthemesh.com/archive/understanding-mesh-networking-part-i/), [Part II](https://inthemesh.com/archive/understanding-mesh-networking-part-ii/), [Slides](https://www.dropbox.com/s/wqksd8dmykev8x7/))

- [Ask Slashdot: Could We Build A Global Wireless Mesh Network?](https://ask.slashdot.org/story/17/04/29/2134234/ask-slashdot-could-we-build-a-global-wireless-mesh-network)

- Contains an overview of different mesh routing strategies: [From MANET To IETF ROLL Standardization: A Paradigm Shift in WSN Routing Protocols](http://www.cttc.es/publication/from-manet-to-ietf-roll-standardization-a-paradigm-shift-in-wsn-routing-protocols/)

## Various Scientific Papers

A [collection](docs/papers.md) of scientific papers somewhat related to Mobile Ad-Hoc Mesh Routing.
