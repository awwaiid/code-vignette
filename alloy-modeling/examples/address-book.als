// Address Book Model
// Classic example modeling an address book with names, addresses, and groups

sig Name {}
sig Addr {}

sig Book {
    addr: Name -> lone Addr
}

pred add[b, b': Book, n: Name, a: Addr] {
    b'.addr = b.addr + n->a
}

pred del[b, b': Book, n: Name] {
    b'.addr = b.addr - n->Addr
}

pred lookup[b: Book, n: Name, a: Addr] {
    n->a in b.addr
}

// Assert that if we add and then delete, we get back to original
assert addThenDelete {
    all b, b', b'': Book, n: Name, a: Addr |
        add[b, b', n, a] and del[b', b'', n] implies
            b.addr = b''.addr or n->a in b.addr
}

run add for 3
check addThenDelete for 3
