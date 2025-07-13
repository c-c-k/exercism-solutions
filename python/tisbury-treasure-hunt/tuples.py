"""Functions to help Azara and Rui locate pirate treasure."""

AZARA_COORDINATE_INDEX = 1
RUI_COORDINATE_INDEX = 1


def get_coordinate(record: tuple[str, str]) -> str:
    """Return coordinate value from a tuple containing the treasure name, and treasure coordinate.

    :param record: tuple with a (treasure, coordinate) pair.
    :return: the extracted map coordinate.
    """

    return record[AZARA_COORDINATE_INDEX]


def convert_coordinate(coordinate: str) -> tuple:
    """Split the given coordinate into tuple containing its individual components.

    :param coordinate: a string map coordinate
    :return: the string coordinate split into its individual components.
    """

    return tuple(coordinate)


def compare_records(
    azara_record: tuple[str, str],
    rui_record: tuple[str, tuple[str, str], str],
) -> bool:
    """Compare two record types and determine if their coordinates match.

    :param azara_record: a (treasure, coordinate) pair.
    :param rui_record: a (location, tuple(coordinate_1, coordinate_2), quadrant) trio.
    :return: do the coordinates match?
    """

    return (
        convert_coordinate(get_coordinate(azara_record))
        == rui_record[RUI_COORDINATE_INDEX]
    )


def create_record(
    azara_record: tuple[str, str],
    rui_record: tuple[str, tuple[str, str], str],
) -> tuple | str:
    """Combine the two record types (if possible) and create a combined record group.

    :param azara_record:a (treasure, coordinate) pair.
    :param rui_record:a (location, coordinate, quadrant) trio.
    :return: the combined record (if compatible), or the string "not a match" (if incompatible).
    """

    if compare_records(azara_record, rui_record):
        return azara_record + rui_record
    return "not a match"


def clean_up(combined_records: tuple) -> str:
    """Clean up a combined record group into a multi-line string of single records.

    :param combined_record_group:everything from both participants.
    :return: everything "cleaned", excess coordinates and information are removed.

    The return statement should be a multi-lined string with items separated by newlines.

    (see HINTS.md for an example).
    """

    cleaned_records: list[str] = []
    for combined_record in combined_records:
        cleaned_records.append(
            str((combined_record[0],) + combined_record[2:])
        )
    cleaned_records.append("")
    return "\n".join(cleaned_records)
