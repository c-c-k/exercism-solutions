"""Functions which helps the locomotive engineer to keep track of the train."""


def get_list_of_wagons(*wagons) -> list:
    """Return a list of wagons.

    :param: arbitrary number of wagons.
    :return: list of wagons.
    """
    return list(wagons)


def fix_list_of_wagons(each_wagons_id: list, missing_wagons: list) -> list:
    """Fix the list of wagons.

    :param each_wagons_id: the list of wagons.
    :param missing_wagons: the list of missing wagons.
    :return: list of wagons.
    """
    wagon_1, wagon_2, wagon_3, *rest = each_wagons_id
    return [wagon_3, *missing_wagons, *rest, wagon_1, wagon_2]


def add_missing_stops(route: dict, **stops) -> dict:
    """Add missing stops to route dict.

    :param route: the dict of routing information.
    :param stops: arbitrary number of stops.
    :return: updated route dictionary.
    """
    route["stops"] = [*stops.values()]
    return route


def extend_route_information(
    route: dict, more_route_information: dict
) -> dict:
    """Extend route information with more_route_information.

    :param route:the route information.
    :param more_route_information: extra route information.
    :return: extended route information.
    """
    return {**route, **more_route_information}


def fix_wagon_depot(wagons_rows: list[list[tuple]]) -> list[list[tuple]]:
    """Fix the list of rows of wagons.

    :param wagons_rows: the list of rows of wagons.
    :return: list of rows of wagons.
    """
    [[r1c1, r2c1, r3c1], [r1c2, r2c2, r3c2], [r1c3, r2c3, r3c3]] = wagons_rows

    return [[r1c1, r1c2, r1c3], [r2c1, r2c2, r2c3], [r3c1, r3c2, r3c3]]
