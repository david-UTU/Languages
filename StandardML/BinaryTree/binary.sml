datatype 'label binarytree = Empty | Node of 'label binarytree * 'label binarytree;

fun lower(nil) = nil | lower(c::cs) = (Char.toLower c)::lower(cs);

fun less_than(a, b) = implode(lower(explode(a))) < implode(lower(explode(b)));

fun insert(x, Empty) = Node(x, Empty, Empty) | insert(x, T as Node(y, l, r)) =
    if x=y then T
    else if less_than(x, y) then Node(y, insert(x, l), r)
        else Node(y, l, insert(x, r));

fun list_to_tree(l) = Empty | list_to_tree(l::ls) = insert(l, list_to_tree(ls));

fun inOrder(Empty) = nil | inOrder(Node(x, l, r)) = inOrder(l) @ x :: inOrder(r);