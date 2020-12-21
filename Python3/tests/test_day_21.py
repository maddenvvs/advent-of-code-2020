from aoc2020.solutions.day_21 import count_allergen_free_ingredients, \
    parse_food_list, \
    find_allergen_list


def test_first_task():
    test_ingredients_text = """mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"""

    test_ingredients = parse_food_list(test_ingredients_text)

    assert count_allergen_free_ingredients(test_ingredients) == 5


def test_second_task():
    test_ingredients_text = """mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"""

    test_ingredients = parse_food_list(test_ingredients_text)

    assert find_allergen_list(test_ingredients) == "mxmxvkd,sqjhc,fvjkl"
