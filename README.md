---
title: TERMAL
subtitle: A TUI multiple alignment viewer
author: Thomas Junier
---

Summary
=======

`termal` is a program for examining multiple sequence **al**ignments in a **term**inal.

Synopsis
========

Just pass your alignment as argument to `termal`:

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

It has the basic features you'd expect of an alignment viewer, such as:

* moving and jumping around
* zoomed in or zoomed-out (whole alignment) views
* consensus sequence
* residue colouring
* representation of conservation

Motivation
==========

Multiple sequence alignment often reside on distant machines like HPC clusters, for two reasons:
1. They may require nontrivial resources to compute (this is especially true of large alignments);
1. 


