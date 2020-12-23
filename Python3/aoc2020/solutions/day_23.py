from __future__ import annotations

from typing import List, Tuple, TypeVar, Generic, Optional, cast

from .solution import Solution

T = TypeVar("T")


class ClassyDoublyLinkedList:
    __slots__ = "prev", "next"

    def __init__(self, size: int):
        self.next = [i+1 for i in range(size)]
        self.prev = [i-1 for i in range(size)]

        self.next[-1] = 0
        self.prev[0] = size-1

    def remove_node(self, node: int):
        self.next[self.prev[node]] = self.next[node]
        self.prev[self.next[node]] = self.prev[node]

    def insert_after(self, after: int, node: int):
        self.next[node] = self.next[after]
        self.next[after] = node
        self.prev[self.next[node]] = node
        self.prev[node] = after


class Node(Generic[T]):
    __slots__ = "value", "_next", "_prev"

    value: T
    _next: Optional[Node[T]]
    _prev: Optional[Node[T]]

    def __init__(self,
                 value: T,
                 _next: Optional[Node[T]] = None,
                 _prev: Optional[Node[T]] = None):
        self.value = value
        self._next = _next
        self._prev = _prev


class DoublyLinkedList(Generic[T]):
    __slots__ = "head"

    head: Optional[Node[T]]

    def __init__(self):
        self.head = None

    def __iter__(self):
        if self.head is not None:
            yield self.head.value

            next_node = self.head._next
            while next_node != self.head:
                yield next_node.value
                next_node = next_node._next

    def add(self, value: T) -> Node[T]:
        new_node = Node(value)

        if self.head is None:
            self.head = new_node
            self.head._next = self.head._prev = self.head
            return self.head

        tail = self.head._prev

        assert tail is not None

        tail._next = new_node
        new_node._prev = tail
        new_node._next = self.head
        self.head._prev = new_node

        return new_node

    def remove_node(self, node: Node[T]) -> Node[T]:
        if self.head is None:
            raise ValueError("Cannot remove node from empty list")

        if self.head._next == self.head:
            assert node == self.head

            self.head._next = self.head._prev = None
            self.head = None

            return node

        if self.head == node:
            new_head = self.head._next

            assert new_head is not None
            new_head._prev = self.head._prev

            assert self.head._prev is not None
            self.head._prev._next = new_head

            self.head._next = self.head._prev = None

            self.head = new_head

        else:
            assert node._prev is not None
            assert node._next is not None

            node._prev._next = node._next
            node._next._prev = node._prev

            node._next = node._prev = None

        return node

    def insert_after(self, after: Node[T], node: Node[T]):
        next_node = after._next

        assert next_node is not None

        after._next = node
        node._prev = after
        node._next = next_node
        next_node._prev = node


def parse_cups(cups_text: str) -> List[int]:
    return [int(c, base=10) for c in cups_text]


def simulate_game_classy_way(cups: List[int], moves: int) -> List[int]:
    linked_list = ClassyDoublyLinkedList(len(cups))
    val2node = [0 for _ in range(len(cups))]

    for i, cup in enumerate(cups):
        val2node[cup-1] = i

    curr_node = 0

    for _ in range(moves):
        removed_nodes = (
            linked_list.next[curr_node],
            linked_list.next[linked_list.next[curr_node]],
            linked_list.next[linked_list.next[linked_list.next[curr_node]]],
        )

        for node in removed_nodes:
            linked_list.remove_node(node)

        value_to_append_after = cups[curr_node] - 1
        if value_to_append_after == 0:
            value_to_append_after = len(cups)

        while value_to_append_after == cups[removed_nodes[0]] \
                or value_to_append_after == cups[removed_nodes[1]] \
                or value_to_append_after == cups[removed_nodes[2]]:
            value_to_append_after -= 1
            if value_to_append_after == 0:
                value_to_append_after = len(cups)

        node_to_insert_after = val2node[value_to_append_after - 1]
        for node in removed_nodes:
            linked_list.insert_after(node_to_insert_after, node)
            node_to_insert_after = node

        curr_node = linked_list.next[curr_node]

    new_cups = [0 for _ in range(len(cups))]
    next_node = 0

    for i in range(len(cups)):
        new_cups[i] = cups[next_node]
        next_node = linked_list.next[next_node]

    return new_cups


def simulate_game(cups: List[int], moves: int) -> List[int]:
    linked_list = DoublyLinkedList[int]()
    val2node = {}

    for cup in cups:
        val2node[cup] = linked_list.add(cup)

    curr_node = val2node[cups[0]]

    for _ in range(moves):

        removed_values = []
        removed_nodes = []
        for _ in range(3):
            removed_node = linked_list.remove_node(
                cast(Node[int], curr_node._next))
            removed_nodes.append(removed_node)
            removed_values.append(removed_node.value)

        value_to_append_after = curr_node.value - 1
        if value_to_append_after == 0:
            value_to_append_after = len(cups)

        while value_to_append_after in removed_values:
            value_to_append_after -= 1
            if value_to_append_after == 0:
                value_to_append_after = len(cups)

        node_to_insert_after = val2node[value_to_append_after]
        for node in removed_nodes:
            linked_list.insert_after(node_to_insert_after, node)
            node_to_insert_after = node

        curr_node = cast(Node[int], curr_node._next)

    return list(linked_list)


def find_1_based_label(cups: List[int]) -> str:
    buffer = cups + cups
    one_idx = buffer.index(1)
    int_label = buffer[one_idx + 1: one_idx + 1 + len(cups) - 1]

    return "".join(map(str, int_label))


def count_1_based_label_after(cups: List[int], moves: int) -> str:
    new_cups = simulate_game_classy_way(cups, moves)

    return find_1_based_label(new_cups)


def count_product_of_two_labels_after_1(cups: List[int]) -> int:
    all_cups = cups + list(range(len(cups) + 1, 1_000_001))
    new_cups = simulate_game_classy_way(all_cups, 10_000_000)
    one_idx = new_cups.index(1)

    return new_cups[(one_idx + 1) % len(new_cups)] * new_cups[(one_idx + 2) % len(new_cups)]


class Day23(Solution):

    def first_task(self, cups_text: str) -> str:
        cups = parse_cups(cups_text)

        return str(count_1_based_label_after(cups, 100))

    def second_task(self, cups_text: str) -> str:
        cups = parse_cups(cups_text)

        return str(count_product_of_two_labels_after_1(cups))
