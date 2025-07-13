"""Functions for compiling dishes and ingredients for a catering company."""

from sets_categories_data import VEGAN
from sets_categories_data import VEGETARIAN
from sets_categories_data import KETO
from sets_categories_data import PALEO
from sets_categories_data import OMNIVORE
from sets_categories_data import ALCOHOLS
from sets_categories_data import SPECIAL_INGREDIENTS

DIET_CATEGORIES = {
    "VEGAN": VEGAN,
    "VEGETARIAN": VEGETARIAN,
    "KETO": KETO,
    "PALEO": PALEO,
    "OMNIVORE": OMNIVORE,
}


def clean_ingredients(dish_name: str, dish_ingredients: list) -> tuple:
    """Remove duplicates from `dish_ingredients`.

    :param dish_name: dish name.
    :param dish_ingredients: list of dish ingredients.
    :return: tuple containing (dish_name, ingredient set).

    This function should return a `tuple` with the name of the dish as the first item,
    followed by the de-duped `set` of ingredients as the second item.
    """

    return dish_name, set(dish_ingredients)


def check_drinks(drink_name: str, drink_ingredients: list) -> str:
    """Append "Cocktail" (alcohol)  or "Mocktail" (no alcohol) to `drink_name`, based on `drink_ingredients`.

    :param drink_name: name of the drink.
    :param drink_ingredients: ingredients in the drink.
    :return: drink_name appended with "Mocktail" or "Cocktail".

    The function should return the name of the drink followed by
    "Mocktail" (non-alcoholic) and drink name followed by "Cocktail"
    (includes alcohol).

    """

    for ingredient in drink_ingredients:
        if ingredient in ALCOHOLS:
            return f"{drink_name} Cocktail"
    return f"{drink_name} Mocktail"


def categorize_dish(dish_name: str, dish_ingredients: set) -> str:
    """Categorize `dish_name` based on `dish_ingredients`.

    :param dish_name: dish to be categorized.
    :param dish_ingredients: ingredients for the dish.
    :return: the dish name appended with ": <CATEGORY>".

    This function should return a string with the `dish name: <CATEGORY>` (which meal category the dish belongs to).
    `<CATEGORY>` can be any one of  (VEGAN, VEGETARIAN, PALEO, KETO, or OMNIVORE).
    All dishes will "fit" into one of the categories imported from `sets_categories_data.py`

    """

    for dc_name, dc_ingredients in DIET_CATEGORIES.items():
        if dish_ingredients <= dc_ingredients:
            return f"{dish_name}: {dc_name}"
    unknown_ingredients = dish_ingredients - set.union(
        *DIET_CATEGORIES.values()
    )
    return f"ERROR: Unkown indgredients: {unknown_ingredients}"


def tag_special_ingredients(dish: tuple) -> tuple:
    """Compare `dish` ingredients to `SPECIAL_INGREDIENTS`.

    :param dish: tuple of (dish name, list of dish ingredients).
    :return: tuple containing (dish name, dish special ingredients).

    Return the dish name followed by the `set` of ingredients that
    require a special note on the dish description. For the purposes of
    this exercise, all allergens or special ingredients that need to be
    tracked are in the SPECIAL_INGREDIENTS constant imported from
    `sets_categories_data.py`.
    """

    return dish[0], SPECIAL_INGREDIENTS.intersection(dish[1])


def compile_ingredients(dishes: list) -> set:
    """Create a master list of ingredients.

    :param dishes: list of dish ingredient sets.
    :return: set of ingredients compiled from `dishes`.

    This function should return a `set` of all ingredients from all listed dishes.
    """

    return set.union(*dishes)


def separate_appetizers(dishes: list, appetizers: list) -> list:
    """Determine which `dishes` are designated `appetizers` and remove them.

    :param dishes: list of dish names.
    :param appetizers: list of appetizer names.
    :return: list of dish names that do not appear on appetizer list.

    The function should return the list of dish names with appetizer
    names removed.
    Either list could contain duplicates and may require de-duping.
    """

    return list(set(dishes).difference(appetizers))


def singleton_ingredients(dishes: list, intersection: set) -> set:
    """Determine which `dishes` have a singleton ingredient (an
    ingredient that only appears once across dishes).

    :param dishes: list of ingredient sets.
    :param intersection: one of the `<CATEGORY>_INTERSECTIONS` sets
    imported from `sets_categories_data.py`.
    :return: containing singleton ingredients.

    Each dish is represented by a `set` of its ingredients.

    Each `<CATEGORY>_INTERSECTIONS` is an `intersection` of all dishes
    in the category. `<CATEGORY>` can be any one of: (VEGAN, VEGETARIAN,
    PALEO, KETO, or OMNIVORE).

    The function should return a `set` of ingredients that only appear
    in a single dish.
    """

    return set.union(*dishes) - intersection
