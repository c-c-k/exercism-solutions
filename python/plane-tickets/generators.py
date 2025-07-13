"""Functions to automate Conda airlines ticketing system."""

from typing import Generator


def generate_seat_letters(number: int) -> Generator:
    """Generate a series of letters for airline seats.

    :param number: total number of seat letters to be generated.
    :return: generator that yields seat letters.

    Seat letters are generated from A to D.
    After D it should start again with A.

    Example: A, B, C, D

    """

    count = 0
    while True:
        for seat_letter in "ABCD":
            yield seat_letter
            count += 1
            if count >= number:
                return


def generate_seats(number: int) -> Generator:
    """Generate a series of identifiers for airline seats.

    :param number: total number of seats to be generated.
    :return: generator that yields seat numbers.

    A seat number consists of the row number and the seat letter.

    There is no row 13.
    Each row has 4 seats.

    Seats should be sorted from low to high.

    Example: 3C, 3D, 4A, 4B

    """

    count = 0
    row = 0
    seat_letters = generate_seat_letters(number)
    while True:
        row += 1 if row != 12 else 2
        for _ in range(4):
            yield str(row) + next(seat_letters)
            count += 1
            if count >= number:
                return


def assign_seats(passengers: list[str]) -> dict:
    """Assign seats to passengers.

    :param passengers: a list of strings containing names of passengers.
    :return: with the names of the passengers as keys and seat numbers as values.

    Example output: {"Adele": "1A", "Björk": "1B"}

    """

    seats = generate_seats(len(passengers))
    return {passenger: seat for passenger, seat in zip(passengers, seats)}


def generate_codes(seat_numbers: list[str], flight_id: str) -> Generator:
    """Generate codes for a ticket.

    :param seat_numbers: list of seat numbers.
    :param flight_id: string containing the flight identifier.
    :return: generator that yields 12 character long ticket codes.

    """

    unused_id_chars = 12 - len(flight_id)
    for seat_number in seat_numbers:
        yield "".join(
            seat_number
            + flight_id
            + "0" * (unused_id_chars - len(seat_number))
        )
