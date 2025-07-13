"""Functions for tracking poker hands and assorted card tasks.

Python list documentation: https://docs.python.org/3/tutorial/datastructures.html
"""


def get_rounds(number: int) -> list:
    """Create a list containing the current and next two round numbers.

    :param number: current round number.
    :return: current round and the two that follow.
    """

    return [number, number + 1, number + 2]


def concatenate_rounds(rounds_1: list, rounds_2: list) -> list:
    """Concatenate two lists of round numbers.

    :param rounds_1: first rounds played.
    :param rounds_2: second set of rounds played.
    :return: all rounds played.
    """

    return rounds_1 + rounds_2


def list_contains_round(rounds: list, number: int) -> bool:
    """Check if the list of rounds contains the specified number.

    :param rounds: rounds played.
    :param number: round number.
    :return: was the round played?
    """

    return number in rounds


def card_average(hand: list) -> float:
    """Calculate and returns the average card value from the list.

    :param hand: cards in hand.
    :return: average value of the cards in the hand.
    """

    return sum(hand) / len(hand)


def approx_average_is_average(hand: list) -> bool:
    """Return if the (average of first and last card values) OR ('middle' card) == calculated average.

    :param hand: cards in hand.
    :return: does one of the approximate averages equal the `true average`?
    """

    average = card_average(hand)
    bounds_average = (hand[0] + hand[-1]) / 2
    median_average = hand[len(hand) // 2]
    return average == bounds_average or average == median_average


def average_even_is_average_odd(hand: list) -> bool:
    """Return if the (average of even indexed card values) == (average of odd indexed card values).

    :param hand: cards in hand.
    :return: are even and odd averages equal?
    """

    odd_average = card_average(hand[0::2])
    even_average = card_average(hand[1::2])
    return odd_average == even_average


def maybe_double_last(hand: list) -> list:
    """Multiply a Jack card value in the last index position by 2.

    :param hand: cards in hand.
    :return: hand with Jacks (if present) value doubled.
    """

    double_last_hand = list(hand)
    if double_last_hand[-1] == 11:
        double_last_hand[-1] = 22
    return double_last_hand
