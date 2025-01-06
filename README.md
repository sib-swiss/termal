---
title: TERMAL
subtitle: A TUI multiple alignment viewer
author: Thomas Junier
---

Summary
=======

Termal is a program for examining multiple sequence **al**ignments in a **term**inal.

Synopsis
========

Just pass your alignment as argument to the `termal` binary:

```bash
$ termal [options] <alignment>
```

Interface
=========

`termal` runs in a terminal and is entirely keyboard-driven. 

Key Bindings
------------

Features
========

Termal has the basic features you'd expect of an alignment viewer, such as:

* moving and jumping around
* zoomed in or zoomed-out (whole alignment) views
* consensus sequence
* residue colouring
* representation of conservation

Motivation
==========

Primary
-------

Multiple sequence alignments often reside on distant machines like HPC clusters, for two reasons:

1. They may require nontrivial resources to compute (this is especially true of large alignments);
1. They may serve as inputs to heavy-duty analyses like computing phylogenetic trees.

Like any other input data, it's a good idea to have a quick look at an alignment before it is fed to an analysis pipeline. There are many fine tools for doing this, but most of them have a graphical user interface, so they (usually) can't be used over SSH. Termal allows you to 

Secondary
---------

