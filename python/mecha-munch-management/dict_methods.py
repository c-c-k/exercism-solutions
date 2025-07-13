"""Functions to manage a users shopping cart items."""

from typing import Iterable

OUT_OF_STOCK = "Out of Stock"
STORE_INVENTORY_STOCK_INDEX = 0

ItemName = str
RecipeName = str
AisleName = str
RefrigirateFlag = bool
Quantity = int
QuantityStatus = int | str
ShoppingCartDict = dict[ItemName, Quantity]
ItemsIterable = Iterable[ItemName]
RecipesDict = dict[RecipeName, ShoppingCartDict]
RecipeTuple = tuple[RecipeName, ShoppingCartDict]
RecipesIterable = Iterable[RecipeTuple]
AislesDict = dict[ItemName, tuple[AisleName, RefrigirateFlag]]
FulfillmentDict = dict[ItemName, tuple[Quantity, AisleName, RefrigirateFlag]]
StoreInventoryDict = dict[
    ItemName, tuple[QuantityStatus, AisleName, RefrigirateFlag]
]


def add_item(
    current_cart: ShoppingCartDict, items_to_add: ItemsIterable
) -> ShoppingCartDict:
    """Add items to shopping cart.

    :param current_cart: the current shopping cart.
    :param items_to_add: items to add to the cart.
    :return: the updated user cart dictionary.
    """

    for item in items_to_add:
        current_cart[item] = current_cart.get(item, 0) + 1
    return current_cart


def read_notes(notes: ItemsIterable) -> ShoppingCartDict:
    """Create user cart from an iterable notes entry.

    :param notes: iterable of items to add to cart.
    :return: a user shopping cart dictionary.
    """

    return add_item({}, notes)


def update_recipes(
    ideas: RecipesDict, recipe_updates: RecipesIterable
) -> RecipesDict:
    """Update the recipe ideas dictionary.

    :param ideas: The "recipe ideas" dict.
    :param recipe_updates: with updates for the ideas section.
    :return: updated "recipe ideas" dict.
    """

    for recipe in recipe_updates:
        ideas[recipe[0]] = recipe[1]
    return ideas


def sort_entries(cart: ShoppingCartDict) -> ShoppingCartDict:
    """Sort a users shopping cart in alphabetically order.

    :param cart: a users shopping cart dictionary.
    :return: users shopping cart sorted in alphabetical order.
    """

    return dict(sorted(cart.items()))


def send_to_store(
    cart: ShoppingCartDict, aisle_mapping: AislesDict
) -> FulfillmentDict:
    """Combine users order to aisle and refrigeration information.

    :param cart: users shopping cart dictionary.
    :param aisle_mapping: aisle and refrigeration information dictionary.
    :return: fulfillment dictionary ready to send to store.
    """

    fulfillment_cart: FulfillmentDict = {}
    for item_name, quantity in cart.items():
        fulfillment_cart[item_name] = [quantity, *aisle_mapping[item_name]]
    return dict(reversed(sorted(fulfillment_cart.items())))


def update_store_inventory(
    fulfillment_cart: FulfillmentDict, store_inventory: StoreInventoryDict
) -> StoreInventoryDict:
    """Update store inventory levels with user order.

    :param fulfillment_cart: fulfillment cart to send to store.
    :param store_inventory: store available inventory
    :return: store_inventory updated.
    """

    for item_name, (quantity, _, _) in fulfillment_cart.items():
        item_stock = store_inventory[item_name][STORE_INVENTORY_STOCK_INDEX]
        if item_stock == OUT_OF_STOCK:
            continue
        item_stock -= quantity
        if item_stock <= 0:
            item_stock = OUT_OF_STOCK
        store_inventory[item_name][STORE_INVENTORY_STOCK_INDEX] = item_stock
    return store_inventory
