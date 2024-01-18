## Piece Table

Span in piece table is called a piece, and it starts with a single piece that is divided
on a smaller pieces on insertion and deletions.

It contains 2 buffers:

    - original -> here we store a original file content, and it's readonly
    - add -> here we store all our insertions, and it's append only

Each piece should contain length, offset of the buffer, and a to which buffer it's
related.

Deletion handled by dividing piece into 2 pieces, one piece points to to items before
deleted item and 2-nd points to the items after deleted item. A special case
occurs when the deleted item is at the beginning or end of the piece in which case we simply adjust
the pointer or the piece length.

Insertion is handled by dividing piece into 3 pieces, one piece points to the old piece
before inserted item, 3-rd points to the old piece after inserted item. The inserted item
is appended to the add buffer and 2-nd piece should point to this item in the add buffer.
If several items are added in a row, the inserted items are combined into one.

### links

    - (DS for text sequences)[https://www.cs.unm.edu/~crowley/papers/sds.pdf]

### Dict

    - **item** -> a basic element aka character;
    - **sequence** -> ordered list of **items**, it keeps items in buffers;
    - **buffer** -> list of addressed memory locations. It contains items from the sequence, but not necessarily in the same order;
    - **span** -> string of items;
    - **descriptor** -> pointer to a span, may be skipped as a buffer may act as descriptor;
