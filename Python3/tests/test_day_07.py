from aoc2020.solutions.day_07 import SHINY_GOLD, BagsRules


def test_first_task() -> None:
    test_bags_rules_1 = """light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."""

    bags_graph = BagsRules.from_rules_text(test_bags_rules_1)
    assert bags_graph.count_bag_colors_containing(SHINY_GOLD) == 4


def test_second_task() -> None:
    test_bags_rules_1 = """light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."""

    bags_graph = BagsRules.from_rules_text(test_bags_rules_1)
    assert bags_graph.count_bags_inside(SHINY_GOLD) == 32


def test_second_task_extended() -> None:
    test_bags_rules_2 = """shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."""

    assert BagsRules.from_rules_text(
        test_bags_rules_2).count_bags_inside(SHINY_GOLD) == 126
