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

Example
=======

```
┌──┌───────────┌ data/aln4.pep - 16/16s (1.00) x 70/561c (0.12) - ────────────────────┐
│ 1│Abro_00865 │ELTDGFHLIIDALKLNGLNTIYGVPGIPITDFGRMAQAEGIRVLSFRHEQNAGYAASIAGFLTK-KPGVC│
│ 2│Aoxa_03215 │NLTDGFHALIDAFKKNDINNIYAVAGIPITDVLRLAVEREMKVIAFRHESNAGHAAAIAGYLTQ-KPGIC│
│ 3│Bden_01758 │ELTDGFHLVIDAMKLNGIDTIYNVPGIPITDLGRMAQAEGIRVLSFRHEQNAGYAAAIAGFLTK-KPGVC│
│ 4│Bphy_06740 │ETTDGFHLVIDALKLNDIKTIFGLVGIPITDLARLAQAEGMRFIGFRHEQHAGNAAAVSGYMTK-KPGIC│
│ 5│Bsp1_02714 │ELTDGFHLVIDALKLNGIDTVYGVPGIPITDLGRMAQAAGIRVLSFRHEQNAGYAASIAGFLTK-RPGVC│
│ 6│Cnec_05522 │AQTDGFHLVIDALKLNGIENIYGLPGIPVTDLARLAQANGMRVISFRHEQNAGNAAAIAGFLTQ-KPGVC│
│ 7│Cox1_00651 │AQTDGFHLVIDALKLNGIENIYGLPGIPVTDLARLAQANGMRVISFRHEQNAGNAAAIAGFLTQ-KPGVC│
│ 8│Cox2_00498 │AQTDGFHLVIDALKLNGIENIYGLPGIPVTDLARLAQANGMRVISFRHEQNAGNAAAIAGFLTQ-KPGVC│
│ 9│Mmas_01638 │ALTDGFHLVIDALKLNGINTIYDVPGIPISDLLRMAQAEGMRVISFRHEQNAGNAAAIAGFLTK-KPGVC│
│10│Msp1_01810 │ELTDGFHLVIDALKLNGIDTIYGVPGIPITDLGRMCQEEGMRVISFRHEQNAGNAAAIAGFLTK-KPGIC│
│11│Ofo1_01671 │ELTDGFHVLIDALKMNDIDTMYGVVGIPITNLARMWQDDGQRFYSFRHEQHAGYAASIAGYIEG-KPGVC│
│12│Ofo2_00503 │ELTDGFHVLIDALKMNDIDTMYGVVGIPITNLARMWQDDGQRFYSFRHEQHAGYAASIAGYIEG-KPGVC│
│13│Ofo3_01693 │ELTDGFHVLKDTLKLNGIDTMYGVVGIPITNLARLWEQDGQKFYSFRHEQHAGYAASIAGYIQGDKPGVC│
│14│Osp1_01723 │ELTDGFHVLKDVLKVNGIDTMYGVVGIPITNLARLWEQDGQKFYSFRHEQHAGYAASIAGYIHGDKPGVC│
│15│Osp2_01577 │ELTDGFHVLMDTLKMNDIDTMYGVVGIPITNLARLWEQDGQKFYSFRHEQHAGYAASIAGYIQGDKPGVC│
│16│Osp3_01912 │ELTDGFHVLIDALKMNDIDTMYGVVGIPITNLARLWQDDGQRFYSFRHEQHAGYAASIAGYIEG-KPGVC│
└──└───────────└🬹🬹🬹🬹🬹🬹🬹🬹🬹═════════════════════════════════════════════════════════════┘
│              │|    :    |    :    |    :    |    :    |    :    |    :    |    :    │
│Position      │0        10        20        30        40        50        60        7│
│Consensus     │elTDGFHlvIDALKlNgIdtiYGvpGIPITdLaRlaqadGmrviSFRHEQnAGyAAaIAGyltk-KPGVC│
│Conservation  │▅▅█████▄▄▆█▆▆█▄█▅▇▃▆▄▇▅▆▄███▆▇▅▆▄█▅▄▅▃▂▇▄▆▅▄▆████▇▅██▄██▅▇▇█▅▄▄▃▁▇██▆█│
└──────────────└──────────────────────────────────────────────────────────────────────┘
```
