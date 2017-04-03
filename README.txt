Graphyll
========

Graphyll is a minimalist static site generator and build automation
tool. It is like GNU Make, but without the logic.

At its present state, Graphyll is a toy project, not meant for
production use. But it is open source (see LICENSE), so do as 
you please.

Current version: 0.0.0


How it works
------------

Graphyll processes a data structure called a graph. Each graph is a
list of nodes. Each node contains three lists of strings: commands,
targets, and dependencies. The targets and dependencies are file
paths. The commands are executable system calls.

The nodes are processed in topological order. That is to say, if node
A has a target that is also a dependency of node B, then A will be
processed before B is processed.

Processing a node means to execute its commands if either (1) any of the 
dependency paths are newer than any of the target paths, or (2) any of
the target paths is missing.


As a command line utility
-------------------------

The graph data structure may be stored in a YAML file. For example,
consider the file `build.yaml`:

````````````````````````````````````````````````````````````````````````
nodes:
  - commands:
      - "pandoc src/post1.txt > dest/post1.html"
    dependencies:
      - "src/post1.txt"
      - "/usr/bin/pandoc"
    targets:
      - "dest/post2.txt"
````````````````````````````````````````````````````````````````````````

This graph has a single node that converts a single .txt file into
.html. But there is no limit to how many nodes a graph contains. And a
node may have any number of commands, dependencies, and targets. Even
zero.

To have graphyll execute this file, run

````````````````````````````````````````````````````````````````````````
$ graphyll build.yaml
````````````````````````````````````````````````````````````````````````
