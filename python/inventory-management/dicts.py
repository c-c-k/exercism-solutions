"""Functions to keep track and alter inventory."""

Item = str
Quantity = int
Inventory = dict[Item, Quantity]
ItemsList = list[Item]
InventoryList = list[tuple[Item, Quantity]]


def create_inventory(items: ItemsList) -> Inventory:
    """Create a dict that tracks the amount (count) of each element on the `items` list.

    :param items: list of items to create an inventory from.
    :return: the inventory dictionary.
    """

    inventory: Inventory = {}
    for item in items:
        inventory[item] = inventory.get(item, 0) + 1
    return inventory


def add_items(inventory: Inventory, items: ItemsList) -> Inventory:
    """Add or increment items in inventory using elements from the items `list`.

    :param inventory: dictionary of existing inventory.
    :param items: list of items to update the inventory with.
    :return: the inventory updated with the new items.
    """

    new_inventory = create_inventory(items)
    for item, quantity in new_inventory.items():
        inventory[item] = inventory.get(item, 0) + quantity
    return inventory


def decrement_items(inventory: Inventory, items: ItemsList) -> Inventory:
    """Decrement items in inventory using elements from the `items` list.

    :param inventory: inventory dictionary.
    :param items: list of items to decrement from the inventory.
    :return: updated inventory with items decremented.
    """

    removed_inventory = create_inventory(items)
    for item, quantity in removed_inventory.items():
        if item in inventory:
            inventory[item] -= quantity
            if inventory[item] < 0:
                inventory[item] = 0
    return inventory


def remove_item(inventory: Inventory, item: str) -> Inventory:
    """Remove item from inventory if it matches `item` string.

    :param inventory:inventory dictionary.
    :param item:item to remove from the inventory.
    :return: updated inventory with item removed. Current inventory if item does not match.
    """

    inventory.pop(item, None)
    return inventory


def list_inventory(inventory: Inventory) -> InventoryList:
    """Create a list containing only available (item_name, item_count > 0) pairs in inventory.

    :param inventory:an inventory dictionary.
    :return: list of key, value pairs from the inventory dictionary.
    """

    inventory_list: InventoryList = []
    for item in sorted(inventory.keys()):
        quantity = inventory[item]
        if quantity > 0:
            inventory_list.append((item, quantity))
    return inventory_list
