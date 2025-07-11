"""Functions to help play and score a game of blackjack.

How to play blackjack:    https://bicyclecards.com/how-to-play/blackjack/
"Standard" playing cards: https://en.wikipedia.org/wiki/Standard_52-card_deck
"""


def value_of_card(card: str) -> int:
    """Determine the scoring value of a card.

    :param card: given card.
    :return: value of a given card.  See below for values.

    1.  "J", "Q", or "K" (otherwise known as "face cards") = 10
    2.  "A" (ace card) = 1
    3.  "2" - "10" = numerical value.
    """

    if card == "A":
        return 1
    if card in "JQK":
        return 10
    return int(card)


def higher_card(card_one: str, card_two: str) -> str | tuple:
    """Determine which card has a higher value in the hand.

    :param card_one, card_two: cards dealt in hand.  See below for values.
    :return: resulting Tuple contains both cards if they are of equal value.

    1.  "J", "Q", or "K" (otherwise known as "face cards") = 10
    2.  "A" (ace card) = 1
    3.  "2" - "10" = numerical value.
    """

    card_one_val = value_of_card(card_one)
    card_two_val = value_of_card(card_two)

    if card_one_val > card_two_val:
        return card_one
    if card_one_val < card_two_val:
        return card_two
    return card_one, card_two


def value_of_ace(card_one: str, card_two: str) -> int:
    """Calculate the most advantageous value for the ace card.

    :param card_one, card_two: card dealt. See below for values.
    :return: either 1 or 11 value of the upcoming ace card.

    1.  "J", "Q", or "K" (otherwise known as "face cards") = 10
    2.  "A" (ace card) = 11 (if already in hand)
    3.  "2" - "10" = numerical value.
    """

    card_one_val = value_of_card(card_one)
    if card_one_val == 1:
        card_one_val = 11
    card_two_val = value_of_card(card_two)
    if card_two_val == 1:
        card_one_val = 11

    card_sum = card_one_val + card_two_val
    if card_sum > 10:
        return 1
    else:
        return 11


def is_blackjack(card_one: str, card_two: str) -> bool:
    """Determine if the hand is a 'natural' or 'blackjack'.

    :param card_one, card_two: card dealt. See below for values.
    :return: is the hand is a blackjack (two cards worth 21).

    1.  "J", "Q", or "K" (otherwise known as "face cards") = 10
    2.  "A" (ace card) = 11 (if already in hand)
    3.  "2" - "10" = numerical value.
    """

    hand = card_one + card_two
    return "A" in hand and (
        "K" in hand or "Q" in hand or "J" in hand or "10" in hand
    )


def can_split_pairs(card_one: str, card_two: str) -> bool:
    """Determine if a player can split their hand into two hands.

    :param card_one, card_two: cards dealt.
    :return: can the hand be split into two pairs? (i.e. cards are of the same value).
    """

    return value_of_card(card_one) == value_of_card(card_two)


def can_double_down(card_one: str, card_two: str) -> bool:
    """Determine if a blackjack player can place a double down bet.

    :param card_one, card_two: first and second cards in hand.
    :return: can the hand can be doubled down? (i.e. totals 9, 10 or 11 points).
    """

    card_one_val = value_of_card(card_one)
    card_two_val = value_of_card(card_two)
    card_sum = card_one_val + card_two_val

    return 9 <= card_sum <= 11
