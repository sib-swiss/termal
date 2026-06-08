# Main Key Bindings

Run `termal -b` to see this message if it doesn't fit on screen.

Arguments (counts, search patterns) and match index are shown in the modeline.

## Scrolling

[count]arrows: scroll by count columns/sequences;
    h,j,k,l are aliases for left, down, up, and right
[count]shift-arrows: scroll by count screenfuls
^,G,g,$: full left, bottom, top, full right

## Jumping (positions)

[count]| : jump to absolute column
[count]- : jump to absolute sequence (by current order)
[count]= : jump to absolute sequence (by file order)
[count]% : jump to vertical position (0–100%)
[count]# : jump to horizontal position (0–100%)

## Zooming

z,Z: next/previous zoom mode

## Searching

Sequence search:
    Search biological sequences. Alignment gaps are ignored during matching;
    matches are projected back onto the alignment.

Alignment search:
    Search displayed alignment rows. Gaps are treated as normal characters,
    so patterns can explicitly match '-' and regex operators count alignment columns.

Header search:
    Search sequence labels/headers.

fs<regexp><Ret> : sequence search
fa<regexp><Ret> : alignment search
fh<regexp><Ret> : sequence search

/ : shortcut for fs
" : shortcut for fh

[count]n,p  : next / previous match
Ret         : current match (e.g. after reordering)
Esc         : cancel search

## Setting the Reference

R<integer><Ret> : set reference to sequence <integer>
R<Ret>          : set reference to consensus

## Difference Modes

dd : diff mode - only show residues when different from the reference
dn : normal mode 

## Adjusting the Panes

[count]<,> : widen/narrow left pane by count columns
wl         : hide/show left pane        
wb         : hide/show bottom pane    
wf         : toggle fullscreen alignment pane 

## Video

s,S: next/previous color scheme
m,M: next/previous color map
i: toggle inverse/direct video

Try dark/inverse for best results (this is the default).

## Metrics and Orderings

o,O: next/previous ordering
t,T: next/previous sequence metric
c,C: next/previous column metric
